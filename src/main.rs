use std::{env, error::Error, fs, process};
use minigrep::Config;
fn main() {
    //iterators
    // iterators produce a series of values
    // we can call .collect to convert it into an collection like vector

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments : {err}");
        process::exit(1);
    });
    println!("Search for - {}", config.query);
    println!("The file targeted - {}", config.target_file);

    if let Err(e) = minigrep::run(config){
        print!("Application Error : {e}");
        process::exit(1);
    }

    // read the file
    // import the fs
}




