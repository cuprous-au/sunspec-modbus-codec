use codegen::{Block, Enum, Field, Function, Scope, Type};
use glob::glob;
use heck::{ToPascalCase, ToSnakeCase};
use std::{collections::HashSet, ffi::OsStr, fmt::write, fs};
use typify::import_types;

// TODO: I've edited this schema file to add `"title": "SunspecModel"`
// This will need to be incorporated into code here or a scripted process
import_types!(schema = "./models/json/schema.json");

#[derive(PartialEq, Eq, Hash)]
enum CodegenFeature {
    HeaplessString,
    Ipv4Addr,
    Ipv6Addr,
}
fn generate_enum(point: &Point) -> Option<Enum> {
    if point.type_ == PointType::Enum16 || point.type_ == PointType::Enum32 {
        let mut enum_def = Enum::new(point.name.to_pascal_case());

        enum_def.vis("pub");

        for symbol in &point.symbols {
            enum_def
                .new_variant(symbol.name.to_pascal_case())
                .discriminant(symbol.value.to_string())
                .doc(
                    vec![
                        symbol.label.clone(),
                        symbol.desc.clone(),
                        symbol.detail.clone(),
                    ]
                    .iter()
                    .flatten()
                    .map(String::as_str)
                    .collect::<Vec<&str>>()
                    .join("\n\n"),
                );
        }

        Some(enum_def)
    } else {
        None
    }
}

fn option_unless_mandatory(point: &Point, field_type: &Type) -> Type {
    match point.mandatory {
        PointMandatory::O => Type::new("Option").generic(field_type).to_owned(),
        PointMandatory::M => field_type.clone(),
    }
}

fn generate_type(point: &Point, features: &mut HashSet<CodegenFeature>) -> Option<Type> {
    match point.r#type_ {
        PointType::Uint16 => Some("u16".into()),
        PointType::Int16 => Some("i16".into()),
        PointType::Int32 => Some("i32".into()),
        PointType::Int64 => Some("i64".into()),
        PointType::Raw16 => Some("u32".into()),
        PointType::Uint32 => Some("u32".into()),
        PointType::Uint64 => Some("u64".into()),
        PointType::Acc16 => Some("u16".into()),
        PointType::Acc32 => Some("u32".into()),
        PointType::Acc64 => Some("u64".into()),
        PointType::Bitfield16 => Some("u16".into()),
        PointType::Bitfield32 => Some("u32".into()),
        PointType::Bitfield64 => Some("u64".into()),
        PointType::Enum16 | PointType::Enum32 => Some(point.name.to_pascal_case().into()),
        PointType::Float32 => Some("f32".into()),
        PointType::Float64 => Some("f64".into()),
        PointType::String => {
            features.insert(CodegenFeature::HeaplessString);
            let str = format!("String<{}>", point.size * 2);
            Some(str.into())
        }
        PointType::Pad => None,
        PointType::Ipaddr => {
            features.insert(CodegenFeature::Ipv4Addr);
            Some("Ipv4Addr".into())
        }
        PointType::Ipv6addr => {
            features.insert(CodegenFeature::Ipv6Addr);
            Some("Ipv6Addr".into())
        }
        PointType::Eui48 => Some("[u8; 6]".into()), // todo: consider a crate like mac-addr
        PointType::Sunssf => Some("u16".into()),
        // this doesn't seem to be referenced outside the spec, but all map back to u16 in the documentation
        PointType::Count => Some("u16".into()),
    }
}

fn generate_field(point: &Point, field_type: &Type) -> Field {
    let mut field = Field::new(
        point.name.to_snake_case(),
        option_unless_mandatory(point, field_type),
    );

    field.doc(
        vec![
            point.label.clone(),
            point.desc.clone(),
            point.detail.clone(),
        ]
        .iter()
        .flatten()
        .map(String::as_str)
        .collect::<Vec<&str>>()
        .join("\n\n"),
    );
    field
}

fn generate_getter(point: &Point, field_type: &Type, name: &String) -> Function {
    let mut func = Function::new(name.to_snake_case());

    if point.mandatory == PointMandatory::O {
        func.line("None");
    } else {
        func.body = None;
    };

    func.ret(option_unless_mandatory(point, field_type))
        .arg_ref_self()
        .doc(
            vec![
                point.label.clone(),
                point.desc.clone(),
                point.detail.clone(),
            ]
            .iter()
            .flatten()
            .map(String::as_str)
            .collect::<Vec<&str>>()
            .join("\n\n"),
        );
    func
}

