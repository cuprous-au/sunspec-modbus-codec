use codegen::{Block, Enum, Function, Scope, Type};

use crate::model_resolution::{
    CodegenFeature, DocLines, PointValueType, ResolvedEnum, ResolvedGroup, ResolvedModel,
    ResolvedPoint,
};
use crate::sunspec_schema::{PointAccess, PointMandatory};

fn doc_text(parts: &DocLines) -> String {
    parts.join("\n\n")
}

fn generate_enum(resolved_enum: &ResolvedEnum) -> Enum {
    let mut enum_def = Enum::new(&resolved_enum.name_pascal_case);

    enum_def
        .vis("pub")
        .repr(&resolved_enum.discriminant_type)
        .derive("Clone")
        .derive("Copy");

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
        model.group.points.len()
    )]
    .into_iter()
    .chain(model.group.points.iter().map(|point| {
        let point_reference = if let PointValueType::StaticValue(value) = &point.value_type {
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
    let mut scope: Scope = Scope::new();

    for model in models {
        scope.import("crate::sunspec::models", &model.name_snake_case);
    }

    scope.import("crate", "ReadablePoint");
    scope.import("crate::buffer", "ModbusBuffer");
    scope.import("crate::buffer", "write_u16");
    scope.import("crate::sunspec::points", "PointReference");

    let adapter_trait = scope
        .new_trait("SunspecAdapterProvider")
        .vis("pub")
        .generic("'a");
    for model in models {
        adapter_trait
            .new_fn(format!("{}_adapter", model.name_snake_case))
            .arg_ref_self()
            .ret(format!(
                "Option<&'a dyn {}::ModelAdapter>",
                model.name_snake_case
            ));
    }

    let trait_struct = scope
        .new_struct("SunspecAdapters")
        .vis("pub")
        .generic("'a")
        .derive("Default");

    for model in models {
        trait_struct
            .field(
                format!("pub {}_adapter", model.name_snake_case),
                format!("Option<&'a dyn {}::ModelAdapter>", model.name_snake_case),
            )
            .vis("pub");
    }

    let trait_struct_impl = scope
        .new_impl("SunspecAdapters<'a>")
        .impl_trait("SunspecAdapterProvider<'a>")
        .generic("'a");

    for model in models {
        trait_struct_impl
            .new_fn(format!("{}_adapter", model.name_snake_case))
            .arg_ref_self()
            .ret(format!(
                "Option<&'a dyn {}::ModelAdapter>",
                model.name_snake_case
            ))
            .line(format!("self.{}_adapter", model.name_snake_case));
    }

    let c_struct = scope
        .new_struct("SunspecExternalAdapters")
        .vis("pub")
        .repr("C")
        .generic("'a");

    for model in models {
        c_struct
            .field(
                format!("pub {}_callback_adapter", model.name_snake_case),
                format!(
                    "Option<&'a {}::{}CallbackAdapter>",
                    model.name_snake_case, model.name_pascal_case
                ),
            )
            .field(
                format!("pub {}_stateful_adapter", model.name_snake_case),
                format!(
                    "Option<&'a {}::{}StatefulAdapter>",
                    model.name_snake_case, model.name_pascal_case
                ),
            )
            .vis("pub");
    }
    let c_struct_impl = scope
        .new_impl("SunspecExternalAdapters<'a>")
        .impl_trait("SunspecAdapterProvider<'a>")
        .generic("'a");

    for model in models {
        c_struct_impl
            .new_fn(format!("{}_adapter", model.name_snake_case))
            .arg_ref_self()
            .ret(format!(
                "Option<&'a dyn {}::ModelAdapter>",
                model.name_snake_case
            ))
            .line(format!("self.{}_callback_adapter", model.name_snake_case))
            .line(format!(
                ".map(|a| a as &'a dyn {}::ModelAdapter)",
                model.name_snake_case
            ))
            .line(format!(
                ".or(self.{}_stateful_adapter.map(|a| a as &'a dyn {}::ModelAdapter))",
                model.name_snake_case, model.name_snake_case,
            ));
    }

    let points_fn = scope
        .new_fn("points_array_and_offset")
        .vis("pub")
        .generic("'a")
        .arg("adapters", "&'a dyn SunspecAdapterProvider<'a>")
        .arg("address", "u16")
        .ret("Option<(&'a [ReadablePoint], u16)>")
        .line("let mut offset = address;");

    let mut header_block = Block::new("if offset < 2");
    header_block.line("return Some((&crate::HEADER_POINTS as &'a [ReadablePoint], offset));");

    points_fn.push_block(header_block);
    let mut else_block = Block::new("else");
    else_block.line("offset -= 2;").after(";");

    points_fn.push_block(else_block);

    for (i, model) in models.iter().enumerate() {
        let name = &model.name_snake_case;

        let adapter_exists_condition = format!("adapters.{name}_adapter().is_some()");
        let within_size_condition = format!("offset < {name}::SIZE");
        let return_points =
            format!("return Some((&{name}::POINTS as &'a [ReadablePoint], offset));");
        if i < models.len() - 1 {
            // Not final block, include full if/else
            let mut outer_block = Block::new(format!("if {adapter_exists_condition}"));
            let mut within_size_block = Block::new(format!("if {within_size_condition}"));
            within_size_block.line(return_points);
            let mut else_block = Block::new("else");
            else_block.line(format!("offset -= {name}::SIZE;"));
            outer_block.push_block(within_size_block);
            outer_block.push_block(else_block);
            points_fn.push_block(outer_block);
        } else {
            let mut block = Block::new(format!(
                "if {adapter_exists_condition} && {within_size_condition}"
            ));
            block.line(return_points);
            points_fn.push_block(block);
        };
    }

    points_fn.line("None");

    let writer_fn = scope
        .new_fn("write_point")
        .vis("pub")
        .generic("'a")
        .generic("'b")
        .arg("adapters", "&'a dyn SunspecAdapterProvider<'a>")
        .arg("point_ref", "&PointReference")
        .arg("buffer", "ModbusBuffer<'b>")
        .arg("offset", "u16")
        .arg("limit", "u16");

    let mut matcher = Block::new("match point_ref");
    matcher.line("PointReference::Static { value } => write_u16(*value, buffer),");
    for model in models {
        let mut match_block = Block::new(format!(
            "PointReference::{} {{ point }} =>",
            model.name_pascal_case
        ));

        match_block
            .line(format!("{}::write_point(", &model.name_snake_case))
            .line(format!(
                "adapters.{}_adapter().unwrap(),",
                &model.name_snake_case
            ))
            .line("point,")
            .line("buffer,")
            .line("offset,")
            .line("limit,")
            .line(");")
            .after(",");

        matcher.push_block(match_block);
    }

    writer_fn.push_block(matcher);

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

pub fn generate_callback_struct(model: &ResolvedModel, scope: &mut Scope) {
    let callback_struct = scope
        .new_struct(format!("{}CallbackAdapter", model.name_pascal_case))
        .vis("pub")
        .repr("C");

    callback_struct.field("context", "*mut c_void");

    for point in model
        .group
        .points
        .iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
    {
        let c_type = if point.point_type.array_length.is_some() {
            format!("*const {}", &point.point_type.c_type)
        } else {
            point.point_type.c_type.clone()
        };
        let fn_ptr = format!("extern \"C\" fn(*const c_void) -> {c_type}");
        callback_struct.field(
            format!("{}_callback", point.name_snake_case),
            if point.mandatory == PointMandatory::M {
                fn_ptr
            } else {
                format!("Option<{fn_ptr}>")
            },
        );

        if point.access == PointAccess::Rw {
            let set_fn_ptr = format!("extern \"C\" fn({c_type}, *mut c_void)");
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
        .group
        .points
        .iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
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

        let value = format!("({})(self.context)", callback_ref);

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
                format!("({})(value.as_ptr(), self.context);", callback_ref)
            } else {
                format!("({})(value, self.context);", callback_ref)
            });

            if point.mandatory == PointMandatory::O {
                setter.line("};");
            }
            callback_impl.push_fn(setter);
        }
    }
}

fn generate_model_length_calculator(group: &ResolvedGroup) -> String {
    if let Some(repeating_block) = &group.repeating_child {
        let repeat_count_pt = repeating_block.repeat_count_point.as_ref().unwrap();
        let repeat_count_ref = match repeat_count_pt.mandatory {
            PointMandatory::M => format!("model.{}()", repeat_count_pt.name_snake_case),
            PointMandatory::O => {
                format!("model.{}().unwrap_or(0)", repeat_count_pt.name_snake_case)
            }
        };
        format!(
            "{} + {repeat_count_ref} * ({})",
            group.static_size,
            generate_model_length_calculator(repeating_block)
        )
    } else {
        group.static_size.to_string()
    }
}

pub fn generate_stateful_struct(model: &ResolvedModel, scope: &mut Scope) {
    let stateful_struct = scope
        .new_struct(format!("{}StatefulAdapter", model.name_pascal_case))
        .vis("pub")
        .repr("C");

    for point in model
        .group
        .points
        .iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
    {
        let c_type = if let Some(array_length) = point.point_type.array_length {
            format!("[{}; {}]", &point.point_type.c_type, array_length)
        } else {
            point.point_type.c_type.clone()
        };
        stateful_struct.field(&point.name_snake_case, c_type);
    }

    let stateful_impl = scope
        .new_impl(format!("{}StatefulAdapter", model.name_pascal_case))
        .impl_trait("ModelAdapter");
    for point in model
        .group
        .points
        .iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
    {
        let mut getter = generate_getter(point);
        getter.body = None;

        let field = if point.point_type.array_length.is_some() {
            format!("self.{}.as_ptr()", point.name_snake_case)
        } else {
            format!("self.{}", point.name_snake_case)
        };
        if point.mandatory == PointMandatory::O {
            getter.line("Some(");
        }

        getter.line(if let Some(cast) = point.point_type.cast_from_c {
            cast(&field)
        } else {
            field
        });

        if point.mandatory == PointMandatory::O {
            getter.line(")");
        }
        stateful_impl.push_fn(getter);

        if let Some(mut setter) = generate_setter(point) {
            if point.point_type.array_length.is_none() {
                setter.line(format!("self.{} = value;", point.name_snake_case));
            } else {
                const ITER: &str = "value.to_bytes_with_nul().iter()";
                let mut block = Block::new(format!(
                    "for (dest, src) in self.{}.iter_mut().zip({})",
                    point.name_snake_case, ITER
                ));
                block.line("*dest = *src as c_char;");
                setter.push_block(block);
            }

            stateful_impl.push_fn(setter);
        }
    }
}

pub fn generate_model(model: &ResolvedModel) -> Scope {
    let mut scope = Scope::new();

    let funcs: Vec<Function> = model
        .group
        .points
        .iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
        .flat_map(|point| {
            let getter = generate_getter(point);
            let setter = generate_setter(point);

            let funcs: Vec<Function> = [Some(getter), setter]
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
            } // CodegenFeature::Ipv4Addr => scope.import("core::net", "Ipv4Addr"),
              // CodegenFeature::Ipv6Addr => scope.import("core::net", "Ipv6Addr"),
        };
    }

    scope.import("core::ffi", "c_void");
    scope.import("crate::buffer", "self");
    scope.import("crate::buffer", "ModbusBuffer");
    scope.import("crate::sunspec", "{PointType, ReadablePoint}");
    scope.import("crate::sunspec::points", "PointReference");

    let size: u16 = model
        .group
        .points
        .iter()
        .map(|point| point.point_type.size)
        .sum();

    scope.raw(format!("pub const SIZE: u16 = {};", size));

    scope.raw(generate_point_array(model));

    let point_enum = scope.new_enum("Point").vis("pub").derive("Debug");

    for point in model.group.points.iter().filter(|point| {
        point.value_type == PointValueType::Adapter
            || point.value_type == PointValueType::ModelLength
    }) {
        point_enum.new_variant(&point.name_pascal_case);
    }

    scope
        .new_fn("model_length")
        .vis("pub")
        .arg("model", "&dyn ModelAdapter")
        .ret("u16")
        .line(generate_model_length_calculator(&model.group));

    let mut writer_block = Block::new("match point");

    for point in model.group.points.iter() {
        let match_arm = format!("Point::{} =>", point.name_pascal_case);
        match point.value_type {
            PointValueType::Adapter => {
                let value_reader = format!("model.{}()", point.name_snake_case);
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

                let mut match_block = Block::new(match_arm);
                if point.mandatory == PointMandatory::M {
                    match_block
                        .line(format!(
                            "buffer::{}({value_reader}{value_cast}{rest_args});",
                            point.point_type.writer_function_name
                        ))
                        .after(",");
                } else {
                    let mut some_block = Block::new(format!("if let Some(value) = {value_reader}"));
                    some_block.line(format!(
                        "buffer::{}(value{value_cast}{rest_args});",
                        point.point_type.writer_function_name
                    ));
                    let mut else_block = Block::new("else");
                    else_block.line("buffer::zero(buffer, offset);");

                    match_block.push_block(some_block);
                    match_block.push_block(else_block);
                };
                writer_block.push_block(match_block);
            }
            PointValueType::ModelLength => {
                let mut match_block = Block::new(match_arm);

                match_block.line("buffer::write_u16(model_length(model) - 2, buffer);");
                writer_block.push_block(match_block);
            }
            PointValueType::StaticValue(_) => {}
        }
    }

    scope
        .new_fn("write_point")
        .generic("'a")
        .vis("pub")
        .arg("model", "&dyn ModelAdapter")
        .arg("point", "&Point")
        .arg("buffer", "ModbusBuffer<'a>")
        .arg("offset", "u16")
        .arg("limit", "u16")
        .push_block(writer_block);

    let root_trait = scope.new_trait("ModelAdapter").vis("pub");

    for func in funcs {
        root_trait.push_fn(func);
    }

    for enum_type in &model.group.enums {
        scope.push_enum(generate_enum(enum_type));
    }

    generate_callback_struct(model, &mut scope);
    generate_stateful_struct(model, &mut scope);

    scope
}
