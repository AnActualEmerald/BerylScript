use super::compiler::{ExprNode, Expression};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
enum Value {
    Null,
    Float(f64),
    EmString(String),
    // Char(u8),
    Name(String),
    Function(Expression, Vec<Value>, ExprNode),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Float(s) => write!(f, "{}", s),
            Value::EmString(s) => write!(f, "{}", s),
            // Value::Char(c) => write!(f, "{}", c),
            Value::Name(n) => write!(f, "{}", n),
            Value::Null => write!(f, "null"),
            Value::Function(n, p, _) => write!(f, "{:?}({:?})", n, p),
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
        //     "Walking tree: \n    Current node: {:?}\n     Current stack: {:?}",
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
            ExprNode::Call(ex, n) => self.do_call(&**ex, &*n, frame),
            ExprNode::Literal(l) => self.make_literal(&**l, frame),
            ExprNode::Name(n) => self.make_name(&**n, frame),
            ExprNode::Func(n, p, b) => self.def_func(n, p, b, frame),
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

    fn def_func(
        &self,
        name: &Expression,
        params: &Vec<ExprNode>,
        body: &ExprNode,
        frame: &mut StackFrame,
    ) -> Value {
        if let Expression::Ident(n) = name {
            let mut args = vec![];
            for e in params.iter() {
                args.push(self.walk_tree(e, frame));
            }
            let f = Value::Function(name.clone(), args, body.clone());
            frame.set_var(n.to_string(), f.clone());
            return f;
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
            Expression::Operator(o) => {
                let l_p = self.walk_tree(&left, frame);
                let r_p = self.walk_tree(&right, frame);

                let f = match l_p {
                    Value::Float(f) => f,
                    Value::Name(n) => {
                        if let Value::Float(f) = frame.get_var(&n) {
                            *f
                        } else {
                            0.0 as f64
                        }
                    }
                    _ => 0.0 as f64,
                };

                let r = match r_p {
                    Value::Float(f) => f,
                    Value::Name(n) => {
                        if let Value::Float(f) = frame.get_var(&n) {
                            *f
                        } else {
                            0.0 as f64
                        }
                    }
                    _ => 0.0 as f64,
                };

                if *o == '+' {
                    return Value::Float(f + r);
                } else if *o == '-' {
                    return Value::Float(f - r);
                } else if *o == '*' {
                    return Value::Float(f * r);
                } else if *o == '/' {
                    return Value::Float(f / r);
                }
            }
            _ => {}
        }

        Value::Null
    }

    fn keyword(&self, name: &Expression, value: &ExprNode, frame: &mut StackFrame) -> Value {
        if let Expression::Key(s) = name {
            if s == "print" {
                // println!("DEBUG: value={:?}", value);
                match value {
                    ExprNode::Call(n, args) => {
                        println!("{}", self.do_call(n, args, frame));
                    }
                    _ => {
                        let tmp = self.walk_tree(&value, frame);
                        // println!("DEBUG: tmp={:?}", tmp);
                        match tmp {
                            Value::EmString(r) => println!("{}", r),
                            // Value::Char(c) => println!("{}", c),
                            Value::Float(i) => println!("{}", i),
                            Value::Name(n) => println!("{}", frame.get_var(&n)),
                            Value::Null => println!("null"),
                            Value::Function(_, _, _) => {
                                println!("{}", self.walk_tree(&value, frame))
                            } // _ => {}
                        }
                    }
                }
            }
        }

        Value::Null
    }

    fn do_call(&self, name: &Expression, param: &Vec<ExprNode>, frame: &mut StackFrame) -> Value {
        if let Expression::Key(_) = name {
            return self.keyword(name, &param[0], frame);
        }

        if let Expression::Ident(n) = name {
            let f: Value;
            {
                f = frame.get_var_copy(n);
            }
            match &f {
                Value::Function(_, p, b) => {
                    if p.len() != param.len() {
                        panic!(
                            "Expected {} arguments for {}, got {}",
                            p.len(),
                            n,
                            param.len()
                        );
                    } else {
                        let mut i = 0;
                        for e in param.iter() {
                            if let Value::Name(arg) = &p[i] {
                                let val = self.walk_tree(&e, frame);
                                match val {
                                    Value::Name(n) => {
                                        let tmp = frame.get_var(&n);
                                        frame.set_var(arg.to_string(), tmp.clone());
                                    }
                                    _ => frame.set_var(arg.to_string(), val),
                                }
                            }
                            i = i + 1;
                        }
                        let ret = self.walk_tree(&b, frame);
                        p.iter().for_each(|e| match e {
                            Value::Name(n) => frame.free_var(n),
                            _ => {}
                        });

                        return ret;
                    }
                }
                _ => panic!("Expected function, found {}", f),
            }
        }

        Value::Null
    }
}

impl StackFrame {
    fn set_var(&mut self, name: String, v: Value) {
        self.stack.insert(name, v);
    }

    fn get_var(&self, name: &String) -> &Value {
        if self.stack.contains_key(name) {
            &self.stack[name]
        } else {
            &Value::Null
        }
    }

    fn get_var_copy(&self, name: &String) -> Value {
        if self.stack.contains_key(name) {
            self.stack[name].clone()
        } else {
            Value::Null
        }
    }

    fn free_var(&mut self, name: &String) {
        if self.stack.contains_key(name) {
            self.stack.remove(name);
        }
    }
}
