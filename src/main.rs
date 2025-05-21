use std::{env, fs};

fn main() {
    //iterators
    // iterators produce a series of values
    // we can call .collect to convert it into an collection like vector

    let args: Vec<String> = env::args().collect();

    let config= Config::new(&args);
    println!("Search for - {}", config.query);
    println!("The file targeted - {}",config.target_file);


    // read the file
    // import the fs

    // let content = fs::read_to_string("story.txt").expect("Must been able to read the file");
    let content = fs::read_to_string(config.target_file).expect("Must been able to read the file");
    

    println!(
        "With text:\n{content}"
    );
}

struct Config {
    query : String,
    target_file :String
}
impl Config {
 fn new(args: &[String])->Config{
        let query = args[1].clone();
        let target_file = args[2].clone();
        Config { query, target_file }
    }   
   
}
 