mod definitions;

use std::io;
use definitions::{POKEMONS, Pokemon};

pub fn list_pokemons() {
    println!("Choose your pokemon:");
    for (index, pokemon) in POKEMONS.iter().enumerate() {
        println!("{} - {}", index + 1, pokemon.name);
    }
}

pub fn choose_pokemon() {
    'main: loop {
        let mut chosen_pokemon: String = String::new();

        io::stdin()
            .read_line(&mut chosen_pokemon)
            .expect("Can't choose a pokemon yet.");

        let chosen_pokemon: usize = match chosen_pokemon
            .trim()
            .parse()
            {
                Ok(selected) => {
                    if selected > 6 {
                        println!("This is not a pokemon.");
                        continue;
                    }
                    selected
                },
                Err(_) => continue
            };
        println!("");
        let chosen_pokemon = &POKEMONS[chosen_pokemon - 1];
        let pokemon: &str = chosen_pokemon.get_name();
        let typing = chosen_pokemon.get_type();

        if chosen_pokemon.is_defeated {
            loop {
                let pronouns = Pokemon::get_pronoums(chosen_pokemon);
                println!(
                    "Oh no! {}'s not ready to fight yet! Should I heal {}?",
                    String::from(&pronouns.0[0..1]).to_uppercase() + &pronouns.0[1..],
                    pronouns.1
                );
                println!("1 - Yes");
                println!("2 - No");
                println!("");

                let mut should_heal: String = String::with_capacity(1);
                io::stdin().read_line(&mut should_heal)
                    .expect("Ops, somehting wrong happened!");
    
                let should_heal: u8 = match should_heal
                    .trim()
                    .parse() {
                        Ok(result) => {
                            if result == 1 || result == 2 {
                                result
                            } else {
                                continue
                            }
                        },
                        Err(_) => continue
                    };
                
                println!("");

                if should_heal == 2 {
                    println!("I forfeit!");
                    break 'main;
                } else {
                    
                    println!("Alright, LET'S BATTLE!");
                }
                break;
            }
        }

        println!("I choose my {} pokemon, {}!", typing, pokemon);
        break;
    }
}
