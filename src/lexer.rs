#[derive(Debug)]
pub struct Token{
    pub name: &'static str,
    pub value: String
}

#[derive(PartialEq, Debug)]
enum State{
    Nothing,
    EmString,
    EmName
}

pub fn tokenize(data:&str)->Vec<Token>{

    let mut result = vec!();
    let mut tok = String::new();
    let mut current_state = State::Nothing;

    let ch = data.chars();

    for c in ch {
        //println!("{:?}", current_state);
        tok.push(c);
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
        } else if tok == format!("fn") { //check for all keywords before adding names
            result.push(Token{name:"symbol", value:tok.clone()});
            current_state = State::Nothing;
            tok = format!("");
        } else if tok == format!("print") {
            result.push(Token{name:"symbol", value:tok.clone()});
            current_state = State::Nothing;
            tok = format!("");
        } else if tok == format!("{{") {
            result.push(Token{name:"openblock", value:tok.clone()});
            current_state = State::Nothing;
            tok = format!("");
        } else if tok == format!("}}") {
            result.push(Token{name:"closeblock", value:tok.clone()});
            current_state = State::Nothing;
            tok = format!("");
        } else if c.is_whitespace() && current_state == State::EmName {
            tok.pop();
            result.push(Token{name:"name", value:tok.clone()});
            current_state = State::Nothing;
            tok = format!("");
        } else if c.is_whitespace() && current_state == State::Nothing {
            current_state = State::EmName;
            tok = format!("");
        }
    }

    result //return the result
}
