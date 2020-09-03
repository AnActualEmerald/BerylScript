#[cfg(test)]
mod tests;

use super::lexer::*;
use std::iter::Peekable;
use std::slice::Iter;

//compiler stuff

//making the nodes hold the actual values instead of the Expressions might be worth it to make
//interpreting easier
#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum ExprNode {
    Operation(Box<Expression>, Box<ExprNode>, Box<ExprNode>), //Operator, Left side, Right side
    StrLiteral(Box<String>),
    NumLiteral(Box<f32>),
    BoolLiteral(bool),
    Name(Box<String>),
    Call(Box<Expression>, Vec<ExprNode>), //name, args
    Block(Vec<ExprNode>),
    Func(Box<Expression>, Vec<ExprNode>, Box<ExprNode>), //Name, params, function body
    Loop(Box<String>, Box<ExprNode>, Box<ExprNode>),     //loop keyword, condition, block
    ForLoopDec(Box<ExprNode>, Box<ExprNode>, Box<ExprNode>), //declaration, condition, incrementation
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
    match word.trim() {
        "print" => ExprNode::Call(Box::new(cur.unwrap().clone()), vec![expr(iter, cur)]),
        "fn" => def_func(iter, cur),
        "return" => ExprNode::ReturnVal(Box::new(expr(iter, cur))),
        "true" => ExprNode::BoolLiteral(true),
        "false" => ExprNode::BoolLiteral(false),
        "while" => ExprNode::Loop(
            Box::new("while".to_string()),
            Box::new(expr(iter, cur)),
            Box::new(expr(iter, cur)),
        ),
        "for" => ExprNode::Loop(
            Box::new("for".to_string()),
            Box::new(make_for_loop(iter, cur)),
            Box::new(expr(iter, cur)),
        ),
        _ => panic!("Unknown keyword {}", word),
    }
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

    if let Some(next) = iter.peek() {
        match next {
            Expression::Operator(_) => match cur {
                Some(Expression::Lparen) => {
                    return expr(iter, cur);
                }
                Some(_) => return expr(iter, t),
                None => {}
            },
            Expression::BoolOp(_) => return expr(iter, t),
            _ => {}
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
            Expression::BoolOp(_) => {
                node = ExprNode::Operation(
                    Box::new(t.unwrap().clone()),
                    Box::new(make_node(cur.unwrap())),
                    Box::new(expr(iter, t)),
                )
            }
            Expression::Word(s) => node = ExprNode::StrLiteral(Box::new(s.to_string())),
            Expression::Number(n) => node = ExprNode::NumLiteral(Box::new(*n)),
            Expression::Key(w) => node = key_word(iter, cur, w),
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
            Expression::Lbrace => {
                node = make_block(iter);
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

fn make_for_loop(iter: &mut Peekable<Iter<'_, Expression>>, cur: Option<&Expression>) -> ExprNode {
    if let Some(Expression::Lparen) = iter.peek() {
        iter.next(); //skip the lparen after the "for" keyword
        let mut name = iter.next(); //grab the next expression to pass it in as cur
        let dec = expr(iter, name); //get the declaration expression (i = 0)
        if let ExprNode::Operation(op, _, _) = &dec {
            //double check to make sure this was an assignment op
            if **op == Expression::Equal {
                let condition = expr(iter, cur); //get the condition expression (i < 10)
                iter.next(); //skip the last semicolon
                name = iter.next(); //get the name again to use as cur
                let increment = expr(iter, name); //get the incrementation expression (i = i + 1)
                return ExprNode::ForLoopDec(
                    Box::new(dec),
                    Box::new(condition),
                    Box::new(increment),
                );
            }
        }
        //for loops don't need to have an assinment op, so that needs to be supported
        iter.next(); //skip the last semicolon
        name = iter.next(); //get the name again to use as cur
        let increment = expr(iter, name); //get the incrementation expression (i = i + 1)
        ExprNode::ForLoopDec(
            Box::new(ExprNode::Illegal(None)),
            Box::new(dec),
            Box::new(increment),
        )
    } else {
        panic!("Expected \"(\", found {:?}", iter.next());
    }
}
