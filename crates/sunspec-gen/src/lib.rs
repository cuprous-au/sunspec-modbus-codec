use codegen::Scope;
use glob::glob;
use rustfmt_wrapper::config::{Config, Edition};
use std::{ffi::OsStr, fs, path::Path};

use crate::code_generation::{
    generate_adapters_mod, generate_model, generate_models_mod, model_c_expressible,
    model_is_repeating,
};
use crate::model_resolution::{ResolvedModel, resolve_model};
use crate::sunspec_schema::SunspecModel;

mod code_generation;
mod model_resolution;
mod sunspec_schema;
const EXCLUDED_MODELS: [&str; 10] = [
    "model_9",
    "model_14",
    "model_302",
    "model_303",
    "model_304",
    "model_601",
    "model_702",
    "model_803",
    "model_804",
    "model_63002",
];

fn model_name_from_path(path: &Path) -> &str {
    path.file_prefix()
        .and_then(OsStr::to_str)
        .expect("Failed to convert Sunspec model JSON file name to string")
}

fn is_included_model(path: &Path) -> bool {
    !EXCLUDED_MODELS.contains(&model_name_from_path(path))
}

fn collect_models(model_glob: &str) -> Vec<ResolvedModel> {
    let mut vec: Vec<ResolvedModel> = glob(model_glob)
        .expect("Failed to find Sunspec model JSON files by glob pattern")
        .filter(|entry| match entry {
            Ok(path) => is_included_model(path),
            Err(_) => true,
        })
        .flat_map(|entry| match entry {
            Ok(path) => {
                let json = fs::read_to_string(&path).expect("Failed to read model JSON to string");
                let model: SunspecModel =
                    serde_json::from_str(&json).expect("Failed to parse Sunspec model as JSON");
                let model_name = model_name_from_path(path.as_path());
                Some(resolve_model(&model, model_name.to_string()))
            }
            Err(e) => {
                println!("{:?}", e);
                None
            }
        })
        .collect();

    vec.sort_unstable_by_key(|model| model.model_number);

    vec
}

fn format_and_write(path: &Path, scope: &Scope) {
    let text_raw = scope.to_string();

    let rustfmt_config: Config = Config {
        edition: Some(Edition::Edition2024),
        ..Default::default()
    };
    let text = rustfmt_wrapper::rustfmt_config(rustfmt_config, &text_raw).unwrap_or_else(|error| {
        println!(
            "cargo:warning=Error formatting file {}: [{}]. File left unformatted.",
            path.to_str().unwrap_or_default(),
            error
        );
        text_raw
    });
    fs::write(path, text).expect("Failed to write generated source to file");
}

pub fn generate() {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let model_glob = format!("{project_root}/models/json/model_*.json");
    let src_path = format!("{project_root}/../sunspec-modbus-lib-rs/src/sunspec");
    let generated_src_dir = Path::new(&src_path);

    fs::create_dir_all(generated_src_dir.join("models"))
        .expect("Failed to create generated source directories");

    let models = collect_models(&model_glob);

    for model in &models {
        format_and_write(
            &generated_src_dir
                .join("models")
                .join(format!("{}.rs", model.name_snake_case)),
            &generate_model(model),
        );
    }

    format_and_write(
        &generated_src_dir.join("models.rs"),
        &generate_models_mod(&models),
    );

    format_and_write(
        &generated_src_dir.join("adapters.rs"),
        &generate_adapters_mod(&models),
    );
}

/// Regenerates `sunspec-modbus-lib-static`'s typed `SunspecAdapter` constructors
/// (`src/generated.rs`). Called from that crate's `build.rs`, before cbindgen runs.
pub fn generate_static_lib() {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let output_path =
        Path::new(project_root).join("../sunspec-modbus-lib-static/src/generated.rs");
    format_and_write(&output_path, &generate_static_lib_adapter_ctors());
}

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
fn generate_static_lib_adapter_ctors() -> Scope {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let model_glob = format!("{project_root}/models/json/model_*.json");
    let models = collect_models(&model_glob);

    let mut scope = Scope::new();
    scope.raw("#![allow(unused_imports)]");
    scope.raw("use core::ffi::c_void;");
    scope.raw(
        "use crate::{SunspecAdapter, SUNSPEC_ADAPTER_CALLBACK, SUNSPEC_ADAPTER_NONE, SUNSPEC_ADAPTER_STATEFUL};",
    );

    for model in models.iter().filter(|model| model_c_expressible(model)) {
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
