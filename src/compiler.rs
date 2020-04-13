extern crate regex;

use regex::Regex;
use std::str;

// Enums are more idomatic and make the resulting Vec much easier to understand
// I may need more types to make things easier to work with but for now I think
// this should suffice
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ident(String),
    Number(f64),
    Word(String),
    Key(String),
    Operator(char),
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
    let valid_symb = Regex::new(r"[\{\}\(\)=;\*\+\-\\]").unwrap();

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
                '-' => result.push(Expression::Operator(c)),
                '/' => result.push(Expression::Operator(c)),
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
    result.push(Expression::Ident("main".to_owned()));
    result.push(Expression::Lparen);
    result.push(Expression::Rparen);
    result.push(Expression::Semicolon);
    result.push(Expression::EOF);
    result //return the result
}

//compiler stuff

//making the nodes hold the actual values instead of the Expressions might be worth it to make
//interpreting easier
#[derive(PartialEq, Debug, Clone)]
pub enum ExprNode {
    Operation(Box<Expression>, Box<ExprNode>, Box<ExprNode>), //Operator, Left side, Right side
    Literal(Box<Expression>),
    Name(Box<Expression>),
    Call(Box<Expression>, Vec<ExprNode>),
    Block(Vec<ExprNode>),
    ParenOp(Box<ExprNode>),
    Func(Box<Expression>, Vec<ExprNode>, Box<ExprNode>), //Name, params, function body
    Illegal(Option<Expression>),
    EOF,
}

pub fn parse(tokens: Vec<Expression>) -> ExprNode {
    //let root = vec!();
    let mut iter = tokens.iter().peekable();
    let current = iter.next();

    let node = make_block(&mut iter, current);

    node
}

fn make_block(
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, Expression>>,
    cur: Option<&Expression>,
) -> ExprNode {
    let mut root = vec![];

    let mut t = cur;
    while t != None && *t.unwrap() != Expression::EOF && *t.unwrap() != Expression::Rbrace {
        match t {
            Some(Expression::Key(s)) => {
                root.push(key_word(iter, t, &s));
            }
            Some(Expression::Ident(_i)) => {
                root.push(expr(iter, t));
            }
            Some(Expression::Lbrace) => {
                t = iter.next();
                root.push(make_block(iter, t));
            }
            _ => {}
        }
        t = iter.next();
    }

    ExprNode::Block(root)
}

fn key_word(
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, Expression>>,
    cur: Option<&Expression>,
    word: &&String,
) -> ExprNode {
    let mut node = ExprNode::Illegal(None);
    match word.trim() {
        "print" => node = ExprNode::Call(Box::new(cur.unwrap().clone()), vec![expr(iter, cur)]),
        "fn" => node = def_func(iter, cur),
        _ => {}
    }

    node
}

fn def_func(
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, Expression>>,
    _cur: Option<&Expression>,
) -> ExprNode {
    let mut name: Expression = Expression::Ident("broken".to_owned());
    let mut params = vec![];
    let mut body: ExprNode = ExprNode::Illegal(None);

    if let Some(n) = iter.next() {
        match n {
            Expression::Ident(_) => name = n.clone(),
            _ => return ExprNode::Illegal(Some(n.clone())),
        }
    }
    loop {
        if let Some(p) = iter.next() {
            match p {
                Expression::Lparen => continue,
                Expression::Rparen => break,
                Expression::Ident(_) => params.push(ExprNode::Name(Box::new(p.clone()))),
                _ => {}
            }
        }
    }

    if let Some(b) = iter.next() {
        match b {
            Expression::Lbrace => {
                let c = iter.next();
                body = make_block(iter, c);
            }
            _ => {}
        }
    }

    ExprNode::Func(Box::new(name), params, Box::new(body))
}

