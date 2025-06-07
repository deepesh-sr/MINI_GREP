use std::{env, error::Error, fs, process};

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

    run(config);

    // read the file
    // import the fs
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let content = fs::read_to_string("story.txt").expect("Must been able to read the file");
    let content = fs::read_to_string(config.target_file)?;
    println!("With text:\n{content}");
    Ok(())
}

struct Config {
    query: String,
    target_file: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided.");
        }
        let query = args[1].clone();
        let target_file = args[2].clone();
        Ok(Config { query, target_file })
    }
}
