mod token;

pub use token::{Token, TokenType};

use std::str::Chars;
use std::iter::Peekable;

struct Lexer<'a> {
    start: usize, //start of the current token
    current: usize, //current location in the source
    line: usize,
    source: String,
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>
}

//heavily inspired by https://craftinginterpreters.com
impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self{
        Lexer{
            start: 0,
            current: 0,
            line: 1,
            source: src.to_owned(),
            chars: src.chars().peekable(),
            tokens: vec![]
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.next()
    }

    fn add_token(&mut self, t: TokenType) {
        if let Some(text) = self.source.get(self.start..self.current){
            
            self.tokens.push(Token::new(t, text.to_owned(), self.line))
        }else {
            panic!("Unexpected end of file at line {}", self.line);
        }  
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        if let Some(c) = self.advance(){
            match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                '[' => self.add_token(TokenType::LeftSquare),
                ']' => self.add_token(TokenType::RightSquare),
                ',' => self.add_token(TokenType::Comma),
                ';' => self.add_token(TokenType::Semicolon),
                '.' => self.add_token(TokenType::Dot),
                '\n' => self.line += 1,
                _ => {
                    println!("Handle other symbols line: {}", self.line)
                }
            }
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::EOF, String::default(), self.line));

        self.tokens.clone()
    }
}

pub fn run<'a>(src: &'a str) -> Vec<Token> {
    let mut lex = Lexer::new(src);
    lex.scan_tokens()
}
