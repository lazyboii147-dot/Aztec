use axum::{extract::Path, Json};
use rust_core::calendar::glyph::Glyph;
use rust_core::pantheon::deity::Deity;
use crate::dto::{GlyphInfo, DeityInfo};

pub async fn glyph(Path(name): Path<String>) -> Json<GlyphInfo> {
    let glyph = Glyph::from_name(&name).unwrap();
    Json(GlyphInfo::from(glyph))
}

pub async fn deity(Path(name): Path<String>) -> Json<DeityInfo> {
    let deity = Deity::from_name(&name).unwrap();
    Json(DeityInfo::from(deity))
}
