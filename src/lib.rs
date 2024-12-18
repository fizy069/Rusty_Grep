use std::{error::Error, fs, vec};

pub struct Config {
    input: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() <= 2 {
            return Err(
                "Too few arguments!\nArgument 1 : string to be searched\nArgument 2 : File path",
            );
        }

        let input = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { input, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // println!("File content :\n{}", contents);
    
    for line in search(&config.input, &contents){
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res : Vec<&str> = vec![];
    for line in contents.lines(){
        if line.contains(query){
            res.push(line);
        }
    }
    res
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
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
