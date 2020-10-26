#[cfg(test)]
mod tests;

use super::lexer::*;
use std::iter::Peekable;
use std::slice::Iter;

//compiler stuff

//making the nodes hold the actual values instead of the Expressions might be worth it to make
//interpreting easier
///Enum with variants for each type of statement or literal in the lang
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
    IfStatement(Box<ExprNode>, Box<ExprNode>, Box<ExprNode>), //condition, body, branch
    ElseStatement(Box<ExprNode>),                             //body
    Array(Vec<ExprNode>),
    Index(Box<ExprNode>, Box<ExprNode>), //array identifier, inedex
    Illegal(Option<Expression>),
    EOF,
}

///Starts the parser
pub fn parse(tokens: Vec<Expression>) -> Result<ExprNode, String> {
    //let root = vec!();
    let iter = tokens.iter();

    make_block(&mut iter.peekable())

    // node
}

///Loops through expressions to generate all of the nodes in a block of code
fn make_block(iter: &mut Peekable<Iter<Expression>>) -> Result<ExprNode, String> {
    let mut root = vec![];

    while let Some(t) = iter.next() {
        match t {
            Expression::EOF => break,
            Expression::Rbrace => {
                // iter.next();
                break;
            }
            Expression::Key(s) => {
                root.push(key_word(iter, Some(t), &s)?);
            }
            Expression::Ident(_) => {
                root.push(expr(iter, Some(t))?);
            }
            Expression::Lbrace => {
                root.push(make_block(iter)?);
            }
            _ => {}//root.push(expr(iter, Some(t))?)} //root.push(read_line(Some(&t), iter)?),
        }
    }

    Ok(ExprNode::Block(root))
}

///Handles all the different keywords
fn key_word(
    iter: &mut Peekable<Iter<'_, Expression>>,
    cur: Option<&Expression>,
    word: &str,
) -> Result<ExprNode, String> {
    match word.trim() {
        "print" | "println" => Ok(ExprNode::Call(
            Box::new(Expression::Key(word.to_owned())),
            vec![read_line(None, iter, &vec![])?],
        )),
        "fn" => def_func(iter, cur),
        "return" => Ok(ExprNode::ReturnVal(Box::new(expr(iter, cur)?))),
        "true" => Ok(ExprNode::BoolLiteral(true)),
        "false" => Ok(ExprNode::BoolLiteral(false)),
        "while" => {
            let con = expr(iter, cur)?;
            iter.next(); // have to skip the closing paren
            iter.next(); // and the opening brace
            let body = make_block(iter)?;
            Ok(ExprNode::Loop(
                Box::new("while".to_string()),
                Box::new(con),
                Box::new(body),
            ))
        }
        "for" => Ok(ExprNode::Loop(
            Box::new("for".to_string()),
            Box::new(make_for_loop(iter)?),
            Box::new(make_block(iter)?),
        )),
        "if" => make_if(iter),
        _ => Err(format!("Unknown keyword {}", word)),
    }
}

///Generates the nodes needed to define a function
fn def_func(
    iter: &mut Peekable<Iter<'_, Expression>>,
    _cur: Option<&Expression>,
) -> Result<ExprNode, String> {
    let mut name: Expression = Expression::Ident("broken".to_owned());
    let mut params = vec![];
    let mut body: ExprNode = ExprNode::Illegal(None);

    if let Some(n) = iter.next() {
        match n {
            Expression::Ident(_) => name = n.clone(),
            _ => return Err(format!("Expected indentifier found {:?}", n)),
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
            body = make_block(iter)?;
        }
    }

    Ok(ExprNode::Func(Box::new(name), params, Box::new(body)))
}

