use core::panic;
use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Fix this : {err}");
        process::exit(1);
    });
    if let Err(e) = run(config){
        println!("An error occured : {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("File content :\n{}", contents);

    Ok(())
}

struct Config {
    input: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
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
