mod actions;

use crate::actions::{list_pokemons, choose_pokemon};

pub fn start() {
    list_pokemons();

    println!("");

    choose_pokemon();
}
