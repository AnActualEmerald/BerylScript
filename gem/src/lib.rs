#![feature(test)]

pub mod interpreter;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod bench;

///Runs the lexer, parser, and interpreter on the provided string
pub fn run(data: String) {
    // println!("Got data: {}", data);
    let tokens = lexer::run(&data);
    // println!("Generated tokens: {:?}", tokens);
    match parser::parse(tokens) {
        Ok(ast) => {
            println!("{:?}", &ast);
            interpreter::run(ast)
        }
        Err(e) => println!("{}", e),
    }
    // println!("Generated ast: {:?}", ast);
}
