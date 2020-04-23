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
    StrLiteral(Box<String>),
    NumLiteral(Box<f32>),
    Name(Box<String>),
    Call(Box<Expression>, Vec<ExprNode>), //name, args
    Block(Vec<ExprNode>),
    Func(Box<Expression>, Vec<ExprNode>, Box<ExprNode>), //Name, params, function body
    Statement(Box<ExprNode>),
    ReturnVal(Box<ExprNode>),
    Illegal(Option<Expression>),
    EOF,
}

pub fn parse(tokens: Vec<Expression>) -> ExprNode {
    //let root = vec!();
    let mut iter = tokens.iter().peekable();

    make_block(&mut iter)

    // node
}

fn make_block(iter: &mut Peekable<Iter<'_, Expression>>) -> ExprNode {
    let mut root = vec![];

    while let Some(t) = iter.next() {
        match t {
            Expression::EOF | Expression::Rbrace => break,
            Expression::Key(s) => {
                root.push(key_word(iter, Some(t), &s));
            }
            Expression::Ident(_) => {
                root.push(expr(iter, Some(t)));
            }
            Expression::Lbrace => {
                root.push(make_block(iter));
            }
            _ => {}
        }
    }

    ExprNode::Block(root)
}

fn key_word(
    iter: &mut Peekable<Iter<'_, Expression>>,
    cur: Option<&Expression>,
    word: &str,
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

    while let Some(p) = iter.next() {
        match p {
            Expression::Lparen => continue,
            Expression::Rparen => break,
            Expression::Ident(i) => params.push(ExprNode::Name(Box::new(i.to_string()))),
            _ => {}
        }
    }

    if let Some(b) = iter.next() {
        if let Expression::Lbrace = b {
            body = make_block(iter);
        }
    }

    ExprNode::Func(Box::new(name), params, Box::new(body))
}

fn expr(iter: &mut Peekable<Iter<'_, Expression>>, cur: Option<&Expression>) -> ExprNode {
    let t = iter.next();
    let mut node: ExprNode = ExprNode::Illegal(None);

    if let Some(Expression::Operator(_)) = iter.peek() {
        match cur {
            Some(Expression::Lparen) => {
                return expr(iter, cur);
            }
            Some(_) => return expr(iter, t),
            None => {}
        }
    }
    if let Some(exp) = t {
        match exp {
            Expression::Equal => {
                if let Some(Expression::Ident(name)) = cur {
                    node = ExprNode::Operation(
                        Box::new(t.unwrap().clone()),
                        Box::new(ExprNode::Name(Box::new(name.to_string()))),
                        Box::new(expr(iter, t)),
                    )
                }
            }
            Expression::Operator(_) => {
                node = ExprNode::Operation(
                    Box::new(t.unwrap().clone()),
                    Box::new(make_node(cur.unwrap())),
                    Box::new(expr(iter, t)),
                )
            }
            Expression::Word(s) => node = ExprNode::StrLiteral(Box::new(s.to_string())),
            Expression::Number(n) => node = ExprNode::NumLiteral(Box::new(*n)),
            Expression::Ident(i) => {
                node = if let Some(Expression::Lparen) = iter.peek() {
                    // iter.next();
                    expr(iter, t)
                } else {
                    ExprNode::Name(Box::new(i.to_string()))
                }
            }
            Expression::Lparen => {
                // println!("This is what cur =  {:?}", cur);
                node = if let Some(Expression::Ident(_)) = cur {
                    ExprNode::Call(Box::new(cur.unwrap().clone()), find_params(iter, cur))
                //if there was an identifier last before the '(', it should be a function call
                } else {
                    //Otherwise it should be a statement
                    ExprNode::Statement(Box::new(expr(iter, cur)))
                }
            }
            Expression::Rparen => {
                node = if let Some(Expression::Semicolon) = iter.peek() {
                    make_node(cur.unwrap())
                } else {
                    let tmp = iter.next();
                    expr(iter, tmp)
                }
            }
            _ => {}
        }
    } else {
        node = ExprNode::EOF;
    }

    node
}

fn make_node(exp: &Expression) -> ExprNode {
    //feels bad to clone here but I don't know if it's avoidable
    match exp.clone() {
        Expression::Word(s) => ExprNode::StrLiteral(Box::new(s)),
        Expression::Number(n) => ExprNode::NumLiteral(Box::new(n)),
        Expression::Ident(i) => ExprNode::Name(Box::new(i)),
        _ => ExprNode::Illegal(Some(exp.clone())),
    }
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
