use heck::{ToPascalCase, ToSnakeCase};
use std::collections::HashSet;

use crate::sunspec_schema::{Point, PointAccess, PointMandatory, PointType, SunspecModel};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodegenFeature {
    String,
    Ipv4Addr,
    Ipv6Addr,
}

pub type DocLines = Vec<String>;

fn cast_string_from_c(value: &str) -> String {
    format!("unsafe {{ CStr::from_ptr({}) }}", value)
}
fn cast_eui48_from_c(value: &str) -> String {
    format!("unsafe {{ &*({} as *const [u8; 6]) }}", value)
}

pub struct ResolvedType {
    pub raw_type: String,
    pub rust_type: String,
    pub c_type: String,
    pub size: u16,
    pub writer_function_name: String,
    pub writer_allow_offset: bool,
    pub writer_value_cast: Option<String>,
    pub cast_from_c: Option<fn(&str) -> String>
}

pub struct ResolvedPoint {
    pub name_pascal_case: String,
    pub name_snake_case: String,
    pub static_value: Option<String>,
    pub point_type: ResolvedType,
    pub access: PointAccess,
    pub mandatory: PointMandatory,
    pub doc: DocLines,
}
pub struct ResolvedEnum {
    pub name_pascal_case: String,
    pub name_snake_case: String,
    pub discriminant_type: String,
    pub values: Vec<EnumValue>,
}

pub struct EnumValue {
    pub name_pascal_case: String,
    pub name_snake_case: String,
    pub discriminant: String,
    pub doc: DocLines,
}

pub struct ResolvedModel {
    pub name_pascal_case: String,
    pub name_snake_case: String,
    pub points: Vec<ResolvedPoint>,
    pub enums: Vec<ResolvedEnum>,
    pub size: u16,
    pub features: HashSet<CodegenFeature>,
    pub doc: DocLines,
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
        PointType::Ipaddr => {
            features.insert(CodegenFeature::Ipv4Addr);
            "Ipv4Addr".to_string()
        }
        PointType::Ipv6addr => {
            features.insert(CodegenFeature::Ipv6Addr);
            "Ipv6Addr".to_string()
        }
        PointType::Eui48 => "eui48".to_string(),
    };

    let is_enum = (point.type_ == PointType::Enum16 || point.type_ == PointType::Enum32)
        && !point.symbols.is_empty();

    let rust_type = match point.type_ {
        PointType::String => "&CStr".to_string(),
        PointType::Eui48 => "&[u8; 6]".to_string(),
        _ if is_enum => point.name.to_pascal_case(),
        _ => base_type.clone(),
    };

    let c_type = match point.type_ {
        PointType::String => "*const c_char".to_string(),
        PointType::Eui48 => "*const u8".to_string(),
        _ if is_enum => point.name.to_pascal_case(),
        _ => base_type.clone(),
    };

    let writer_function_name = match point.type_ {
        _ => format!("write_{}", base_type.to_snake_case()),
    };

    let writer_allow_offset = base_type != "u16" && base_type != "i16";

    let writer_value_cast = match point.type_ {
        _ if is_enum => Some(format!(" as {}", base_type)),
        _ => None,
    };

    let cast_from_c: Option<fn(&str) -> String> = match point.type_ {
        PointType::String => Some(cast_string_from_c),
        PointType::Eui48 => Some(cast_eui48_from_c),
        _ => None
    };

    ResolvedType {
        raw_type: point.type_.to_string().to_pascal_case(),
        rust_type,
        c_type,
        size: point.size as u16,
        writer_function_name,
        writer_allow_offset,
        writer_value_cast,
        cast_from_c,
    }
}

pub fn resolve_point(
    point: &Point,
    features: &mut HashSet<CodegenFeature>,
    model_size: u16,
) -> Option<ResolvedPoint> {
    let point_type = resolve_point_type(point, features);
    let name = point.label.as_ref().unwrap_or(&point.name).to_string();

    let doc = [
        point.label.as_ref(),
        point.desc.as_ref(),
        point.detail.as_ref(),
    ]
    .iter()
    .flat_map(|r| r.map(|s| s.clone()))
    .collect();

    let static_value = if point.type_ == PointType::Pad {
        Some("0".to_string())
    } else if let Some(value) = &point.value {
        Some(value.to_string())
    } else if point.name == "L" {
        Some((model_size - 2).to_string())
    } else {
        None
    };

    Some(ResolvedPoint {
        name_snake_case: name.to_snake_case(),
        name_pascal_case: name.to_pascal_case(),
        point_type,
        static_value,
        access: point.access,
        mandatory: point.mandatory,
        doc,
    })
}

pub fn resolve_enum(point: &Point) -> Option<ResolvedEnum> {
    if point.type_ == PointType::Enum16 || point.type_ == PointType::Enum32 {
        let values: Vec<EnumValue> = point
            .symbols
            .iter()
            .map(|symbol| EnumValue {
                name_snake_case: symbol.name.to_snake_case(),
                name_pascal_case: symbol.name.to_pascal_case(),
                discriminant: symbol.value.to_string(),
                doc: [
                    symbol.label.as_ref(),
                    symbol.desc.as_ref(),
                    symbol.detail.as_ref(),
                ]
                .iter()
                .flat_map(|r| r.map(|s| s.clone()))
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

pub fn resolve_model(model: &SunspecModel, file_name: String) -> ResolvedModel {
    let mut features = HashSet::new();

    let model_size = model
        .group
        .points
        .iter()
        .map(|point| point.size as u16)
        .sum();

    let points: Vec<ResolvedPoint> = model
        .group
        .points
        .iter()
        .flat_map(|point| resolve_point(point, &mut features, model_size))
        .collect();

    let enums: Vec<ResolvedEnum> = model.group.points.iter().flat_map(resolve_enum).collect();

    let doc = [
        model.label.as_ref(),
        model.desc.as_ref(),
        model.detail.as_ref(),
    ]
    .iter()
    .flat_map(|r| r.map(|s| s.clone()))
    .collect();

    ResolvedModel {
        name_snake_case: file_name.to_snake_case(),
        name_pascal_case: file_name.to_pascal_case(),
        points,
        features,
        size: model_size,
        doc,
        enums,
    }
}