///Reads to the end of the current line, stopping at the first semicolon or lbrace, or the specified deliminator
pub fn read_line<'a>(
    prev: Option<&Vec<Expression>>,
    iter: &mut Peekable<Iter<Expression>>,
    delim: &Vec<&Expression>,
) -> Result<ExprNode, String> {
    //iterate through the next set of expressions until we get to a ';'
    let mut accum = if let Some(v) = prev {
        v.clone()
    } else {
        Vec::new()
    };
    
    for exp in iter.take_while(|e| !(delim.contains(e)|| Expression::Lbrace == **e)) {
        match exp {
            Expression::Lbracket => {
                return make_array(iter);
            }
            Expression::Operator(_) => {
                return Ok(ExprNode::Operation(
                    Box::new(exp.clone()),
                    Box::new(expr(&mut accum.iter().peekable(), None)?),
                    Box::new(read_line(None, iter, delim)?),
                ));
            }
            Expression::BoolOp(_) => {
                return Ok(ExprNode::Operation(
                    Box::new(exp.clone()),
                    Box::new(expr(&mut accum.iter().peekable(), None)?),
                    Box::new(read_line(None, iter, delim)?),
                ))
            }
            Expression::CompoundOp(_) => {
                let left = expr(&mut accum.iter().peekable(), None)?;
                return make_compound_op(left, exp, iter);
            }
            Expression::Equal => {
                return Ok(ExprNode::Operation(
                    Box::new(exp.clone()),
                    Box::new(expr(&mut accum.iter().peekable(), None)?),
                    Box::new(read_line(None, iter, &vec![])?),
                ))
            }
            _ => accum.push(exp.clone()),
        }
    }

    Ok(expr(&mut accum.iter().peekable(), None)?)
}

fn expr(
    iter: &mut Peekable<Iter<'_, Expression>>,
    cur: Option<&Expression>,
) -> Result<ExprNode, String> {
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
        // println!(
        //     "The expresion was: {:?}, current Expression was {:?}",
        //     &t, cur
        // );
        match exp {
            Expression::Equal => {
                if let Some(Expression::Ident(name)) = cur {
                    node = ExprNode::Operation(
                        Box::new(t.unwrap().clone()),
                        Box::new(ExprNode::Name(Box::new(name.to_string()))),
                        Box::new(read_line(None, iter, &vec![&Expression::Semicolon])?),
                    )
                }
            }
            Expression::Operator(_) => {
                node = ExprNode::Operation(
                    Box::new(t.unwrap().clone()),
                    Box::new(make_node(cur.unwrap())),
                    Box::new(read_line(None, iter, &vec![&Expression::Semicolon])?),
                )
            }
            Expression::CompoundOp(_) => {
                node = make_compound_op(make_node(cur.unwrap()), t.unwrap(), iter)?;
            }
            Expression::BoolOp(_) => {
                node = ExprNode::Operation(
                    Box::new(t.unwrap().clone()),
                    Box::new(make_node(cur.unwrap())),
                    Box::new(expr(iter, t)?),
                )
            }
            Expression::Word(s) => node = ExprNode::StrLiteral(Box::new(s.to_string())),
            Expression::Number(n) => node = ExprNode::NumLiteral(Box::new(*n)),
            Expression::Key(w) => node = key_word(iter, cur, w)?,
            Expression::Ident(i) => {
                // println!("Next exp: {:?}", iter.peek());
                node = match iter.peek() {
                    Some(Expression::Lparen) => expr(iter, t)?,
                    Some(Expression::Lbracket) => index_array(t.unwrap(), iter)?,
                    Some(Expression::Operator(_)) => expr(iter, t)?,
                    Some(Expression::Equal) => expr(iter, t)?,
                    _ => ExprNode::Name(Box::new(i.to_string())),
                }
            }
            Expression::Lparen => {
                // println!("This is what cur =  {:?}", cur);
                node = if let Some(Expression::Ident(_)) = cur {
                    ExprNode::Call(Box::new(cur.unwrap().clone()), find_params(iter)?)
                //if there was an identifier last before the '(', it should be a function call
                } else {
                    //Otherwise it should be a statement
                    ExprNode::Statement(Box::new(expr(iter, cur)?))
                }
            }
            Expression::Rparen => {
                node = if let Some(Expression::Semicolon) = iter.peek() {
                    make_node(cur.unwrap())
                } else {
                    let tmp = iter.next();
                    expr(iter, tmp)?
                }
            }
            Expression::Lbrace => {
                node = make_block(iter)?;
            }
            Expression::Lbracket => {
                node = match cur {
                    Some(Expression::Ident(_)) => read_line(
                        Some(&vec![cur.unwrap().clone(), t.unwrap().clone()]),
                        iter,
                        &vec![],
                    )?,
                    _ => make_array(iter)?,
                }
            }
            Expression::Semicolon => {}
            _ => node = expr(iter, cur)?,
        }
    } else {
        node = ExprNode::EOF;
    }

    Ok(node)
}

