use codegen::Scope;
use glob::glob;
use rustfmt_wrapper::config::{Config, Edition};
use std::{ffi::OsStr, fs, path::Path};

use crate::code_generation::{
    generate_adapter_structs, generate_model, generate_models_mod, generate_point_types,
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

    if let Err(error) = fs::remove_dir_all(generated_src_dir)
        && error.kind() != std::io::ErrorKind::NotFound
    {
        panic!("Failed to remove generated source directory: {error}");
    }
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
        &generated_src_dir.join("points.rs"),
        &generate_point_types(&models),
    );
    format_and_write(
        &generated_src_dir.join("adapters.rs"),
        &generate_adapter_structs(&models),
    );
    format_and_write(
        &generated_src_dir.join("models.rs"),
        &generate_models_mod(&models),
    );
}
