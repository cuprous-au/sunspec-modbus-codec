fn main() {
    sunspec_gen::generate();

    println!("cargo:rerun-if-changed=build.rs");
}