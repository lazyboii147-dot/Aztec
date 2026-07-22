use crate::calendar::glyph::Glyph;
use crate::animals::spirit::AnimalSpirit;

pub fn glyph_to_animal(g: Glyph) -> Option<AnimalSpirit> {
    use AnimalSpirit::*;
    match g {
        Glyph::Cipactli => Some(Crocodile),
        Glyph::Ehecatl => None,
        Glyph::Calli => None,
        Glyph::Cuetzpalin => Some(Lizard),
        Glyph::Coatl => Some(Serpent),
        Glyph::Miquiztli => Some(Vulture),
        Glyph::Mazatl => Some(Deer),
        Glyph::Tochtli => Some(Rabbit),
        Glyph::Atl => None,
        Glyph::Itzcuintli => Some(Dog),
        Glyph::Ozomahtli => Some(Monkey),
        Glyph::Malinalli => None,
        Glyph::Acatl => None,
        Glyph::Ocelotl => Some(Jaguar),
        Glyph::Cuauhtli => Some(Eagle),
        Glyph::Cozcacuauhtli => Some(Vulture),
        Glyph::Ollin => None,
        Glyph::Tecpatl => None,
        Glyph::Quiahuitl => None,
        Glyph::Xochitl => None,
    }
}
