use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    East,
    North,
    West,
    South,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Glyph {
    Cipactli,
    Ehecatl,
    Calli,
    Cuetzpalin,
    Coatl,
    Miquiztli,
    Mazatl,
    Tochtli,
    Atl,
    Itzcuintli,
    Ozomahtli,
    Malinalli,
    Acatl,
    Ocelotl,
    Cuauhtli,
    Cozcacuauhtli,
    Ollin,
    Tecpatl,
    Quiahuitl,
    Xochitl,
}

impl Glyph {
    pub fn direction(self) -> Direction {
        use Direction::*;
        match self {
            Glyph::Cipactli => East,
            Glyph::Ehecatl => North,
            Glyph::Calli => West,
            Glyph::Cuetzpalin => South,
            Glyph::Coatl => East,
            Glyph::Miquiztli => North,
            Glyph::Mazatl => West,
            Glyph::Tochtli => South,
            Glyph::Atl => East,
            Glyph::Itzcuintli => North,
            Glyph::Ozomahtli => West,
            Glyph::Malinalli => South,
            Glyph::Acatl => East,
            Glyph::Ocelotl => North,
            Glyph::Cuauhtli => West,
            Glyph::Cozcacuauhtli => South,
            Glyph::Ollin => East,
            Glyph::Tecpatl => North,
            Glyph::Quiahuitl => West,
            Glyph::Xochitl => South,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Glyph::Cipactli => "Cipactli",
            Glyph::Ehecatl => "Ehecatl",
            Glyph::Calli => "Calli",
            Glyph::Cuetzpalin => "Cuetzpalin",
            Glyph::Coatl => "Coatl",
            Glyph::Miquiztli => "Miquiztli",
            Glyph::Mazatl => "Mazatl",
            Glyph::Tochtli => "Tochtli",
            Glyph::Atl => "Atl",
            Glyph::Itzcuintli => "Itzcuintli",
            Glyph::Ozomahtli => "Ozomahtli",
            Glyph::Malinalli => "Malinalli",
            Glyph::Acatl => "Acatl",
            Glyph::Ocelotl => "Ocelotl",
            Glyph::Cuauhtli => "Cuauhtli",
            Glyph::Cozcacuauhtli => "Cozcacuauhtli",
            Glyph::Ollin => "Ollin",
            Glyph::Tecpatl => "Tecpatl",
            Glyph::Quiahuitl => "Quiahuitl",
            Glyph::Xochitl => "Xochitl",
        }
    }
}
