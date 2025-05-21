use std::{env, fs};

fn main() {
    //iterators
    // iterators produce a series of values
    // we can call .collect to convert it into an collection like vector

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_patch = &args[2];

    println!("Search for - {}", query);
    println!("The file targeted - {file_patch}");

    // read the file
    // import the fs

    let content = fs::read_to_string("story.txt").expect("Must been able to read the file");

    println!(
        "With text:\n{content}"
    );
}
