extern crate test;

use super::*;
use test::Bencher;

#[bench]
fn lexing_bench(b: &mut Bencher) {
    let dummy = "fn test(){ print \"Hello, World!\"; }";
    b.iter(|| lexer::run(dummy));
}

#[bench]
fn parsing_bench(b: &mut Bencher) {
    use lexer::Expression;
    let dummy = vec![
        Expression::Key("fn".to_owned()),
        Expression::Ident("test".to_owned()),
        Expression::Lparen,
        Expression::Rparen,
        Expression::Lbrace,
        Expression::Key("print".to_owned()),
        Expression::Word("hello world".to_owned()),
        Expression::Semicolon,
        Expression::Rbrace,
        Expression::EOF,
    ];

    b.iter(move || parser::parse(dummy.clone()));
}

#[bench]
fn exec_bench(b: &mut Bencher) {
    use lexer::Expression;
    let dummy = parser::parse(vec![
        Expression::Key("fn".to_owned()),
        Expression::Ident("main".to_owned()),
        Expression::Lparen,
        Expression::Rparen,
        Expression::Lbrace,
        Expression::Key("print".to_owned()),
        Expression::Word("hello world".to_owned()),
        Expression::Semicolon,
        Expression::Rbrace,
        Expression::Ident("main".to_owned()),
        Expression::Lparen,
        Expression::Rparen,
        Expression::Semicolon,
        Expression::EOF,
    ]);

    b.iter(|| interpreter::run(dummy.clone()));
}
