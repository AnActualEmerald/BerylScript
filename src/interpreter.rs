use super::compiler::{ExprNode, Expression};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum Value {
    Null,
    Float(f64),
    EmString(String),
    // Char(u8),
    Name(String),
    // Function(),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Float(s) => write!(f, "{}", s),
            Value::EmString(s) => write!(f, "{}", s),
            // Value::Char(c) => write!(f, "{}", c),
            Value::Name(n) => write!(f, "{}", n),
            Value::Null => write!(f, "null"),
        }
    }
}

struct StackFrame {
    stack: HashMap<String, Value>,
}

struct Runtime {
    // tree: ExprNode,
// stack: Vec<StackFrame>,
}

pub fn run(tree: ExprNode) {
    let r = Runtime {
        // tree: tree.clone(),
        // stack: vec![],
    };
    // r.find_global_vars();
    let mut glob_frame = StackFrame {
        stack: HashMap::new(),
    };
    r.walk_tree(&tree, &mut glob_frame);
}

impl Runtime {
    fn walk_tree(&self, node: &ExprNode, frame: &mut StackFrame) -> Value {
        // println!(
        //     "Walking tree: \n    Current node: {:?}     Current stack: {:?}",
        //     node, frame.stack
        // );
        match node {
            ExprNode::Block(v) => {
                // let mut n_frame = StackFrame {
                //     stack: HashMap::new(),
                // };
                for e in v.iter() {
                    self.walk_tree(&e, frame);
                }
                return Value::Null;
            }
            ExprNode::Operation(o, l, r) => self.do_operation(&**o, &**l, &**r, frame),
            ExprNode::Call(ex, n) => self.do_call(&**ex, &**n, frame),
            ExprNode::Literal(l) => self.make_literal(&**l, frame),
            ExprNode::Name(n) => self.make_name(&**n, frame),
            _ => Value::Null,
        }
        // Value::Null
    }

    fn make_name(&self, name: &Expression, _frame: &mut StackFrame) -> Value {
        if let Expression::Ident(i) = name {
            return Value::Name(i.clone().to_owned());
        }
        Value::Null
    }

    fn make_literal(&self, lit: &Expression, _frame: &mut StackFrame) -> Value {
        match lit {
            Expression::Word(w) => return Value::EmString(String::from(w)),
            Expression::Number(n) => return Value::Float(*n),
            _ => return Value::Null,
        }
        // Value::Null
    }

    fn do_operation(
        &self,
        opr: &Expression,
        left: &ExprNode,
        right: &ExprNode,
        frame: &mut StackFrame,
    ) -> Value {
        match opr {
            Expression::Equal => {
                if let Value::Name(n) = self.walk_tree(&left, frame) {
                    let v = self.walk_tree(&right, frame);
                    frame.set_var(n, v);
                    return Value::Null;
                }
            }
            _ => {}
        }

        Value::Null
    }

    fn keyword(&self, name: &Expression, value: &ExprNode, frame: &mut StackFrame) -> Value {
        if let Expression::Key(s) = name {
            if s == "print" {
                match self.walk_tree(&value, frame) {
                    Value::EmString(r) => println!("{}", r),
                    // Value::Char(c) => println!("{}", c),
                    Value::Float(i) => println!("{}", i),
                    Value::Name(n) => println!("{}", frame.get_var(n)),
                    Value::Null => println!("null"),
                }
            }
        }

        Value::Null
    }

    fn do_call(&self, name: &Expression, param: &ExprNode, frame: &mut StackFrame) -> Value {
        if let Expression::Key(_) = name {
            return self.keyword(name, param, frame);
        }

        if let Expression::Ident(f) = name {
            let p = self.walk_tree(&param, frame);
            frame.set_var(format!("{}_var", f), p);
            // return self.walk_tree(&frame.stack[f], frame);
            return Value::Null;
        }

        Value::Null
    }
}

impl StackFrame {
    fn set_var(&mut self, name: String, v: Value) {
        self.stack.insert(name, v);
    }

    fn get_var(&self, name: String) -> &Value {
        if self.stack.contains_key(&name) {
            &self.stack[&name]
        } else {
            &Value::Null
        }
    }
}
