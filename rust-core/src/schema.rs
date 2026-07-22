use serde::Serialize;

#[derive(Serialize)]
pub struct EnumVariant {
    pub name: String,
    pub doc: Option<String>,
}

#[derive(Serialize)]
pub struct EnumSchema {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

pub fn export_schema() -> Vec<EnumSchema> {
    vec![
        EnumSchema {
            name: "Glyph".into(),
            variants: crate::calendar::glyph::Glyph::all()
                .iter()
                .map(|g| EnumVariant {
                    name: g.name().into(),
                    doc: None,
                })
                .collect(),
        },
        EnumSchema {
            name: "Deity".into(),
            variants: crate::pantheon::deity::Deity::all()
                .iter()
                .map(|d| EnumVariant {
                    name: d.name().into(),
                    doc: None,
                })
                .collect(),
        },
        // add AnimalSpirit, etc.
    ]
}
impl Glyph {
    pub fn all() -> &'static [Glyph] {
        &[
            Glyph::Cipactli,
            Glyph::Ehecatl,
            // ...
            Glyph::Xochitl,
        ]
    }
}
