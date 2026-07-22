use rust_core::schema::export_schema;
use std::fs;

fn main() {
    let schema = export_schema();
    let json = serde_json::to_string_pretty(&schema).unwrap();
    fs::write("schema.json", json).unwrap();
}
