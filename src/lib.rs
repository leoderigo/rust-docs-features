pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase(); 
    contents.lines()
        .filter(|lines| lines.to_lowercase().contains(query))
        .collect()
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
