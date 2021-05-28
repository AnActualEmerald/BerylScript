#![feature(test)]

#[macro_use]
extern crate lalrpop_util;

mod interpreter;
mod parser;

lalrpop_mod!(arguments);

///Runs the lexer, parser, and interpreter on the provided string
pub fn run(data: String, args: &str, debug: bool) {
    let ast = parser::parse(&data);
    let p_args = arguments::ArgsParser::new().parse(&args).unwrap();
    interpreter::run(*ast, p_args);
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
