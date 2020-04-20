use crate::gem::lexer::{tokenize, Expression};

#[test]
fn token_generation() {
    let dummy = "fn test() { print \"hello world\"; }";
    let expected = vec![
        Expression::Key("fn".to_owned()),
        Expression::Ident("test".to_owned()),
        Expression::Lparen,
        Expression::Rparen,
        Expression::Lbrace,
        Expression::Key("print".to_owned()),
        Expression::Word("hello world".to_owned()),
        Expression::Semicolon,
        Expression::Rbrace,
        Expression::Ident("main".to_owned()), //Exposing the janky way I'm calling the main function
        Expression::Lparen,
        Expression::Rparen,
        Expression::Semicolon,
        Expression::EOF,
    ];

    assert_eq!(expected, tokenize(dummy));
}
