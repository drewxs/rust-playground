use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("query string is required"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("file path is required"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitve() {
        let query = "Lorem";
        let contents = "\
Lorem ipsum dolor sit amet
consectetur adipiscing elit
Cras vel augue vel magna
Donec ultrices et nisilorem";
        assert_eq!(vec!["Lorem ipsum dolor sit amet"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "Lorem";
        let contents = "\
Lorem ipsum dolor sit amet
consectetur adipiscing elit
Cras vel augue vel magna
Donec ultrices et nisilorem";
        assert_eq!(
            vec!["Lorem ipsum dolor sit amet", "Donec ultrices et nisilorem"],
            search_case_insensitive(query, contents)
        );
    }
}
