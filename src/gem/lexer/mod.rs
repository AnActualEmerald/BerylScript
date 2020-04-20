extern crate regex;

#[cfg(test)]
mod tests;

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
    Comment,
}

pub fn tokenize(data: &str) -> Vec<Expression> {
    let mut result = vec![];
    let mut tok = String::new();
    let mut current_state = State::Nothing;

    let ch = data.chars().clone();

    let valid_chars = Regex::new(r"\D+[[:word:]]*").unwrap();
    let valid_num = Regex::new(r"\d*").unwrap();
    let valid_symb = Regex::new(r"[\{\}\(\)=;\*\+\-/#]").unwrap();

    // This whole thing could use a mutable iterator to check over the data until it finds
    // somthing of interest i.e. the closing " or whatever, but idk if that's faster or better
    // so this is what I'll stick with for now
    for c in ch {
        if current_state == State::Comment {
            if c == '\n' {
                current_state = State::Nothing;
                tok = format!("");
                continue;
            } else {
                continue;
            }
        } else if !c.is_whitespace() && current_state != State::EmString {
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
                '#' => {
                    current_state = State::Comment;
                    continue;
                }
                _ => {}
            }
            tok = format!("");
            current_state = State::Nothing;
        } else if valid_chars.is_match(&tok) && current_state != State::EmString {
            match tok.as_str() {
                "fn" => {
                    result.push(Expression::Key(tok.clone()));
                    current_state = State::Nothing;
                    tok = format!("");
                }
                "print" => {
                    result.push(Expression::Key(tok.clone()));
                    current_state = State::Nothing;
                    tok = format!("");
                }
                "return" => {
                    result.push(Expression::Key(tok.clone()));
                    current_state = State::Nothing;
                    tok = format!("");
                }
                _ => current_state = State::EmName,
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
