use heck::{ToPascalCase, ToSnakeCase};
use std::collections::HashSet;

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
    pub raw_type: String,
    pub rust_type: String,
    pub c_type: String,
    pub size: u16,
    pub writer_function_name: String,
    pub writer_allow_offset: bool,
    pub writer_value_cast: Option<String>,
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
    pub name_pascal_case: String,
    pub name_snake_case: String,
    pub internal_name: String,
    pub value_type: PointValueType,
    pub point_type: ResolvedType,
    pub access: PointAccess,
    pub mandatory: PointMandatory,
    pub doc: DocLines,
    pub size: u16,
}

#[derive(Clone)]
pub struct ResolvedEnum {
    pub name_pascal_case: String,
    pub name_snake_case: String,
    pub discriminant_type: String,
    pub values: Vec<EnumValue>,
}

#[derive(Clone)]
pub struct EnumValue {
    pub name_pascal_case: String,
    pub discriminant: String,
    pub doc: DocLines,
}

pub struct ResolvedModel {
    pub name_pascal_case: String,
    pub name_snake_case: String,
    pub features: HashSet<CodegenFeature>,
    pub model_number: u16,
    pub group: ResolvedGroup,
}

pub struct ResolvedGroup {
    pub name_pascal_case: String,
    pub name_snake_case: String,
    pub static_size: u16,
    pub repeat_count_point: Option<ResolvedPoint>,
    pub points: Vec<ResolvedPoint>,
    pub enums: Vec<ResolvedEnum>,
    pub repeating_child: Option<Box<ResolvedGroup>>,
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

fn resolve_point_type(point: &Point, features: &mut HashSet<CodegenFeature>) -> ResolvedType {
    let base_type = match point.type_ {
        PointType::Uint16
        | PointType::Raw16
        | PointType::Acc16
        | PointType::Bitfield16
        | PointType::Pad
        | PointType::Sunssf
        | PointType::Count
        | PointType::Enum16 => "u16".to_string(),
        PointType::Uint32 | PointType::Acc32 | PointType::Bitfield32 | PointType::Enum32 => {
            "u32".to_string()
        }
        PointType::Uint64 | PointType::Acc64 | PointType::Bitfield64 => "u64".to_string(),
        PointType::Int16 => "i16".to_string(),
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
        _ if is_enum => point.name.to_pascal_case(),
        _ => base_type.clone(),
    };

    let c_type = match point.type_ {
        PointType::String => "c_char".to_string(),
        PointType::Eui48 => "u8".to_string(),
        PointType::Ipv6addr => "u16".to_string(),
        _ if is_enum => point.name.to_pascal_case(),
        _ => base_type.clone(),
    };

    let array_length = match point.type_ {
        PointType::String => Some(point.size * 2),
        PointType::Eui48 => Some(6),
        PointType::Ipv6addr => Some(8),
        _ => None,
    };

    let writer_function_name = format!("write_{}", base_type.to_snake_case());

    let writer_allow_offset = base_type != "u16" && base_type != "i16";

    let writer_value_cast = match point.type_ {
        _ if is_enum => Some(format!(" as {}", base_type)),
        _ => None,
    };

    let cast_from_c: Option<fn(&str) -> String> = match point.type_ {
        PointType::String => Some(cast_string_from_c),
        PointType::Eui48 => Some(cast_eui48_from_c),
        PointType::Ipv6addr => Some(cast_ipv6_from_c),
        _ => None,
    };

    ResolvedType {
        raw_type: point.type_.to_string().to_pascal_case(),
        rust_type,
        c_type,
        size: point.size as u16,
        array_length,
        writer_function_name,
        writer_allow_offset,
        writer_value_cast,
        cast_from_c,
    }
}

pub fn resolve_point(
    point: &Point,
    features: &mut HashSet<CodegenFeature>,
) -> Option<ResolvedPoint> {
    let point_type = resolve_point_type(point, features);

    let name = point
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

    let doc = [
        point.label.as_ref(),
        point.desc.as_ref(),
        point.detail.as_ref(),
    ]
    .iter()
    .flat_map(|r| r.cloned())
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
        name_snake_case: name.to_snake_case(),
        name_pascal_case: name.to_pascal_case(),
        internal_name: point.name.clone(),
        point_type,
        value_type,
        access: point.access,
        mandatory: point.mandatory,
        size: point.size as u16,
        doc,
    })
}

pub fn resolve_enum(point: &Point) -> Option<ResolvedEnum> {
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
                name_snake_case: point.name.to_snake_case(),
                name_pascal_case: point.name.to_pascal_case(),
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

pub fn resolve_group(
    group: &Group,
    top_level_points: Option<&[ResolvedPoint]>,
    features: &mut HashSet<CodegenFeature>,
) -> ResolvedGroup {
    let root_points: Vec<ResolvedPoint> = group
        .points
        .iter()
        .flat_map(|point| resolve_point(point, features))
        .collect();

    let (repeating_groups, groups): (Vec<ResolvedGroup>, Vec<ResolvedGroup>) = group
        .groups
        .iter()
        .map(|child| resolve_group(child, top_level_points.or(Some(&root_points)), features))
        .partition(|g| g.repeat_count_point.is_some());

    let points: Vec<ResolvedPoint> = root_points
        .into_iter()
        .chain(groups.iter().flat_map(|g| {
            g.points.iter().cloned().map(|p| ResolvedPoint {
                name_pascal_case: format!("{}{}", &g.name_pascal_case, &p.name_pascal_case),
                name_snake_case: format!("{}_{}", &g.name_snake_case, &p.name_snake_case),
                ..p
            })
        }))
        .collect();

    let mut enums: Vec<ResolvedEnum> = group
        .points
        .iter()
        .flat_map(resolve_enum)
        .chain(groups.iter().flat_map(|g| g.enums.iter().cloned()))
        .collect();

    enums.sort_by_key(|e| e.name_snake_case.clone());
    enums.dedup_by_key(|e| e.name_snake_case.clone());

    let size = points.iter().map(|point| point.size).sum();

    let count_name = match &group.count {
        GroupCount::String(s) => Some(s.clone()),
        _ => None,
    };

    let repeat_count_point = count_name
        .and_then(|n| {
            top_level_points
                .into_iter()
                .flatten()
                .find(|p| p.internal_name == n)
        })
        .cloned();

    ResolvedGroup {
        name_pascal_case: group.name.to_pascal_case(),
        name_snake_case: group.name.to_snake_case(),
        static_size: size,
        repeat_count_point,
        points,
        enums,
        repeating_child: repeating_groups.into_iter().map(Box::new).next(),
    }
}

pub fn resolve_model(model: &SunspecModel, file_name: String) -> ResolvedModel {
    let model_number: u16 = file_name[6..].parse().unwrap();
    let mut features: HashSet<CodegenFeature> = HashSet::new();
    let group = resolve_group(&model.group, None, &mut features);

    ResolvedModel {
        model_number,
        name_snake_case: file_name.to_snake_case(),
        name_pascal_case: file_name.to_pascal_case(),
        group,
        features,
    }
}
