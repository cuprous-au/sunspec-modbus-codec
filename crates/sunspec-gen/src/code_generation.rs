use codegen::{Block, Enum, Function, Impl, Scope, Struct, Type};

use crate::model_resolution::{
    CodegenFeature, DocLines, PointValueType, ResolvedEnum, ResolvedGroup, ResolvedModel,
    ResolvedPoint,
};
use crate::sunspec_schema::{PointAccess, PointMandatory};

fn doc_text(parts: &DocLines) -> String {
    parts.join("\n\n")
}

fn generate_enum(resolved_enum: &ResolvedEnum, scope: &mut Scope) {
    let enum_name = &resolved_enum.name_pascal_case;
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
    let mut func = Function::new(format!("set_{}", point.name_snake_case));

    if point.mandatory == PointMandatory::M {
        func.body = None;
    };

    func.arg("value", &point.point_type.rust_type)
        .arg_mut_self()
        .doc(doc_text(&point.doc));
    func
}

fn generate_point_arrays(group: &ResolvedGroup, scope: &mut Scope, args: Vec<String>) {
    let mut address_counter = 0;
    let arg_vec: Vec<&str> = args.iter().map(|_| "u16").collect::<Vec<&str>>();
    let group_index_args = if arg_vec.len() == 1 {
        arg_vec.join(", ")
    } else {
        format!("({})", arg_vec.join(", "))
    };
    let lines: Vec<String> = [format!(
        "static {name_prefix}POINTS: [PointDetails<{group_index_args}>; {len}] = [",
        name_prefix = if args.is_empty() {
            "".to_string()
        } else {
            format!("{}_", group.name_short.to_uppercase())
        },
        len = group.points.len()
    )]
    .into_iter()
    .chain(group.points.iter().map(|point| {
        let point_reference = if args.is_empty() {
            format!("Point::{} ", point.name_pascal_case)
        } else {
            format!(
                "Point::{} {{ {} }}",
                point.name_pascal_case,
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

    if let Some((_, inner_group)) = &group.repeating_child {
        let mut inner_args = args.clone();
        inner_args.push(format!("{}_index", inner_group.name_short));
        generate_point_arrays(inner_group, scope, inner_args);
    }
}

pub fn generate_adapter_structs(models: &[ResolvedModel]) -> Scope {
    let mut scope: Scope = Scope::new();

    for model in models {
        scope.import("crate::sunspec::models", &model.name_snake_case);
    }

    scope.import("crate::buffer", "WritableRegisterBuffer");
    scope.import("crate::buffer", "ReadableRegisterBuffer");
    scope.import("crate::cursor", "Cursor");
    scope.import("crate::cursor", "CursorResult");
    scope.import("crate", "ModbusException");
    scope.import("core::convert", "Infallible");

    let adapter_trait = scope
        .new_trait("SunspecAdapterProvider")
        .vis("pub")
        .generic("'a");
    for model in models {
        adapter_trait
            .new_fn(format!("{}_adapter", model.name_snake_case))
            .arg_mut_self()
            .ret(format!(
                "Option<&mut (dyn {}::ModelAdapter + 'a)>",
                model.name_snake_case
            ));

        adapter_trait
            .new_fn(format!("{}_adapter_ref", model.name_snake_case))
            .arg_ref_self()
            .ret(format!(
                "Option<&(dyn {}::ModelAdapter + 'a)>",
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
                format!(
                    "Option<&'a mut dyn {}::ModelAdapter>",
                    model.name_snake_case
                ),
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
            .arg_mut_self()
            .ret(format!(
                "Option<&mut (dyn {}::ModelAdapter + 'a)>",
                model.name_snake_case
            ))
            .line(format!(
                "self.{}_adapter.as_deref_mut()",
                model.name_snake_case
            ));

        trait_struct_impl
            .new_fn(format!("{}_adapter_ref", model.name_snake_case))
            .arg_ref_self()
            .ret(format!(
                "Option<&(dyn {}::ModelAdapter + 'a)>",
                model.name_snake_case
            ))
            .line(format!("self.{}_adapter.as_deref()", model.name_snake_case));
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
                    "Option<&'a mut {}::{}CallbackAdapter>",
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
                        "Option<&'a mut {}::{}StatefulAdapter>",
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
            .arg_mut_self()
            .ret(format!(
                "Option<&mut (dyn {}::ModelAdapter + 'a)>",
                model.name_snake_case
            ))
            .line(format!(
                "self.{}_callback_adapter.as_deref_mut()",
                model.name_snake_case
            ))
            .line(format!(
                ".map(|a| a as &mut (dyn {}::ModelAdapter + 'a))",
                model.name_snake_case
            ));

        if model.group.repeating_child.is_none() {
            impl_fn.line(format!(
                ".or_else(|| self.{}_stateful_adapter.as_deref_mut().map(|a| a as &mut (dyn {}::ModelAdapter + 'a)))",
                model.name_snake_case, model.name_snake_case,
            ));
        }

        let ref_impl_fn = c_struct_impl
            .new_fn(format!("{}_adapter_ref", model.name_snake_case))
            .arg_ref_self()
            .ret(format!(
                "Option<&(dyn {}::ModelAdapter + 'a)>",
                model.name_snake_case
            ))
            .line(format!(
                "self.{}_callback_adapter.as_deref()",
                model.name_snake_case
            ))
            .line(format!(
                ".map(|a| a as &(dyn {}::ModelAdapter + 'a))",
                model.name_snake_case
            ));

        if model.group.repeating_child.is_none() {
            ref_impl_fn.line(format!(
                ".or_else(|| self.{}_stateful_adapter.as_deref().map(|a| a as &(dyn {}::ModelAdapter + 'a)))",
                model.name_snake_case, model.name_snake_case,
            ));
        }
    }

    let read_fn = scope
        .new_fn("traverse_adapters_read")
        .vis("pub")
        .generic("'a")
        .generic("'b")
        .arg("adapters", "&dyn SunspecAdapterProvider<'a>")
        .arg("buffer", "&'b mut WritableRegisterBuffer<'b>")
        .arg("offset", "u16")
        .arg("limit", "u16")
        .ret("Option<u16>");

    read_fn.line("let mut cursor: Cursor<Infallible> = Cursor::new(offset, limit);");
    read_fn.line("");

    let mut suns_prefix_block = Block::new("cursor.visit_source_block(2, |offset, from, len|");
    suns_prefix_block
        .line("buffer.slice(from, len).write_string(c\"SunS\", offset);")
        .line("Ok(())")
        .after(")?;");
    read_fn.push_block(suns_prefix_block);

    for model in models {
        let name = &model.name_snake_case;

        read_fn.line("cursor.visit_optional_source_block_ref(");
        read_fn.line(format!("adapters.{name}_adapter_ref(),"));
        read_fn.line(format!("{name}::model_length,"));
        read_fn.line(format!(
            "|adapter, offset, from, len| {{ {name}::traverse_points_read(adapter, &mut buffer.slice(from, len), offset); Ok(()) }},"
        ));
        read_fn.line(")?;");
    }
    read_fn.line("");

    read_fn.line("cursor.target_offset");

    let write_fn = scope
        .new_fn("traverse_adapters_write")
        .vis("pub")
        .generic("'a")
        .generic("'b")
        .arg("adapters", "&mut dyn SunspecAdapterProvider<'a>")
        .arg("buffer", "&'b ReadableRegisterBuffer<'b>")
        .arg("offset", "u16")
        .arg("limit", "u16")
        .ret("Result<Option<u16>, ModbusException>");

    write_fn.line("let mut cursor = Cursor::new(offset, limit);");
    write_fn.line("");

    let mut suns_prefix_skip_block = Block::new("cursor.visit_source_block(2, |_, _, _|");
    suns_prefix_skip_block.line("Ok(())").after(");");
    write_fn.push_block(suns_prefix_skip_block);

    for model in models.iter().filter(|m| m.group.writable) {
        let name = &model.name_snake_case;

        write_fn.line("cursor.visit_optional_source_block(");
        write_fn.line(format!("adapters.{name}_adapter(),"));
        write_fn.line(format!("{name}::model_length,"));
        write_fn.line(format!(
            "|adapter, offset, from, len| {name}::traverse_points_write(adapter, &buffer.slice(from, len), offset),"
        ));
        write_fn.line(");");
    }
    write_fn.line("");

    let mut result_match = Block::new("match cursor.result()");
    result_match.line("CursorResult::Complete => Ok(None),");
    result_match.line("CursorResult::Incomplete(offset) => Ok(Some(offset)),");
    result_match.line("CursorResult::Error(e) => Err(e),");
    write_fn.push_block(result_match);

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
            format!("*const {}", point.point_type.c_type)
        } else {
            point.point_type.c_type.clone()
        };

        let group_index_args = ", u16".repeat(point.block_indices.len());

        let fn_ptr = format!("extern \"C\" fn(*const c_void{group_index_args}) -> {c_type}");
        callback_struct
            .new_field(
                format!("{}_callback", point.name_snake_case),
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
                    format!("set_{}_callback", point.name_snake_case),
                    if point.mandatory == PointMandatory::M {
                        set_fn_ptr
                    } else {
                        format!("Option<{set_fn_ptr}>")
                    },
                )
                .doc(doc_text(&point.doc));
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
        getter.doc("");

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
            setter.doc("");

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
            format!("[{}; {}]", point.point_type.c_type, array_length)
        } else {
            point.point_type.c_type.clone()
        };
        stateful_struct
            .new_field(format!("pub {}", point.name_snake_case), c_type)
            .doc(doc_text(&point.doc));
    }

    if let Some((_, inner_group)) = &group.repeating_child {
        let name = format!("{model_name}{}", inner_group.name_pascal_case);
        if let Some((array_len, inner_generics)) = generics.split_first() {
            let generic_str = inner_generics.join(", ");
            stateful_struct.field(
                format!("pub {}", inner_group.name_snake_case),
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
        getter.doc("");

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
            setter.doc("");

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
    for point in &group.points {
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
        match &point.value_type {
            PointValueType::Adapter => {
                let dereferenced_args: Vec<String> =
                    index_args.iter().map(|arg| format!("*{arg}")).collect();
                let value_reader = format!(
                    "model.{}({})",
                    point.name_snake_case,
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

                match_block.line("buffer.write_u16(model_length(model) - 2);");
                writer_block.push_block(match_block);
            }
            PointValueType::StaticValue(s) => {
                let mut match_block = Block::new(match_arm);

                match_block.line(format!("buffer.write_u16({s});"));
                writer_block.push_block(match_block);
            }
        }
    }

    if let Some((_, inner_group)) = &group.repeating_child {
        populate_model_writer(inner_group, writer_block);
    }
}

fn populate_model_reader(group: &ResolvedGroup, reader_block: &mut Block) {
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
                    "model.set_{}({parsed_value_name}{rest_args});",
                    point.name_snake_case
                ))
                .line("Ok(())")
                .after(",");
            reader_block.push_block(match_block);
        }
    }

    if let Some((_, inner_group)) = &group.repeating_child {
        populate_model_reader(inner_group, reader_block);
    } else {
        let mut unmatched_block = Block::new("_ =>");
        unmatched_block.line("Err(ModbusException::IllegalDataAddress)");
        reader_block.push_block(unmatched_block);
    }
}

fn chain_repeating_group_iterator(
    group: &ResolvedGroup,
    address_offset: String,
    indices: Vec<String>,
) -> Vec<String> {
    let prefix = &group.name_short;
    let mut index_args = indices.clone();
    index_args.push(format!("{prefix}_index"));

    let index_arg_str = if index_args.len() == 1 {
        index_args[0].clone()
    } else {
        format!("({})", index_args.join(", "))
    };

    let mut result = vec![
        format!(".chain((0..{prefix}_count).flat_map(move |{prefix}_index| {{"),
        format!("let {prefix}_address = {address_offset} + {prefix}_index * {prefix}_size;"),
        format!("{}_POINTS.iter()", prefix.to_uppercase()),
        format!(
            ".map(move |p| ({prefix}_address + p.start_address, p.size, (p.point)({index_arg_str})))"
        ),
    ];

    if let Some((_, inner_group)) = &group.repeating_child {
        result.extend(chain_repeating_group_iterator(
            inner_group,
            format!("{prefix}_address"),
            index_args,
        ))
    }
    result.push("}))".to_string());
    result
}

enum TraverseDirection {
    Read,
    Write,
}

fn generate_traverse_points_fn(
    model: &ResolvedModel,
    scope: &mut Scope,
    direction: TraverseDirection,
) {
    let fn_name = match direction {
        TraverseDirection::Read => "traverse_points_read",
        TraverseDirection::Write => "traverse_points_write",
    };

    let fn_def = scope.new_fn(fn_name).vis("pub").generic("'a");

    match direction {
        TraverseDirection::Read => {
            fn_def
                .arg("model", "&dyn ModelAdapter")
                .arg("buffer", "&mut WritableRegisterBuffer<'a>")
                .arg("offset", "u16");
        }
        TraverseDirection::Write => {
            fn_def
                .arg("model", "&mut dyn ModelAdapter")
                .arg("buffer", "&ReadableRegisterBuffer<'a>")
                .arg("offset", "u16")
                .ret("Result<(), ModbusException>");
        }
    }

    fn_def.line("let until = offset + buffer.len();");
    fn_def.line("let mut cursor = 0;");
    fn_def.line("");

    let mut group = &model.group;
    let mut group_stack = vec![];
    while let Some((count_point, inner_group)) = &group.repeating_child {
        group_stack.push((count_point, inner_group));
        group = inner_group;
    }

    let mut child_group: Option<&ResolvedGroup> = None;
    for (count_point, inner_group) in group_stack.iter().rev() {
        let prefix = &inner_group.name_short;
        let count_accessor = if count_point.mandatory == PointMandatory::M {
            format!("model.{}()", count_point.name_snake_case)
        } else {
            format!(
                "model.{}().unwrap_or_default()",
                count_point.name_snake_case
            )
        };
        fn_def.line(format!("let {prefix}_count = {count_accessor};"));
        if let Some(child) = child_group {
            fn_def.line(format!(
                "let {prefix}_size = {static_size} + {child_prefix}_count * {child_prefix}_size;",
                static_size = inner_group.static_size,
                child_prefix = child.name_short
            ));
        } else {
            fn_def.line(format!(
                "let {prefix}_size = {static_size};",
                static_size = inner_group.static_size
            ));
        }
        fn_def.line("");

        child_group = Some(inner_group);
    }

    fn_def.line("let iter = POINTS.iter()");
    fn_def.line(".map(|p| (p.start_address, p.size, (p.point)(())))");

    if let Some((_, inner_group)) = &model.group.repeating_child {
        for s in
            chain_repeating_group_iterator(inner_group, model.group.static_size.to_string(), vec![])
        {
            fn_def.line(s);
        }
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
                .line("model,")
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
                .line("model,")
                .line("&point,")
                .line("&buffer.slice(cursor, word_count),")
                .line(")?;");
        }
    }

    point_block.line("cursor += word_count;");
    fn_def.push_block(point_block);

    if matches!(direction, TraverseDirection::Write) {
        fn_def.line("");
        fn_def.line("Ok(())");
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
    scope.import("crate::buffer", "WritableRegisterBuffer");
    scope.import("crate::buffer", "ReadableRegisterBuffer");
    scope.import("crate", "ModbusException");
    scope.import("core::cmp", "min");

    let size: u16 = model
        .group
        .points
        .iter()
        .map(|point| point.point_type.size)
        .sum();

    scope.raw(format!("pub const SIZE: u16 = {};", size));

    generate_point_arrays(&model.group, &mut scope, vec![]);

    let point_enum = scope.new_enum("Point").vis("pub").derive("Debug");

    add_group_variants(&model.group, point_enum, vec![]);

    scope
        .new_struct("PointDetails")
        .generic("GroupIndexArgs")
        .derive("Debug")
        .field("point", "fn(GroupIndexArgs) -> Point")
        .field("start_address", "u16")
        .field("size", "u16");

    scope
        .new_fn("model_length")
        .vis("pub")
        .arg("model", "&dyn ModelAdapter")
        .ret("u16")
        .line(generate_model_length_calculator(&model.group));

    generate_traverse_points_fn(model, &mut scope, TraverseDirection::Read);
    generate_traverse_points_fn(model, &mut scope, TraverseDirection::Write);

    let mut writer_block = Block::new("match point");
    populate_model_writer(&model.group, &mut writer_block);

    scope
        .new_fn("write_point_to_buffer")
        .generic("'a")
        .vis("pub")
        .arg("model", "&dyn ModelAdapter")
        .arg("point", "&Point")
        .arg("buffer", "&mut WritableRegisterBuffer<'a>")
        .arg("offset", "u16")
        .push_block(writer_block);

    let reader_fn = scope
        .new_fn("read_point_from_buffer")
        .generic("'a")
        .vis("pub")
        .arg("model", "&mut dyn ModelAdapter")
        .arg("point", "&Point")
        .arg("buffer", "&ReadableRegisterBuffer<'a>")
        .ret("Result<(), ModbusException>");

    if model.group.writable {
        let mut reader_block = Block::new("match point");
        populate_model_reader(&model.group, &mut reader_block);

        reader_fn.push_block(reader_block);
    } else {
        reader_fn.line("Err(ModbusException::IllegalDataAddress)");
    }

    let root_trait = scope.new_trait("ModelAdapter").vis("pub");

    for func in funcs {
        root_trait.push_fn(func);
    }

    for enum_type in &model.group.enums {
        generate_enum(enum_type, &mut scope);
    }

    generate_callback_struct(model, &mut scope);
    generate_stateful_struct(model, &mut scope);

    scope
}
