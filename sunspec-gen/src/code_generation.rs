use codegen::{Block, Enum, Function, Scope, Type};

use crate::model_resolution::{
    CodegenFeature, DocLines, ResolvedEnum, ResolvedModel, ResolvedPoint,
};
use crate::sunspec_schema::{PointAccess, PointMandatory};

fn doc_text(parts: &DocLines) -> String {
    parts.join("\n\n")
}

fn generate_enum(resolved_enum: &ResolvedEnum) -> Enum {
    let mut enum_def = Enum::new(&resolved_enum.name_pascal_case);

    enum_def.vis("pub").repr(&resolved_enum.discriminant_type);

    for value in &resolved_enum.values {
        enum_def
            .new_variant(&value.name_pascal_case)
            .discriminant(&value.discriminant)
            .doc(doc_text(&value.doc));
    }

    enum_def
}

fn option_unless_mandatory(point: &ResolvedPoint) -> Type {
    let inner_type: Type = (&point.point_type.rust_type).into();
    match point.mandatory {
        PointMandatory::O => Type::new("Option").generic(inner_type).to_owned(),
        PointMandatory::M => inner_type,
    }
}

fn generate_getter(point: &ResolvedPoint) -> Function {
    let mut func = Function::new(&point.name_snake_case);

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

fn generate_setter(point: &ResolvedPoint) -> Option<Function> {
    match point.access {
        PointAccess::R => None,
        PointAccess::Rw => {
            let mut func = Function::new(format!("set_{}", &point.name_snake_case));

            if point.mandatory == PointMandatory::M {
                func.body = None;
            };

            func.arg("value", &point.point_type.rust_type)
                .arg_mut_self()
                .doc(doc_text(&point.doc));
            Some(func)
        }
    }
}

fn generate_point_array(model: &ResolvedModel) -> String {
    let lines: Vec<String> = [format!(
        "pub static POINTS: [ReadablePoint; {}] = [",
        model.points.len()
    )]
    .into_iter()
    .chain(model.points.iter().map(|point| {
        let point_reference = if let Some(value) = &point.static_value {
            format!("PointReference::Static {{ value: {} }}", value)
        } else {
            format!(
                "PointReference::{} {{ point: Point::{} }}",
                model.name_pascal_case, point.name_pascal_case
            )
        };

        format!(
            "    ReadablePoint {{
        reference: {point_reference},
        size: {point_size},
        data_type: PointType::{point_type},
        writeable: {writeable},
    }},",
            point_size = point.point_type.size,
            point_type = point.point_type.raw_type,
            writeable = point.access == PointAccess::Rw
        )
    }))
    .chain(["];".to_string()])
    .collect();

    lines.join("\n")
}

pub fn generate_point_types(models: &[ResolvedModel]) -> Scope {
    let mut scope = Scope::new();

    for model in models {
        scope.import("crate::sunspec::models", &model.name_snake_case);
    }

    let point_reference_enum = scope.new_enum("PointReference").vis("pub").derive("Debug");

    for model in models {
        point_reference_enum
            .new_variant(&model.name_pascal_case)
            .named("point", format!("{}::Point", model.name_snake_case));
    }

    point_reference_enum
        .new_variant("Static")
        .named("value", "u16");

    scope
}

pub fn generate_adapter_structs(models: &[ResolvedModel]) -> Scope {
    let mut scope = Scope::new();

    for model in models {
        scope.import("crate::sunspec::models", &model.name_snake_case);
    }

    let trait_struct = scope.new_struct("SunspecAdapters").vis("pub").generic("'a");

    for model in models {
        trait_struct
            .field(
                format!("pub {}_adapter", model.name_snake_case),
                format!("Option<&'a dyn {}::ModelAdapter>", model.name_snake_case),
            )
            .vis("pub");
    }

    let c_struct = scope
        .new_struct("SunspecCallbackAdapters")
        .vis("pub")
        .repr("C")
        .generic("'a");

    for model in models {
        c_struct
            .field(
                format!("pub {}_adapter", model.name_snake_case),
                format!(
                    "Option<&'a {}::{}CallbackAdapter>",
                    model.name_snake_case, model.name_pascal_case
                ),
            )
            .vis("pub");
    }

    scope
}

pub fn generate_models_mod(models: &[ResolvedModel]) -> Scope {
    let mut scope = Scope::new();
    scope.raw("#![allow(unused_variables)]");

    models.iter().for_each(|model| {
        scope.raw(format!("pub mod {};", model.name_snake_case));
    });

    scope
}

