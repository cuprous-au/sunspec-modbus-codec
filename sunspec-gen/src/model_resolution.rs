use heck::{ToPascalCase, ToSnakeCase};
use std::collections::HashSet;

use crate::sunspec_schema::{Point, PointAccess, PointMandatory, PointType, SunspecModel};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodegenFeature {
    String,
    Ipv4Addr,
    Ipv6Addr,
}

#[derive(Clone, Copy)]
pub struct WriterSpec {
    pub function_name: &'static str,
    pub allow_offset: bool,
    pub enum_value_cast: Option<&'static str>,
}

pub type DocLines = Vec<String>;

pub struct ResolvedPoint {
    pub name_pascal_case: String,
    pub name_snake_case: String,
    pub raw_type: String,
    pub rust_type: String,
    pub c_type: String,
    pub size: u16,
    pub static_value: Option<String>,
    pub writer: Option<WriterSpec>,
    pub access: PointAccess,
    pub mandatory: PointMandatory,
    pub doc: DocLines,
}
pub struct ResolvedEnum {
    pub name_pascal_case: String,
    pub name_snake_case: String,
    pub size: usize,
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

fn writer(function_name: &'static str, allow_offset: bool) -> Option<WriterSpec> {
    Some(WriterSpec {
        function_name,
        allow_offset,
        enum_value_cast: None,
    })
}

fn resolve_point_type(
    point: &Point,
    features: &mut HashSet<CodegenFeature>,
) -> Option<(String, String, Option<WriterSpec>)> {
    match point.type_ {
        PointType::Uint16 => Some((
            "u16".to_string(),
            "u16".to_string(),
            writer("write_u16", false),
        )),
        PointType::Int16 => Some((
            "i16".to_string(),
            "i16".to_string(),
            writer("write_i16", false),
        )),
        PointType::Int32 => Some((
            "i32".to_string(),
            "i32".to_string(),
            writer("write_i32", true),
        )),
        PointType::Int64 => Some((
            "i64".to_string(),
            "i64".to_string(),
            writer("write_i64", true),
        )),
        PointType::Raw16 => Some((
            "u32".to_string(),
            "u32".to_string(),
            writer("write_u16", false),
        )),
        PointType::Uint32 => Some((
            "u32".to_string(),
            "u32".to_string(),
            writer("write_u32", true),
        )),
        PointType::Uint64 => Some((
            "u64".to_string(),
            "u64".to_string(),
            writer("write_u64", true),
        )),
        PointType::Acc16 => Some((
            "u16".to_string(),
            "u16".to_string(),
            writer("write_u16", false),
        )),
        PointType::Acc32 => Some((
            "u32".to_string(),
            "u32".to_string(),
            writer("write_u32", true),
        )),
        PointType::Acc64 => Some((
            "u64".to_string(),
            "u64".to_string(),
            writer("write_u64", true),
        )),
        PointType::Bitfield16 => Some((
            "u16".to_string(),
            "u16".to_string(),
            writer("write_u16", false),
        )),
        PointType::Bitfield32 => Some((
            "u32".to_string(),
            "u32".to_string(),
            writer("write_u32", true),
        )),
        PointType::Bitfield64 => Some((
            "u64".to_string(),
            "u64".to_string(),
            writer("write_u64", true),
        )),
        PointType::Enum16 => Some((
            point.name.to_pascal_case(),
            point.name.to_pascal_case(),
            Some(WriterSpec {
                function_name: "write_u16",
                allow_offset: false,
                enum_value_cast: Some(" as u16"),
            }),
        )),
        PointType::Enum32 => Some((
            point.name.to_pascal_case(),
            point.name.to_pascal_case(),
            Some(WriterSpec {
                function_name: "write_u32",
                allow_offset: true,
                enum_value_cast: Some(" as u32"),
            }),
        )),
        PointType::Float32 => Some((
            "f32".to_string(),
            "f32".to_string(),
            writer("write_f32", true),
        )),
        PointType::Float64 => Some((
            "f64".to_string(),
            "f64".to_string(),
            writer("write_f64", true),
        )),
        PointType::String => {
            features.insert(CodegenFeature::String);
            Some((
                "&CStr".to_string(),
                "*const c_char".to_string(),
                writer("write_string", true),
            ))
        }
        PointType::Pad => Some((
            "u16".to_string(),
            "u16".to_string(),
            writer("write_u16", false),
        )),
        PointType::Ipaddr => {
            features.insert(CodegenFeature::Ipv4Addr);
            Some((
                "Ipv4Addr".to_string(),
                "Ipv4Addr".to_string(),
                writer("write_ipaddr", true),
            ))
        }
        PointType::Ipv6addr => {
            features.insert(CodegenFeature::Ipv6Addr);
            Some((
                "Ipv6Addr".to_string(),
                "Ipv6Addr".to_string(),
                writer("write_ipv6addr", true),
            ))
        }
        PointType::Eui48 => Some((
            "[u8; 6]".to_string(),
            "[u8; 6]".to_string(),
            writer("write_eui48", true),
        )),
        PointType::Sunssf => Some((
            "u16".to_string(),
            "u16".to_string(),
            writer("write_u16", false),
        )),
        PointType::Count => Some((
            "u16".to_string(),
            "u16".to_string(),
            writer("write_u16", false),
        )),
    }
}

pub fn resolve_point(
    point: &Point,
    features: &mut HashSet<CodegenFeature>,
    model_size: u16,
) -> Option<ResolvedPoint> {
    let (rust_type, c_type, writer) = resolve_point_type(point, features)?;
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
        raw_type: point.type_.to_string().to_pascal_case(),
        rust_type,
        c_type,
        size: point.size as u16,
        static_value,
        writer,
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

        Some(ResolvedEnum {
            name_snake_case: point.name.to_snake_case(),
            name_pascal_case: point.name.to_pascal_case(),
            size: match point.type_ {
                PointType::Enum16 => 16,
                PointType::Enum32 => 32,
                _ => 0,
            },
            values,
        })
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
