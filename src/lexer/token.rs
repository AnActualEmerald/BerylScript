
#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    text: String,
    line: usize,
}

impl Token {
    pub fn new(t: TokenType, text: String, line: usize) -> Self {
        Token {
            token_type: t,
            text,
            line
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenType {
    RightParen,
    LeftParen,
    RightBrace, //}
    LeftBrace, //{
    RightSquare, //]
    LeftSquare, //[
    GreaterThan, //>
    GreaterEqual, //>=
    LessThan, //<
    LessEqual, //<=
    Equal,//=
    EqualEqual, //==
    NotEqual, // !=
    Bang, // !
    DoubleQuote,
    SingleQuote,
    Semicolon,
    Comma,
    Dot,
    Fn,
    While,
    If,
    Else,
    Class,
    Null,
    True,
    False,
    Number(f32),
    String(String),
    Identifier(String),
    EOF,
}