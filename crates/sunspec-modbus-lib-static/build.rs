use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=../sunspec-modbus-lib-rs/Cargo.toml");
    println!("cargo:rerun-if-changed=../sunspec-modbus-lib-rs/src");
    println!("cargo:rerun-if-changed=../sunspec-gen/src");
    println!("cargo:rerun-if-changed=../sunspec-gen/models");
    for model in sunspec_gen::all_model_names() {
        println!(
            "cargo:rerun-if-env-changed=CARGO_FEATURE_{}",
            model.to_uppercase()
        );
    }

    let included_models = sunspec_gen::enabled_model_features();
    sunspec_gen::generate_static_lib(&included_models);

    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let config_path = PathBuf::from(&crate_dir).join("cbindgen.toml");
    let output_path = PathBuf::from(&crate_dir).join("sunspec_modbus_codec.h");

    let config = cbindgen::Config::from_file(&config_path).unwrap_or_else(|error| {
        panic!(
            "failed to read cbindgen config at {:?}: {error}",
            config_path
        )
    });

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .unwrap_or_else(|error| panic!("failed to generate C bindings: {error}"))
        .write_to_file(output_path);
}
