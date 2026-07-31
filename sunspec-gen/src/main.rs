use codegen::Scope;
use glob::glob;
use heck::ToSnakeCase;
use std::{ffi::OsStr, fs, path::Path};

use crate::code_generation::{
    generate_adapter_structs, generate_model, generate_models_mod, generate_point_types,
};
use crate::model_resolution::{ResolvedModel, resolve_model};
use crate::sunspec_schema::SunspecModel;

mod code_generation;
mod model_resolution;
mod sunspec_schema;
const EXCLUDED_MODELS: [&str; 8] = [
    "model_9",
    "model_14",
    "model_302",
    "model_303",
    "model_304",
    "model_601",
    "model_702",
    "model_63002",
];
const GENERATED_SRC_DIR: &str = "../src/sunspec";

fn model_name_from_path(path: &Path) -> &str {
    path.file_prefix().and_then(OsStr::to_str).unwrap()
}

fn is_included_model(path: &Path) -> bool {
    !EXCLUDED_MODELS.contains(&model_name_from_path(path))
}

fn collect_models(model_glob: &str) -> Vec<ResolvedModel> {
    glob(model_glob)
        .unwrap()
        .filter(|entry| match entry {
            Ok(path) => is_included_model(path),
            Err(_) => true,
        })
        .flat_map(|entry| match entry {
            Ok(path) => {
                let json = fs::read_to_string(&path).unwrap();
                let model: SunspecModel = serde_json::from_str(&json).unwrap();
                let model_name = model_name_from_path(path.as_path());
                Some(resolve_model(&model, model_name.to_string()))
            }
            Err(e) => {
                println!("{:?}", e);
                None
            }
        })
        .collect()
}

fn format_and_write(path: String, scope: &Scope) -> std::io::Result<()> {
    let text = rustfmt_wrapper::rustfmt(scope.to_string()).unwrap();
    fs::write(format!("{}/{}", GENERATED_SRC_DIR, path), text)
}

fn main() -> () {
    let model_glob = "./models/json/model_*.json";

    fs::remove_dir_all(GENERATED_SRC_DIR).unwrap();
    fs::create_dir_all(format!("{}/models", GENERATED_SRC_DIR)).unwrap();

    let models = collect_models(model_glob);

    for model in &models {
        format_and_write(
            format!("models/{}.rs", model.name_snake_case),
            &generate_model(model),
        )
        .unwrap();
    }
    
    format_and_write("points.rs".to_string(), &generate_point_types(&models)).unwrap();
    format_and_write("adapters.rs".to_string(), &generate_adapter_structs(&models)).unwrap();
    format_and_write("models.rs".to_string(), &generate_models_mod(&models)).unwrap();
}
