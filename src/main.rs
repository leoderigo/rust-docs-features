use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_for = &args[1];
    let pokemon = &args[2];
    let filepath: String = format!("entries/{}.txt", pokemon);

    println!("Searching for \"{}\" in {} pokedex entry", search_for, pokemon);

    let contents = fs::read_to_string(filepath)
        .expect("Should have been a pokemon name with lowercase");

    println!("::{}", contents);
}
