use colored::*;

use std::env;
use std::error::Error;
use std::fs;

// Global structure containing the program parameters.
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    colored_output: bool,
}

impl Config {
    // Create the configuration from arguments and environment variable.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let colored_output = true;
        Ok(Config {
            query,
            file_path,
            ignore_case,
            colored_output,
        })
    }
}

// Run is the fonction called by main.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut results = Vec::new();

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            results.push(line.to_string());
        }
    } else {
        for line in search(&config.query, &contents) {
            results.push(line.to_string());
        }
    }

    if config.colored_output {
        results = colorize(&config.query, results.clone());
    }
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// Takes a vector of strings, colorize the query on each and return an updated
// vector.
pub fn colorize(query: &str, inputs: Vec<String>) -> Vec<String> {
    let mut results = Vec::new();
    for line in inputs {
        let colored_line = line.replace(&query, &query.red().to_string());
        results.push(colored_line);
    }
    results
}

// Returns a vector of string slices with each line from contents that contains
// query string slice.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// Returns a vector of string slices with each line from contents that contains
// query string slice ignoring the case.
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_sensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
