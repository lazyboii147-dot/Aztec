use serde::Serialize;
use rust_core::calendar::glyph::Glyph;
use rust_core::pantheon::deity::Deity;

#[derive(Serialize)]
pub struct GlyphInfo {
    pub name: String,
    pub direction: String,
}

impl From<Glyph> for GlyphInfo {
    fn from(g: Glyph) -> Self {
        Self {
            name: g.name().into(),
            direction: format!("{:?}", g.direction()),
        }
    }
}

#[derive(Serialize)]
pub struct DeityInfo {
    pub name: String,
    pub domain: String,
}

impl From<Deity> for DeityInfo {
    fn from(d: Deity) -> Self {
        Self {
            name: d.name().into(),
            domain: format!("{:?}", d.domain()),
        }
    }
}
