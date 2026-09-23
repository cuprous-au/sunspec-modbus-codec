use heck::{ToPascalCase, ToSnakeCase};
use std::collections::HashSet;

use crate::naming::{Name, NameRef, NameTable, Named};
use crate::sunspec_schema::{
    Group, GroupCount, Point, PointAccess, PointMandatory, PointType, SunspecModel,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodegenFeature {
    String,
}

pub type DocLines = Vec<String>;

#[derive(Clone)]
pub struct ResolvedType {
    pub rust_type: String,
    pub c_type: String,
    pub size: u16,
    pub writer_function_name: String,
    pub reader_function_name: String,
    pub writer_allow_offset: bool,
    pub enum_repr: Option<String>,
    pub array_length: Option<i64>,
    pub cast_from_c: Option<fn(&str) -> String>,
}

#[derive(Clone, PartialEq)]
pub enum PointValueType {
    Adapter,
    StaticValue(String),
    ModelLength,
}

#[derive(Clone)]
pub struct ResolvedPoint {
    pub name: NameRef,
    pub internal_name: String,
    pub value_type: PointValueType,
    pub point_type: ResolvedType,
    pub access: PointAccess,
    pub mandatory: PointMandatory,
    pub block_indices: Vec<BlockIndex>,
    pub doc: DocLines,
    pub size: u16,
}

impl Named for ResolvedPoint {
    fn name_ref(&self) -> &NameRef {
        &self.name
    }
}

#[derive(Clone)]
pub struct BlockIndex {
    pub index_name: String,
}

#[derive(Clone)]
pub struct ResolvedEnum {
    pub name: NameRef,
    /// This enum's bare SunSpec type name (e.g. `St`), before the model prefix baked into
    /// [`Self::name`] to keep it globally unique for the generated C header. `generate_enum`
    /// re-exposes it as a `pub type` alias to `name`, so Rust code - which doesn't share C's
    /// flat, single-namespace problem - can still use the short, spec-matching name.
    pub short_name_pascal_case: String,
    pub discriminant_type: String,
    pub values: Vec<EnumValue>,
}

impl Named for ResolvedEnum {
    fn name_ref(&self) -> &NameRef {
        &self.name
    }
}

#[derive(Clone)]
pub struct EnumValue {
    pub name_pascal_case: String,
    pub discriminant: String,
    pub doc: DocLines,
}

pub struct CountPoint {
    pub point: ResolvedPoint,
}

pub struct ResolvedModel {
    pub name: NameRef,
    pub features: HashSet<CodegenFeature>,
    pub model_number: u16,
    pub group: ResolvedGroup,
    pub count_points: Vec<CountPoint>,
}

impl Named for ResolvedModel {
    fn name_ref(&self) -> &NameRef {
        &self.name
    }
}

#[derive(Clone)]
pub struct ResolvedGroup {
    pub name: NameRef,
    pub name_short: String,
    /// This group's own short name alone, ignoring any derived prefixes.
    /// Used for identifiers that only ever need to be unique within their own function,
    /// closure, or enum variant - a loop index, a getter argument, a `Point` field
    pub local_name_short: String,
    /// The number of registers occupied by this group's own [`Self::static_points`] alone - not
    /// a fixed subgroup's, and not a repeating one's variable contribution. Codegen sums this
    /// transitively through fixed subgroups on demand where it needs the total for a whole
    /// subtree; see `total_static_size` in `code_generation`.
    pub static_size: u16,
    /// This group's own direct points, in schema order - never a fixed subgroup's, which has
    /// exactly one instance and so shares this group's own struct/impl, but keeps its points on
    /// its own `Subgroup` so the wire position of each subgroup - fixed or repeating - stays
    /// visible to address generation. Consumers that don't care about wire position (name-based
    /// codegen) use [`Self::flattened_points`]/[`Self::flattened_repeats`] instead.
    pub static_points: Vec<ResolvedPoint>,
    pub enums: Vec<ResolvedEnum>,
    /// Every direct subgroup, fixed and repeating alike, in schema (wire) order. A repeating
    /// subgroup's block has a variable runtime length, so anything that follows it on the wire -
    /// including a later *fixed* sibling's points - only has a runtime-computed address; keeping
    /// fixed and repeating subgroups in one ordered list (distinguished by `Subgroup::count`) is
    /// what lets address generation get that right.
    pub subgroups: Vec<Subgroup>,
    pub count_points: Vec<NameRef>,
    pub writable: bool,
}

/// One of a group's direct subgroups, in schema order.
#[derive(Clone)]
pub struct Subgroup {
    /// `None` for a fixed subgroup (exactly one instance, inline within the parent); `Some` for
    /// a repeating one (an array of instances), naming the point holding its runtime count.
    pub count: Option<ResolvedPoint>,
    pub group: Box<ResolvedGroup>,
}

impl Named for ResolvedGroup {
    fn name_ref(&self) -> &NameRef {
        &self.name
    }
}

impl ResolvedGroup {
    /// Every point that belongs on this group's own struct/trait impl directly, in schema order -
    /// this group's own [`Self::static_points`], plus (transitively) every fixed subgroup's,
    /// since a fixed subgroup has exactly one instance and never gets a struct of its own. Used
    /// by consumers (stateful/callback codegen) that address fields by name and don't need to
    /// distinguish wire position.
    pub fn flattened_points(&self) -> Vec<&ResolvedPoint> {
        self.static_points
            .iter()
            .chain(
                self.subgroups
                    .iter()
                    .filter(|subgroup| subgroup.count.is_none())
                    .flat_map(|subgroup| subgroup.group.flattened_points()),
            )
            .collect()
    }

    /// Every repeating subgroup in this group's subtree that isn't itself nested inside another
    /// repeating subgroup - this group's own direct repeating children, plus (transitively) any
    /// nested inside a fixed subgroup (e.g. model 708's `Crv`, whose `MustTrip`/`MayTrip`/
    /// `MomCess` fixed subgroups each wrap their own repeating `Pt` array), in schema order.
    pub fn flattened_repeats(&self) -> Vec<(&ResolvedPoint, &ResolvedGroup)> {
        self.subgroups
            .iter()
            .flat_map(|subgroup| match &subgroup.count {
                Some(count_point) => vec![(count_point, subgroup.group.as_ref())],
                None => subgroup.group.flattened_repeats(),
            })
            .collect()
    }
}

fn cast_string_from_c(value: &str) -> String {
    format!("unsafe {{ CStr::from_ptr({}) }}", value)
}
fn cast_eui48_from_c(value: &str) -> String {
    format!("unsafe {{ &*({} as *const [u8; 6]) }}", value)
}
fn cast_ipv6_from_c(value: &str) -> String {
    format!("unsafe {{ &*({} as *const [u16; 8]) }}", value)
}

/// Everything needed to name a group or point relative to its ancestors, threaded down through
/// `resolve_group`'s recursion. Three of the four fields are rebuilt at each level (see each
/// one); `model_file_name` alone stays constant for the whole model.
#[derive(Clone)]
pub(crate) struct NamingContext {
    /// Chain of ancestor labels, prepended by each fixed subgroup as resolution descends; reset
    /// to `""` on entering a repeating group (or the top-level model group), since that gets its
    /// own generated scope and so needs no ancestor qualification. Example: descending into
    /// model 708's `MustTrip` then `Crv` builds `"Crv MustTrip"`, keeping its points distinct
    /// from `MayTrip`'s own identically-named `Crv`.
    identifier_text: String,
    /// The short-form counterpart to `identifier_text`, used for compact names like a fixed
    /// group's array-constant prefix. Inherits the parent's own `name_short` through nested
    /// fixed groups, but resets to `""` on entering a repeating group or the top-level group.
    /// Example: `"must_trip"`, naming model 708's `MUST_TRIP_POINTS` constant.
    short_text: String,
    /// Set only when directly inside a repeating group, to that group's own bare schema name -
    /// disambiguates its points from another repeating group's same-named ones, since
    /// `identifier_text` resets instead of accumulating for repeating groups. Example:
    /// `Some("Pt")` for model 708's `Crv`'s point array, giving point names like `"Crv Pt
    /// DeptRef"`.
    name_prefix: Option<String>,
    /// The SunSpec model JSON file's own name (e.g. `"model_708"`) - unlike the other three
    /// fields, never changes as this descends into subgroups. Used only to build the
    /// model-prefixed FFI name (e.g. `Model708Ena`) that keeps the generated C header's flat
    /// namespace collision-free; irrelevant to Rust-side naming, which is already unique per
    /// model without it.
    model_file_name: String,
}

fn resolve_point_type(
    point: &Point,
    features: &mut HashSet<CodegenFeature>,
    naming: &NamingContext,
) -> ResolvedType {
    let base_type = match point.type_ {
        PointType::Uint16
        | PointType::Raw16
        | PointType::Acc16
        | PointType::Bitfield16
        | PointType::Pad
        | PointType::Count
        | PointType::Enum16 => "u16".to_string(),
        PointType::Uint32 | PointType::Acc32 | PointType::Bitfield32 | PointType::Enum32 => {
            "u32".to_string()
        }
        PointType::Uint64 | PointType::Acc64 | PointType::Bitfield64 => "u64".to_string(),
        PointType::Int16 | PointType::Sunssf => "i16".to_string(),
        PointType::Int32 => "i32".to_string(),
        PointType::Int64 => "i64".to_string(),
        PointType::Float32 => "f32".to_string(),
        PointType::Float64 => "f64".to_string(),
        PointType::String => {
            features.insert(CodegenFeature::String);
            "string".to_string()
        }
        PointType::Ipaddr => "u32".to_string(),
        PointType::Ipv6addr => "Ipv6Addr".to_string(),
        PointType::Eui48 => "eui48".to_string(),
    };

    let is_enum = (point.type_ == PointType::Enum16 || point.type_ == PointType::Enum32)
        && !point.symbols.is_empty();

    let rust_type = match point.type_ {
        PointType::String => "&CStr".to_string(),
        PointType::Eui48 => "&[u8; 6]".to_string(),
        PointType::Ipv6addr => "&[u16; 8]".to_string(),
        // The bare, spec-matching name (e.g. `St`) - safe here since this is only used in
        // Rust-facing signatures (getters/setters/trait methods), which cbindgen never sees and
        // which Rust's own module system already keeps distinct model-to-model. It's a `pub
        // type` alias to the model-prefixed declaration `c_type` names below - see
        // `generate_enum`.
        _ if is_enum => point.name.to_pascal_case(),
        _ => base_type.clone(),
    };

    // Enum type names are prefixed with their model (e.g. `Model2St`, not `St`) here because
    // cbindgen flattens every model's types into one C namespace by their bare Rust name -
    // unprefixed, the many models that happen to reuse a short SunSpec type name like `St` or
    // `Ena` for their own, distinct enum would collide when more than one is compiled into the
    // same header. This is the type FFI-facing (`#[repr(C)]`) struct fields reference, so it
    // must be the actual declaration's name, not the `rust_type` alias above. See `resolve_enum`,
    // which names the enum declaration itself the same way.
    let c_type = match point.type_ {
        PointType::String => "c_char".to_string(),
        PointType::Eui48 => "u8".to_string(),
        PointType::Ipv6addr => "u16".to_string(),
        _ if is_enum => format!(
            "{}{}",
            naming.model_file_name.to_pascal_case(),
            point.name.to_pascal_case()
        ),
        _ => base_type.clone(),
    };

    let array_length = match point.type_ {
        PointType::String => Some(point.size * 2),
        PointType::Eui48 => Some(6),
        PointType::Ipv6addr => Some(8),
        _ => None,
    };

    let writer_function_name = format!("write_{}", base_type.to_snake_case());
    let reader_function_name = match point.type_ {
        PointType::String => format!("read_{}::<{}>", base_type.to_snake_case(), point.size * 2),
        _ => format!("read_{}", base_type.to_snake_case()),
    };

    let writer_allow_offset = base_type != "u16" && base_type != "i16";

    let cast_from_c: Option<fn(&str) -> String> = match point.type_ {
        PointType::String => Some(cast_string_from_c),
        PointType::Eui48 => Some(cast_eui48_from_c),
        PointType::Ipv6addr => Some(cast_ipv6_from_c),
        _ => None,
    };

    let enum_repr: Option<String> = match point.type_ {
        _ if is_enum => Some(base_type),
        _ => None,
    };

    ResolvedType {
        rust_type,
        c_type,
        size: point.size as u16,
        array_length,
        writer_function_name,
        reader_function_name,
        writer_allow_offset,
        enum_repr,
        cast_from_c,
    }
}

pub(crate) fn resolve_point(
    point: &Point,
    features: &mut HashSet<CodegenFeature>,
    naming: &NamingContext,
    block_indices: Vec<BlockIndex>,
) -> Option<ResolvedPoint> {
    let point_type = resolve_point_type(point, features, naming);

    let main_name = point
        .label
        .clone()
        .map(|label| {
            if point.access == PointAccess::Rw && label.starts_with("Set ") {
                label[4..].to_string()
            } else {
                label
            }
        })
        .unwrap_or(point.name.clone());

    let name = format!(
        "{} {} {main_name}",
        naming.identifier_text,
        naming.name_prefix.clone().get_or_insert_default(),
    );

    let label = if let Some(label) = &point.label {
        format!("{} ({})", label, point.name)
    } else {
        point.name.clone()
    };

    let doc = [Some(label), point.desc.clone(), point.detail.clone()]
        .into_iter()
        .flatten()
        .collect();

    let value_type = if point.type_ == PointType::Pad {
        PointValueType::StaticValue("0".to_string())
    } else if let Some(value) = &point.value {
        PointValueType::StaticValue(value.to_string())
    } else if point.name == "L" {
        PointValueType::ModelLength
    } else {
        PointValueType::Adapter
    };

    Some(ResolvedPoint {
        name: Name::new(name.to_snake_case(), name.to_pascal_case()),
        internal_name: point.name.clone(),
        point_type,
        value_type,
        block_indices,
        access: point.access,
        mandatory: point.mandatory,
        size: point.size as u16,
        doc,
    })
}

pub fn resolve_enum(point: &Point, naming: &NamingContext) -> Option<ResolvedEnum> {
    if point.type_ == PointType::Enum16 || point.type_ == PointType::Enum32 {
        let values: Vec<EnumValue> = point
            .symbols
            .iter()
            .map(|symbol| EnumValue {
                name_pascal_case: symbol.name.to_pascal_case(),
                discriminant: symbol.value.to_string(),
                doc: [
                    symbol.label.as_ref(),
                    symbol.desc.as_ref(),
                    symbol.detail.as_ref(),
                ]
                .iter()
                .flat_map(|r| r.cloned())
                .collect(),
            })
            .collect();

        if values.is_empty() {
            None
        } else {
            Some(ResolvedEnum {
                // Prefixed with the model, matching `resolve_point_type`'s `enum_name` above -
                // this is the declaration the FFI-facing (`c_type`) field types reference.
                name: Name::new(
                    format!(
                        "{}_{}",
                        naming.model_file_name.to_snake_case(),
                        point.name.to_snake_case()
                    ),
                    format!(
                        "{}{}",
                        naming.model_file_name.to_pascal_case(),
                        point.name.to_pascal_case()
                    ),
                ),
                short_name_pascal_case: point.name.to_pascal_case(),
                discriminant_type: match point.type_ {
                    PointType::Enum16 => "u16".to_string(),
                    PointType::Enum32 => "u32".to_string(),
                    _ => "".to_string(),
                },
                values,
            })
        }
    } else {
        None
    }
}

/// The (snake_case name, PascalCase name, local short name) a schema group will resolve to,
/// given the naming context inherited from its ancestors - computed straight from the schema,
/// before the group itself is resolved. `resolve_group` uses this for its own identity; a
/// parent discovering a repeating child also uses it to build that child's [`BlockIndex`]
/// (which needs the child's own final name) before recursing into it, since both reduce to
/// exactly the same values.
fn group_identity(schema_group: &Group, identifier_text: &str) -> (String, String, String) {
    let label = schema_group.label.as_ref().unwrap_or(&schema_group.name);
    let name = format!("{identifier_text} {label}");
    (
        name.to_snake_case(),
        name.to_pascal_case(),
        schema_group.name.to_snake_case(),
    )
}

pub(crate) fn resolve_group(
    group: &Group,
    top_level_points_opt: Option<&[ResolvedPoint]>,
    features: &mut HashSet<CodegenFeature>,
    naming: &NamingContext,
    block_indices: Vec<BlockIndex>,
) -> ResolvedGroup {
    let (name_snake, name_pascal, local_name_short) =
        group_identity(group, &naming.identifier_text);

    let static_points: Vec<ResolvedPoint> = group
        .points
        .iter()
        .flat_map(|point| resolve_point(point, features, naming, block_indices.clone()))
        .collect();

    let top_level_points = top_level_points_opt.unwrap_or(&static_points);

    let name_short = format!("{} {}", naming.short_text, group.name).to_snake_case();

    // A repeating group (or the top-level model group, detected by `top_level_points_opt` being
    // `None`) gets its own generated struct/array scope, so its subgroups don't need any
    // ancestor qualification to stay unique - `short_text` resets to empty for them. A fixed
    // group has no scope of its own, so its subgroups inherit its `name_short` instead, to stay
    // distinguishable from a same-named subgroup under a sibling fixed group - e.g. model 708's
    // `MustTrip`/`MayTrip`/`MomCess`, each wrapping their own `Pt` array.
    let resets_for_children =
        top_level_points_opt.is_none() || matches!(group.count, GroupCount::String(_));
    let child_short_text = if resets_for_children {
        ""
    } else {
        name_short.as_str()
    };

    // Every direct subgroup, fixed and repeating alike, resolved in schema (wire) order - a
    // repeating subgroup's block has a variable runtime length, so a later sibling (fixed or
    // repeating) only has a runtime-computed address, and losing the relative order between
    // siblings would break address generation.
    let subgroups: Vec<Subgroup> = group
        .groups
        .iter()
        .filter_map(|child| match &child.count {
            // Fixed subgroups don't create a separate repeating structure on the wire - SunSpec
            // uses them purely to group related points/subgroups under a label. They don't add a
            // block index either - only a repeating group does - so `block_indices` passes
            // through unchanged.
            GroupCount::Integer(_) => {
                let child_label = child.label.as_ref().unwrap_or(&child.name);
                Some(Subgroup {
                    count: None,
                    group: Box::new(resolve_group(
                        child,
                        Some(top_level_points),
                        features,
                        &NamingContext {
                            identifier_text: format!("{child_label} {}", naming.identifier_text),
                            short_text: child_short_text.to_string(),
                            ..naming.clone()
                        },
                        block_indices.clone(),
                    )),
                })
            }
            GroupCount::String(count_name) => {
                let count_point = top_level_points
                    .iter()
                    .find(|p| p.internal_name == *count_name)?;

                let (_, _, index_prefix) = group_identity(child, &naming.identifier_text);
                let mut child_block_indices = block_indices.clone();
                child_block_indices.push(BlockIndex {
                    index_name: format!("{index_prefix}_index"),
                });

                Some(Subgroup {
                    count: Some(count_point.clone()),
                    group: Box::new(resolve_group(
                        child,
                        Some(top_level_points),
                        features,
                        &NamingContext {
                            short_text: child_short_text.to_string(),
                            name_prefix: Some(child.name.clone()),
                            ..naming.clone()
                        },
                        child_block_indices,
                    )),
                })
            }
        })
        .collect();

    // Every subgroup's own `.enums` already recursively contains everything in its subtree,
    // fixed or repeating, so bubbling them up here doesn't need to distinguish the two.
    let mut enums: Vec<ResolvedEnum> = group
        .points
        .iter()
        .flat_map(|point| resolve_enum(point, naming))
        .chain(
            subgroups
                .iter()
                .flat_map(|subgroup| subgroup.group.enums.iter().cloned()),
        )
        .collect();

    enums.sort_by_key(|e| e.name_snake_case());
    enums.dedup_by_key(|e| e.name_snake_case());

    let static_size = static_points.iter().map(|point| point.size).sum::<u16>();

    // A subgroup's own `writable` already reflects its whole subtree, fixed or repeating, so
    // this doesn't need to distinguish the two either.
    let writable = subgroups.iter().any(|subgroup| subgroup.group.writable)
        || static_points.iter().any(|p| p.access == PointAccess::Rw);

    let count_points: Vec<NameRef> = subgroups
        .iter()
        .flat_map(|Subgroup { group, count }| {
            count
                .iter()
                .map(|point| point.name.clone())
                .chain(group.count_points.iter().cloned())
        })
        .fold(Vec::new(), |mut points, point| {
            if !points
                .iter()
                .any(|existing| std::rc::Rc::ptr_eq(existing, &point))
            {
                points.push(point);
            }
            points
        });

    ResolvedGroup {
        name: Name::new(name_snake, name_pascal),
        name_short,
        local_name_short,
        static_size,
        static_points,
        enums,
        subgroups,
        writable,
        count_points,
    }
}

/// Registers every point and group in `group`'s subtree with the model-wide
/// `point_names`/`group_names` tables ahead of a single [`NameTable::deduplicate`] pass over
/// each.
fn register_names(group: &ResolvedGroup, point_names: &mut NameTable, group_names: &mut NameTable) {
    group_names.register_unique(group.name.clone());
    for point in &group.static_points {
        point_names.register(
            point.name.clone(),
            (
                point.internal_name.to_snake_case(),
                point.internal_name.to_pascal_case(),
            ),
        );
    }
    for subgroup in &group.subgroups {
        register_names(&subgroup.group, point_names, group_names);
    }
}

/// Collects one [`CountPoint`] per distinct repeat count in `group`'s subtree, in the order
/// each is first encountered. A count point reused by more than one repeating child (see
/// `resolve_model`) is collected only once.
fn collect_count_points(group: &ResolvedGroup, count_points: &mut Vec<CountPoint>) {
    for subgroup in &group.subgroups {
        if let Some(count_point) = &subgroup.count
            && !count_points
                .iter()
                .any(|cp| cp.point.internal_name == count_point.internal_name)
        {
            count_points.push(CountPoint {
                point: count_point.clone(),
            });
        }
        collect_count_points(&subgroup.group, count_points);
    }
}

pub fn resolve_model(model: &SunspecModel, file_name: String) -> ResolvedModel {
    let model_number: u16 = file_name[6..].parse().expect(
        "Unable to extract model number from name (expected name in structure model_X.json)",
    );
    let mut features: HashSet<CodegenFeature> = HashSet::new();
    let group = resolve_group(
        &model.group,
        None,
        &mut features,
        &NamingContext {
            identifier_text: String::new(),
            short_text: String::new(),
            name_prefix: None,
            model_file_name: file_name.clone(),
        },
        vec![],
    );

    let mut point_names = NameTable::default();
    let mut group_names = NameTable::default();
    register_names(&group, &mut point_names, &mut group_names);
    point_names.deduplicate();
    group_names.deduplicate();

    // `group.enums` already collects every enum in the model: resolve_group bubbles them up
    // through fixed subgroups and every repeating child into the top-level group's own list.
    let mut enum_names = NameTable::default();
    for resolved_enum in &group.enums {
        enum_names.register_unique(resolved_enum.name.clone());
    }
    enum_names.deduplicate();

    // A count point can be shared by more than one repeating child - model 708's `NPt` governs
    // all three of `Crv`'s curve-point arrays. Per the SunSpec spec that's one field on the
    // model, read independently by each array, not one field per array, so it's collected once
    // (by its underlying SunSpec name) no matter how many repeating children reuse it.
    let mut count_points = vec![];
    collect_count_points(&group, &mut count_points);

    ResolvedModel {
        model_number,
        name: Name::new(file_name.to_snake_case(), file_name.to_pascal_case()),
        group,
        features,
        count_points,
    }
}
