use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub target_file: String,
}
impl Config {
   pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided.");
        }
        let query = args[1].clone();
        let target_file = args[2].clone();
        Ok(Config { query, target_file })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let content = fs::read_to_string("story.txt").expect("Must been able to read the file");
    let content = fs::read_to_string(config.target_file)?;
    println!("With text:\n{content}");
    Ok(())
}
