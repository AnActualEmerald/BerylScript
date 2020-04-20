use crate::interpreter::*;
use crate::lexer::Expression;
use crate::parser::ExprNode;
use std::collections::HashMap;

#[test]
fn generate_literals() {
    let dummy_string = ExprNode::Literal(Box::new(Expression::Word("Test".to_owned())));
    let dummy_number = ExprNode::Literal(Box::new(Expression::Number(69.0)));

    let expected_string = Value::EmString("Test".to_owned());
    let expected_number = Value::Float(69.0);

    let mut r = Runtime { returning: false };
    let mut stack = StackFrame {
        stack: HashMap::new(),
    };
    assert_eq!(r.walk_tree(&dummy_string, &mut stack), expected_string);

    assert_eq!(r.walk_tree(&dummy_number, &mut stack), expected_number);
}

#[test]
fn assign_vars() {
    let op = ExprNode::Operation(
        Box::new(Expression::Equal),
        Box::new(ExprNode::Name(Box::new(Expression::Ident(
            "test".to_owned(),
        )))),
        Box::new(ExprNode::Literal(Box::new(Expression::Word(
            "this is a test".to_owned(),
        )))),
    );

    let expected = Value::EmString("this is a test".to_owned());

    let mut r = Runtime { returning: false };
    let mut stack = StackFrame {
        stack: HashMap::new(),
    };
    r.walk_tree(&op, &mut stack);
    assert_eq!(stack.get_var(&"test".to_owned()), &expected);
}