fn make_compound_op(
    ident: ExprNode,
    compop: &Expression,
    iter: &mut Peekable<Iter<'_, Expression>>,
) -> Result<ExprNode, String> {
    //idk if this is good or not but I don't see why such a niche function needs to be defined outside
    //of the only place it's ever used
    let make_op = |op, right| {
        ExprNode::Operation(
            Box::new(Expression::Equal),
            Box::new(ident.clone()),
            Box::new(ExprNode::Operation(
                Box::new(op),
                Box::new(ident),
                Box::new(right),
            )),
        )
    };

    if let Expression::CompoundOp(tmp) = compop {
        match tmp.as_str() {
            "+=" => {
                let op = Expression::Operator('+');
                let right = read_line(None, iter, &vec![])?;
                //converts 'x += y' to 'x = x + y'
                Ok(make_op(op, right))
            }
            "-=" => {
                let op = Expression::Operator('-');
                let right = read_line(None, iter, &vec![])?;
                Ok(make_op(op, right))
            }
            "*=" => {
                let op = Expression::Operator('*');
                let right = read_line(None, iter, &vec![])?;
                Ok(make_op(op, right))
            }
            "/=" => {
                let op = Expression::Operator('/');
                let right = read_line(None, iter, &vec![])?;
                Ok(make_op(op, right))
            }
            "++" => {
                let op = Expression::Operator('+');
                let right = ExprNode::NumLiteral(Box::new(1.0));
                Ok(make_op(op, right))
            }
            "--" => {
                let op = Expression::Operator('-');
                let right = ExprNode::NumLiteral(Box::new(1.0));
                Ok(make_op(op, right))
            }
            _ => Err(format!("Unknown compound operator {}", tmp)),
        }
    } else {
        Err(format!("Compound op wasn't a compound op"))
    }
}

fn build_chain_back(
    ident: &Expression,
    iter: &mut Peekable<std::iter::Rev<Iter<'_, ExprNode>>>,
) -> Option<ExprNode> {
    if let Some(_) = iter.peek() {
        let index = iter.next().unwrap();

        if let Some(op) = build_chain_back(ident, iter) {
            //If the next call in the chain returns something, build it normally
            Some(ExprNode::Operation(
                Box::new(Expression::Lbracket),
                Box::new(op),
                Box::new(index.clone()),
            ))
        } else {
            //If not, we've reached the end and shoud return the root node with the actual identifier
            Some(ExprNode::Operation(
                Box::new(Expression::Lbracket),
                Box::new(make_node(ident)),
                Box::new(index.clone()),
            ))
        }
    } else {
        None
    }
}

fn index_array(
    ident: &Expression,
    iter: &mut Peekable<Iter<'_, Expression>>,
) -> Result<ExprNode, String> {
    //check if we need to skip the bracket or not
    if let Some(Expression::Lbracket) = iter.peek() {
        iter.next();
    }
    let mut multidex = vec![];
    let index = expr(iter, None)?;
    iter.next(); //skip the closing rbracket
    if let Some(Expression::Lbracket) = iter.peek() {
        //accumulate all of the index operations
        multidex.push(index);
        while let Some(Expression::Lbracket) = iter.next() {
            multidex.push(expr(iter, None)?);
            iter.next(); //skip the closing rbracket
        }
        Ok(build_chain_back(ident, &mut multidex.iter().rev().peekable()).unwrap())
    } else {
        //Don't neext to do all that fancy stuff if there's only one index instruction
        Ok(ExprNode::Index(Box::new(make_node(ident)), Box::new(index)))
    }
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
) -> Result<Vec<ExprNode>, String> {
    let mut params = vec![];
    loop {
        // println!("{:?} is next in params", peekable.peek());
        // println!("{:?} is the accumulated params", params);
        // std::io::stdin().read_line(&mut String::new());
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
            Some(Expression::Lbrace) => {
                return Err("Can't have block in function parameters".to_owned());
            }
            _ => params.push(read_line(None, peekable, &vec![&Expression::Comma, &Expression::Rparen])?),
        }
    }
    Ok(params)
}

