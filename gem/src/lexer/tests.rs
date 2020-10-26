use crate::lexer;
use crate::lexer::Expression;

#[test]
fn token_generation() {
    let dummy = "fn test() { print \"hello world\"; }";
    let expected = vec![
        Expression::Key("fn".to_owned()),
        Expression::Ident("test".to_owned()),
        Expression::Lparen,
        Expression::Rparen,
        Expression::Lbrace,
        Expression::Ident("print".to_owned()),
        Expression::Word("hello world".to_owned()),
        Expression::Semicolon,
        Expression::Rbrace,
    ];

    assert_eq!(expected, lexer::run(dummy));
}
