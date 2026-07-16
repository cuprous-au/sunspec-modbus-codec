use codegen::{Enum, Field, Function, Scope, Type};
use glob::glob;
use heck::{ToPascalCase, ToSnakeCase};
use std::{collections::HashSet, ffi::OsStr, fs};
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

fn generate_getter(point: &Point, field_type: &Type) -> Function {
    let mut func = Function::new(point.name.to_snake_case());

    if point.mandatory == PointMandatory::O {
        func.line("None");
    } else {
        func.body = None;
    };

    func.ret(option_unless_mandatory(point, field_type)).arg_ref_self().doc(
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

fn generate_setter(point: &Point, field_type: &Type) -> Option<Function> {
    match point.access {
        PointAccess::R => None,
        PointAccess::Rw => {
            let mut func = Function::new(format!("set_{}", point.name.to_snake_case()));

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

fn generate_model(model: SunspecModel, name: String) -> String {
    let mut scope = Scope::new();
    let mut features: HashSet<CodegenFeature> = HashSet::new();
    let model_name = model.group.name.to_pascal_case();

    let point_types: Vec<(Type, &Point)> = model
        .group
        .points
        .iter()
        .flat_map(|point| generate_type(point, &mut features).map(|t| (t, point)))
        .collect();

    let enums: Vec<Enum> = model
        .group
        .points
        .iter()
        .flat_map(|point| generate_enum(&point))
        .collect();

    let fields: Vec<Field> = point_types
        .iter()
        .map(|(field_type, point)| generate_field(point, &field_type))
        .collect();

    let funcs: Vec<Function> = point_types
        .iter()
        .flat_map(|(field_type, point)| {
            let getter = generate_getter(point, &field_type);
            let setter = generate_setter(point, &field_type);

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

    if name != model_name {
        scope.new_type_alias(&name, &model_name).vis("pub");
    }

    let root_struct = scope.new_struct(&model_name).vis("pub");

    if let Some(desc) = model.group.desc {
        root_struct.doc(desc);
    };

    for field in fields {
        root_struct.push_field(field);
    }

    let root_trait = scope.new_trait("ModelCallbacks");

    for func in funcs {
        root_trait.push_fn(func);
    }

    let register_readers_func = scope.new_fn(&model.group.name.to_snake_case())

    for enum_type in enums {
        scope.push_enum(enum_type);
    }

    scope.to_string()
}

fn main() -> () {
    static GENERATED_SRC_DIR: &str = "../src/sunspec";
    let model_glob = "./models/json/model_*.json";

    fs::remove_dir_all(GENERATED_SRC_DIR).unwrap();
    fs::create_dir_all(GENERATED_SRC_DIR).unwrap();
    let models: Vec<String> = glob(model_glob)
        .unwrap()
        .flat_map(|entry| match entry {
            Ok(path) => {
                let json = fs::read_to_string(&path).unwrap();
                let model: SunspecModel = serde_json::from_str(&json).unwrap();
                let model_name = path.file_prefix().map(OsStr::to_str).flatten().unwrap();

                fs::write(
                    format!("{}/{}.rs", GENERATED_SRC_DIR, model_name.to_snake_case()),
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
        format!("{}.rs", GENERATED_SRC_DIR),
        models
            .into_iter()
            .map(|n| format!("pub mod {};", n))
            .collect::<Vec<String>>()
            .join("\n\n"),
    )
    .unwrap()
}
