#![allow(clippy::if_same_then_else)]
extern crate regex;

#[cfg(test)]
mod tests;

use regex::Regex;
use std::process;
use std::str;

// Enums are more idomatic and make the resulting Vec much easier to understand
// I may need more types to make things easier to work with but for now I think
// this should suffice
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Ident(String),
    Number(f32),
    Word(String),
    Key(String),
    Operator(char),
    BoolOp(String),
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
    Comment,
}

pub fn tokenize(data: &str) -> Vec<Expression> {
    let mut result = vec![];
    let mut tok = String::new();
    let mut current_state = State::Nothing;

    let valid_chars = Regex::new(r"\D+[[:word:]]*").unwrap();
    let valid_num = Regex::new(r"\d*").unwrap();
    let valid_symb = Regex::new(r"[\{\}\(\)=;\*\+\-/#!]").unwrap();

    let mut ch = data.chars().peekable();

    while let Some(c) = ch.next() {
        // println!(
        //     "Current char: {}\nNext char: {}\n Current token: {}",
        //     c,
        //     ch.peek().unwrap_or(&'?'),
        //     tok
        // );
        match current_state {
            State::Comment => {
                if c == '\n' {
                    current_state = State::Nothing;
                    tok.clear();
                }
            }
            State::EmString => {
                if c == '"' {
                    result.push(Expression::Word(tok.clone()));
                    tok.clear();
                    current_state = State::Nothing;
                } else {
                    tok.push(c);
                }
            }
            State::EmNumber => {
                if let Some(s) = ch.peek() {
                    if c.is_whitespace()
                        || valid_symb.is_match(&s.to_string())
                        || valid_symb.is_match(&c.to_string())
                    {
                        current_state = State::Nothing;
                        if !c.is_whitespace() && !valid_symb.is_match(&c.to_string()) {
                            //need this because of the peek, the current char
                            //could be part of the thing we're accumulating
                            tok.push(c);
                        }
                        result.push(Expression::Number(tok.parse::<f32>().unwrap_or_else(|e| {
                            println!("Got this error message ({}) when parsing this: {}", e, tok);
                            process::exit(-1);
                        })));
                        tok.clear();
                    } else {
                        tok.push(c);
                    }
                }
            }
            State::EmName => {
                if let Some(s) = ch.peek() {
                    if c.is_whitespace()
                        //next char
                        || valid_symb.is_match(&s.to_string())
                    //current char
                    || valid_symb.is_match(&c.to_string())
                    {
                        current_state = State::Nothing;
                        if !valid_symb.is_match(&c.to_string()) && !c.is_whitespace() {
                            //need this because of the peek, the current char
                            //could be part of the thing we're accumulating
                            tok.push(c);
                        }
                        match tok.as_str() {
                            "fn" => {
                                result.push(Expression::Key(tok.to_string()));
                                tok.clear();
                            }
                            "print" => {
                                result.push(Expression::Key(tok.to_string()));
                                tok.clear();
                            }
                            "return" => {
                                result.push(Expression::Key(tok.to_string()));
                                tok.clear();
                            }
                            "true" => {
                                result.push(Expression::Key(tok.to_string()));
                                tok.clear();
                            }
                            "false" => {
                                result.push(Expression::Key(tok.to_string()));
                                tok.clear();
                            }
                            "while" => {
                                result.push(Expression::Key(tok.to_string()));
                                tok.clear();
                            }
                            "for" => {
                                result.push(Expression::Key(tok.to_string()));
                                tok.clear();
                            }
                            _ => {
                                result.push(Expression::Ident(tok.to_string()));

                                tok.clear();
                            }
                        }
                    } else {
                        tok.push(c);
                    }
                }
            }
            State::Nothing => match c {
                '"' => {
                    current_state = State::EmString;
                    tok.clear();
                }
                '{' => result.push(Expression::Lbrace),
                '}' => result.push(Expression::Rbrace),
                '(' => result.push(Expression::Lparen),
                ')' => result.push(Expression::Rparen),
                '=' => {
                    if let Some(sym) = ch.peek() {
                        if *sym == '=' {
                            ch.next();
                            result.push(Expression::BoolOp("==".to_owned()));
                        } else {
                            result.push(Expression::Equal);
                        }
                    }
                }
                '*' => result.push(Expression::Operator(c)),
                '+' => result.push(Expression::Operator(c)),
                '-' => result.push(Expression::Operator(c)),
                '/' => result.push(Expression::Operator(c)),
                ';' => result.push(Expression::Semicolon),
                '!' => {
                    if let Some(sym) = ch.peek() {
                        if *sym == '=' {
                            ch.next();
                            result.push(Expression::BoolOp("!=".to_owned()));
                        } else {
                            //if we need other operators to do with the bang they would go here
                        }
                    }
                }
                '<' => {
                    if let Some(sym) = ch.peek() {
                        if *sym == '=' {
                            ch.next();
                            result.push(Expression::BoolOp("<=".to_owned()));
                        } else {
                            result.push(Expression::BoolOp("<".to_owned()));
                        }
                    }
                }
                '>' => {
                    if let Some(sym) = ch.peek() {
                        if *sym == '=' {
                            ch.next();
                            result.push(Expression::BoolOp(">=".to_owned()));
                        } else {
                            result.push(Expression::BoolOp(">".to_owned()));
                        }
                    }
                }
                '#' => {
                    current_state = State::Comment;
                }
                ' ' | '\n' => {}
                _ => {
                    tok.push(c);
                    if valid_chars.is_match(&tok) {
                        current_state = State::EmName;
                    } else if valid_num.is_match(&tok) {
                        current_state = State::EmNumber;
                    }
                }
            },
        }
    }
    result.push(Expression::Ident("main".to_owned()));
    result.push(Expression::Lparen);
    result.push(Expression::Rparen);
    result.push(Expression::Semicolon);
    result.push(Expression::EOF);
    result //return the result
}
