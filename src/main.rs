use std::{env, fs};

//if you program arg could contain invalid unicode, use std::env::arg_os
// it return OsStrings instead of string
fn main (){
    let args : Vec<String> = env::args().collect();
    
    let word_to_be_searched = &args[1];
    let file_path = &args[2];

    let file_content = fs::read_to_string(file_path).expect("Should have been able to read to file");

    println!("Contents of file : {}",file_content)


}