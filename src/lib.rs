pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> impl Iterator<Item = &'a str> {
    contents.lines()
        .filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase(); 
    contents.lines()
        .filter(move |lines| lines.to_lowercase().contains(&query))
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

        let mut results = search(query, contents);

        vec!["Here, there are no pokemons."].into_iter()
            .for_each(|line| assert_eq!(line, results.next().unwrap()));
    }

    #[test]
    fn case_incensitive() {
        let query = "no";
        let contents = DEFAULT_TEXT;

        let mut results = search_case_insensitive(query, contents);

        vec!["Here, there are no pokemons.", "Nor here."].into_iter()
            .for_each(|line| assert_eq!(line, results.next().unwrap()));
    }
}
