use std::fs;

fn main() {
    let schema = fs::read_to_string("schema.json").unwrap();
    let enums: Vec<EnumSchema> = serde_json::from_str(&schema).unwrap();

    let mut output = String::new();
    for e in enums {
        output.push_str(&format!("public enum {} {{\n", e.name));
        for v in e.variants {
            output.push_str(&format!("    {},\n", v.name));
        }
        output.push_str("}\n\n");
    }

    fs::write("../csharp-bindings/src/Models/Enums.cs", output).unwrap();
}
