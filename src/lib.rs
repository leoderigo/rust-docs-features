pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matched_lines: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if !line.contains(query) { continue };
        matched_lines.push(line);
    }
    matched_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "pokemons";
        let contents = "\
There are no pokemons here.
Nor here.
Or even here.";

        assert_eq!(vec!["There are no pokemons here."], search(query, contents));
    }
}
