use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs;

pub fn get_u8_from_input() -> u8 {
    use std::io::stdin;

    let mut text: String = String::new();
    stdin().read_line(&mut text).expect("Could not get user input");

    let value: u8 = match text.trim().parse() {
        Ok(result) => result,
        _ => get_u8_from_input(),
    };

    value
}

#[derive(Deserialize, Serialize)]
pub struct Year {
    value: u16
}

impl Year {
    pub fn new(value: u16) -> Self {
        if value > 9999 || value < 1000 {
            panic!("A Year from this implementation should range between 1000 to 9999");
        }
        return Year { value: value };
    }

    pub fn get(&self) -> u16 {
        self.value
    }
}

pub fn get_serialized_full_table<T: DeserializeOwned>(table_path: &str) -> Vec<T> {
    let values: Vec<T> = match fs::read_to_string(&table_path) {
        Err(err) => {
            println!("Error on reading path: {}", table_path);
            panic!("{err}");
        },
        Ok(table_str) => {
            match serde_json::from_str(&table_str) {
                Err(err) => {
                    println!("Error on desserializing from {}", table_path);
                    panic!("{err}");
                },
                Ok(converted) => converted
            }
        }
    };

    values
}
