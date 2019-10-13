mod lexer;

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();


    let data = load_source_file(&args[1]);
    let tokens = lexer::tokenize(&data);
    println!("{:?}", tokens);
}


fn load_source_file(path: &str) -> String {
    let res = fs::read_to_string(path).expect(&format!("Had a problem reading the file at {}", path));
    res
}
