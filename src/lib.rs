#![feature(test)]

pub mod interpreter;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod bench;

///Runs the lexer, parser, and interpreter on the provided string
pub fn run(data: String, args: &str, debug: bool) {
    let tokens = lexer::run(&data);
    if debug {
        println!("Generated tokens: {:?}", tokens);
    }
    match parser::parse(tokens) {
        Ok(ast) => {
            let args = parser::read_line(None, &mut lexer::run(&format!("[{}]", args)).iter().peekable(), &vec![&lexer::Expression::Semicolon]).unwrap();

            if debug {
                println!("{:?}", &ast);
                println!("{:?}", args);
            }

            interpreter::run(ast, args)
        }
        Err(e) => println!("{}", e),
    }
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
