#[cfg(test)]
mod tests;

use super::lexer::*;
use std::iter::Peekable;
use std::slice::Iter;

//compiler stuff

//making the nodes hold the actual values instead of the Expressions might be worth it to make
//interpreting easier
#[derive(PartialEq, Debug, Clone)]
pub enum ExprNode {
    Operation(Box<Expression>, Box<ExprNode>, Box<ExprNode>), //Operator, Left side, Right side
    Literal(Box<Expression>),
    Name(Box<Expression>),
    Call(Box<Expression>, Vec<ExprNode>), //name, args
    Block(Vec<ExprNode>),
    ParenOp(Box<ExprNode>),
    Func(Box<Expression>, Vec<ExprNode>, Box<ExprNode>), //Name, params, function body
    Statement(Box<ExprNode>),
    ReturnVal(Box<ExprNode>),
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

fn make_block(iter: &mut Peekable<Iter<'_, Expression>>, cur: Option<&Expression>) -> ExprNode {
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
    iter: &mut Peekable<Iter<'_, Expression>>,
    cur: Option<&Expression>,
    word: &&String,
) -> ExprNode {
    let mut node = ExprNode::Illegal(None);
    match word.trim() {
        "print" => node = ExprNode::Call(Box::new(cur.unwrap().clone()), vec![expr(iter, cur)]),
        "fn" => node = def_func(iter, cur),
        "return" => node = ExprNode::ReturnVal(Box::new(expr(iter, cur))),
        _ => {}
    }

    node
}

fn def_func(iter: &mut Peekable<Iter<'_, Expression>>, _cur: Option<&Expression>) -> ExprNode {
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

fn expr(iter: &mut Peekable<Iter<'_, Expression>>, cur: Option<&Expression>) -> ExprNode {
    let t = iter.next();
    let mut node: ExprNode = ExprNode::Illegal(None);
    if t.is_none() {
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
                // iter.next();
                expr(iter, t)
            } else {
                ExprNode::Name(Box::new(t.unwrap().clone()))
            }
        }
        Some(Expression::Lparen) => {
            // println!("This is what cur =  {:?}", cur);
            node = if let Some(Expression::Ident(_)) = cur {
                ExprNode::Call(Box::new(cur.unwrap().clone()), find_params(iter, cur))
            //if there was an identifier last before the '(', it should be a function call
            } else {
                //Otherwise it should be a statement
                ExprNode::Statement(Box::new(expr(iter, cur)))
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
    peekable: &mut Peekable<Iter<'_, Expression>>,
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
