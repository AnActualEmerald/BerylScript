extern crate regex;

use regex::Regex;
use std::str;

#[derive(Debug)]
pub struct Token{
    pub name: &'static str,
    pub value: String
}

#[derive(PartialEq, Debug)]
enum State{
    Nothing,
    EmString,
    EmName,
    EmNumber
}

pub fn tokenize(data:&str)->Vec<Token>{

    let mut result = vec!();
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
                result.push(Token{name:"string", value:tok.clone()});
                tok = format!("");
                current_state = State::Nothing;
            }else {
                current_state = State::EmString;
                tok = format!("");
            }
        } else if valid_symb.is_match(&tok) || c.is_whitespace() && current_state != State::EmString {
            if !c.is_whitespace() {tok.pop();}
            match current_state {
                State::EmName => result.push(Token{name:"name", value:tok.clone()}),
                State::EmNumber => result.push(Token{name:"number", value:tok.clone()}),
                _ => {}
            }
            match c {
                '{' => result.push(Token{name:"lbracket", value:c.to_string()}),
                '}' => result.push(Token{name:"rbracket", value:c.to_string()}),
                '(' => result.push(Token{name:"lparen", value:c.to_string()}),
                ')' => result.push(Token{name:"rparen", value:c.to_string()}),
                '=' => result.push(Token{name:"equals", value:c.to_string()}),
                ';' => result.push(Token{name:"semicolon", value:c.to_string()}),
                _ => {}
            }
            tok = format!("");
            current_state = State::Nothing;
        } else if valid_chars.is_match(&tok) && current_state != State::EmString{
            if tok == format!("fn") { //check for all keywords
                result.push(Token{name:"keyword", value:tok.clone()});
                current_state = State::Nothing;
                tok = format!("");
            } else if tok == format!("print") {
                result.push(Token{name:"keyword", value:tok.clone()});
                current_state = State::Nothing;
                tok = format!("");
            } else{
                current_state = State::EmName;
            }
        } else if valid_num.is_match(&tok) && current_state != State::EmString{
            current_state = State::EmNumber
        }

    }

    result //return the result
}
