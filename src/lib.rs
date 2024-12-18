use std::env;
use std::{error::Error, fs, vec};

pub struct Config {
    input: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err(
                "Too few arguments!\nArgument 1 : string to be searched\nArgument 2 : File path
                Optional Argument 3 : ignore_case (true/false)",
            );
        }

        let input = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case;

        if args.len() > 3 {
            ignore_case = args[3].to_lowercase() == "true";
        } else {
            ignore_case = env::var("IGNORE_CASE").is_ok();
        }

        Ok(Config {
            input,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // println!("File content :\n{}", contents);
    if config.ignore_case {
        for line in search_case_insensitive(&config.input, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.input, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
