use sunspec_gen::{generate, generate_model_features, generate_static_lib};

fn main() {
    generate();
    generate_static_lib();
    generate_model_features();
}
