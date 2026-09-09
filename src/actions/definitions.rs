pub struct Pokemon<'a> {
    pub name: &'a str,
    typing: PokemonTyping,
    pub is_defeated: bool,
    gender: PokemonGender
}

impl <'a>Pokemon<'a> {
    pub fn get_name(&self) -> &str {
        for (char_index, byte) in self.name.as_bytes().iter().enumerate() {
            if b'(' == *byte {
                return &self.name[..char_index - 1];
            }
        }
        &self.name[..]
    }

    pub fn get_type(&self) -> String {
        let mut types = String::from(self.typing.0.to_str());

        match &self.typing.1 {
            Some(typing) => {
                types += "/";
                types += typing.to_str();
            }
            None => ()
        }

        types
    }

    pub fn get_pronoums(pokemon: &Pokemon) -> (&'a str, &'a str) {
        match pokemon.gender {
            PokemonGender::Male => ("he", "him"),
            PokemonGender::Female => ("she", "her"),
            PokemonGender::None => ("it", "it")
        }
    }
}

struct PokemonTyping (PokemonType, Option<PokemonType>);

enum PokemonType {
    Grass,
    Water,
    Fire,
    Electric,
    Rock,
    Ground,
    Metal,
    Ice,
    Ghost,
    Psychic,
    Fighter,
    Normal,
    Dark,
    Fairy,
    Dragon,
    Poison,
    Bug,
    Flying,
}

impl PokemonType {
    fn to_str(&self) -> &str {
        match self {
            PokemonType::Bug => "Bug",
            PokemonType::Dark => "Dark",
            PokemonType::Dragon => "Dragon",
            PokemonType::Electric => "Electric",
            PokemonType::Fairy => "Fairy",
            PokemonType::Fighter => "Fighter",
            PokemonType::Fire => "Fire",
            PokemonType::Flying => "Flying",
            PokemonType::Ghost => "Ghost",
            PokemonType::Grass => "Grass",
            PokemonType::Ground => "Ground",
            PokemonType::Ice => "Ice",
            PokemonType::Metal => "Metal",
            PokemonType::Normal => "Normal",
            PokemonType::Poison => "Poison",
            PokemonType::Psychic => "Psychic",
            PokemonType::Rock => "Rock",
            PokemonType::Water => "Water",
        }
    }
}

enum PokemonGender {
    Male,
    Female,
    None
}

pub const POKEMONS: [Pokemon; 6] = [
    Pokemon {
        name: "Breloom",
        typing: PokemonTyping(PokemonType::Grass, Some(PokemonType::Fighter)),
        is_defeated: false,
        gender: PokemonGender::Male
    },
    Pokemon {
        name: "Shiny Yanma",
        typing: PokemonTyping(PokemonType::Bug, Some(PokemonType::Flying)),
        is_defeated: false,
        gender: PokemonGender::Male
    },
    Pokemon {
        name: "Raichu",
        typing: PokemonTyping(PokemonType::Electric, None),
        is_defeated: true,
        gender: PokemonGender::Male
    },
    Pokemon {
        name: "Zapdos (Galarian)",
        typing: PokemonTyping(PokemonType::Fighter, Some(PokemonType::Flying)),
        is_defeated: false,
        gender: PokemonGender::Male
    },
    Pokemon {
        name: "Shiny Raichu (Alolan)",
        typing: PokemonTyping(PokemonType::Electric, Some(PokemonType::Psychic)),
        is_defeated: true,
        gender: PokemonGender::Female
    },
    Pokemon {
        name: "Magneton",
        typing: PokemonTyping(PokemonType::Electric, Some(PokemonType::Metal)),
        is_defeated: true,
        gender: PokemonGender::None
    },
];
