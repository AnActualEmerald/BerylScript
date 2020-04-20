use crate::gem::lexer::Expression;
use crate::gem::parser::*;

#[test]
fn ast_generation() {
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

    let expected = ExprNode::Block(vec![ExprNode::Func(
        Box::new(Expression::Ident("test".to_owned())),
        vec![],
        Box::new(ExprNode::Block(vec![ExprNode::Call(
            Box::new(Expression::Key("print".to_owned())),
            vec![ExprNode::Literal(Box::new(Expression::Word(
                "hello world".to_owned(),
            )))],
        )])),
    )]);

    assert_eq!(parse(dummy), expected);
}
