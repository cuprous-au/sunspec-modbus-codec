fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../sunspec-gen/src");
    println!("cargo:rerun-if-changed=../sunspec-gen/models");
    println!("cargo:rerun-if-changed=../sunspec-gen/Cargo.toml");
    for model in sunspec_gen::all_model_names() {
        println!(
            "cargo:rerun-if-env-changed=CARGO_FEATURE_{}",
            model.to_uppercase()
        );
    }

    let included_models = sunspec_gen::enabled_model_features();
    sunspec_gen::generate(&included_models);
}
