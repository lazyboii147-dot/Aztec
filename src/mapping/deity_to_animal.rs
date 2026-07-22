use crate::pantheon::deity::Deity;
use crate::animals::spirit::AnimalSpirit;

pub fn deity_to_primary_animal(d: Deity) -> Option<AnimalSpirit> {
    use AnimalSpirit::*;
    match d {
        Deity::Tezcatlipoca => Some(Jaguar),
        Deity::Quetzalcoatl => Some(Serpent),
        Deity::Huitzilopochtli => Some(Hummingbird),
        Deity::Tlaloc => Some(Crocodile),
        Deity::Mictlantecuhtli => Some(Vulture),
        Deity::Xolotl => Some(Dog),
        Deity::Huehuecoyotl => Some(Coyote),
        Deity::Tonatiuh => Some(Eagle),
        _ => None,
    }
}
