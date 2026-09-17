pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matched_lines: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if !line.contains(query) { continue };
        matched_lines.push(line);
    }
    matched_lines
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matched_lines: Vec<&'a str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if !line.to_lowercase().contains(&query) { continue };
        matched_lines.push(line);
    }
    matched_lines
}

#[cfg(test)]
mod tests {
    use super::*;
    const DEFAULT_TEXT: &str = "\
Here, there are no pokemons.
Nor here.
Or even here.";

    #[test]
    fn case_sensitive() {
        let query = "pokemons";
        let contents = DEFAULT_TEXT;

        assert_eq!(vec!["Here, there are no pokemons."], search(query, contents));
    }

    #[test]
    fn case_incensitive() {
        let query = "no";
        let contents = DEFAULT_TEXT;

        assert_eq!(vec!["Here, there are no pokemons.", "Nor here."], search_case_insensitive(query, contents));
    }
}
