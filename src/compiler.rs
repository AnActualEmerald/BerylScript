extern crate regex;

use regex::Regex;
use std::str;

// Enums are more idomatic and make the resulting Vec much easier to understand
// I may need more types to make things easier to work with but for now I think
// this should suffice
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ident(String),
    Operator(char),
    Number(f64),
    Word(String),
    Key(String),
    Equal,
    Rparen,
    Lparen,
    Rbrace,
    Lbrace,
    Semicolon,
    EOF,
}

#[derive(PartialEq, Debug)]
enum State {
    Nothing,
    EmString,
    EmName,
    EmNumber,
}

pub fn tokenize(data: &str) -> Vec<Expression> {
    let mut result = vec![];
    let mut tok = String::new();
    let mut current_state = State::Nothing;

    let ch = data.chars().clone();

    let valid_chars = Regex::new(r"\D+[[:word:]]*").unwrap();
    let valid_num = Regex::new(r"\d*").unwrap();
    let valid_symb = Regex::new(r"[\{\}\(\)=;]").unwrap();

    // This whole thing could use a mutable iterator to check over the data until it finds
    // somthing of interest i.e. the closing " or whatever, but idk if that's faster or better
    // so this is what I'll stick with for now
    for c in ch {
        if !c.is_whitespace() && current_state != State::EmString {
            tok.push(c);
        } else if current_state == State::EmString {
            tok.push(c);
        }

        if c == '"' {
            if current_state == State::EmString {
                tok.pop();
                result.push(Expression::Word(tok.clone()));
                tok = format!("");
                current_state = State::Nothing;
            } else {
                current_state = State::EmString;
                tok = format!("");
            }
        } else if valid_symb.is_match(&tok) || c.is_whitespace() && current_state != State::EmString
        {
            if !c.is_whitespace() {
                tok.pop();
            }
            match current_state {
                State::EmName => result.push(Expression::Ident(tok.clone())),
                State::EmNumber => {
                    result.push(Expression::Number(tok.clone().parse::<f64>().unwrap()))
                }
                _ => {}
            }
            match c {
                '{' => result.push(Expression::Lbrace),
                '}' => result.push(Expression::Rbrace),
                '(' => result.push(Expression::Lparen),
                ')' => result.push(Expression::Rparen),
                '=' => result.push(Expression::Equal),
                '*' => result.push(Expression::Operator(c)),
                '+' => result.push(Expression::Operator(c)),
                ';' => result.push(Expression::Semicolon),
                _ => {}
            }
            tok = format!("");
            current_state = State::Nothing;
        } else if valid_chars.is_match(&tok) && current_state != State::EmString {
            if tok == format!("fn") {
                //check for all keywords
                result.push(Expression::Key(tok.clone()));
                current_state = State::Nothing;
                tok = format!("");
            } else if tok == format!("print") {
                result.push(Expression::Key(tok.clone()));
                current_state = State::Nothing;
                tok = format!("");
            } else {
                current_state = State::EmName;
            }
        } else if valid_num.is_match(&tok) && current_state != State::EmString {
            current_state = State::EmNumber
        }
    }
    result.push(Expression::EOF);
    result //return the result
}

//compiler stuff
#[derive(PartialEq, Debug, Clone)]
pub enum ExprNode {
    Operation(Box<Expression>, Box<ExprNode>, Box<ExprNode>), //Operator, Left side, Right side
    Literal(Box<Expression>),
    Name(Box<Expression>),
    Call(Box<Expression>, Box<ExprNode>),
    Block(Vec<ExprNode>),
    Illegal,
    EOF,
}

pub fn parse(tokens: Vec<Expression>) -> ExprNode {
    //let root = vec!();
    let mut iter = tokens.iter();
    let current = iter.next();

    let node = make_block(&mut iter, current);

    // while *current.unwrap() != Expression::EOF {
    //     match current {
    //         Some(Expression::Key(s)) => {
    //             if s == "print"  {

    //             }
    //         },
    //         None => {
    //         }
    //         _ => {}
    //     }

    // }

    node
}

fn make_block(iter: &mut std::slice::Iter<'_, Expression>, cur: Option<&Expression>) -> ExprNode {
    let mut root = vec![];

    let mut t = cur;
    while t != None && *t.unwrap() != Expression::EOF {
        match t {
            Some(Expression::Key(s)) => {
                root.push(key_word(iter, t, &s));
            }
            Some(Expression::Ident(_i)) => {
                root.push(expr(iter, t));
            }
            _ => {}
        }
        t = iter.next();
    }

    ExprNode::Block(root)
}

fn key_word(
    iter: &mut std::slice::Iter<'_, Expression>,
    cur: Option<&Expression>,
    word: &&String,
) -> ExprNode {
    let mut node = ExprNode::Illegal;
    match word.trim() {
        "print" => node = ExprNode::Call(Box::new(cur.unwrap().clone()), Box::new(expr(iter, cur))),
        _ => {}
    }

    node
}

fn expr(iter: &mut std::slice::Iter<'_, Expression>, cur: Option<&Expression>) -> ExprNode {
    let t = iter.next();
    let mut node: ExprNode = ExprNode::Illegal;
    if t == None {
        node = ExprNode::EOF;
        return node;
    }
    match t {
        Some(Expression::Equal) => {
            node = ExprNode::Operation(
                Box::new(t.unwrap().clone()),
                Box::new(ExprNode::Name(Box::new(cur.unwrap().clone()))),
                Box::new(expr(iter, cur)),
            );
        }
        Some(Expression::Operator(_o)) => {
            node = ExprNode::Operation(
                Box::new(t.unwrap().clone()),
                Box::new(node),
                Box::new(expr(iter, cur)),
            );
        }
        Some(Expression::Word(_s)) => {
            node = ExprNode::Literal(Box::new(t.unwrap().clone()));
        }
        Some(Expression::Number(_n)) => {
            node = ExprNode::Literal(Box::new(t.unwrap().clone()));
        }
        Some(Expression::Ident(_i)) => {
            node = ExprNode::Name(Box::new(t.unwrap().clone()));
        }
        _ => {}
    }

    node
}
