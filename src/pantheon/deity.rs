use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeityDomain {
    Supreme,
    Solar,
    Lunar,
    Earth,
    Fertility,
    Rain,
    Wind,
    War,
    Death,
    Underworld,
    Stars,
    Pleasure,
    Alcohol,
    Craft,
    Trickster,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Deity {
    Ometeotl,
    Quetzalcoatl,
    Tezcatlipoca,
    Huitzilopochtli,
    Tlaloc,
    Tonatiuh,
    Coyolxauhqui,
    Coatlicue,
    Chalchiuhtlicue,
    Xochiquetzal,
    Tlazolteotl,
    Mayahuel,
    Centeotl,
    Mictlantecuhtli,
    Xolotl,
    Xiuhtecuhtli,
    Huehuecoyotl,
    CentzonMimixcoa,
    CentzonHuitznahua,
    CentzonTotochtin,
}

impl Deity {
    pub fn domain(self) -> DeityDomain {
        use DeityDomain::*;
        match self {
            Deity::Ometeotl => Supreme,
            Deity::Quetzalcoatl => Wind,
            Deity::Tezcatlipoca => Trickster,
            Deity::Huitzilopochtli => War,
            Deity::Tlaloc => Rain,
            Deity::Tonatiuh => Solar,
            Deity::Coyolxauhqui => Lunar,
            Deity::Coatlicue => Earth,
            Deity::Chalchiuhtlicue => Fertility,
            Deity::Xochiquetzal => Fertility,
            Deity::Tlazolteotl => Fertility,
            Deity::Mayahuel => Alcohol,
            Deity::Centeotl => Fertility,
            Deity::Mictlantecuhtli => Underworld,
            Deity::Xolotl => Death,
            Deity::Xiuhtecuhtli => Craft,
            Deity::Huehuecoyotl => Trickster,
            Deity::CentzonMimixcoa => Stars,
            Deity::CentzonHuitznahua => Stars,
            Deity::CentzonTotochtin => Alcohol,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Deity::Ometeotl => "Ometeotl",
            Deity::Quetzalcoatl => "Quetzalcoatl",
            Deity::Tezcatlipoca => "Tezcatlipoca",
            Deity::Huitzilopochtli => "Huitzilopochtli",
            Deity::Tlaloc => "Tlaloc",
            Deity::Tonatiuh => "Tonatiuh",
            Deity::Coyolxauhqui => "Coyolxauhqui",
            Deity::Coatlicue => "Coatlicue",
            Deity::Chalchiuhtlicue => "Chalchiuhtlicue",
            Deity::Xochiquetzal => "Xochiquetzal",
            Deity::Tlazolteotl => "Tlazolteotl",
            Deity::Mayahuel => "Mayahuel",
            Deity::Centeotl => "Centeotl",
            Deity::Mictlantecuhtli => "Mictlantecuhtli",
            Deity::Xolotl => "Xolotl",
            Deity::Xiuhtecuhtli => "Xiuhtecuhtli",
            Deity::Huehuecoyotl => "Huehuecoyotl",
            Deity::CentzonMimixcoa => "Centzon Mimixcoa",
            Deity::CentzonHuitznahua => "Centzon Huitznahua",
            Deity::CentzonTotochtin => "Centzon Totochtin",
        }
    }
}
