#[cfg(test)]
mod tests;

use super::lexer::Expression;
use super::parser::ExprNode;
use std::collections::HashMap;
use std::fmt;
use std::process;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Value {
    Null,
    Float(f32),
    EmString(String),
    EmBool(bool),
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
            Value::EmBool(b) => write!(f, "{}", b),
        }
    }
}

struct StackFrame {
    stack: HashMap<String, Value>,
}

struct Runtime {
    // tree: ExprNode,
    // stack: Vec<StackFrame>,
    returning: bool,
}

pub fn run(tree: ExprNode) {
    let mut r = Runtime {
        // tree: tree.clone(),
        // stack: vec![],
        returning: false,
    };
    // r.find_global_vars();
    let mut glob_frame = StackFrame {
        stack: HashMap::new(),
    };
    r.walk_tree(&tree, &mut glob_frame);
    // println!("{:?}", glob_frame.stack);
}

impl Runtime {
    fn walk_tree(&mut self, node: &ExprNode, frame: &mut StackFrame) -> Value {
        // println!(
        //     "Walking tree: \n    Current node: {:?}\n     Current stack: {:?}",
        //     node, frame.stack
        // );
        let res: Value;
        match node {
            ExprNode::Block(v) => {
                // let mut n_frame = StackFrame {
                //     stack: HashMap::new(),
                // };
                let mut ret = Value::Null;
                for e in v.iter() {
                    match e {
                        ExprNode::ReturnVal(v) => {
                            ret = self.walk_tree(v, frame);
                            break;
                        }
                        _ => {
                            self.walk_tree(e, frame);
                        }
                    }
                    if self.returning {
                        break;
                    }
                }
                return ret;
            }
            ExprNode::Operation(o, l, r) => res = self.do_operation(&**o, &**l, &**r, frame),
            ExprNode::Call(ex, n) => res = self.do_call(&**ex, &*n, frame),
            ExprNode::StrLiteral(s) => res = Value::EmString(*s.clone()),
            ExprNode::NumLiteral(n) => res = Value::Float(**n),
            ExprNode::BoolLiteral(b) => res = Value::EmBool(*b),
            ExprNode::Name(n) => res = frame.get_var_copy(n),
            ExprNode::Func(n, p, b) => res = self.def_func(n, p, b, frame),
            ExprNode::Statement(e) => res = self.walk_tree(&**e, frame),
            ExprNode::Loop(ty, con, block) => res = self.do_loop(&**ty, &**con, &**block, frame),
            _ => res = Value::Null,
        }
        self.returning = false;
        res
        // Value::Null
    }

    fn do_loop(
        &mut self,
        ty: &str,
        condition: &ExprNode,
        block: &ExprNode,
        frame: &mut StackFrame,
    ) -> Value {
        match ty {
            "while" => {
                let mut ret = Value::Null;
                // println!("Loop condition: {:?}\nBlock to run: {:?}", condition, block);
                // println!(
                //     "Condition is currently: {:?}",
                //     self.walk_tree(&condition, frame)
                // );
                while self.walk_tree(&condition, frame) == Value::EmBool(true) {
                    ret = self.walk_tree(&block, frame);
                    if self.returning {
                        break;
                    }
                }
                ret
            }
            "for" => {
                let mut ret = Value::Null;
                if let ExprNode::ForLoopDec(dec, con, inc) = condition {
                    if let ExprNode::Illegal(_) = **dec {
                        while self.walk_tree(&con, frame) == Value::EmBool(true) {
                            //walk the tree to execute the loop body
                            ret = self.walk_tree(&block, frame);
                            if self.returning {
                                break;
                            }
                            //perform the incrementation
                            self.walk_tree(&inc, frame);
                        }
                    } else {
                        self.walk_tree(&dec, frame);
                        while self.walk_tree(&con, frame) == Value::EmBool(true) {
                            //walk the tree to execute the loop body
                            ret = self.walk_tree(&block, frame);
                            if self.returning {
                                break;
                            }
                            //perform the incrementation
                            self.walk_tree(&inc, frame);
                        }
                    }
                }

                ret
            }
            _ => Value::Null,
        }
    }

    fn def_func(
        &mut self,
        name: &Expression,
        params: &[ExprNode],
        body: &ExprNode,
        frame: &mut StackFrame,
    ) -> Value {
        if let Expression::Ident(n) = name {
            let mut args = vec![];
            params.iter().for_each(|e| {
                if let ExprNode::Name(n) = e {
                    args.push(Value::Name(n.to_string()));
                }
            });
            let f = Value::Function(name.clone(), args, body.clone());
            frame.set_var(n.to_string(), f.clone());
            return f;
        } else {
            println!("Expected identifier, found {:?}", name);
            process::exit(-1); //If we don't get a name for the funciton, we should exit since things will break
        }
    }

