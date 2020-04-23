#![feature(test)]

pub mod interpreter;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod bench;

pub fn run(data: String) {
    // println!("Got data: {}", data);
    let tokens = lexer::tokenize(&data);
    let ast = parser::parse(tokens);
    interpreter::run(ast);
}
