use codegen::{Block, Enum, Function, Impl, Scope, Struct, Type};

use crate::model_resolution::{
    CodegenFeature, DocLines, PointValueType, ResolvedEnum, ResolvedGroup, ResolvedModel,
    ResolvedPoint,
};
use crate::naming::Named;
use crate::sunspec_schema::{PointAccess, PointMandatory};

fn doc_text(parts: &DocLines) -> String {
    parts.join("\n\n")
}

fn generate_enum(resolved_enum: &ResolvedEnum, scope: &mut Scope) {
    let enum_name = &resolved_enum.name_pascal_case();
    let enum_def = scope.new_enum(enum_name);

    enum_def
        .vis("pub")
        .repr(&resolved_enum.discriminant_type)
        .derive("Clone")
        .derive("Copy")
        .derive("Debug")
        .derive("PartialEq");

    for value in &resolved_enum.values {
        enum_def
            .new_variant(&value.name_pascal_case)
            .discriminant(&value.discriminant)
            .doc(doc_text(&value.doc));
    }

    let enum_impl = scope.new_impl(enum_name);

    let fn_impl = enum_impl
        .new_fn("from_repr")
        .vis("pub")
        .arg("repr", &resolved_enum.discriminant_type)
        .ret(format!("Option<{enum_name}>"));

    let mut repr_match = Block::new("match repr");
    for value in &resolved_enum.values {
        repr_match.line(format!(
            "{} => Some({enum_name}::{}),",
            value.discriminant, value.name_pascal_case,
        ));
    }
    repr_match.line("_ => None");

    fn_impl.push_block(repr_match);

    let short_name = &resolved_enum.short_name_pascal_case;
    scope.raw(format!(
        "/// Short, spec-matching alias for [`{enum_name}`] - only `{enum_name}` (unique per \
         model) reaches the generated C header, since cbindgen has no module system to keep \
         `{short_name}` distinct from another model's point of the same name.\n\
         pub type {short_name} = {enum_name};"
    ));
}

fn option_unless_mandatory(point: &ResolvedPoint) -> Type {
    let inner_type: Type = (&point.point_type.rust_type).into();
    match point.mandatory {
        PointMandatory::O => Type::new("Option").generic(inner_type).to_owned(),
        PointMandatory::M => inner_type,
    }
}

fn generate_getter(point: &ResolvedPoint) -> Function {
    let mut func = Function::new(point.name_snake_case());

    if point.mandatory == PointMandatory::O {
        func.line("None");
    } else {
        func.body = None;
    };

    func.ret(option_unless_mandatory(point))
        .arg_ref_self()
        .doc(doc_text(&point.doc));
    func
}

fn generate_setter(point: &ResolvedPoint) -> Function {
    let mut func = Function::new(format!("set_{}", point.name_snake_case()));

    if point.mandatory == PointMandatory::M {
        func.body = None;
    };

    func.arg("value", &point.point_type.rust_type)
        .arg_mut_self()
        .doc(doc_text(&point.doc));
    func
}

/// Emits `group`'s own `_POINTS` static array (just [`ResolvedGroup::static_points`] - never a
/// fixed subgroup's, since those may sit after a variable-length repeating sibling on the wire
/// and can't share this group's simple locally-addressed array), then recurses into every
/// subgroup in [`ResolvedGroup::subgroups`] in schema order, fixed and repeating alike - each
/// gets its own array the same way, since only the top-level model group's array is the
/// well-known, unprefixed `POINTS`.
fn generate_point_arrays(
    group: &ResolvedGroup,
    scope: &mut Scope,
    args: Vec<String>,
    is_root: bool,
) {
    let mut address_counter = 0;
    let arg_vec: Vec<&str> = args.iter().map(|_| "u16").collect::<Vec<&str>>();
    let group_index_args = if arg_vec.len() == 1 {
        arg_vec.join(", ")
    } else {
        format!("({})", arg_vec.join(", "))
    };
    let lines: Vec<String> = [format!(
        "static {name_prefix}POINTS: [PointDetails<{group_index_args}>; {len}] = [",
        name_prefix = if is_root {
            "".to_string()
        } else {
            format!("{}_", group.name_short.to_uppercase())
        },
        len = group.static_points.len()
    )]
    .into_iter()
    .chain(group.static_points.iter().map(|point| {
        let point_reference = if args.is_empty() {
            format!("Point::{} ", point.name_pascal_case())
        } else {
            format!(
                "Point::{} {{ {} }}",
                point.name_pascal_case(),
                args.join(", "),
            )
        };

        let point_def = format!(
            "    PointDetails {{
        point: |{group_index_args}| {point_reference},
        size: {point_size},
        start_address: {start_address},
    }},",
            group_index_args = if args.len() == 1 {
                args.join(", ")
            } else {
                format!("({})", args.join(", "))
            },
            point_size = point.point_type.size,
            start_address = { address_counter }
        );

        address_counter += point.point_type.size;
        point_def
    }))
    .chain(["];".to_string()])
    .collect();

    scope.raw(lines.join("\n"));

    for subgroup in &group.subgroups {
        match &subgroup.count {
            Some(_) => {
                let mut inner_args = args.clone();
                inner_args.push(format!("{}_index", subgroup.group.local_name_short));
                generate_point_arrays(&subgroup.group, scope, inner_args, false);
            }
            None => {
                generate_point_arrays(&subgroup.group, scope, args.clone(), false);
            }
        }
    }
}

pub fn generate_models_mod(models: &[ResolvedModel]) -> Scope {
    let mut scope = Scope::new();
    scope.raw("#![allow(unused_variables)]");

    models.iter().for_each(|model| {
        scope.raw(format!(
            "{}\npub mod {};",
            model_cfg_attribute(model),
            model.name_snake_case()
        ));
    });

    scope
}

/// As [`generate_models_mod`], for `sunspec-modbus-lib-static`'s mirrored `src/sunspec/models.rs`
/// - one entry per [`generate_static_model`], so limited to C-expressible models.
pub fn generate_static_models_mod(models: &[ResolvedModel]) -> Scope {
    let mut scope = Scope::new();

    models
        .iter()
        .filter(|model| model_c_expressible(model))
        .for_each(|model| {
            scope.raw(format!(
                "{}\npub mod {};",
                model_cfg_attribute(model),
                model.name_snake_case()
            ));
        });

    scope
}

