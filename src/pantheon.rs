/// Aztec Pantheon module defining major deities.

pub enum Deity {
    Huitzilopochtli,
    Quetzalcoatl,
    Tezcatlipoca,
    Tlaloc,
    Chalchiuhtlicue,
}

pub struct God {
    pub name: Deity,
    pub domain: &'static str,
    pub symbol: &'static str,
}
