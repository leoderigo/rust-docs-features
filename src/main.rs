use std::env;
use trpl::block_on;

use testing_the_docs::get_page_title;

fn main() {
    let result = block_on(run());
    println!("{result}");
}

async fn run() -> String {
    // get args
    let urls = match get_urls(env::args()) {
        Ok(urls) => urls,
        Err(err) => {
            return format!("It was not possible to get the urls: {err}");
        }
    };
    // get page title
    let result = trpl::select(
        get_page_title(&urls.0),
        get_page_title(&urls.1)
    ).await;

    let result = match result {
        trpl::Either::Left(a) => a,
        trpl::Either::Right(b) => b
    };

    let (url, title) = match result {
        Err(err) => return format!("Could not get page title: {err}"),
        Ok(result) => result
    };

    // print messages
    match title {
        None => format!("URL {url} has no title"),
        Some(title) => format!("URL {url} has the following title: {title}")
    }
}

fn get_urls(mut args: impl Iterator<Item = String>) -> Result<URLs, &'static str> {
    args.next();
    let first = args.next();
    if let None = first {
        return Err("Need two URLs")
    };
    let second = args.next();
    if let None = second {
        return Err("Need more one URL")
    };

    Ok(URLs(first.unwrap(), second.unwrap()))
}

struct URLs (String, String);