/// The `ReadBinding` / `WriteBinding` enums and their `read_model` / `write_model` drivers.
///
/// Each enum has one variant per model, pairing the model marker (e.g. `model_1::Model1`)
/// with a borrow of that model's per-model adapter trait - with the ref being mutable for writes.
/// `WriteBinding`'s model variants carry an adapter only for models with writable points.
///
/// Both enums also have a non-model `Extern` variant carrying a C [`StaticModelSpec`] vtable
/// and an untyped adapter pointer, for maps whose models aren't known statically (the FFI
/// `ModelList` in `sunspec-modbus-lib-static`). `read_model` / `write_model` traverse a model
/// variant against the cursor directly, and forward an `Extern` variant through the vtable.
pub fn generate_adapters_mod(models: &[ResolvedModel]) -> Scope {
    let mut scope = Scope::new();
    scope.import("crate::sunspec::models", "*");
    scope.import("core::ffi", "c_void");
    scope.import("crate", "ModbusException");
    scope.import("crate", "ModelSpec");
    scope.import("crate::buffer", "ReadableRegisterBuffer");
    scope.import("crate::buffer", "WritableRegisterBuffer");
    scope.import("crate::cursor", "Cursor");
    scope.import("crate::model", "StaticModelSpec");

    let read_enum = scope
        .new_enum("ReadBinding")
        .vis("pub")
        .generic("'a")
        .r#macro("#[non_exhaustive]")
        .doc(
            "One model's read side: a statically known model paired with a shared borrow of\n\
             its per-model [`ReadAdapter`](crate::sunspec::models) trait, or an\n\
             [`Extern`](ReadBinding::Extern) block dispatched through a C [`StaticModelSpec`] vtable.",
        );
    for model in models {
        let sc = &model.name_snake_case();
        let pc = &model.name_pascal_case();
        read_enum
            .new_variant(pc)
            .annotation(model_cfg_attribute(model))
            .tuple(format!("&'a {sc}::{pc}"))
            .tuple(format!("&'a dyn {sc}::ReadAdapter"));
    }
    read_enum
        .new_variant("Extern")
        .doc(
            "A model dispatched through a C [`StaticModelSpec`] vtable rather than a statically\n\
             known model type. Produced by the FFI `ModelList` in `sunspec-modbus-lib-static`.\n\
             \n\
             # Safety\n\
             Building this variant asserts `descriptor` and `adapter` uphold\n\
             [`StaticModelSpec::visit_read`]'s contract: `adapter` is either null or points to\n\
             a live `Model<id>CallbackAdapter` valid for the traversal.",
        )
        .named("descriptor", "&'a StaticModelSpec")
        .named("adapter", "*const c_void")
        .named("repeat_count_0", "u16")
        .named("repeat_count_1", "u16");

    let write_enum = scope
        .new_enum("WriteBinding")
        .vis("pub")
        .generic("'a")
        .r#macro("#[non_exhaustive]")
        .doc(
            "One model's write side: a statically known model, carrying a uniquely borrowed\n\
             reference to its per-model [`WriteAdapter`](crate::sunspec::models) trait when the\n\
             model has writable points, or an [`Extern`](WriteBinding::Extern) block dispatched\n\
             through a C [`StaticModelSpec`] vtable. A variant with no adapter rejects every\n\
             write in its block.",
        );
    for model in models {
        let sc = &model.name_snake_case();
        let pc = &model.name_pascal_case();
        let variant = write_enum
            .new_variant(pc)
            .annotation(model_cfg_attribute(model))
            .tuple(format!("&'a {sc}::{pc}"));
        if model.group.writable {
            variant.tuple(format!("&'a mut dyn {sc}::WriteAdapter"));
        }
    }
    write_enum
        .new_variant("Extern")
        .doc(
            "A model dispatched through a C [`StaticModelSpec`] vtable rather than a statically\n\
             known model type. Produced by the FFI `ModelList` in `sunspec-modbus-lib-static`.\n\
             \n\
             # Safety\n\
             Building this variant asserts `descriptor` and `adapter` uphold\n\
             [`StaticModelSpec::visit_write`]'s contract: `adapter` is either null or\n\
             uniquely borrowable, pointing to a live `Model<id>CallbackAdapter` valid for the\n\
             traversal.",
        )
        .named("descriptor", "&'a StaticModelSpec")
        .named("adapter", "*mut c_void")
        .named("repeat_count_0", "u16")
        .named("repeat_count_1", "u16");

    let mut read_match = Block::new("match adapter");
    for model in models {
        let pc = &model.name_pascal_case();
        let mut model_match = Block::new(format!(
            "{}\nReadBinding::{pc}(model, adapter) => ",
            model_cfg_attribute(model)
        ));

        model_match.line("cursor.visit_source_block(");
        model_match.line("model.model_length(),");
        model_match.line("|offset, from, len| { model.traverse_points_read(&*adapter, &mut buffer.slice(from, len), offset) }");
        model_match.line(");");
        read_match.push_block(model_match);
    }
    {
        let mut extern_match = Block::new(
            "ReadBinding::Extern { descriptor, adapter, repeat_count_0, repeat_count_1 } =>",
        );
        extern_match.line(
            "// SAFETY: `ReadBinding::Extern` upholds `StaticModelSpec::visit_read`'s contract by construction.",
        );
        let mut call = Block::new("unsafe");
        call.line(
            "(descriptor.visit_read)(adapter, repeat_count_0, repeat_count_1, cursor, buffer);",
        );
        extern_match.push_block(call);
        read_match.push_block(extern_match);
    }
    scope
        .new_fn("read_model")
        .vis("pub")
        .generic("'a")
        .arg("adapter", "ReadBinding<'a>")
        .arg("cursor", "&mut Cursor<ModbusException>")
        .arg("buffer", "&mut WritableRegisterBuffer<'_>")
        .doc(
            "Encode one model's block into `buffer` from its matched [`ReadBinding`] variant,\n\
             advancing `cursor` across the block whether or not it produces data.",
        )
        .push_block(read_match);

    let mut write_match = Block::new("match adapter");
    for model in models {
        let pc = &model.name_pascal_case();

        let arm = if model.group.writable {
            format!("WriteBinding::{pc}(model, adapter) =>")
        } else {
            format!("WriteBinding::{pc}(model) =>")
        };
        let mut model_match = Block::new(format!("{}\n{arm}", model_cfg_attribute(model)));

        model_match.line("cursor.visit_source_block(");
        model_match.line("model.model_length(),");

        if model.group.writable {
            model_match.line(
                "|offset, from, len| { model.traverse_points_write(adapter, &buffer.slice(from, len), offset) }"
            );
        } else {
            model_match.line("|_, _, _| Err(ModbusException::IllegalDataAddress)");
        }
        model_match.line(");");
        write_match.push_block(model_match);
    }
    {
        let mut extern_match = Block::new(
            "WriteBinding::Extern { descriptor, adapter, repeat_count_0, repeat_count_1 } =>",
        );
        extern_match.line(
            "// SAFETY: `WriteBinding::Extern` upholds `StaticModelSpec::visit_write`'s contract by construction.",
        );
        let mut call = Block::new("unsafe");
        call.line(
            "(descriptor.visit_write)(adapter, repeat_count_0, repeat_count_1, cursor, buffer);",
        );
        extern_match.push_block(call);
        write_match.push_block(extern_match);
    }
    scope
        .new_fn("write_model")
        .vis("pub")
        .generic("'a")
        .arg("adapter", "WriteBinding<'a>")
        .arg("cursor", "&mut Cursor<ModbusException>")
        .arg("buffer", "&ReadableRegisterBuffer<'_>")
        .doc(
            "Decode one model's block from `buffer` into its matched [`WriteBinding`] variant,\n\
             advancing `cursor` across the block. A non-writable variant rejects the write with\n\
             [`ModbusException::IllegalDataAddress`].",
        )
        .push_block(write_match);

    scope
}