fn expr(
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, Expression>>,
    cur: Option<&Expression>,
) -> ExprNode {
    let t = iter.next();
    let mut node: ExprNode = ExprNode::Illegal(None);
    if t == None {
        node = ExprNode::EOF;
        return node;
    }
    if let Some(Expression::Operator(_o)) = iter.peek() {
        match cur {
            Some(Expression::Lparen) => {
                return expr(iter, cur);
            }
            Some(_) => return expr(iter, t),
            None => {}
        }
    }

    match t {
        Some(Expression::Equal) => {
            node = ExprNode::Operation(
                Box::new(t.unwrap().clone()),
                Box::new(ExprNode::Name(Box::new(cur.unwrap().clone()))),
                Box::new(expr(iter, t)),
            )
        }
        Some(Expression::Operator(_)) => {
            node = if let Some(Expression::Lparen) = cur {
                let tmp = ExprNode::ParenOp(Box::new(expr(iter, t)));
                tmp
            } else {
                ExprNode::Operation(
                    Box::new(t.unwrap().clone()),
                    Box::new(make_node(cur.unwrap())),
                    Box::new(expr(iter, t)),
                )
            }
        }
        Some(Expression::Word(_s)) => node = ExprNode::Literal(Box::new(t.unwrap().clone())),
        Some(Expression::Number(_n)) => node = ExprNode::Literal(Box::new(t.unwrap().clone())),
        Some(Expression::Ident(_i)) => {
            node = if let Some(Expression::Lparen) = iter.peek() {
                expr(iter, cur)
            } else {
                ExprNode::Name(Box::new(t.unwrap().clone()))
            }
        }
        Some(Expression::Lparen) => {
            node = if let Some(Expression::Ident(_)) = cur {
                ExprNode::Call(Box::new(cur.unwrap().clone()), find_params(iter, cur))
            //if there was an identifier last before the '(', it should be a function call
            } else {
                make_paren_oper(iter, cur) //Otherwise it should be an operation
            }
        }
        Some(Expression::Rparen) => {
            node = if let Some(Expression::Semicolon) = iter.peek() {
                make_node(cur.unwrap())
            } else {
                let tmp = iter.next();
                expr(iter, tmp)
            }
        }
        _ => {}
    }

    node
}

fn make_paren_oper(
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, Expression>>,
    cur: Option<&Expression>,
) -> ExprNode {
    let mut last = cur.unwrap();
    let mut c = iter.next();
    let mut node = ExprNode::Illegal(None);
    let mut op = ExprNode::Illegal(None);
    println!("DEBUG: What is iter.peek(): {:?}", iter.peek());
    match iter.peek() {
        Some(Expression::Rparen) => {
            println!("DEBUG: What is iter.peek(): {:?}", iter.peek());
            iter.next();
            println!("DEBUG: What is iter.peek(): {:?}", iter.peek());
            match iter.peek() {
                Some(Expression::Operator(_o)) => {
                    println!("DEBUG: What is iter.peek(): {:?}", iter.peek());
                    node = if let Some(ex) = iter.next() {
                        ExprNode::Operation(
                            Box::new(ex.clone()),
                            Box::new(make_node(&last)),
                            Box::new(op.clone()),
                        )
                    } else {
                        op
                    }
                }
                _ => node = op,
            }
        }
        Some(Expression::Lparen) => node = make_paren_oper(iter, Some(&last)),
        Some(Expression::Operator(_o)) => {
            println!("DEBUG: iter.peek()={:?}\nc={:?}", iter.peek(), c);
            op = expr(iter, c);
            println!("DEBUG: iter.peek()={:?}\nc={:?}", iter.peek(), c);
            loop {
                if let Some(ex) = iter.peek() {
                    match ex {
                        Expression::Rparen => {
                            iter.next();
                        }
                        Expression::Operator(_) => {
                            node = ExprNode::Operation(
                                Box::new(iter.next().unwrap().clone()),
                                Box::new(make_node(iter.next().unwrap())),
                                Box::new(op.clone()),
                            )
                        }
                        Expression::Semicolon => break,
                        _ => break,
                    }
                } else {
                    node = op;
                    break;
                }
            }
        }

        Some(_) => {}
        None => {}
    }
    last = c.unwrap();
    c = iter.next();

    node
}

fn make_node(exp: &Expression) -> ExprNode {
    let node: ExprNode;
    match exp {
        Expression::Word(_s) => {
            node = ExprNode::Literal(Box::new(exp.clone()));
        }
        Expression::Number(_n) => {
            node = ExprNode::Literal(Box::new(exp.clone()));
        }
        Expression::Ident(_i) => {
            node = ExprNode::Name(Box::new(exp.clone()));
        }
        _ => node = ExprNode::Illegal(Some(exp.clone())),
    }

    node
}

fn find_params(
    peekable: &mut std::iter::Peekable<std::slice::Iter<'_, Expression>>,
    cur: Option<&Expression>,
) -> Vec<ExprNode> {
    let mut params = vec![];
    loop {
        match peekable.peek() {
            Some(Expression::Lparen) => {
                peekable.next();
                continue;
            }
            Some(Expression::Rparen) => {
                peekable.next();
                break;
            }
            Some(Expression::Semicolon) => break,
            Some(Expression::Lbrace) => panic!("Can't have block in function parameters"),
            _ => params.push(expr(peekable, cur)),
        }
    }
    params
}
