use sunspec_gen::{
    generate, generate_model_features, generate_model_registry, generate_static_models,
};

fn main() {
    generate();
    generate_static_models();
    generate_model_features();
    generate_model_registry();
}
