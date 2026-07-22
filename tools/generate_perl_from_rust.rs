use std::fs;

fn main() {
    let schema = fs::read_to_string("schema.json").unwrap();
    let enums: Vec<EnumSchema> = serde_json::from_str(&schema).unwrap();

    let mut output = String::new();
    output.push_str("package Aztec::Enums;\nuse strict;\nuse warnings;\n\n");

    for e in enums {
        output.push_str(&format!("our @{} = (\n", e.name));
        for v in e.variants {
            output.push_str(&format!("    '{}',\n", v.name));
        }
        output.push_str(");\n\n");
    }

    output.push_str("1;\n");

    fs::write("../perl-bindings/lib/Aztec/Enums.pm", output).unwrap();
}
