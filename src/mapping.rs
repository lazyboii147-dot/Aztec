/// Geographical and spatial mapping for Aztec territories and cosmology.

pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

pub struct Landmark {
    pub name: &'static str,
    pub location: Coordinate,
    pub description: &'static str,
}
