use std::fmt::Display;

lalrpop_mod!(grammar);

//making the nodes hold the actual values instead of the Expressions might be worth it to make
//interpreting easier
///Enum with variants for each type of statement or literal in the lang
#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum Node {
    Operation(Box<Node>, Operator, Box<Node>), //Left side, Operator, Right side
    StrLiteral(String),
    NumLiteral(f32),
    BoolLiteral(bool),
    Name(String),
    Call(Box<Node>, Vec<Box<Node>>), //name, args
    MethodCall(Box<Node>, Vec<Node>),
    Block(Vec<Box<Node>>),
    Func(Box<Node>, Vec<Box<Node>>, Vec<Box<Node>>), //Name, params, function body
    Class(Box<Node>, Box<Node>),                     //name, body
    New(Box<Node>, Vec<Node>),                       //name params
    Loop(Box<String>, Box<Node>, Box<Node>),         //loop keyword, condition, block
    ForLoopDec(Box<Node>, Box<Node>, Box<Node>),     //declaration, condition, incrementation
    Statement(Box<Node>),
    ReturnVal(Box<Node>),
    IfStatement(Box<Node>, Box<Node>, Box<Node>), //condition, body, branch
    ElseStatement(Box<Node>),                     //body
    Array(Vec<Node>),
    Index(Box<Node>, Box<Node>), //array identifier, inedex
    Operator(char),
    None,
    EOF,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StrLiteral(s) => write!(f, "{}", s),
            Self::NumLiteral(n) => write!(f, "{}", n),
            Self::BoolLiteral(b) => write!(f, "{}", b),
            Self::Name(n) => write!(f, "{}", n),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Equals,
    Dot,
    EqualTo,
    NotEqualTo,
    Greater,
    Less,
    GreaterOrEq,
    LessOrEq,
}

impl Node {
    ///Returns the inner value of a node as a string if possible
    pub fn inner(&self) -> String {
        match self {
            Node::StrLiteral(l) => l.to_string(),
            Node::NumLiteral(l) => l.to_string(),
            Node::BoolLiteral(l) => l.to_string(),
            Node::Name(l) => l.to_string(),
            _ => panic!("Can't unwrap {:?}", self),
        }
    }
}

pub fn parse<'input>(text: &'input str) -> Option<Box<Node>> {
    match grammar::NodeParser::new().parse(text) {
        Ok(n) => Some(n),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}
