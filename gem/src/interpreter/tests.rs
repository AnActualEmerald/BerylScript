use crate::interpreter::*;
use crate::lexer;
use crate::lexer::Expression;
use crate::parser;
use crate::parser::ExprNode;
use std::collections::HashMap;

#[test]
fn generate_literals() {
    let dummy_string = ExprNode::StrLiteral(Box::new("Test".to_owned()));
    let dummy_number = ExprNode::NumLiteral(Box::new(69.0 as f32));

    let expected_string = Value::EmString("Test".to_owned());
    let expected_number = Value::Float(69.0);

    let mut r = Runtime::new();
    let mut stack = StackFrame {
        stack: HashMap::new(),
    };
    assert_eq!(
        r.walk_tree(&dummy_string, &mut stack).unwrap(),
        expected_string
    );

    assert_eq!(
        r.walk_tree(&dummy_number, &mut stack).unwrap(),
        expected_number
    );
}

#[test]
fn assign_vars() {
    let op = ExprNode::Operation(
        Box::new(Expression::Equal),
        Box::new(ExprNode::Name(Box::new("test".to_owned()))),
        Box::new(ExprNode::StrLiteral(Box::new("this is a test".to_owned()))),
    );

    let expected = Value::EmString("this is a test".to_owned());

    let mut r = Runtime::new();
    let mut stack = StackFrame {
        stack: HashMap::new(),
    };
    r.walk_tree(&op, &mut stack).expect("Unable to walk tree");
    assert_eq!(stack.get_var(&"test"), &expected);
}

#[test]
fn looping() {
    let ty = String::from("while");
    let condition = ExprNode::Operation(
        Box::new(Expression::BoolOp("<".to_owned())),
        Box::new(ExprNode::Name(Box::new("i".to_owned()))),
        Box::new(ExprNode::NumLiteral(Box::new(10 as f32))),
    );
    let block = ExprNode::Block(vec![ExprNode::Operation(
        Box::new(Expression::Equal),
        Box::new(ExprNode::Name(Box::new("i".to_owned()))),
        Box::new(ExprNode::Operation(
            Box::new(Expression::Operator('+')),
            Box::new(ExprNode::Name(Box::new("i".to_owned()))),
            Box::new(ExprNode::NumLiteral(Box::new(1.0 as f32))),
        )),
    )]);
    // let loop_test = ExprNode::Loop(Box::new(ty), Box::new(condition), Box::new(block));
    let mut r = Runtime::new();
    let mut stack = StackFrame {
        stack: HashMap::new(),
    };
    stack.set_var(String::from("i"), Value::Float(0.0 as f32));
    r.do_loop(&ty, &condition, &block, &mut stack)
        .expect("Error executing loop");

    assert_eq!(*stack.get_var("i"), Value::Float(10.0));

    //test for loops here

    //include a file with the code to test in it so it can be updated more easily
    if let Ok(dummy) = parser::parse(lexer::run(include_str!("test_files/for_test.em"))) {
        let mut runtime = Runtime::new();
        let mut frame = StackFrame::new();

        repl_run(dummy, &mut runtime, &mut frame).expect("Unable to perform run");

        return assert_eq!(Value::Float(10.0), *frame.get_var("result"));
    }

    assert!(false);
}

//this effectively also tests if arrays is working correctly due to the way the test file is written
#[test]
fn if_elif_else() {
    if let Ok(dummy) = parser::parse(lexer::run(include_str!("test_files/if_elif_else_test.em"))) {
        let mut runtime = Runtime::new();
        let mut frame = StackFrame::new();

        match repl_run(dummy, &mut runtime, &mut frame) {
            Ok(_) => {}
            Err(e) => println!("Got this error running the if test: {}", e),
        }

        if let Value::EmArray(v) = frame.get_var("res") {
            for val in v {
                match **val {
                    Value::EmBool(b) => return assert!(b),
                    _ => {}
                }
            }
        }

        assert!(false);
    }
}