/// Marker-struct constructor expression for a model's `StaticModelSpec` dispatch handlers.
///
/// Repeating-group models pull their repeat counts from the `repeat_count_0` /
/// `repeat_count_1` handler parameters, in `count_points` order.
fn model_marker_ctor(model: &ResolvedModel) -> String {
    let path = model.name_pascal_case();
    if model.count_points.is_empty() {
        path
    } else {
        let fields: Vec<String> = model
            .count_points
            .iter()
            .enumerate()
            .map(|(index, count)| {
                format!("{}: repeat_count_{index}", count.point.name_snake_case())
            })
            .collect();
        format!("{path} {{ {} }}", fields.join(", "))
    }
}

/// The C descriptor carries two repeat counts; a model needing more can't be built from C
/// and so gets no `SUNSPEC_MODEL_<id>` static.
pub(crate) fn model_c_expressible(model: &ResolvedModel) -> bool {
    model.count_points.len() <= 2
}

/// Whether this model has any writable points - see [`ResolvedGroup::writable`].
pub(crate) fn model_is_writable(model: &ResolvedModel) -> bool {
    model.group.writable
}

/// The Cargo feature that gates a model's generated code - named after its SunSpec model
/// number rather than its (label-derived, possibly-deduplicated) name, so it stays stable and
/// predictable regardless of naming changes elsewhere.
pub(crate) fn model_feature_name(model: &ResolvedModel) -> String {
    format!("model_{}", model.model_number)
}

/// The `#[cfg(...)]` attribute text gating a model's generated code on [`model_feature_name`].
pub(crate) fn model_cfg_attribute(model: &ResolvedModel) -> String {
    format!("#[cfg(feature = \"{}\")]", model_feature_name(model))
}

