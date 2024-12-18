use std::{env, process};
use rusty_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Fix this : {err}");
        process::exit(1);
    });
    if let Err(e) = rusty_grep::run(config){
        eprintln!("An error occured : {e}");
        process::exit(1);
    }
}
