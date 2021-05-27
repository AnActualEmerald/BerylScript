#![feature(test)]

#[macro_use]
extern crate lalrpop_util;

pub mod parser;

///Runs the lexer, parser, and interpreter on the provided string
pub fn run(data: String, args: &str, debug: bool) {
    println!("{:?}", *parser::parse(&data));
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
