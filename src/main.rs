use minigrep::{search, search_case_insensitive};
use std::{env, error::Error, fs, process};

//if you program arg could contain invalid unicode, use std::env::arg_os
// it return OsStrings instead of string
fn main() {
    let args: Vec<String> = env::args().collect();
    // let word_to_be_searched = &args[1];
    // let file_path = &args[2];

    //returning a config struct Config { word , filepath}
    // let config = parse_config(&args);

    //calling the same function with Config::new  ,as we implemented
    // let config = Config::new(&args);

    // using Config::build
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // modularised it in run function
    // added error handeling, as the way run is written , rust compiler thinks error handeling

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    // let file_content = fs::read_to_string(config.file_path).expect("Should have been able to read to file");
    // println!("Contents of file :\n{}",file_content)

    // we have to put our logic out of the main file, for now we are creating just an function outside for config.
}
// implementing Box<dyn Error>, that will make the return type implement the dyanamic error trait.

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content =
        fs::read_to_string(config.file_path).expect("Should have been able to read to file");
    // println!("Contents of file :\n{}",file_content);

    // let results = if config.ignore_case || config.argument == String::from("yes"){
    //     search_case_insensitive(&config.word, &file_content)
    // } else {
    //     search(&config.word, &file_content)
    // };
    let results = if config.ignore_case{
        search_case_insensitive(&config.word, &file_content)
    } else {
        search(&config.word, &file_content)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}
//creating a struct Config , so that the maintainer will get a good idea of data args.
pub struct Config {
    pub word: String,
    pub file_path: String,
    pub ignore_case: bool,
    // pub argument : String,
}

// Returning in form of enum
// pub fn parse_config(args : &[String])-> (&str,&str){

//     let word_to_be_searched_2 = &args[1];
//     let file_path_2 = &args[2];

//     (word_to_be_searched_2,file_path_2)
// }

//Returning in form of Struct

// pub fn parse_config(args : &[String])-> Config{

//     let word_to_be_searched_2 = args[1].clone();
//     let file_path_2 = args[2].clone();

//     Config { word: word_to_be_searched_2, file_path: file_path_2 }
// }

// as parse_config in creating an instance of the Config, we gonna imp the same function for config as new.
// also in future we can call Config::new to create an instance.

impl Config {
    // pub fn new(args : &[String])-> Config{
    // changed the function name from "new" to "build" coz programmer expect new function to never fail, but here we can
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len()  < 3 {
            return Err("Not enough arguments");
        }

        let word_to_be_searched_2 = args[1].clone();
        let file_path_2 = args[2].clone();
        //3rd arg for case , checking the 
        // let argument = args[3].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            word: word_to_be_searched_2,
            file_path: file_path_2,
            ignore_case : ignore_case,
            // argument 
        })
    }
}
