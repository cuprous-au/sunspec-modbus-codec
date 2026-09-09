use codegen::Scope;

use crate::{generate_models::model_is_repeating, model_resolution::ResolvedModel};


/// Typed `SunspecAdapter` constructors, one set per C-expressible model.
///
/// `sunspec_model_<id>_callback(*mut Model<id>CallbackAdapter)` /
/// `sunspec_model_<id>_stateful(*mut Model<id>StatefulAdapter)` 
/// build a `SunspecAdapter` with the matching `model_spec`, `kind` and adapter pointer, so a C
/// caller cannot pair the wrong adapter type with a model (the C compiler rejects a mismatched
/// pointer) or leave the `model_spec`/`kind`/`adapter` triple inconsistent. `_stateful` is
/// generated only for non-repeating models, matching the dispatch code (repeating models have
/// no C-usable stateful adapter).
///
/// These are real `#[unsafe(no_mangle)] pub extern "C" fn`s, generated directly into
/// `sunspec-modbus-lib-static` rather than spliced into the header as C text: cbindgen exports
/// them itself, and naming `Model<id>{Stateful,Callback}Adapter` in their signatures is what
/// makes cbindgen carry those structs' field layout — no force-listing needed.
pub fn generate_static_lib_adapter_ctors(models: &[ResolvedModel]) -> Scope {
    let mut scope = Scope::new();
    scope.raw("#![allow(unused_imports)]");
    scope.raw("use core::ffi::c_void;");
    scope.raw(
        "use crate::{SunspecAdapter, SUNSPEC_ADAPTER_CALLBACK, SUNSPEC_ADAPTER_NONE, SUNSPEC_ADAPTER_STATEFUL};",
    );

    for model in models.iter() {
        let n = model.model_number;
        let sc = &model.name_snake_case;
        let pc = &model.name_pascal_case;
        let module = format!("sunspec_modbus_lib_rs::sunspec::models::{sc}");

        scope.raw(format!(
            "/// Build a [`SunspecAdapter`] for model {n} backed by a callback adapter.\n\
             #[unsafe(no_mangle)]\n\
             pub extern \"C\" fn sunspec_model_{n}_callback(\n\
             \x20   adapter: *mut {module}::{pc}CallbackAdapter,\n\
             ) -> SunspecAdapter {{\n\
             \x20   SunspecAdapter {{\n\
             \x20       model_spec: &{module}::SUNSPEC_MODEL_{n},\n\
             \x20       kind: SUNSPEC_ADAPTER_CALLBACK,\n\
             \x20       adapter: adapter as *mut c_void,\n\
             \x20   }}\n\
             }}\n"
        ));

        if !model_is_repeating(model) {
            scope.raw(format!(
                "/// Build a [`SunspecAdapter`] for model {n} backed by a stateful adapter.\n\
                 #[unsafe(no_mangle)]\n\
                 pub extern \"C\" fn sunspec_model_{n}_stateful(\n\
                 \x20   adapter: *mut {module}::{pc}StatefulAdapter,\n\
                 ) -> SunspecAdapter {{\n\
                 \x20   SunspecAdapter {{\n\
                 \x20       model_spec: &{module}::SUNSPEC_MODEL_{n},\n\
                 \x20       kind: SUNSPEC_ADAPTER_STATEFUL,\n\
                 \x20       adapter: adapter as *mut c_void,\n\
                 \x20   }}\n\
                 }}\n"
            ));
        }
    }

    scope
}
