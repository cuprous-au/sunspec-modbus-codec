use std::env;

use sunspec_gen::{generate, generate_static_lib};

/// `sunspec-gen <rs|static> [--model <name>]...`
///
/// The first argument picks which crate's sources to (re)generate: `rs` for
/// `sunspec-modbus-lib-rs`, `static` for `sunspec-modbus-lib-static`'s typed adapter
/// constructors. Each `--model <name>` (e.g. `--model model_103`) adds one model to the set to
/// generate; with none given, nothing is generated. `build.rs` in each crate calls the library
/// functions directly with the model list derived from its enabled Cargo features — this CLI is
/// for regenerating sources by hand during development.
fn main() {
    let mut args = env::args().skip(1);
    let mode = args
        .next()
        .expect("expected a mode argument: `rs` or `static`");

    let mut models = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--model" => {
                let name = args
                    .next()
                    .unwrap_or_else(|| panic!("--model requires a value"));
                models.push(name);
            }
            other => panic!("unrecognised argument {other:?}: expected `--model <name>`"),
        }
    }

    match mode.as_str() {
        "rs" => generate(&models),
        "static" => generate_static_lib(&models),
        other => panic!("unrecognised mode {other:?}: expected `rs` or `static`"),
    }
}