fn generate_setter(point: &Point, field_type: &Type, name: &String) -> Option<Function> {
    match point.access {
        PointAccess::R => None,
        PointAccess::Rw => {
            let mut func = Function::new(format!("set_{}", name.to_snake_case()));

            if point.mandatory == PointMandatory::M {
                func.body = None;
            };

            func.arg("value", field_type).arg_mut_self().doc(
                vec![
                    point.label.clone(),
                    point.desc.clone(),
                    point.detail.clone(),
                ]
                .iter()
                .flatten()
                .map(String::as_str)
                .collect::<Vec<&str>>()
                .join("\n\n"),
            );
            Some(func)
        }
    }
}

fn generate_point_array(model: &SunspecModel, model_name: String, model_size: u16) -> String {
    let lines: Vec<String> = [format!(
        "pub static POINTS: [ReadablePoint; {}] = [",
        model.group.points.len()
    )]
    .into_iter()
    .chain(model.group.points.iter().map(|point| {
        let point_reference = if point.type_ == PointType::Pad {
            "PointReference::Static { value: 0 }".to_string()
        } else if let Some(value) = &point.value {
            format!("PointReference::Static {{ value: {} }}", value)
        } else if point.name == "L" {
            format!("PointReference::Static {{ value: {} }}", model_size - 2)
        } else {
            format!(
                "PointReference::{} {{ point: Point::{} }}",
                model_name,
                point.label.as_ref().unwrap_or(&point.name).to_pascal_case()
            )
        };

        format!(
            "    ReadablePoint {{
        reference: {point_reference},
        size: {point_size},
        data_type: PointType::{point_type},
        writeable: {writeable},
    }},",
            point_size = point.size,
            point_type = point.type_.to_string().to_pascal_case(),
            writeable = point.access == PointAccess::Rw
        )
    }))
    .chain(["];".to_string()])
    .collect();

    lines.join("\n")
}

fn generate_point_types(models: &Vec<String>) -> String {
    let mut scope = Scope::new();

    for model in models {
        scope.import("crate::sunspec::models", model.to_snake_case());
    }

    let point_reference_enum = scope.new_enum("PointReference").vis("pub").derive("Debug");

    for model in models {
        point_reference_enum
            .new_variant(model.to_pascal_case())
            .named("point", format!("{}::Point", model.to_snake_case()));
    }

    point_reference_enum
        .new_variant("Static")
        .named("value", "u16");

    scope.to_string()
}

