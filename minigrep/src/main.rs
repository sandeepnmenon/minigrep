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
    fn build(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Pass the searchstring and the file path");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error passing arguments: {err}");
        process::exit(3);
    });

    if let Err(error) = run(config) {
        println!("Applicaiton error: {error}");
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
