/// Aztec cosmology outlining the heavens, earth, and underworlds.

pub enum Realm {
    Heaven(u8), // 13 heavens
    Earth,
    Underworld(u8), // 9 levels of Mictlan
}

pub struct CosmicLevel {
    pub realm: Realm,
    pub name: &'static str,
}