/// The per-model C-FFI dispatch appended to a model's module: a `#[unsafe(no_mangle)] pub
/// static SUNSPEC_MODEL_<id>: StaticModelSpec` plus its three handler functions.
///
/// A C `SunspecModelBinding` holds a `*const StaticModelSpec` pointing at that static, so
/// `sunspec-modbus-lib-static` dispatches straight through the function pointers with no
/// model-id lookup. `None` for models that can't be expressed through the two-repeat-count
/// C descriptor.
fn generate_c_model_dispatch(model: &ResolvedModel, scope: &mut Scope) {
    if !model_c_expressible(model) {
        return;
    }

    let n = model.model_number;
    let sc = &model.name_snake_case();
    let pc = &model.name_pascal_case();
    let ctor = model_marker_ctor(model);
    let writable = model.group.writable;
    // Non-repeating models ignore the repeat counts; repeating models consume them in `ctor`.

    let count_arg_0 = format!(
        "{}repeat_count_0",
        if !model.count_points.is_empty() {
            ""
        } else {
            "_"
        }
    );
    let count_arg_1 = format!(
        "{}repeat_count_1",
        if model.count_points.len() > 1 {
            ""
        } else {
            "_"
        }
    );

    scope.raw(format!(
        "/// C-FFI dispatch descriptor for SunSpec model {n}. A C `SunspecModelBinding` points\n\
         /// at this static, so the service functions dispatch with no model-id lookup.\n\
         #[unsafe(no_mangle)]\n\
         pub static SUNSPEC_MODEL_{n}: StaticModelSpec = StaticModelSpec {{\n\
         \x20   id: {n},\n\
         \x20   length: {sc}_c_length,\n\
         \x20   writable: {writable},\n\
         \x20   visit_read: {sc}_c_visit_read,\n\
         \x20   visit_write: {sc}_c_visit_write,\n\
         }};\n\n"
    ));

    scope
        .new_fn(format!("{sc}_c_length"))
        .arg(&count_arg_0, "u16")
        .arg(&count_arg_1, "u16")
        .ret("u16")
        .line(format!("({ctor}).model_length()"));

    // A null `adapter` means no adapter for this block; a single `visit_source_block` consumes
    // the model's words either way, encoding the block from a present adapter or filling it with
    // the SunSpec "not implemented" value for an absent one.
    scope.raw(format!(
        "/// # Safety\n\
         /// `adapter` must be null or point to a live `Model{n}CallbackAdapter`, valid for the\n\
         /// duration of the call.\n\
         unsafe fn {sc}_c_visit_read(\n\
         \x20   adapter: *const c_void,\n\
         \x20   {count_arg_0}: u16,\n\
         \x20   {count_arg_1}: u16,\n\
         \x20   cursor: &mut Cursor<ModbusException>,\n\
         \x20   buffer: &mut WritableRegisterBuffer<'_>,\n\
         ) {{\n\
         \x20   let model = {ctor};\n\
         \x20   // SAFETY: as required by this function's own contract.\n\
         \x20   let adapter = unsafe {{ (adapter as *const {pc}CallbackAdapter).as_ref() }};\n\
         \x20   cursor.visit_source_block(model.model_length(), |offset, from, len| {{\n\
         \x20       let mut block = buffer.slice(from, len);\n\
         \x20       match adapter {{\n\
         \x20           Some(adapter) => model.traverse_points_read(adapter, &mut block, offset),\n\
         \x20           None => {{\n\
         \x20               block.fill(&[0xff, 0xff]);\n\
         \x20               Ok(())\n\
         \x20           }}\n\
         \x20       }}\n\
         \x20   }});\n\
         }}\n\n"
    ));

    let write_body = if model.group.writable {
        format!(
            "    let model = {ctor};\n\
             \x20   // SAFETY: as required by this function's own contract.\n\
             \x20   let adapter = unsafe {{ (adapter as *mut {pc}CallbackAdapter).as_mut() }};\n\
             \x20   cursor.visit_source_block(model.model_length(), |offset, from, len| match adapter {{\n\
             \x20       Some(adapter) => model.traverse_points_write(adapter, &buffer.slice(from, len), offset),\n\
             \x20       None => Err(ModbusException::IllegalDataAddress),\n\
             \x20   }});\n"
        )
    } else {
        // No writable points: consume the block and reject every write that lands in it.
        format!(
            "    let _ = (adapter, buffer);\n\
             \x20   let model = {ctor};\n\
             \x20   cursor.visit_source_block(model.model_length(), |_, _, _| {{\n\
             \x20       Err(ModbusException::IllegalDataAddress)\n\
             \x20   }});\n"
        )
    };
    scope.raw(format!(
        "/// # Safety\n\
         /// As for [`{sc}_c_visit_read`], and `adapter` must be uniquely borrowable for the call.\n\
         unsafe fn {sc}_c_visit_write(\n\
         \x20   adapter: *mut c_void,\n\
         \x20   {count_arg_0}: u16,\n\
         \x20   {count_arg_1}: u16,\n\
         \x20   cursor: &mut Cursor<ModbusException>,\n\
         \x20   buffer: &ReadableRegisterBuffer<'_>,\n\
         ) {{\n\
         {write_body}\
         }}\n\n"
    ));
}

fn generate_callback_functions(group: &ResolvedGroup, callback_struct: &mut Struct) {
    for point in group
        .flattened_points()
        .into_iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
    {
        let c_type = if point.point_type.array_length.is_some() {
            format!("*const {}", point.point_type.c_type)
        } else {
            point.point_type.c_type.clone()
        };

        let group_index_args = ", u16".repeat(point.block_indices.len());

        let fn_ptr = format!("extern \"C\" fn(*const c_void{group_index_args}) -> {c_type}");
        callback_struct
            .new_field(
                format!("{}_callback", point.name_snake_case()),
                if point.mandatory == PointMandatory::M {
                    fn_ptr
                } else {
                    format!("Option<{fn_ptr}>")
                },
            )
            .doc(doc_text(&point.doc));

        if point.access == PointAccess::Rw {
            let set_fn_ptr = format!("extern \"C\" fn({c_type}, *mut c_void{group_index_args})");
            callback_struct
                .new_field(
                    format!("set_{}_callback", point.name_snake_case()),
                    if point.mandatory == PointMandatory::M {
                        set_fn_ptr
                    } else {
                        format!("Option<{set_fn_ptr}>")
                    },
                )
                .doc(doc_text(&point.doc));
        };
    }

    for (_, inner_group) in group.flattened_repeats() {
        generate_callback_functions(inner_group, callback_struct);
    }
}

fn generate_callback_read_handlers(group: &ResolvedGroup, callback_impl: &mut Impl) {
    for point in group
        .flattened_points()
        .into_iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
    {
        let group_index_args = point
            .block_indices
            .iter()
            .map(|block_index| format!(", {}", block_index.index_name))
            .collect::<Vec<_>>()
            .join("");

        let mut getter = generate_getter(point);
        for block_index in &point.block_indices {
            getter.arg(&block_index.index_name, "u16");
        }

        getter.body = None;
        getter.doc("");

        let callback_ref = if point.mandatory == PointMandatory::M {
            format!("self.{}_callback", point.name_snake_case())
        } else {
            getter.line(format!(
                "self.{}_callback.map(|callback| {{",
                point.name_snake_case()
            ));
            "callback".to_string()
        };

        let value = format!("({})(self.context{group_index_args})", callback_ref);

        getter.line(if let Some(cast) = point.point_type.cast_from_c {
            cast(&value)
        } else {
            value
        });
        if point.mandatory == PointMandatory::O {
            getter.line("})");
        }
        callback_impl.push_fn(getter);
    }

    for (_, inner_group) in group.flattened_repeats() {
        generate_callback_read_handlers(inner_group, callback_impl);
    }
}

fn generate_callback_write_handlers(group: &ResolvedGroup, callback_impl: &mut Impl) {
    for point in group.flattened_points().into_iter().filter(|point| {
        point.value_type == PointValueType::Adapter && point.access == PointAccess::Rw
    }) {
        let group_index_args = point
            .block_indices
            .iter()
            .map(|block_index| format!(", {}", block_index.index_name))
            .collect::<Vec<_>>()
            .join("");

        let mut setter = generate_setter(point);
        for block_index in &point.block_indices {
            setter.arg(&block_index.index_name, "u16");
        }
        let callback_ref = if point.mandatory == PointMandatory::O {
            setter.line(format!(
                "if let Some(callback) = self.set_{}_callback {{",
                point.name_snake_case(),
            ));
            "callback".to_string()
        } else {
            format!("self.set_{}_callback", point.name_snake_case())
        };

        setter.line(if point.point_type.cast_from_c.is_some() {
            format!(
                "({})(value.as_ptr(), self.context{group_index_args});",
                callback_ref
            )
        } else {
            format!("({})(value, self.context{group_index_args});", callback_ref)
        });

        if point.mandatory == PointMandatory::O {
            setter.line("};");
        }
        callback_impl.push_fn(setter);
    }

    for (_, inner_group) in group.flattened_repeats() {
        generate_callback_write_handlers(inner_group, callback_impl);
    }
}

