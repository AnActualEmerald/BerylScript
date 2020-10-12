#![feature(test)]

pub mod interpreter;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod bench;

///Runs the lexer, parser, and interpreter on the provided string
pub fn run(data: String, debug: bool) {
    let tokens = lexer::run(&data);
    if debug {
        println!("Generated tokens: {:?}", tokens);
    }
    match parser::parse(tokens) {
        Ok(ast) => {
            if debug {
                println!("{:?}", &ast);
            }
            interpreter::run(ast)
        }
        Err(e) => println!("{}", e),
    }
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
