use codegen::{Block, Enum, Function, Impl, Scope, Struct, Type};

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

fn generate_setter(point: &ResolvedPoint) -> Function {
    let mut func = Function::new(format!("set_{}", &point.name_snake_case));

    if point.mandatory == PointMandatory::M {
        func.body = None;
    };

    func.arg("value", &point.point_type.rust_type)
        .arg_mut_self()
        .doc(doc_text(&point.doc));
    func
}

fn generate_point_array(group: &ResolvedGroup, model: &ResolvedModel) -> String {
    let lines: Vec<String> = [format!(
        "pub static POINTS: [ReadablePoint; {}] = [",
        group.points.len()
    )]
    .into_iter()
    .chain(group.points.iter().map(|point| {
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
            .vis("pub");

        if model.group.repeating_child.is_none() {
            // The C ffi can't handle the required generics to support arbitrary length arrays, so we can't include stateful adapters for repeating groups
            c_struct
                .field(
                    format!("pub {}_stateful_adapter", model.name_snake_case),
                    format!(
                        "Option<&'a {}::{}StatefulAdapter>",
                        model.name_snake_case, model.name_pascal_case
                    ),
                )
                .vis("pub");
        }
    }

    let c_struct_impl = scope
        .new_impl("SunspecExternalAdapters<'a>")
        .impl_trait("SunspecAdapterProvider<'a>")
        .generic("'a");

    for model in models {
        let impl_fn = c_struct_impl
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
            ));

        if model.group.repeating_child.is_none() {
            impl_fn.line(format!(
                ".or(self.{}_stateful_adapter.map(|a| a as &'a dyn {}::ModelAdapter))",
                model.name_snake_case, model.name_snake_case,
            ));
        }
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

fn generate_callback_functions(group: &ResolvedGroup, callback_struct: &mut Struct) {
    for point in group
        .points
        .iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
    {
        let c_type = if point.point_type.array_length.is_some() {
            format!("*const {}", &point.point_type.c_type)
        } else {
            point.point_type.c_type.clone()
        };

        let group_index_args = ", u16".repeat(point.block_indices.len());

        let fn_ptr = format!("extern \"C\" fn(*const c_void{group_index_args}) -> {c_type}");
        callback_struct.field(
            format!("{}_callback", point.name_snake_case),
            if point.mandatory == PointMandatory::M {
                fn_ptr
            } else {
                format!("Option<{fn_ptr}>")
            },
        );

        if point.access == PointAccess::Rw {
            let set_fn_ptr = format!("extern \"C\" fn({c_type}, *mut c_void{group_index_args})");
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

    if let Some((_, inner_group)) = &group.repeating_child {
        generate_callback_functions(inner_group, callback_struct);
    }
}

fn generate_callback_handlers(group: &ResolvedGroup, callback_impl: &mut Impl) {
    for point in group
        .points
        .iter()
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

        let callback_ref = if point.mandatory == PointMandatory::M {
            format!("self.{}_callback", point.name_snake_case)
        } else {
            getter.line(format!(
                "self.{}_callback.map(|callback| {{",
                point.name_snake_case
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

        if matches!(point.access, PointAccess::Rw) {
            let mut setter = generate_setter(point);
            for block_index in &point.block_indices {
                setter.arg(&block_index.index_name, "u16");
            }
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
    }

    if let Some((_, inner_group)) = &group.repeating_child {
        generate_callback_handlers(inner_group, callback_impl);
    }
}

pub fn generate_callback_struct(model: &ResolvedModel, scope: &mut Scope) {
    let callback_struct = scope
        .new_struct(format!("{}CallbackAdapter", model.name_pascal_case))
        .vis("pub")
        .repr("C");

    callback_struct.field("context", "*mut c_void");

    generate_callback_functions(&model.group, callback_struct);

    let callback_impl = scope
        .new_impl(format!("{}CallbackAdapter", model.name_pascal_case))
        .impl_trait("ModelAdapter");

    generate_callback_handlers(&model.group, callback_impl);
}

fn generate_model_length_calculator(group: &ResolvedGroup) -> String {
    if let Some((repeat_count_pt, repeating_block)) = &group.repeating_child {
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

pub fn populate_stateful_struct(
    model_name: String,
    struct_name: String,
    group: &ResolvedGroup,
    scope: &mut Scope,
    generics: &[String],
) {
    let stateful_struct = scope
        .new_struct(format!("{model_name}{struct_name}"))
        .vis("pub")
        .repr("C");
    for generic in generics {
        stateful_struct.generic(format!("const {generic}: usize"));
    }

    for point in group
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

    if let Some((_, inner_group)) = &group.repeating_child {
        let name = format!("{model_name}{}", inner_group.name_pascal_case);
        if let Some((array_len, inner_generics)) = generics.split_first() {
            let generic_str = inner_generics.join(", ");
            stateful_struct.field(
                &inner_group.name_snake_case,
                format!("[{name}<{generic_str}>; {array_len}]"),
            );
            populate_stateful_struct(
                model_name,
                inner_group.name_pascal_case.clone(),
                inner_group,
                scope,
                &generics[1..],
            );
        }
    }
}

fn collect_struct_generics(group: &ResolvedGroup) -> Vec<String> {
    if let Some((count_point, inner_group)) = &group.repeating_child {
        let mut params = collect_struct_generics(inner_group);
        params.insert(0, count_point.name_snake_case.to_uppercase());
        params
    } else {
        vec![]
    }
}

fn generate_stateful_handlers(group: &ResolvedGroup, stateful_impl: &mut Impl) {
    for point in group
        .points
        .iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
    {
        let group_index_access = point
            .block_indices
            .iter()
            .map(|block_index| {
                format!(
                    ".{}[{} as usize]",
                    block_index.group_name, block_index.index_name
                )
            })
            .collect::<Vec<_>>()
            .join("");

        let mut getter = generate_getter(point);
        for block_index in &point.block_indices {
            getter.arg(&block_index.index_name, "u16");
        }

        getter.body = None;

        let field = if point.point_type.array_length.is_some() {
            format!(
                "self{group_index_access}.{}.as_ptr()",
                point.name_snake_case
            )
        } else {
            format!("self{group_index_access}.{}", point.name_snake_case)
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

        if matches!(point.access, PointAccess::Rw) {
            let mut setter = generate_setter(point);
            for block_index in &point.block_indices {
                setter.arg(&block_index.index_name, "u16");
            }
            if point.point_type.array_length.is_none() {
                setter.line(format!(
                    "self{group_index_access}.{} = value;",
                    point.name_snake_case
                ));
            } else {
                const ITER: &str = "value.to_bytes_with_nul().iter()";
                let mut block = Block::new(format!(
                    "for (dest, src) in self{group_index_access}.{}.iter_mut().zip({})",
                    point.name_snake_case, ITER
                ));
                block.line("*dest = *src as c_char;");
                setter.push_block(block);
            }

            stateful_impl.push_fn(setter);
        }
    }

    if let Some((_, inner_group)) = &group.repeating_child {
        generate_stateful_handlers(inner_group, stateful_impl);
    }
}

pub fn generate_stateful_struct(model: &ResolvedModel, scope: &mut Scope) {
    let generics = collect_struct_generics(&model.group);
    populate_stateful_struct(
        model.name_pascal_case.clone(),
        "StatefulAdapter".to_string(),
        &model.group,
        scope,
        &generics,
    );
    let stateful_impl = scope
        .new_impl(format!(
            "{}StatefulAdapter<{}>",
            model.name_pascal_case,
            generics.join(", ")
        ))
        .impl_trait("ModelAdapter");

    for generic in generics {
        stateful_impl.generic(format!("const {generic}: usize"));
    }

    generate_stateful_handlers(&model.group, stateful_impl);
}

fn add_group_variants(group: &ResolvedGroup, target_enum: &mut Enum, counters: Vec<String>) {
    for point in group.points.iter().filter(|point| {
        point.value_type == PointValueType::Adapter
            || point.value_type == PointValueType::ModelLength
    }) {
        let variant = target_enum.new_variant(&point.name_pascal_case);
        for counter in &counters {
            variant.named(counter, "u16");
        }
    }
    if let Some((_, child_group)) = &group.repeating_child {
        let mut new_counters = counters.clone();
        new_counters.push(format!("{}_index", child_group.name_short));
        add_group_variants(child_group, target_enum, new_counters);
    }
}

pub fn generate_funcs(group: &ResolvedGroup) -> Vec<Function> {
    group
        .points
        .iter()
        .filter(|point| point.value_type == PointValueType::Adapter)
        .flat_map(|point| {
            let mut getter = generate_getter(point);
            point.block_indices.iter().for_each(|block_index| {
                getter.arg(&block_index.index_name, "u16");
            });
            match point.access {
                PointAccess::R => vec![getter],
                PointAccess::Rw => {
                    let mut setter = generate_setter(point);
                    point.block_indices.iter().for_each(|block_index| {
                        setter.arg(&block_index.index_name, "u16");
                    });
                    vec![getter, setter]
                }
            }
        })
        .chain(if let Some((_, inner_group)) = &group.repeating_child {
            generate_funcs(inner_group)
        } else {
            vec![]
        })
        .collect()
}

fn populate_model_writer(group: &ResolvedGroup, writer_block: &mut Block) {
    for point in group.points.iter() {
        let index_args: Vec<String> = point
            .block_indices
            .iter()
            .map(|block_index| block_index.index_name.clone())
            .collect();
        let match_arm = if index_args.is_empty() {
            format!("Point::{} =>", point.name_pascal_case)
        } else {
            format!(
                "Point::{} {{ {} }} =>",
                point.name_pascal_case,
                index_args.join(", ")
            )
        };
        match point.value_type {
            PointValueType::Adapter => {
                let dereferenced_args: Vec<String> =
                    index_args.iter().map(|arg| format!("*{arg}")).collect();
                let value_reader = format!(
                    "model.{}({})",
                    point.name_snake_case,
                    dereferenced_args.join(", ")
                );
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

    if let Some((_, inner_group)) = &group.repeating_child {
        populate_model_writer(inner_group, writer_block);
    }
}

pub fn generate_model(model: &ResolvedModel) -> Scope {
    let mut scope = Scope::new();

    let funcs: Vec<Function> = generate_funcs(&model.group);

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

    scope.raw(generate_point_array(&model.group, model));

    let point_enum = scope.new_enum("Point").vis("pub").derive("Debug");

    add_group_variants(&model.group, point_enum, vec![]);

    scope
        .new_fn("model_length")
        .vis("pub")
        .arg("model", "&dyn ModelAdapter")
        .ret("u16")
        .line(generate_model_length_calculator(&model.group));

    let mut writer_block = Block::new("match point");
    populate_model_writer(&model.group, &mut writer_block);

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
