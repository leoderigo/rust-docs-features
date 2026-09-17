use std::{env, error, fs, process};
use testing_the_docs::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config= Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem building configuration file: {}", err);
        process::exit(1);
    });
    
    if let Err(err) = run(config) {
        eprintln!("Error during the execution of the application: {}", err);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    println!("Searching for '\"{}\" in {} pokedex entry", config.query, config.pokemon);
    println!("");
    let contents = fs::read_to_string(config.filepath)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
 
    Ok(())
}

struct Config {
    query: String,
    pokemon: String,
    filepath: String,
    ignore_case: bool
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let pokemon = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            pokemon: pokemon.clone(),
            query: query,
            filepath: format!("entries/{}.txt", pokemon),
            ignore_case: ignore_case
        })
    }
}
