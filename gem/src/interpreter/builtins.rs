use std::collections::HashMap;
use crate::interpreter::Value;

pub fn get_functions() -> HashMap<String, Box<dyn Fn(Vec<Value>) -> Value>> {
    let mut hash: HashMap<String, Box<dyn Fn(Vec<Value>) -> Value>> = HashMap::new();
    hash.insert("print".to_owned(), Box::new(em_print));
    hash.insert("println".to_owned(), Box::new(em_println));
    hash
}

fn em_print(args: Vec<Value>) -> Value {
    print!("{}", args[0]);
    Value::Null
}

fn em_println(args: Vec<Value>) -> Value {
    println!("{}", args[0]);
    Value::Null
}