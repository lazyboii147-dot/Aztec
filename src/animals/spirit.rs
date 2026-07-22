use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnimalSpirit {
    Jaguar,
    Eagle,
    Serpent,
    Coyote,
    Dog,
    Rabbit,
    Deer,
    Monkey,
    Vulture,
    Crocodile,
    Lizard,
    Owl,
    Hummingbird,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnimalDomain {
    War,
    Night,
    Solar,
    Fertility,
    Death,
    Trickster,
    Rain,
    Underworld,
    Knowledge,
}

impl AnimalSpirit {
    pub fn primary_domain(self) -> AnimalDomain {
        use AnimalDomain::*;
        match self {
            AnimalSpirit::Jaguar => Night,
            AnimalSpirit::Eagle => Solar,
            AnimalSpirit::Serpent => Knowledge,
            AnimalSpirit::Coyote => Trickster,
            AnimalSpirit::Dog => Underworld,
            AnimalSpirit::Rabbit => Fertility,
            AnimalSpirit::Deer => Fertility,
            AnimalSpirit::Monkey => Trickster,
            AnimalSpirit::Vulture => Death,
            AnimalSpirit::Crocodile => Creation,
            AnimalSpirit::Lizard => Fertility,
            AnimalSpirit::Owl => Night,
            AnimalSpirit::Hummingbird => War,
        }
    }
}

