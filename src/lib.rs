#![feature(test)]

#[macro_use]
extern crate lalrpop_util;

mod interpreter;
mod parser;

use parser::Node;

///Runs the lexer, parser, and interpreter on the provided string
pub fn run(data: String, args: &Vec<&str>, debug: bool) {
    if let Some(ast) = parser::parse(&data) {
        let p_args: Node = Node::Array(
            args.iter()
                .map(|a| Node::StrLiteral(a.to_string()))
                .collect(),
        );
        if debug {
            println!("{:?}", *ast);
        }
        interpreter::run(*ast, p_args);
    }
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
