#![allow(clippy::if_same_then_else)]
extern crate regex;

#[cfg(test)]
mod tests;

use regex::Regex;
use std::iter::Peekable;
use std::process;
use std::str::Chars;

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

pub fn run(data: &str) -> Vec<Expression> {
    Lexer::new().tokenize(data)
}

#[derive(PartialEq, Debug)]
enum State {
    Nothing,
    EmString,
    EmName,
    EmNumber,
    Comment,
}

struct Lexer {
    current_state: State,
    token: String,
    valid_num: Regex,
    valid_chars: Regex,
    valid_symb: Regex,
    check: bool,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            current_state: State::Nothing,
            token: String::new(),
            valid_num: Regex::new(r"\d*").unwrap(),
            valid_chars: Regex::new(r"\D+[[:word:]]*").unwrap(),
            valid_symb: Regex::new(r"[\{\}\(\)=;\*\+\-/#!,\t\n]").unwrap(),
            check: false,
        }
    }

    pub fn run(data: &str) -> Vec<Expression> {
        Lexer::new().tokenize(data)
    }
    pub fn tokenize(&mut self, data: &str) -> Vec<Expression> {
        let mut result = vec![];

        let mut ch = data.chars().peekable();

        while let Some(c) = ch.next() {
            // println!(
            //     "Current char: {:?}\nNext char: {:?}\nCurrent token: {}",
            //     c,
            //     ch.peek().unwrap_or(&'?'),
            //     self.token
            // );
            match self.current_state {
                State::Comment => {
                    if c == '\n' {
                        self.current_state = State::Nothing;
                        self.token.clear();
                    }
                }
                State::EmString => {
                    if c == '"' {
                        result.push(Expression::Word(self.token.clone()));
                        self.token.clear();
                        self.current_state = State::Nothing;
                    } else {
                        self.token.push(c);
                    }
                }
                State::EmNumber => {
                    if let Some(r) = self.num_handle(c) {
                        result.push(r);
                    }
                }
                State::EmName => {
                    if let Some(r) = self.name_handle(c) {
                        result.push(r);
                    }
                }
                State::Nothing => {
                    if let Some(r) = self.nothing_handle(c, &mut ch) {
                        result.push(r);
                    }
                }
            }

            //check after everything for a nothing state to ensure the
            //current character is processed correctly
            if self.check {
                if let Some(r) = self.nothing_handle(c, &mut ch) {
                    result.push(r);
                }
                self.check = false;
            }
            // println!("Current result: {:?}", result);
        }
        // result.push(Expression::Ident("main".to_owned()));
        // result.push(Expression::Lparen);
        // result.push(Expression::Rparen);
        // result.push(Expression::Semicolon);
        // result.push(Expression::EOF);
        result //return the result
    }

    fn num_handle(&mut self, c: char) -> Option<Expression> {
        let result: Option<Expression>;
        if c.is_whitespace() || self.valid_symb.is_match(&c.to_string()) {
            self.current_state = State::Nothing;
            if !c.is_whitespace() && c.is_numeric() || c == '.' {
                //the current char could be part of the thing we're accumulating
                self.token.push(c);
            }
            result = Some(Expression::Number(
                self.token.parse::<f32>().unwrap_or_else(|e| {
                    println!(
                        "Got this error message ({}) when parsing this: {}",
                        e, self.token
                    );
                    process::exit(-1);
                }),
            ));
            self.token.clear();
            self.check = true;
        } else {
            self.token.push(c);
            result = None;
        }
        result
    }

    fn name_handle(&mut self, c: char) -> Option<Expression> {
        let result: Option<Expression>;
        if c.is_whitespace() || self.valid_symb.is_match(&c.to_string()) {
            self.current_state = State::Nothing;
            if (c.is_alphabetic()) && !c.is_whitespace() {
                //current char could be part of the thing we're accumulating
                self.token.push(c);
            }
            match self.token.as_str() {
                "fn" => {
                    result = Some(Expression::Key(self.token.to_string()));
                    self.token.clear();
                }
                "print" => {
                    result = Some(Expression::Key(self.token.to_string()));
                    self.token.clear();
                }
                "return" => {
                    result = Some(Expression::Key(self.token.to_string()));
                    self.token.clear();
                }
                "true" | "false" => {
                    result = Some(Expression::Key(self.token.to_string()));
                    self.token.clear();
                }
                "while" | "for" => {
                    result = Some(Expression::Key(self.token.to_string()));
                    self.token.clear();
                }
                "if" | "else" | "elif" => {
                    result = Some(Expression::Key(self.token.to_string()));
                    self.token.clear();
                }
                _ => {
                    result = Some(Expression::Ident(self.token.to_string()));

                    self.token.clear();
                }
            }
            self.check = true;
        } else {
            self.token.push(c);
            result = None;
        }
        result
    }

    fn nothing_handle(&mut self, c: char, ch: &mut Peekable<Chars<'_>>) -> Option<Expression> {
        // println!(
        //     "This is what it looks like when you call char.to_string(): {:?}",
        //     c.to_string()
        // );
        match c {
            ',' | '\t' | ' ' | '\n' | '\r' => None,
            '"' => {
                self.current_state = State::EmString;
                self.token.clear();
                None
            }
            '{' => Some(Expression::Lbrace),
            '}' => Some(Expression::Rbrace),
            '(' => Some(Expression::Lparen),
            ')' => Some(Expression::Rparen),
            '=' => {
                if let Some(sym) = ch.peek() {
                    if *sym == '=' {
                        ch.next();
                        Some(Expression::BoolOp("==".to_owned()))
                    } else {
                        Some(Expression::Equal)
                    }
                } else {
                    None
                }
            }

            '*' => Some(Expression::Operator(c)),
            '+' => Some(Expression::Operator(c)),
            '-' => Some(Expression::Operator(c)),
            '/' => {
                if let Some(sym) = ch.peek() {
                    if *sym == '/' {
                        ch.next();
                        self.current_state = State::Comment;
                        None
                    } else {
                        Some(Expression::Operator(c))
                    }
                } else {
                    None
                }
            }
            ';' => Some(Expression::Semicolon),
            '!' => {
                if let Some(sym) = ch.peek() {
                    if *sym == '=' {
                        ch.next();
                        Some(Expression::BoolOp("!=".to_owned()))
                    } else {
                        None
                        //if we need other operators to do with the bang they would go here
                    }
                } else {
                    None
                }
            }

            '<' => {
                if let Some(sym) = ch.peek() {
                    if *sym == '=' {
                        ch.next();
                        Some(Expression::BoolOp("<=".to_owned()))
                    } else {
                        Some(Expression::BoolOp("<".to_owned()))
                    }
                } else {
                    None
                }
            }

            '>' => {
                if let Some(sym) = ch.peek() {
                    if *sym == '=' {
                        ch.next();
                        Some(Expression::BoolOp(">=".to_owned()))
                    } else {
                        Some(Expression::BoolOp(">".to_owned()))
                    }
                } else {
                    None
                }
            }

            // '#' => {
            //     self.current_state = State::Comment;
            //     None
            // }
            _ => {
                self.token.push(c);
                if self.valid_chars.is_match(&self.token) {
                    self.current_state = State::EmName;
                } else if self.valid_num.is_match(&self.token) {
                    self.current_state = State::EmNumber;
                }
                // println!(
                //     "this is the token: {:?} and this is the char: {:?}",
                //     self.token, c
                // );

                None
            }
        }
    }
}
