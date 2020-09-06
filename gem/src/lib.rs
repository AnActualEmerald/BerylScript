// #![feature(test)]

pub mod interpreter;
pub mod lexer;
pub mod parser;

// #[cfg(test)]
// mod bench;

pub fn run(data: String) {
    // println!("Got data: {}", data);
    let tokens = lexer::run(&data);
    // println!("Generated tokens: {:?}", tokens);
    let ast = parser::parse(tokens);
    // println!("Generated ast: {:?}", ast);
    interpreter::run(ast);
}
