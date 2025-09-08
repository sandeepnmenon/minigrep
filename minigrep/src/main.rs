use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file path"),
        };
        Ok(Config {
            query: query,
            file_path: file_path,
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error passing arguments: {err}");
        process::exit(3);
    });

    if let Err(error) = run(config) {
        eprintln!("Applicaiton error: {error}");
        process::exit(3);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}
