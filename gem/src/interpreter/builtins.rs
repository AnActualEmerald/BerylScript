use std::collections::HashMap;
use crate::interpreter::Value;
use console::Term;

pub fn get_functions() -> HashMap<String, Box<dyn Fn(Vec<Value>) -> Value>> {
    let mut hash: HashMap<String, Box<dyn Fn(Vec<Value>) -> Value>> = HashMap::new();
    hash.insert("print".to_owned(), Box::new(em_print));
    hash.insert("println".to_owned(), Box::new(em_println));
    hash.insert("number".to_owned(), Box::new(em_number));
    hash.insert("readln".to_owned(), Box::new(em_readln));
    hash.insert("read".to_owned(), Box::new(em_read));

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

fn em_number(args: Vec<Value>) -> Value {
    let v = args[0].clone();
    match v {
        Value::EmString(s) => {
            if let Ok(p) = s.parse::<f32>() {
                Value::Float(p)
            }else {
                Value::Null
            }
        }
        Value::EmBool(b) => Value::Float(b as i32 as f32),
        Value::Float(_) => v,
        _ => Value::Null
    }
}

fn em_readln(args: Vec<Value>) -> Value {
    let buf = Term::stdout();
    if args.len() > 0 {
        buf.write_str(&format!("{}", args[0])).unwrap_or(());
    }
    let input = buf.read_line();
    match input {
        Ok(s) => Value::EmString(s),
        Err(_) => Value::Null
    }
}

fn em_read(args: Vec<Value>) -> Value {
    let buf = Term::stdout();
    if args.len() > 0 {
        buf.write_str(&format!("{}", args[0])).unwrap_or(());
    }
    let input = buf.read_char();
    match input {
        Ok(s) => Value::EmString(String::from(s)),
        Err(_) => Value::Null
    }
}

// fn em_readKey(args: Vec<Value>) -> Value {
//     let buf = Term::stdout();
//     if args.len() > 0 {
//         buf.write_str(&format!("{}", args[0])).unwrap_or(());
//     }
//     let input = buf.read_key();
//     match input {
//         Ok(s) => Value::EmString(String::from(s)),
//         Err(_) => Value::Null
//     }
// }