fn make_for_loop(iter: &mut Peekable<Iter<'_, Expression>>) -> Result<ExprNode, String> {
    match iter.peek() {
        Some(Expression::Lparen) => {
            iter.next(); //skip the lparen after the "for" keyword
            let name = iter.next(); //grab the next expression to pass it in as cur
            let dec = expr(iter, name)?; //get the declaration expression (i = 0)
            if let ExprNode::Operation(op, _, _) = &dec {
                //double check to make sure this was an assignment op
                if **op == Expression::Equal {
                    let condition = read_line(None, iter, &vec![&Expression::Semicolon])?; //get the condition expression (i < 10)
                    let increment = read_line(None, iter, &vec![&Expression::Rparen])?; //get the incrementation expression (i = i + 1)
                    return Ok(ExprNode::ForLoopDec(
                        Box::new(dec),
                        Box::new(condition),
                        Box::new(increment),
                    ));
                }
            }
            //for loops don't need to have an assinment op, so that needs to be supported
            iter.next(); //skip the last semicolon
            let increment = read_line(None, iter, &vec![&Expression::Rparen])?; //get the incrementation expression (i = i + 1)
            iter.next();
            iter.next(); //skipping the closing paren and opening braceso that the body can be parsed properly
            Ok(ExprNode::ForLoopDec(
                Box::new(ExprNode::Illegal(None)),
                Box::new(dec),
                Box::new(increment),
            ))
        }
        Some(_) | None => Err(format!("Expected \"(\", found {:?}", iter.next())),
    }
}

fn make_if(iter: &mut Peekable<Iter<'_, Expression>>) -> Result<ExprNode, String> {
    if let Some(Expression::Lparen) = iter.peek() {
        // iter.next();
        let condition = expr(iter, None)?; //get the conditional statement for the if
        iter.next(); //skip the closing paren
        iter.next(); //skip the opening brace
        let block = make_block(iter)?; //get the body of the if

        let mut branch = ExprNode::Illegal(None);


        if let Some(Expression::Key(w)) = iter.peek() {
            match w.as_str() {
                "else" => {
                    iter.next(); //skip the else expression
                    iter.next(); //skip the opening brace
                    branch = make_block(iter)?; //push on the body of the else statement
                }
                "elif" => {
                    iter.next();
                    branch = make_if(iter)?;
                }
                _ => {}
            }
        }
        Ok(ExprNode::IfStatement(
            Box::new(condition),
            Box::new(block),
            Box::new(branch),
        ))
    } else {
        Err(format!("Expected \"(\" found {}", iter.next().unwrap()))
    }
}

fn make_array(iter: &mut Peekable<Iter<'_, Expression>>) -> Result<ExprNode, String> {
    let mut res = vec![];
    loop {
        // println!("{:?} is next in make_array", iter.peek());
        // println!("{:?} is the accumulated array", res);
        // std::io::stdin().read_line(&mut String::new());
        match iter.peek() {
            Some(Expression::Rbracket) | Some(Expression::Semicolon) => {
                iter.next();
                return Ok(ExprNode::Array(res));
            }
            Some(Expression::Lbracket) => {
                iter.next();
                res.push(make_array(iter)?)
            }
            Some(Expression::Comma) => {
                iter.next();
                continue;
            }
            None => return Ok(ExprNode::Array(res)),//return Err("Unexpected end of file".to_owned()),
            _ => res.push(read_line(None, iter, &vec![&Expression::Comma, &Expression::Rbracket])?),
        }
    }
}
