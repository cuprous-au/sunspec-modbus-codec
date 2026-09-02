use codegen::Scope;
use glob::glob;
use rustfmt_wrapper::config::{Config, Edition};
use std::{ffi::OsStr, fs, path::Path};

use crate::code_generation::{
    generate_model, generate_models_mod, model_c_expressible, model_is_repeating,
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

    // if let Err(error) = fs::remove_dir_all(generated_src_dir)
    //     && error.kind() != std::io::ErrorKind::NotFound
    // {
    //     panic!("Failed to remove generated source directory: {error}");
    // }
    // fs::create_dir_all(generated_src_dir.join("models"))
    //     .expect("Failed to create generated source directories");

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
}

/// C declarations for the per-model `SUNSPEC_MODEL_<id>` dispatch descriptors, to splice into
/// the cbindgen header (which cannot export `static`s from a dependency crate).
///
/// A C caller sets each `SunspecModelBinding.model` to the address of the matching entry.
pub fn c_model_externs() -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let model_glob = format!("{project_root}/models/json/model_*.json");
    let models = collect_models(&model_glob);

    let mut out = String::new();
    out.push_str("#ifdef __cplusplus\nextern \"C\" {\n#endif // __cplusplus\n\n");
    out.push_str(
        "/*\n\
         \x20* Per-model dispatch descriptors, one per SunSpec model the codec supports. Point\n\
         \x20* each `SunspecModelBinding.model` at the address of the entry for that model,\n\
         \x20* e.g. `.model = &SUNSPEC_MODEL_103`.\n\
         \x20*/\n",
    );
    out.push_str("struct CModel;\n");
    for model in models.iter().filter(|model| model_c_expressible(model)) {
        out.push_str(&format!(
            "extern const struct CModel SUNSPEC_MODEL_{};\n",
            model.model_number
        ));
    }
    out.push_str("\n#ifdef __cplusplus\n} // extern \"C\"\n#endif // __cplusplus\n");
    out
}

/// Typed `static inline` constructors for [`SunspecAdapter`], one set per model, to splice
/// into the cbindgen header **after** the struct definitions (via `config.trailer`).
///
/// `sunspec_model_<id>_callback(Model<id>CallbackAdapter *)` /
/// `sunspec_model_<id>_stateful(Model<id>StatefulAdapter *)` /
/// `sunspec_model_<id>_none(void)` build a `SunspecAdapter` with the matching `model`, `kind`
/// and adapter pointer, so a C caller cannot pair the wrong adapter type with a model (the C
/// compiler rejects a mismatched pointer) or leave the `model`/`kind`/`adapter` triple
/// inconsistent. `_stateful` is emitted only for non-repeating models, matching the dispatch
/// code (repeating models have no C-usable stateful adapter).
pub fn c_model_adapter_constructors() -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let model_glob = format!("{project_root}/models/json/model_*.json");
    let models = collect_models(&model_glob);

    let mut out = String::new();
    out.push_str(
        "#ifndef SUNSPEC_MODBUS_CODEC_ADAPTER_CTORS\n\
         #define SUNSPEC_MODBUS_CODEC_ADAPTER_CTORS\n\n\
         /*\n\
         \x20* Typed builders for `SunspecAdapter`. Prefer these over a raw struct literal: the C\n\
         \x20* compiler then checks that the adapter pointer matches the model, and `model` / `kind`\n\
         \x20* are always filled consistently. Example:\n\
         \x20*\n\
         \x20*     SunspecAdapter read_adapters[] = {\n\
         \x20*         sunspec_model_1_callback(&common),\n\
         \x20*         sunspec_model_103_stateful(&inverter),\n\
         \x20*     };\n\
         \x20*/\n",
    );

    for model in models.iter().filter(|model| model_c_expressible(model)) {
        let n = model.model_number;
        let pc = &model.name_pascal_case;

        out.push_str(&format!(
            "static inline SunspecAdapter sunspec_model_{n}_callback({pc}CallbackAdapter *adapter) {{\n\
             \x20   SunspecAdapter s = {{ &SUNSPEC_MODEL_{n}, SUNSPEC_ADAPTER_CALLBACK, adapter }};\n\
             \x20   return s;\n\
             }}\n"
        ));
        if !model_is_repeating(model) {
            out.push_str(&format!(
                "static inline SunspecAdapter sunspec_model_{n}_stateful({pc}StatefulAdapter *adapter) {{\n\
                 \x20   SunspecAdapter s = {{ &SUNSPEC_MODEL_{n}, SUNSPEC_ADAPTER_STATEFUL, adapter }};\n\
                 \x20   return s;\n\
                 }}\n"
            ));
        }
        out.push_str(&format!(
            "static inline SunspecAdapter sunspec_model_{n}_none(void) {{\n\
             \x20   SunspecAdapter s = {{ &SUNSPEC_MODEL_{n}, SUNSPEC_ADAPTER_NONE, NULL }};\n\
             \x20   return s;\n\
             }}\n"
        ));
    }

    out.push_str("\n#endif /* SUNSPEC_MODBUS_CODEC_ADAPTER_CTORS */\n");
    out
}

/// Names of the per-model adapter structs a C caller must be able to allocate:
/// `Model<id>CallbackAdapter` for every C-expressible model, plus `Model<id>StatefulAdapter`
/// for the non-repeating ones (repeating models have no C-usable stateful adapter).
///
/// Nothing in `sunspec-modbus-lib-static`'s signatures names these types (the adapter is
/// carried as `void*`), so they must be force-listed in cbindgen's `export.include` for the
/// header to carry their field layout.
pub fn c_adapter_struct_names() -> Vec<String> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let model_glob = format!("{project_root}/models/json/model_*.json");
    let models = collect_models(&model_glob);

    let mut names = Vec::new();
    for model in models.iter().filter(|model| model_c_expressible(model)) {
        let pc = &model.name_pascal_case;
        names.push(format!("{pc}CallbackAdapter"));
        if !model_is_repeating(model) {
            names.push(format!("{pc}StatefulAdapter"));
        }
    }
    names
}
