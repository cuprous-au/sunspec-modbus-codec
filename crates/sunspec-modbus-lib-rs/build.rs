fn main() {
    sunspec_gen::generate();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../sunspec-gen/src");
    println!("cargo:rerun-if-changed=../sunspec-gen/models");
    println!("cargo:rerun-if-changed=../sunspec-gen/Cargo.toml");
}
