use codegen::{Block, Enum, Field, Function, Scope, Type};
use glob::glob;
use heck::{ToPascalCase, ToSnakeCase};
use std::{collections::HashSet, ffi::OsStr, fmt::write, fs};

use crate::sunspec_schema::{Point, PointAccess, PointMandatory, PointType, SunspecModel};

mod sunspec_schema;

#[derive(PartialEq, Eq, Hash)]
enum CodegenFeature {
    String,
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

fn generate_type(point: &Point, features: &mut HashSet<CodegenFeature>) -> Option<String> {
    match point.r#type_ {
        PointType::Uint16 => Some("u16".to_string()),
        PointType::Int16 => Some("i16".to_string()),
        PointType::Int32 => Some("i32".to_string()),
        PointType::Int64 => Some("i64".to_string()),
        PointType::Raw16 => Some("u32".to_string()),
        PointType::Uint32 => Some("u32".to_string()),
        PointType::Uint64 => Some("u64".to_string()),
        PointType::Acc16 => Some("u16".to_string()),
        PointType::Acc32 => Some("u32".to_string()),
        PointType::Acc64 => Some("u64".to_string()),
        PointType::Bitfield16 => Some("u16".to_string()),
        PointType::Bitfield32 => Some("u32".to_string()),
        PointType::Bitfield64 => Some("u64".to_string()),
        PointType::Enum16 | PointType::Enum32 => Some(point.name.to_pascal_case()),
        PointType::Float32 => Some("f32".to_string()),
        PointType::Float64 => Some("f64".to_string()),
        PointType::String => {
            features.insert(CodegenFeature::String);
            Some("&CStr".to_string())
        }
        PointType::Pad => None,
        PointType::Ipaddr => {
            features.insert(CodegenFeature::Ipv4Addr);
            Some("Ipv4Addr".to_string())
        }
        PointType::Ipv6addr => {
            features.insert(CodegenFeature::Ipv6Addr);
            Some("Ipv6Addr".to_string())
        }
        PointType::Eui48 => Some("[u8; 6]".to_string()), // todo: consider a crate like mac-addr
        PointType::Sunssf => Some("u16".to_string()),
        // this doesn't seem to be referenced outside the spec, but all map back to u16 in the documentation
        PointType::Count => Some("u16".to_string()),
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

fn generate_point_types(models: &Vec<String>) -> Scope {
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

    scope
}

fn generate_adapter_structs(models: &Vec<String>) -> Scope {
    let mut scope = Scope::new();

    for model in models {
        scope.import("crate::sunspec::models", model.to_snake_case());
    }

    let trait_struct = scope.new_struct("SunspecAdapters").vis("pub").generic("'a");

    for model in models {
        trait_struct.field(
            format!("{}_adapter", model.to_snake_case()),
            format!("Option<&'a dyn {}::ModelAdapter>", model.to_snake_case()),
        );
    }

    let c_struct = scope
        .new_struct("SunspecCallbackAdapters")
        .vis("pub")
        .repr("C")
        .generic("'a");

    for model in models {
        c_struct.field(
            format!("{}_adapter", model.to_snake_case()),
            format!(
                "Option<&'a dyn {}::{}CallbackAdapter>",
                model.to_snake_case(),
                model.to_pascal_case()
            ),
        );
    }

    scope
}

fn generate_model(model: SunspecModel, name: String) -> Scope {
    let mut scope = Scope::new();
    let mut features: HashSet<CodegenFeature> = HashSet::new();

    let model_name = name.to_pascal_case();

    let point_types: Vec<(String, &Point, &String)> = model
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
        .map(|(field_type, point, _)| generate_field(point, &field_type.into()))
        .collect();

    let funcs: Vec<Function> = point_types
        .iter()
        // TODO: These fields (id and model length) are static and don't need an adapter method, but we should make this data driven
        .skip(2)
        .flat_map(|(field_type, point, name)| {
            let getter = generate_getter(point, &field_type.into(), name);
            let setter = generate_setter(point, &field_type.into(), name);

            let funcs: Vec<Function> = vec![Some(getter), setter]
                .iter()
                .filter_map(|x| x.clone())
                .collect();
            funcs
        })
        .collect();

    for feature in &features {
        match feature {
            CodegenFeature::String => {
                scope.import("core::ffi", "CStr");
                scope.import("core::ffi", "c_char")
            }
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

    scope
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

    let callback_struct = scope
        .new_struct(format!("{}CallbackAdapter", name))
        .vis("pub")
        .repr("C");

    for (point_type, point, name) in point_types.iter().skip(2) {
        let c_type = match point.type_ {
            PointType::String => "*const c_char",
            _ => point_type,
        };
        let fn_ptr = format!("extern \"C\" fn() -> {c_type}");
        callback_struct.field(
            format!("{}_callback", name.to_snake_case()),
            if point.mandatory == PointMandatory::M {
                fn_ptr
            } else {
                format!("Option<{fn_ptr}>")
            },
        );

        if point.access == PointAccess::Rw {
            let set_fn_ptr = format!("extern \"C\" fn({c_type})");
            callback_struct.field(
                format!("set_{}_callback", name.to_snake_case()),
                if point.mandatory == PointMandatory::M {
                    set_fn_ptr
                } else {
                    format!("Option<{set_fn_ptr}>")
                },
            );
        };
    }

    let callback_impl = scope
        .new_impl(format!("{}CallbackAdapter", name))
        .impl_trait("ModelAdapter");
    for (field_type, point, name) in point_types.iter().skip(2) {
        let mut getter = generate_getter(point, &field_type.into(), name);
        getter.body = None;

        let callback_ref = if point.mandatory == PointMandatory::M {
            format!("self.{}_callback", name.to_snake_case())
        } else {
            getter.line(format!(
                "self.{}_callback.map(|callback| {{",
                name.to_snake_case()
            ));
            "callback".to_string()
        };

        getter.line(if point.type_ == PointType::String {
            format!("unsafe {{ CStr::from_ptr(({})()) }}", callback_ref)
        } else {
            format!("({})()", callback_ref)
        });
        if point.mandatory == PointMandatory::O {
            getter.line("})");
        }
        callback_impl.push_fn(getter);

        if let Some(mut setter) = generate_setter(point, &field_type.into(), name) {
            let callback_ref = if point.mandatory == PointMandatory::O {
                setter.line(format!(
                    "if let Some(callback) = self.set_{}_callback {{",
                    name.to_snake_case(),
                ));
                "callback".to_string()
            } else {
                format!("self.set_{}_callback", name.to_snake_case())
            };

        setter.line(if point.type_ == PointType::String {
            format!("({})(value.as_ptr());", callback_ref)
        } else {
            format!("({})(value);", callback_ref)
        });

            if point.mandatory == PointMandatory::O {
                setter.line("};");
            }
            callback_impl.push_fn(setter);
        }
    }

    scope
}
static GENERATED_SRC_DIR: &str = "../src/sunspec";

fn format_and_write(path: String, scope: &Scope) -> std::io::Result<()> {
    let text = rustfmt_wrapper::rustfmt(scope.to_string()).unwrap();
    fs::write(format!("{}/{}", GENERATED_SRC_DIR, path), text)
}

fn main() -> () {
    let model_glob = "./models/json/model_*.json";

    fs::remove_dir_all(GENERATED_SRC_DIR).unwrap();
    fs::create_dir_all(format!("{}/models", GENERATED_SRC_DIR)).unwrap();

    let models: Vec<String> = glob(model_glob)
        .unwrap()
        .filter(|entry| match entry {
            Ok(path) => {
                let model_name = path.file_prefix().map(OsStr::to_str).flatten().unwrap();
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

                format_and_write(
                    format!("models/{}.rs", model_name.to_snake_case()),
                    &generate_model(model, model_name.to_pascal_case()),
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

    format_and_write("points.rs".to_string(), &generate_point_types(&models)).unwrap();
    format_and_write(
        "adapters.rs".to_string(),
        &generate_adapter_structs(&models),
    )
    .unwrap();

    let mut mod_scope = Scope::new();

    models.into_iter().for_each(|n| {
        mod_scope.raw(format!("pub mod {};", n));
    });

    format_and_write("models.rs".to_string(), &mod_scope).unwrap();
}
