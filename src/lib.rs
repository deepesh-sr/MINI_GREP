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

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Safe";
        let content = "\
Rust:
Safe , Fast , Productive.
Pick three.";

        assert_eq!(vec!["Safe , Fast , Productive."], search(query, content));
    }
}