    fn do_operation(
        &mut self,
        opr: &Expression,
        left: &ExprNode,
        right: &ExprNode,
        frame: &mut StackFrame,
    ) -> Value {
        match opr {
            Expression::Equal => {
                if let ExprNode::Name(n) = left {
                    let v = self.walk_tree(&right, frame);
                    frame.set_var(n.to_string(), v);
                    return Value::Null;
                } else {
                    println!("Expected name, found {:?}", left);
                    process::exit(-1);
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
                            0.0 as f32
                        }
                    }
                    _ => 0.0 as f32,
                };

                let r = match r_p {
                    Value::Float(f) => f,
                    Value::Name(n) => {
                        if let Value::Float(f) = frame.get_var(&n) {
                            *f
                        } else {
                            0.0 as f32
                        }
                    }
                    _ => 0.0 as f32,
                };

                if *o == '+' {
                    return Value::Float(f + r);
                } else if *o == '-' {
                    return Value::Float(f - r);
                } else if *o == '*' {
                    return Value::Float(f * r);
                } else if *o == '/' {
                    return Value::Float(f / r);
                } else {
                    println!("Invalid Operator: {}", o);
                    process::exit(-1);
                }
            }
            Expression::BoolOp(op) => {
                let l_p = self.walk_tree(&left, frame);
                let r_p = self.walk_tree(&right, frame);
                // println!("{} {} {}", l_p, op, r_p);
                match op.as_str() {
                    "==" => return Value::EmBool(l_p == r_p),
                    "!=" => return Value::EmBool(l_p != r_p),
                    ">=" => return Value::EmBool(l_p >= r_p),
                    "<=" => return Value::EmBool(l_p <= r_p),
                    "<" => return Value::EmBool(l_p < r_p),
                    ">" => return Value::EmBool(l_p > r_p),
                    _ => {
                        println!("Invalid Operator: {}", op);
                        process::exit(-1);
                    }
                }
            }
            _ => {}
        }

        Value::Null
    }

    fn keyword(&mut self, name: &Expression, value: &ExprNode, frame: &mut StackFrame) -> Value {
        if let Expression::Key(s) = name {
            match s.as_str() {
                "print" => {
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
                                Value::EmBool(b) => println!("{}", b),
                            }
                        }
                    }
                }
                "return" => {
                    self.returning = true;
                    match value {
                        ExprNode::Call(n, args) => {
                            return self.do_call(n, args, frame);
                        }
                        _ => {
                            return self.walk_tree(&value, frame);
                        }
                    }
                }
                // "true" => return Value::EmBool(true),
                // "false" => return Value::EmBool(false),
                _ => {}
            }
        }

        Value::Null
    }

    fn do_call(&mut self, name: &Expression, param: &[ExprNode], frame: &mut StackFrame) -> Value {
        match name {
            Expression::Key(_) => return self.keyword(name, &param[0], frame),
            Expression::Ident(n) => {
                let func: Value;
                {
                    func = frame.get_var_copy(n);
                }
                match &func {
                    Value::Function(_, params, body) => {
                        if params.len() != param.len() {
                            panic!(
                                "Expected {} arguments for {}, got {}",
                                params.len(),
                                n,
                                param.len()
                            );
                        } else {
                            for (i, e) in param.iter().enumerate() {
                                if let Value::Name(arg) = &params[i] {
                                    let val = self.walk_tree(&e, frame);
                                    match val {
                                        Value::Name(n) => {
                                            let tmp = frame.get_var(&n).clone();
                                            frame.set_var(arg.to_string(), tmp);
                                            //I'd really like to not have to copy here
                                        }
                                        _ => frame.set_var(arg.to_string(), val),
                                    }
                                }
                            }
                            let ret = self.walk_tree(&body, frame);
                            params.iter().for_each(|e| {
                                if let Value::Name(n) = e {
                                    frame.free_var(n)
                                }
                            });

                            return ret;
                        }
                    }
                    _ => {
                        println!("Expected function, found {}", func);
                        process::exit(-1);
                    }
                }
            }
            _ => {
                println!("Expected keyword or identifier, found {:?}", name);
                process::exit(-1);
            }
        }
    }
}

impl StackFrame {
    fn set_var(&mut self, name: String, v: Value) {
        self.stack.insert(name, v);
    }

    fn get_var(&self, name: &str) -> &Value {
        if self.stack.contains_key(name) {
            &self.stack[name]
        } else {
            &Value::Null
        }
    }

    fn get_var_copy(&self, name: &str) -> Value {
        if self.stack.contains_key(name) {
            self.stack[name].clone()
        } else {
            Value::Null
        }
    }

    fn free_var(&mut self, name: &str) {
        if self.stack.contains_key(name) {
            self.stack.remove(name);
        }
    }
}