pub fn generate_callback_struct(model: &ResolvedModel, scope: &mut Scope) {
    let callback_struct = scope
        .new_struct(format!("{}CallbackAdapter", model.name_pascal_case()))
        .vis("pub")
        .repr("C");

    callback_struct.field("context", "*mut c_void");

    generate_callback_functions(&model.group, callback_struct);

    let reader_impl = scope
        .new_impl(format!("{}CallbackAdapter", model.name_pascal_case()))
        .impl_trait("ReadAdapter");
    generate_callback_read_handlers(&model.group, reader_impl);

    if model.group.writable {
        let writer_impl = scope
            .new_impl(format!("{}CallbackAdapter", model.name_pascal_case()))
            .impl_trait("WriteAdapter");

        generate_callback_write_handlers(&model.group, writer_impl);
    }
}

fn generate_model_length_calculator(group: &ResolvedGroup) -> String {
    let terms: Vec<String> = group
        .flattened_repeats()
        .into_iter()
        .map(|(count_point, repeating_block)| {
            format!(
                "self.{} * ({})",
                count_point.name_snake_case(),
                generate_model_length_calculator(repeating_block)
            )
        })
        .collect();

    let static_size = total_static_size(group);
    if terms.is_empty() {
        static_size.to_string()
    } else {
        format!("{} + {}", static_size, terms.join(" + "))
    }
}

fn add_group_variants(group: &ResolvedGroup, target_enum: &mut Enum, counters: Vec<String>) {
    for point in group.flattened_points() {
        let variant = target_enum.new_variant(point.name_pascal_case());
        for counter in &counters {
            variant.named(counter, "u16");
        }
    }
    for (_, child_group) in group.flattened_repeats() {
        let mut new_counters = counters.clone();
        new_counters.push(format!("{}_index", child_group.local_name_short));
        add_group_variants(child_group, target_enum, new_counters);
    }
}

pub fn generate_readers(group: &ResolvedGroup) -> Vec<Function> {
    group
        .flattened_points()
        .into_iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
        .map(|point| {
            let mut getter = generate_getter(point);
            point.block_indices.iter().for_each(|block_index| {
                getter.arg(&block_index.index_name, "u16");
            });
            getter
        })
        .chain(
            group
                .flattened_repeats()
                .into_iter()
                .flat_map(|(_, inner_group)| generate_readers(inner_group)),
        )
        .collect()
}

pub fn generate_writers(group: &ResolvedGroup) -> Vec<Function> {
    group
        .flattened_points()
        .into_iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
        .filter(|point| point.access == PointAccess::Rw)
        .map(|point| {
            let mut setter = generate_setter(point);
            point.block_indices.iter().for_each(|block_index| {
                setter.arg(&block_index.index_name, "u16");
            });
            setter
        })
        .chain(
            group
                .flattened_repeats()
                .into_iter()
                .flat_map(|(_, inner_group)| generate_writers(inner_group)),
        )
        .collect()
}

fn populate_model_writer(group: &ResolvedGroup, writer_block: &mut Block) {
    for point in group.flattened_points() {
        let index_args: Vec<String> = point
            .block_indices
            .iter()
            .map(|block_index| block_index.index_name.clone())
            .collect();
        let match_arm = if index_args.is_empty() {
            format!("Point::{} =>", point.name_pascal_case())
        } else {
            format!(
                "Point::{} {{ {} }} =>",
                point.name_pascal_case(),
                index_args.join(", ")
            )
        };
        match &point.value_type {
            PointValueType::Adapter => {
                let dereferenced_args: Vec<String> =
                    index_args.iter().map(|arg| format!("*{arg}")).collect();
                let value_reader = format!(
                    "adapter.{}({})",
                    point.name_snake_case(),
                    dereferenced_args.join(", ")
                );
                let rest_args = if point.point_type.writer_allow_offset {
                    ", offset"
                } else {
                    ""
                };

                let value_cast = point
                    .point_type
                    .enum_repr
                    .as_ref()
                    .map(|ty| format!(" as {ty}"))
                    .clone()
                    .unwrap_or_default();

                let mut match_block = Block::new(match_arm);
                if point.mandatory == PointMandatory::M {
                    match_block
                        .line(format!(
                            "buffer.{}({value_reader}{value_cast}{rest_args});",
                            point.point_type.writer_function_name
                        ))
                        .after(",");
                } else {
                    let mut some_block = Block::new(format!("if let Some(value) = {value_reader}"));
                    some_block.line(format!(
                        "buffer.{}(value{value_cast}{rest_args});",
                        point.point_type.writer_function_name
                    ));
                    let mut else_block = Block::new("else");
                    else_block.line("buffer.zero();");

                    match_block.push_block(some_block);
                    match_block.push_block(else_block);
                };
                writer_block.push_block(match_block);
            }
            PointValueType::ModelLength => {
                let mut match_block = Block::new(match_arm);

                match_block.line("buffer.write_u16(model.model_length() - 2);");
                writer_block.push_block(match_block);
            }
            PointValueType::StaticValue(s) => {
                let mut match_block = Block::new(match_arm);

                match_block.line(format!("buffer.write_u16({s});"));
                writer_block.push_block(match_block);
            }
        }
    }

    for (_, inner_group) in group.flattened_repeats() {
        populate_model_writer(inner_group, writer_block);
    }
}

