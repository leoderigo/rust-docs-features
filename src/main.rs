use std::{env, error, fs, process};
use testing_the_docs::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config= Config::build(&args).unwrap_or_else(|err| {
        println!("Problem building configuration file: {}", err);
        process::exit(1);
    });
    
    if let Err(err) = run(config) {
        println!("Error during the execution of the application: {}", err);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    println!("Searching for '\"{}\" in {} pokedex entry", config.query, config.pokemon);
    println!("");
    let contents = fs::read_to_string(config.filepath)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }
 
    Ok(())
}

struct Config {
    query: String,
    pokemon: String,
    filepath: String
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &str> {
        let (query, pokemon) = match args.len() {
            3 => {
                (args[1].clone(), args[2].clone())
            },
            _ => { return Err("Wrong number of arguments"); }
        };
        Ok(Config { pokemon: pokemon.clone(), query: query, filepath: format!("entries/{}.txt", pokemon) })
    }
}