pub fn generate_model(model: &ResolvedModel) -> Scope {
    let mut scope = Scope::new();
    // scope.raw("#![allow(unused_variables)]");

    let funcs: Vec<Function> = model
        .points
        .iter()
        .filter(|point| point.static_value.is_none())
        .flat_map(|point| {
            let getter = generate_getter(point);
            let setter = generate_setter(point);

            let funcs: Vec<Function> = vec![Some(getter), setter]
                .iter()
                .filter_map(|x| x.clone())
                .collect();
            funcs
        })
        .collect();

    for feature in &model.features {
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

    let size: u16 = model.points.iter().map(|point| point.point_type.size).sum();

    scope.raw(format!("pub const SIZE: u16 = {};", size));

    scope.raw(generate_point_array(&model));

    let point_enum = scope.new_enum("Point").vis("pub").derive("Debug");

    for point in model
        .points
        .iter()
        .filter(|point| point.static_value.is_none())
    {
        point_enum.new_variant(&point.name_pascal_case);
    }

    let mut writer_block = Block::new("match point");

    for point in model
        .points
        .iter()
        .filter(|point| point.static_value.is_none())
    {
        if point.static_value.is_none() {
            let value_reader = format!("model.{}()", point.name_snake_case);
            let match_arm = format!("Point::{} =>", point.name_pascal_case);
            let rest_args = if point.point_type.writer_allow_offset {
                ", buffer, offset, limit"
            } else {
                ", buffer"
            };

            let value_cast = point
                .point_type
                .writer_value_cast
                .clone()
                .unwrap_or_default();

            let line = if point.mandatory == PointMandatory::M {
                format!(
                    "{match_arm} serialisation::{}({value_reader}{value_cast}{rest_args}),",
                    point.point_type.writer_function_name
                )
            } else {
                format!(
                    "{match_arm} if let Some(value) = {value_reader} {{ serialisation::{}(value{value_cast}{rest_args}); }},",
                    point.point_type.writer_function_name
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

    for enum_type in &model.enums {
        scope.push_enum(generate_enum(&enum_type));
    }

    let callback_struct = scope
        .new_struct(format!("{}CallbackAdapter", model.name_pascal_case))
        .vis("pub")
        .repr("C");

    for point in model
        .points
        .iter()
        .filter(|point| point.static_value.is_none())
    {
        let c_type = &point.point_type.c_type;
        let fn_ptr = format!("extern \"C\" fn() -> {c_type}");
        callback_struct.field(
            format!("{}_callback", point.name_snake_case),
            if point.mandatory == PointMandatory::M {
                fn_ptr
            } else {
                format!("Option<{fn_ptr}>")
            },
        );

        if point.access == PointAccess::Rw {
            let set_fn_ptr = format!("extern \"C\" fn({c_type})");
            callback_struct.field(
                format!("set_{}_callback", point.name_snake_case),
                if point.mandatory == PointMandatory::M {
                    set_fn_ptr
                } else {
                    format!("Option<{set_fn_ptr}>")
                },
            );
        };
    }

    let callback_impl = scope
        .new_impl(format!("{}CallbackAdapter", model.name_pascal_case))
        .impl_trait("ModelAdapter");
    for point in model
        .points
        .iter()
        .filter(|point| point.static_value.is_none())
    {
        let mut getter = generate_getter(point);
        getter.body = None;

        let callback_ref = if point.mandatory == PointMandatory::M {
            format!("self.{}_callback", point.name_snake_case)
        } else {
            getter.line(format!(
                "self.{}_callback.map(|callback| {{",
                point.name_snake_case
            ));
            "callback".to_string()
        };

        let value = format!("({})()", callback_ref);

        getter.line(if let Some(cast) = point.point_type.cast_from_c {
            cast(&value)
        } else {
            value
        });
        if point.mandatory == PointMandatory::O {
            getter.line("})");
        }
        callback_impl.push_fn(getter);

        if let Some(mut setter) = generate_setter(point) {
            let callback_ref = if point.mandatory == PointMandatory::O {
                setter.line(format!(
                    "if let Some(callback) = self.set_{}_callback {{",
                    point.name_snake_case,
                ));
                "callback".to_string()
            } else {
                format!("self.set_{}_callback", point.name_snake_case)
            };

            setter.line(if point.point_type.cast_from_c.is_some() {
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