fn populate_model_reader(group: &ResolvedGroup, reader_block: &mut Block) {
    for point in group.flattened_points() {
        let index_args: Vec<String> = point
            .block_indices
            .iter()
            .map(|block_index| block_index.index_name.clone())
            .collect();
        let match_arm = if index_args.is_empty() {
            format!("Point::{} =>", point.name_pascal_case())
        } else {
            format!(
                "Point::{} {{ {} }} =>",
                point.name_pascal_case(),
                index_args.join(", ")
            )
        };
        if point.value_type == PointValueType::Adapter && point.access == PointAccess::Rw {
            let dereferenced_args: Vec<String> =
                index_args.iter().map(|arg| format!("*{arg}")).collect();
            let rest_args = if dereferenced_args.is_empty() {
                "".to_string()
            } else {
                format!(", {}", dereferenced_args.join(", "))
            };

            let mut match_block = Block::new(match_arm);
            match_block.line(format!(
                "let value = buffer.{}()?;",
                point.point_type.reader_function_name
            ));

            let parsed_value_name = if point.point_type.enum_repr.is_some() {
                match_block.line(format!(
                    "let enum_value = {}::from_repr(value).ok_or(ModbusException::IllegalDataValue)?;",
                    point.point_type.rust_type
                ));
                "enum_value"
            } else if point.point_type.rust_type == "&CStr" {
                match_block.line(
                    "let str_value = CStr::from_bytes_with_nul(&value).map_err(|_| ModbusException::IllegalDataValue)?;");
                "str_value"
            } else {
                "value"
            };

            match_block
                .line(format!(
                    "adapter.set_{}({parsed_value_name}{rest_args});",
                    point.name_snake_case()
                ))
                .line("Ok(())")
                .after(",");
            reader_block.push_block(match_block);
        }
    }

    for (_, inner_group) in group.flattened_repeats() {
        populate_model_reader(inner_group, reader_block);
    }
}

/// The total non-repeating byte count of one instance of `group`, across its whole subtree -
/// [`ResolvedGroup::static_size`] (its own points), plus every fixed subgroup's, transitively. A
/// repeating subgroup's own contribution is variable and excluded here; see [`group_size_expr`],
/// which adds it back in as a runtime term.
fn total_static_size(group: &ResolvedGroup) -> u16 {
    group.static_size
        + group
            .subgroups
            .iter()
            .filter(|subgroup| subgroup.count.is_none())
            .map(|subgroup| total_static_size(&subgroup.group))
            .sum::<u16>()
}

/// The total size of one instance of `group` - its [`total_static_size`] plus every repeating
/// descendant's own contribution (found transitively, through any fixed subgroups wrapping one -
/// see [`ResolvedGroup::flattened_repeats`]). Used both for a repeating group's array stride
/// (`{prefix}_size`) and, in [`chain_repeating_group_iterator`], to advance past a fixed
/// sibling's entire span.
fn group_size_expr(group: &ResolvedGroup) -> String {
    let child_terms: Vec<String> = group
        .flattened_repeats()
        .into_iter()
        .map(|(_, child)| {
            let prefix = &child.name_short;
            format!("{prefix}_count * {prefix}_size")
        })
        .collect();
    let static_size = total_static_size(group);
    if child_terms.is_empty() {
        static_size.to_string()
    } else {
        format!("{} + {}", static_size, child_terms.join(" + "))
    }
}

/// Formats a `Point` closure's index arguments: a bare value when there's exactly one, a tuple
/// when there's more than one, or `()` when there's none - matching how a point with no
/// enclosing repeat is called elsewhere.
fn format_index_args(indices: &[String]) -> String {
    match indices {
        [] => "()".to_string(),
        [single] => single.clone(),
        _ => format!("({})", indices.join(", ")),
    }
}

/// Emits the iterator-chaining code for every subgroup of `group`, recursively, in schema (wire)
/// order. A repeating subgroup's block has a runtime-variable length, so anything that follows
/// it on the wire - including a later *fixed* sibling's own points - only has a runtime-computed
/// address from that point on; `running_offset` starts as a plain compile-time offset and
/// becomes a runtime expression the moment it has to account for a repeating block.
fn chain_repeating_group_iterator(
    group: &ResolvedGroup,
    address_offset: String,
    indices: Vec<String>,
) -> Vec<String> {
    let mut result = vec![];
    let mut running_offset = address_offset;

    for subgroup in &group.subgroups {
        let inner_group = &subgroup.group;
        let prefix = &inner_group.name_short;

        match &subgroup.count {
            Some(_) => {
                // The loop variable only needs to be unique within this closure, unlike `prefix`
                // (shared with the enclosing function's `_count`/`_size` locals and the
                // `_POINTS` static, which do need disambiguating) - so it's named after the
                // group alone, not qualified by any fixed group it's nested inside.
                let index = &inner_group.local_name_short;
                let mut index_args = indices.clone();
                index_args.push(format!("{index}_index"));
                let index_arg_str = format_index_args(&index_args);

                result.push(format!(
                    ".chain((0..{prefix}_count).flat_map(move |{index}_index| {{"
                ));
                result.push(format!(
                    "let {prefix}_address = {running_offset} + {index}_index * {prefix}_size;"
                ));
                result.push(format!("{}_POINTS.iter()", prefix.to_uppercase()));
                result.push(format!(
                    ".map(move |p| ({prefix}_address + p.start_address, p.size, (p.point)({index_arg_str})))"
                ));

                // `inner_group`'s own subgroups (if any) are nested inside each of its
                // instances, right after that instance's own points - not at the instance's own
                // start address, which is where `inner_group`'s own `_POINTS` are addressed from.
                result.extend(chain_repeating_group_iterator(
                    inner_group,
                    format!("{prefix}_address + {}", inner_group.static_size),
                    index_args,
                ));

                result.push("}))".to_string());

                running_offset = format!("{running_offset} + {prefix}_count * {prefix}_size");
            }
            None => {
                let index_arg_str = format_index_args(&indices);

                // Exactly one instance, inline at the current offset - no loop, no new index.
                result.push(format!(
                    ".chain({}_POINTS.iter().map(move |p| ({running_offset} + p.start_address, p.size, (p.point)({index_arg_str}))))",
                    prefix.to_uppercase()
                ));

                result.extend(chain_repeating_group_iterator(
                    inner_group,
                    format!("{running_offset} + {}", inner_group.static_size),
                    indices.clone(),
                ));

                running_offset = format!("{running_offset} + {}", group_size_expr(inner_group));
            }
        }
    }

    result
}

