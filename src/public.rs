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