fn generate_model(model: SunspecModel, name: String) -> String {
    let mut scope = Scope::new();
    let mut features: HashSet<CodegenFeature> = HashSet::new();

    let model_name = name.to_pascal_case();

    let point_types: Vec<(Type, &Point, &String)> = model
        .group
        .points
        .iter()
        .flat_map(|point| {
            generate_type(point, &mut features)
                .map(|t| (t, point, point.label.as_ref().unwrap_or(&point.name)))
        })
        .collect();

    let enums: Vec<Enum> = model
        .group
        .points
        .iter()
        .flat_map(|point| generate_enum(&point))
        .collect();

    let fields: Vec<Field> = point_types
        .iter()
        .map(|(field_type, point, _)| generate_field(point, &field_type))
        .collect();

    let funcs: Vec<Function> = point_types
        .iter()
        // TODO: These fields (id and model length) are static and don't need an adapter method, but we should make this data driven
        .skip(2)
        .flat_map(|(field_type, point, name)| {
            let getter = generate_getter(point, &field_type, name);
            let setter = generate_setter(point, &field_type, name);

            let funcs: Vec<Function> = vec![Some(getter), setter]
                .iter()
                .filter_map(|x| x.clone())
                .collect();
            funcs
        })
        .collect();

    for feature in &features {
        match feature {
            CodegenFeature::HeaplessString => scope.import("heapless", "String"),
            CodegenFeature::Ipv4Addr => scope.import("core::net", "Ipv4Addr"),
            CodegenFeature::Ipv6Addr => scope.import("core::net", "Ipv6Addr"),
        };
    }

    scope.import("crate", "serialisation");
    scope.import("crate::sunspec", "{PointType, ReadablePoint}");
    scope.import("crate::sunspec::points", "PointReference");

    let size: u16 = model
        .group
        .points
        .iter()
        .map(|point| point.size as u16)
        .sum();

    scope.raw(format!("pub const SIZE: u16 = {};", size));

    scope.raw(generate_point_array(&model, model_name, size));

    let point_enum = scope.new_enum("Point").vis("pub").derive("Debug");

    for (_, _, name) in point_types.iter().skip(2) {
        point_enum.new_variant(name.to_pascal_case());
    }

    let mut writer_block = Block::new("match point");

    for (_, point, name) in point_types.iter().skip(2) {
        if let Some((writer, allow_offset)) = match point.type_ {
            PointType::Int16 => Some(("write_i16", false)),
            PointType::Int32 => Some(("write_i32", true)),
            PointType::Int64 => Some(("write_i64", true)),
            PointType::Uint16
            | PointType::Raw16
            | PointType::Acc16
            | PointType::Bitfield16
            | PointType::Enum16
            | PointType::Sunssf
            | PointType::Count => Some(("write_u16", false)),
            PointType::Uint32 | PointType::Acc32 | PointType::Bitfield32 | PointType::Enum32 => {
                Some(("write_u32", true))
            }
            PointType::Uint64 | PointType::Acc64 | PointType::Bitfield64 => {
                Some(("write_u64", true))
            }
            PointType::Float32 => Some(("write_f32", true)),
            PointType::Float64 => Some(("write_f64", true)),
            PointType::String => Some(("write_string", true)),
            PointType::Ipaddr => Some(("write_ipaddr", true)),
            PointType::Ipv6addr => Some(("write_ipv6addr", true)),
            PointType::Eui48 => Some(("write_eui48", true)),
            PointType::Pad => None,
        } {
            let value_reader = format!("model.{}()", name.to_snake_case());
            let match_arm = format!("Point::{} =>", name.to_pascal_case());
            let rest_args = if allow_offset {
                ", buffer, offset, limit"
            } else {
                ", buffer"
            };

            let value_cast = match point.type_ {
                PointType::Enum16 => " as u16",
                PointType::Enum32 => " as u32",
                _ => "",
            };

            let line = if point.mandatory == PointMandatory::M {
                format!(
                    "{match_arm} serialisation::{writer}({value_reader}{value_cast}{rest_args}),"
                )
            } else {
                format!(
                    "{match_arm} if let Some(value) = {value_reader} {{ serialisation::{writer}(value{value_cast}{rest_args}); }},"
                )
            };

            writer_block.line(line);
        }
    }

    let writer_func = scope
        .new_fn("write_point")
        .vis("pub")
        .arg("model", "&dyn ModelAdapter")
        .arg("point", "&Point")
        .arg("buffer", "&mut [u16]")
        .arg("offset", "u16")
        .arg("limit", "u16")
        .push_block(writer_block);

    let root_trait = scope.new_trait("ModelAdapter").vis("pub");

    for func in funcs {
        root_trait.push_fn(func);
    }

    for enum_type in enums {
        scope.push_enum(enum_type);
    }

    scope.to_string()
}

fn main() -> () {
    static GENERATED_SRC_DIR: &str = "../src/sunspec";
    let model_glob = "./models/json/model_*.json";

    fs::remove_dir_all(GENERATED_SRC_DIR).unwrap();
    fs::create_dir_all(format!("{}/models", GENERATED_SRC_DIR)).unwrap();

    let models: Vec<String> = glob(model_glob)
        .unwrap()
        .filter(|entry| match entry {
            Ok(path) => {
                let model_name = path.file_prefix().map(OsStr::to_str).flatten().unwrap();
                println!("{model_name}");
                model_name != "model_9"
                    && model_name != "model_14"
                    && model_name != "model_302"
                    && model_name != "model_303"
                    && model_name != "model_304"
                    && model_name != "model_601"
                    && model_name != "model_702"
                    && model_name != "model_63002"
            }
            _ => true,
        })
        .flat_map(|entry| match entry {
            Ok(path) => {
                let json = fs::read_to_string(&path).unwrap();
                let model: SunspecModel = serde_json::from_str(&json).unwrap();
                let model_name = path.file_prefix().map(OsStr::to_str).flatten().unwrap();

                fs::write(
                    format!(
                        "{}/models/{}.rs",
                        GENERATED_SRC_DIR,
                        model_name.to_snake_case()
                    ),
                    generate_model(model, model_name.to_pascal_case()),
                )
                .unwrap();

                Some(model_name.to_string())
            }
            Err(e) => {
                println!("{:?}", e);
                None
            }
        })
        .collect();

    fs::write(
        format!("{}/points.rs", GENERATED_SRC_DIR),
        generate_point_types(&models),
    )
    .unwrap();

    let mut mod_scope = Scope::new();

    models.into_iter().for_each(|n| {
        mod_scope.raw(format!("pub mod {};", n));
    });

    fs::write(
        format!("{}/models.rs", GENERATED_SRC_DIR),
        mod_scope.to_string(),
    )
    .unwrap()
}