/// Emits `let {prefix}_count = self.<count>;` / `let {prefix}_size = ...;` declarations for
/// every repeating group nested under `group`, in a bottom-up order so each declaration can
/// reference its own children's `_count`/`_size` locals - mirroring the structure
/// `chain_repeating_group_iterator` walks to build the matching address expressions. A group
/// with several repeating children (model 708's `Crv`) sums each one's contribution to its own
/// `_size`, since one instance of the group contains all of them. Order among siblings doesn't
/// matter here - each declaration only references its own descendants, never a sibling's - so
/// this walks [`ResolvedGroup::flattened_repeats`] rather than [`ResolvedGroup::subgroups`]
/// directly, skipping straight past any fixed wrapper.
fn declare_repeat_counts_and_sizes(group: &ResolvedGroup, fn_def: &mut Function) {
    for (count_point, inner_group) in group.flattened_repeats() {
        declare_repeat_counts_and_sizes(inner_group, fn_def);

        let prefix = &inner_group.name_short;
        fn_def.line(format!(
            "let {prefix}_count = self.{};",
            count_point.name_snake_case()
        ));
        fn_def.line(format!(
            "let {prefix}_size = {};",
            group_size_expr(inner_group)
        ));
        fn_def.line("");
    }
}

enum TraverseDirection {
    Read,
    Write,
}

fn generate_traverse_points_fn(
    model: &ResolvedModel,
    scope: &mut Impl,
    direction: TraverseDirection,
) {
    let fn_name = match direction {
        TraverseDirection::Read => "traverse_points_read",
        TraverseDirection::Write => "traverse_points_write",
    };

    let fn_def = scope.new_fn(fn_name).generic("'a");

    match direction {
        TraverseDirection::Read => {
            fn_def
                .arg_ref_self()
                .arg("adapter", "&Self::ReadAdapter")
                .arg("buffer", "&mut WritableRegisterBuffer<'a>")
                .arg("offset", "u16")
                .ret("Result<(), ModbusException>");
        }
        TraverseDirection::Write => {
            fn_def
                .arg_ref_self()
                .arg("adapter", "&mut Self::WriteAdapter")
                .arg("buffer", "&ReadableRegisterBuffer<'a>")
                .arg("offset", "u16")
                .ret("Result<(), ModbusException>");
        }
    }

    fn_def.line("let until = offset + buffer.len();");
    fn_def.line("let mut cursor = 0;");
    fn_def.line("");

    declare_repeat_counts_and_sizes(&model.group, fn_def);

    fn_def.line("let iter = POINTS.iter()");
    fn_def.line(".map(|p| (p.start_address, p.size, (p.point)(())))");

    for s in
        chain_repeating_group_iterator(&model.group, model.group.static_size.to_string(), vec![])
    {
        fn_def.line(s);
    }

    fn_def.line(".skip_while(|(start,size,_)| offset >= start + size)");
    fn_def.line(".take_while(|(start,_,_)| until > *start);");
    fn_def.line("");
    let mut point_block = Block::new("for (start, size, point) in iter");
    point_block.line("let point_offset = offset.saturating_sub(start);");
    point_block.line("let word_count = min(size - point_offset, buffer.len() - cursor);");

    match direction {
        TraverseDirection::Read => {
            point_block
                .line("write_point_to_buffer(")
                .line("self,")
                .line("adapter,")
                .line("&point,")
                .line("&mut buffer.slice(cursor, word_count),")
                .line("point_offset,")
                .line(");");
        }
        TraverseDirection::Write => {
            let mut offset_guard = Block::new("if point_offset > 0");
            offset_guard.line("return Err(ModbusException::IllegalDataAddress)");
            point_block.push_block(offset_guard);

            point_block
                .line("read_point_from_buffer(")
                .line("self,")
                .line("adapter,")
                .line("&point,")
                .line("&buffer.slice(cursor, word_count),")
                .line(")?;");
        }
    }

    point_block.line("cursor += word_count;");
    fn_def.push_block(point_block);

    fn_def.line("");
    fn_def.line("Ok(())");
}

