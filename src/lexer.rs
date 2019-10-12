use std::collections::hash_map;

pub struct token{
    pub name: &str,
    pub value: &str
}

let keywords: HashMap<&str, bool> = [("fn", true), ("int", true), ("string", true), ("print", true)].iter().cloned().collect(); //set up a hashmap of keywords to check against later

enum State{
    em_string,
    em_var
}

pub fn tokenize(data:str)->Vec<token>{
    let mut result = vec!();
    let mut tok = String::new();



    for c in data.chars().iter(){
        tok.push(c);
        if tok == String::from("\"") {

        }
    }
}
