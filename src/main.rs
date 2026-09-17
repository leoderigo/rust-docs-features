use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_for = &args[1];
    let filepath = &args[2];

    println!("We are searching for {} in {}", search_for, filepath);
}