pub fn generate_model(model: &ResolvedModel) -> Scope {
    let mut scope = Scope::new();

    for feature in &model.features {
        match feature {
            CodegenFeature::String => scope.import("core::ffi", "CStr"),
            // CodegenFeature::Ipv4Addr => scope.import("core::net", "Ipv4Addr"),
            // CodegenFeature::Ipv6Addr => scope.import("core::net", "Ipv6Addr"),
        };
    }

    scope.import("crate::buffer", "WritableRegisterBuffer");
    scope.import("crate::buffer", "ReadableRegisterBuffer");
    scope.import("crate::model", "ModelSpec");
    scope.import("crate", "ModbusException");
    scope.import("core::cmp", "min");

    generate_point_arrays(&model.group, &mut scope, vec![], true);

    let point_enum = scope.new_enum("Point").vis("pub").derive("Debug");

    add_group_variants(&model.group, point_enum, vec![]);

    scope
        .new_struct("PointDetails")
        .generic("GroupIndexArgs")
        .field("point", "fn(GroupIndexArgs) -> Point")
        .field("start_address", "u16")
        .field("size", "u16");

    let model_struct = scope.new_struct(model.name_pascal_case()).vis("pub");

    model_struct.doc(format!(
        "Marker for SunSpec model {}. Add it to a [`Sunspec`](crate::Sunspec) model list.",
        model.model_number
    ));

    for count_point in &model.count_points {
        model_struct
            .new_field(
                count_point.point.name_snake_case(),
                &count_point.point.point_type.rust_type,
            )
            .vis("pub")
            .doc("Number of repeating blocks in this model instance.");
    }

    let model_impl = scope
        .new_impl(model.name_pascal_case())
        .generic("'ad")
        .impl_trait("ModelSpec<'ad>");

    model_impl.associate_const("MODEL_ID", "u16", model.model_number.to_string(), "");
    model_impl.associate_type("ReadAdapter", "dyn ReadAdapter + 'ad");
    model_impl.associate_type("WriteAdapter", "dyn WriteAdapter + 'ad");

    model_impl
        .new_fn("model_length")
        .arg_ref_self()
        .ret("u16")
        .line(generate_model_length_calculator(&model.group));

    generate_traverse_points_fn(model, model_impl, TraverseDirection::Read);
    generate_traverse_points_fn(model, model_impl, TraverseDirection::Write);

    let mut writer_block = Block::new("match point");
    populate_model_writer(&model.group, &mut writer_block);

    scope
        .new_fn("write_point_to_buffer")
        .generic("'a")
        .vis("pub(crate)")
        .arg("model", format!("&{}", model.name_pascal_case()))
        .arg("adapter", "&dyn ReadAdapter")
        .arg("point", "&Point")
        .arg("buffer", "&mut WritableRegisterBuffer<'a>")
        .arg("offset", "u16")
        .push_block(writer_block);

    let reader_fn = scope
        .new_fn("read_point_from_buffer")
        .generic("'a")
        .vis("pub(crate)")
        .arg("model", format!("&{}", model.name_pascal_case()))
        .arg("adapter", "&mut dyn WriteAdapter")
        .arg("point", "&Point")
        .arg("buffer", "&ReadableRegisterBuffer<'a>")
        .ret("Result<(), ModbusException>");

    if model.group.writable {
        let mut reader_block = Block::new("match point");
        populate_model_reader(&model.group, &mut reader_block);

        let mut unmatched_block = Block::new("_ =>");
        unmatched_block.line("Err(ModbusException::IllegalDataAddress)");
        reader_block.push_block(unmatched_block);

        reader_fn.push_block(reader_block);
    } else {
        reader_fn.line("Err(ModbusException::IllegalDataAddress)");
    }

    let read_trait = scope.new_trait("ReadAdapter").vis("pub");

    for func in generate_readers(&model.group) {
        read_trait.push_fn(func);
    }

    let write_trait = scope.new_trait("WriteAdapter").vis("pub");

    for func in generate_writers(&model.group) {
        write_trait.push_fn(func);
    }

    for enum_type in &model.group.enums {
        generate_enum(enum_type, &mut scope);
    }

    scope
}

/// The C-FFI counterpart to [`generate_model`], generated instead into
/// `sunspec-modbus-lib-static`'s mirrored `src/sunspec/models/<name>.rs`: the model's
/// [`Model<id>CallbackAdapter`](generate_callback_struct) and its `StaticModelSpec` dispatch.
/// Kept out of `sunspec-modbus-lib-rs` entirely, so a pure-Rust consumer of that crate never
/// sees C-FFI types. `None` for models that can't be expressed through the two-repeat-count C
/// descriptor (see [`model_c_expressible`]) - there is nothing C-facing to generate for them.
pub fn generate_static_model(model: &ResolvedModel) -> Option<Scope> {
    if !model_c_expressible(model) {
        return None;
    }

    let mut scope = Scope::new();
    let sc = &model.name_snake_case();

    for feature in &model.features {
        match feature {
            CodegenFeature::String => {
                scope.import("core::ffi", "CStr");
                scope.import("core::ffi", "c_char")
            }
        };
    }

    scope.import("core::ffi", "c_void");
    scope.import("sunspec_modbus_lib_rs::buffer", "WritableRegisterBuffer");
    scope.import("sunspec_modbus_lib_rs::buffer", "ReadableRegisterBuffer");
    scope.import("sunspec_modbus_lib_rs", "ModbusException");
    scope.import("sunspec_modbus_lib_rs::cursor", "Cursor");
    scope.import("sunspec_modbus_lib_rs::model", "ModelSpec");
    scope.import("sunspec_modbus_lib_rs::model", "StaticModelSpec");
    scope.import("crate", "SunspecAdapter");
    // Glob, not a named list: besides the model marker and its `ReadAdapter`/`WriteAdapter`,
    // the callback struct's getters/setters can reference the model's own point enums (e.g.
    // `WSetEna`) by their short, unqualified names.
    scope.import(format!("sunspec_modbus_lib_rs::sunspec::models::{sc}"), "*");

    generate_callback_struct(model, &mut scope);
    generate_c_model_dispatch(model, &mut scope);
    generate_adapter_ctor(model, &mut scope);

    Some(scope)
}

/// Typed `SunspecAdapter` constructor for one model:
/// `sunspec_model_<id>_callback(*mut Model<id>CallbackAdapter)` builds a `SunspecAdapter` with
/// the matching `model_spec` and adapter pointer, so a C caller cannot pair the wrong adapter
/// type with a model (the C compiler rejects a mismatched pointer) or leave the
/// `model_spec`/`adapter` pair inconsistent.
///
/// A real `#[unsafe(no_mangle)] pub extern "C" fn`, generated directly into
/// `sunspec-modbus-lib-static` rather than spliced into the header as C text: cbindgen exports
/// it itself, and naming `Model<id>CallbackAdapter` in its signature is what makes cbindgen
/// carry that struct's field layout — no force-listing needed. Generated alongside
/// [`Model<id>CallbackAdapter`](generate_callback_struct) and its `SUNSPEC_MODEL_<id>` static
/// (see [`generate_c_model_dispatch`]), so it refers to both by their local, unqualified names.
fn generate_adapter_ctor(model: &ResolvedModel, scope: &mut Scope) {
    let n = model.model_number;
    let pc = &model.name_pascal_case();

    scope.raw(format!(
        "/// Build a [`SunspecAdapter`] for model {n} backed by a callback adapter.\n\
         #[unsafe(no_mangle)]\n\
         pub extern \"C\" fn sunspec_model_{n}_callback(\n\
         \x20   adapter: *mut {pc}CallbackAdapter,\n\
         ) -> SunspecAdapter {{\n\
         \x20   SunspecAdapter {{\n\
         \x20       model_spec: &SUNSPEC_MODEL_{n},\n\
         \x20       adapter: adapter as *mut c_void,\n\
         \x20   }}\n\
         }}\n"
    ));
}
