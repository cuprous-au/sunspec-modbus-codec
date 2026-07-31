use std::{fs, path::Path};

use serde_json::Value;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let schema_path = "models/json/schema.json";
    println!("cargo:rerun-if-changed={}", schema_path);
    let schema_raw = fs::read_to_string(schema_path).unwrap();
    let mut schema_json: Value = serde_json::from_str(&schema_raw).unwrap();

    schema_json.as_object_mut().unwrap().insert(
        "title".to_string(),
        serde_json::Value::String("SunspecModel".to_string()),
    );

    let schema: schemars::schema::RootSchema = serde_json::from_value(schema_json).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();

    let contents = rustfmt_wrapper::rustfmt(type_space.to_stream().to_string()).unwrap();

    let mut out_file = Path::new("src").to_path_buf();
    out_file.push("sunspec_schema.rs");
    fs::write(out_file, contents).unwrap();
}
