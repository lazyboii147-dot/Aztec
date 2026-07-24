
/// Sacred animals in Aztec mythology and cosmology.

pub enum SacredAnimal {
    Eagle,
    Jaguar,
    Serpent,
    Xoloitzcuintli,
    Hummingbird,
}

pub struct AnimalAttribute {
    pub species: SacredAnimal,
    pub mythological_significance: &'static str,
}
