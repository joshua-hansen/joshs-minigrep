use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Throw away executable

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Please provide a search query"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Please provide a file path"),
        };

        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        if !ignore_case {
            ignore_case = args
                .filter(|arg| arg.contains("--ignore-case") || arg.contains("-i"))
                .any(|_| true);
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    results.iter().for_each(|line| println!("{}", line));
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    contents.lines()
        .filter(|line| line.contains(&query))
        .for_each(|line| results.push(line));
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .for_each(|line| results.push(line));
    results
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = indoc! {"
        Rust:
        Safe, Fast, Productive.
        Pick three."
        };

        assert_eq!(
            vec!["Safe, Fast, Productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = indoc! {"\
            Rust:
            Safe, Fast, Productive.
            Pick three.
            Duct tape."
        };

        assert_eq!(
            vec!["Safe, Fast, Productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = indoc! {"\
            Rust:
            Safe, Fast, Productive.
            Pick three.
            Duct tape."
        };

        assert_eq!(
            vec!["Safe, Fast, Productive.", "Duct tape."],
            search_case_insensitive(query, contents)
        )
    }
}
