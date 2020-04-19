mod interpreter;
mod lexer;
mod parser;

use std::env;
use std::fs;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();

    let now = SystemTime::now();
    let data = load_source_file(&args[1]);
    // println!("File: {}", data);
    let tokens = lexer::tokenize(&data);
    // println!("{:?}", tokens);
    let ast = parser::parse(tokens);
    // println!("{:?}", ast);
    interpreter::run(ast);
    if let Ok(el) = now.elapsed() {
        println!("Took {}s", el.as_secs_f32());
    }
}

fn load_source_file(path: &str) -> String {
    let res =
        fs::read_to_string(path).expect(&format!("Had a problem reading the file at {}", path));
    res
}
