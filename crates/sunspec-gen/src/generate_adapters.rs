use codegen::{Block, Scope};

use crate::model_resolution::ResolvedModel;


/// The `ReadBinding` / `WriteBinding` enums and their `read_model` / `write_model` drivers.
///
/// Each enum has one variant per model, pairing the model marker (e.g. `model_1::Model1`)
/// with a borrow of that model's per-model adapter trait - with the ref being mutable for writes.
/// `WriteBinding`'s model variants carry an adapter only for models with writable points.
///
/// Both enums also have a non-model `Extern` variant carrying a C [`StaticModelSpec`] vtable
/// and an untyped adapter pointer, for maps whose models aren't known statically (the FFI
/// `ModelList` in `sunspec-modbus-lib-static`). `read_model` / `write_model` traverse a model
/// variant against the cursor directly, and forward an `Extern` variant through the vtable.
pub fn generate_adapters_mod(models: &[ResolvedModel]) -> Scope {
    let mut scope = Scope::new();
    scope.import("crate::sunspec::models", "*");
    scope.import("core::ffi", "c_void");
    scope.import("crate", "ModbusException");
    scope.import("crate", "ModelSpec");
    scope.import("crate::buffer", "ReadableRegisterBuffer");
    scope.import("crate::buffer", "WritableRegisterBuffer");
    scope.import("crate::cursor", "Cursor");
    scope.import("crate::model", "StaticModelSpec");

    let read_enum = scope
        .new_enum("ReadBinding")
        .vis("pub")
        .generic("'a")
        .r#macro("#[non_exhaustive]")
        .doc(
            "One model's read side: a statically known model paired with a shared borrow of\n\
             its per-model [`ReadAdapter`](crate::sunspec::models) trait, or an\n\
             [`Extern`](ReadBinding::Extern) block dispatched through a C [`StaticModelSpec`] vtable.",
        );
    for model in models {
        let sc = &model.name_snake_case;
        let pc = &model.name_pascal_case;
        read_enum
            .new_variant(pc)
            .tuple(format!("&'a {sc}::{pc}"))
            .tuple(format!("&'a dyn {sc}::ReadAdapter"));
    }
    read_enum
        .new_variant("Extern")
        .doc(
            "A model dispatched through a C [`StaticModelSpec`] vtable rather than a statically\n\
             known model type. Produced by the FFI `ModelList` in `sunspec-modbus-lib-static`.\n\
             \n\
             # Safety\n\
             Building this variant asserts `descriptor` and `adapter` uphold\n\
             [`StaticModelSpec::visit_read`]'s contract: for `kind` 1 or 2, `adapter` points to\n\
             a live `Model<id>{Stateful,Callback}Adapter` valid for the traversal.",
        )
        .named("descriptor", "&'a StaticModelSpec")
        .named("kind", "u8")
        .named("adapter", "*const c_void")
        .named("repeat_count_0", "u16")
        .named("repeat_count_1", "u16");

    let write_enum = scope
        .new_enum("WriteBinding")
        .vis("pub")
        .generic("'a")
        .r#macro("#[non_exhaustive]")
        .doc(
            "One model's write side: a statically known model, carrying a uniquely borrowed\n\
             reference to its per-model [`WriteAdapter`](crate::sunspec::models) trait when the\n\
             model has writable points, or an [`Extern`](WriteBinding::Extern) block dispatched\n\
             through a C [`StaticModelSpec`] vtable. A variant with no adapter rejects every\n\
             write in its block.",
        );
    for model in models {
        let sc = &model.name_snake_case;
        let pc = &model.name_pascal_case;
        let variant = write_enum.new_variant(pc).tuple(format!("&'a {sc}::{pc}"));
        if model.group.writable {
            variant.tuple(format!("&'a mut dyn {sc}::WriteAdapter"));
        }
    }
    write_enum
        .new_variant("Extern")
        .doc(
            "A model dispatched through a C [`StaticModelSpec`] vtable rather than a statically\n\
             known model type. Produced by the FFI `ModelList` in `sunspec-modbus-lib-static`.\n\
             \n\
             # Safety\n\
             Building this variant asserts `descriptor` and `adapter` uphold\n\
             [`StaticModelSpec::visit_write`]'s contract: for `kind` 1 or 2, `adapter` is\n\
             uniquely borrowable and points to a live `Model<id>{Stateful,Callback}Adapter`\n\
             valid for the traversal.",
        )
        .named("descriptor", "&'a StaticModelSpec")
        .named("kind", "u8")
        .named("adapter", "*mut c_void")
        .named("repeat_count_0", "u16")
        .named("repeat_count_1", "u16");

    let mut read_match = Block::new("match adapter");
    for model in models {
        let pc = &model.name_pascal_case;
        let mut model_match = Block::new(format!("ReadBinding::{pc}(model, adapter) => "));

        model_match.line("cursor.visit_source_block(");
        model_match.line("model.model_length(),");
        model_match.line("|offset, from, len| { model.traverse_points_read(&*adapter, &mut buffer.slice(from, len), offset) }");
        model_match.line(");");
        read_match.push_block(model_match);
    }
    {
        let mut extern_match = Block::new(
            "ReadBinding::Extern { descriptor, kind, adapter, repeat_count_0, repeat_count_1 } =>",
        );
        extern_match.line(
            "// SAFETY: `ReadBinding::Extern` upholds `StaticModelSpec::visit_read`'s contract by construction.",
        );
        let mut call = Block::new("unsafe");
        call.line(
            "(descriptor.visit_read)(kind, adapter, repeat_count_0, repeat_count_1, cursor, buffer);",
        );
        extern_match.push_block(call);
        read_match.push_block(extern_match);
    }
    scope
        .new_fn("read_model")
        .vis("pub")
        .generic("'a")
        .arg("adapter", "ReadBinding<'a>")
        .arg("cursor", "&mut Cursor<ModbusException>")
        .arg("buffer", "&mut WritableRegisterBuffer<'_>")
        .doc(
            "Encode one model's block into `buffer` from its matched [`ReadBinding`] variant,\n\
             advancing `cursor` across the block whether or not it produces data.",
        )
        .push_block(read_match);

    let mut write_match = Block::new("match adapter");
    for model in models {
        let pc = &model.name_pascal_case;

        let mut model_match = Block::new(if model.group.writable {
            format!("WriteBinding::{pc}(model, adapter) =>")
        } else {
            format!("WriteBinding::{pc}(model) =>")
        });

        model_match.line("cursor.visit_source_block(");
        model_match.line("model.model_length(),");

        if model.group.writable {
            model_match.line(
                "|offset, from, len| { model.traverse_points_write(adapter, &buffer.slice(from, len), offset) }"
            );
        } else {
            model_match.line("|_, _, _| Err(ModbusException::IllegalDataAddress)");
        }
        model_match.line(");");
        write_match.push_block(model_match);
    }
    {
        let mut extern_match = Block::new(
            "WriteBinding::Extern { descriptor, kind, adapter, repeat_count_0, repeat_count_1 } =>",
        );
        extern_match.line(
            "// SAFETY: `WriteBinding::Extern` upholds `StaticModelSpec::visit_write`'s contract by construction.",
        );
        let mut call = Block::new("unsafe");
        call.line(
            "(descriptor.visit_write)(kind, adapter, repeat_count_0, repeat_count_1, cursor, buffer);",
        );
        extern_match.push_block(call);
        write_match.push_block(extern_match);
    }
    scope
        .new_fn("write_model")
        .vis("pub")
        .generic("'a")
        .arg("adapter", "WriteBinding<'a>")
        .arg("cursor", "&mut Cursor<ModbusException>")
        .arg("buffer", "&ReadableRegisterBuffer<'_>")
        .doc(
            "Decode one model's block from `buffer` into its matched [`WriteBinding`] variant,\n\
             advancing `cursor` across the block. A non-writable variant rejects the write with\n\
             [`ModbusException::IllegalDataAddress`].",
        )
        .push_block(write_match);

    scope
}