use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=../sunspec-modbus-lib-rs/Cargo.toml");
    println!("cargo:rerun-if-changed=../sunspec-modbus-lib-rs/src");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let config_path = PathBuf::from(&crate_dir).join("cbindgen.toml");
    let output_path = PathBuf::from(&crate_dir).join("sunspec_modbus_codec.h");

    println!("cargo:rerun-if-changed=../sunspec-gen/src");
    println!("cargo:rerun-if-changed=../sunspec-gen/models");

    let mut config = cbindgen::Config::from_file(&config_path).unwrap_or_else(|error| {
        panic!(
            "failed to read cbindgen config at {:?}: {error}",
            config_path
        )
    });

    // cbindgen cannot export `static`s from a dependency crate, so the per-model
    // `SUNSPEC_MODEL_<id>` dispatch descriptors are declared by hand-spliced C, before the
    // struct definitions (they are referenced only by pointer).
    let externs = sunspec_gen::c_model_externs();
    config.after_includes = Some(match config.after_includes.take() {
        Some(existing) => format!("{existing}\n{externs}"),
        None => externs,
    });

    // Typed `SunspecAdapter` constructors go after the struct definitions.
    let ctors = sunspec_gen::c_model_adapter_constructors();
    config.trailer = Some(match config.trailer.take() {
        Some(existing) => format!("{existing}\n{ctors}"),
        None => ctors,
    });

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .unwrap_or_else(|error| panic!("failed to generate C bindings: {error}"))
        .write_to_file(output_path);
}
