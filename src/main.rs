use std::env;

//if you program arg could contain invalid unicode, use std::env::arg_os
// it return OsStrings instead of string
fn main (){
    let args : Vec<String> = env::args().collect();
    dbg!(args);
}