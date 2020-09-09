#[cfg(test)]
mod tests;

use super::lexer::Expression;
use super::parser::ExprNode;
use std::fmt;
use std::{cell::RefCell, collections::HashMap};

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

pub struct StackFrame {
    stack: HashMap<String, Value>,
}

pub struct Runtime {
    // tree: ExprNode,
    // stack: Vec<StackFrame>,
    heap: HashMap<String, RefCell<Value>>,
    returning: bool,
}

pub fn repl_run(
    tree: ExprNode,
    runtime: &mut Runtime,
    glob_frame: &mut StackFrame,
) -> Result<String, String> {
    match runtime.walk_tree(&tree, glob_frame) {
        Ok(val) => Ok(format!("{}", val)),
        Err(e) => Err(e),
    }
}

pub fn run(tree: ExprNode) {
    let mut r = Runtime::new();
    // r.find_global_vars();
    let mut glob_frame = StackFrame::new();
    //define all functions and any global variables
    if let Err(e) = r.walk_tree(&tree, &mut glob_frame) {
        println!("Interpreter crashed because: {}", e);
    }

    if let Err(e) = r.do_call(&Expression::Ident("main".to_owned()), &[], &mut glob_frame) {
        println!("Interpreter crashed because: {}", e);
    }
    // println!("{:?}", glob_frame.stack);
}

// Basically *is* the interpreter, walks throught the AST and executes the nodes as needed
impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            heap: HashMap::new(),
            returning: false,
        }
    }

    fn walk_tree(&mut self, node: &ExprNode, frame: &mut StackFrame) -> Result<Value, String> {
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
                        /*When we run into a ReturnVal, it needs special treatment so we know to stop executing the
                         *current block once we get whatever the value is
                         **/
                        ExprNode::ReturnVal(v) => {
                            ret = self.walk_tree(v, frame)?;
                            break;
                        }
                        _ => {
                            self.walk_tree(e, frame)?;
                        }
                    }
                    if self.returning {
                        //if the returning flag has been set, then break out of the loop and stop executing this block
                        //This is for return statements that don't return anything
                        break;
                    }
                }
                return Ok(ret);
            }
            ExprNode::Operation(o, l, r) => res = self.do_operation(&**o, &**l, &**r, frame)?,
            ExprNode::Call(ex, n) => res = self.do_call(&**ex, &*n, frame)?,
            ExprNode::StrLiteral(s) => res = Value::EmString(*s.clone()),
            ExprNode::NumLiteral(n) => res = Value::Float(**n),
            ExprNode::BoolLiteral(b) => res = Value::EmBool(*b),
            ExprNode::Name(n) => res = frame.get_var_copy(n),
            ExprNode::Func(n, p, b) => res = self.def_func(n, p, b)?, //don't need the stackframe here because functions are stored on the heap
            ExprNode::Statement(e) => res = self.walk_tree(&**e, frame)?,
            ExprNode::Loop(ty, con, block) => res = self.do_loop(&**ty, &**con, &**block, frame)?,
            ExprNode::IfStatement(con, body, branch) => {
                res = self.do_if(con, body, branch, frame)?
            }
            _ => res = Value::Null,
        }
        //Reset the returning flag, since we're returning whatever value we got anyways
        self.returning = false;
        Ok(res)
    }

    fn do_loop(
        &mut self,
        ty: &str,
        condition: &ExprNode,
        block: &ExprNode,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        match ty {
            "while" => {
                let mut ret = Value::Null;
                // println!("Loop condition: {:?}\nBlock to run: {:?}", condition, block);
                // println!(
                //     "Condition is currently: {:?}",
                //     self.walk_tree(&condition, frame)
                // );
                while self.walk_tree(&condition, frame)? == Value::EmBool(true) {
                    ret = self.walk_tree(&block, frame)?;
                    if self.returning {
                        break;
                    }
                }
                Ok(ret)
            }
            "for" => {
                let mut ret = Value::Null;
                if let ExprNode::ForLoopDec(dec, con, inc) = condition {
                    if let ExprNode::Illegal(_) = **dec {
                        while self.walk_tree(&con, frame)? == Value::EmBool(true) {
                            //walk the tree to execute the loop body
                            ret = self.walk_tree(&block, frame)?;
                            if self.returning {
                                break;
                            }
                            //perform the incrementation
                            self.walk_tree(&inc, frame)?;
                        }
                    } else {
                        self.walk_tree(&dec, frame)?;
                        while self.walk_tree(&con, frame)? == Value::EmBool(true) {
                            //walk the tree to execute the loop body
                            ret = self.walk_tree(&block, frame)?;
                            if self.returning {
                                break;
                            }
                            //perform the incrementation
                            self.walk_tree(&inc, frame)?;
                        }
                    }
                }

                Ok(ret)
            }
            _ => Ok(Value::Null),
        }
    }
    /**Define a function and save it as a variable in the heap */
    fn def_func(
        &mut self,
        name: &Expression,
        params: &[ExprNode],
        body: &ExprNode,
    ) -> Result<Value, String> {
        if let Expression::Ident(n) = name {
            let mut args = vec![];
            params.iter().for_each(|e| {
                if let ExprNode::Name(n) = e {
                    args.push(Value::Name(n.to_string()));
                }
            });
            let f = Value::Function(name.clone(), args, body.clone());
            self.heap.insert(n.to_owned(), RefCell::new(f.clone()));
            Ok(f)
        } else {
            Err(format!("Expected identifier, found {:?}", name))
            //If we don't get a name for the funciton, we should exit since things will break
        }
    }

    fn do_operation(
        &mut self,
        opr: &Expression,
        left: &ExprNode,
        right: &ExprNode,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        match opr {
            Expression::Equal => {
                return if let ExprNode::Name(n) = left {
                    let v = self.walk_tree(&right, frame)?;
                    frame.set_var(n.to_string(), v);
                    Ok(Value::Null)
                } else {
                    Err(format!("Expected name, found {:?}", left))
                };
            }
            Expression::Operator(o) => {
                let l_p = self.walk_tree(&left, frame)?;
                let r_p = self.walk_tree(&right, frame)?;

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

                return if *o == '+' {
                    Ok(Value::Float(f + r))
                } else if *o == '-' {
                    Ok(Value::Float(f - r))
                } else if *o == '*' {
                    Ok(Value::Float(f * r))
                } else if *o == '/' {
                    Ok(Value::Float(f / r))
                } else {
                    Err(format!("Invalid Operator: {}", o))
                };
            }
            Expression::BoolOp(op) => {
                let l_p = self.walk_tree(&left, frame);
                let r_p = self.walk_tree(&right, frame);
                // println!("{} {} {}", l_p, op, r_p);
                return match op.as_str() {
                    "==" => Ok(Value::EmBool(l_p == r_p)),
                    "!=" => Ok(Value::EmBool(l_p != r_p)),
                    ">=" => Ok(Value::EmBool(l_p >= r_p)),
                    "<=" => Ok(Value::EmBool(l_p <= r_p)),
                    "<" => Ok(Value::EmBool(l_p < r_p)),
                    ">" => Ok(Value::EmBool(l_p > r_p)),
                    _ => Err(format!("Invalid Operator: {}", op)),
                };
            }
            _ => return Ok(Value::Null),
        }
    }

    fn keyword(
        &mut self,
        name: &Expression,
        value: &ExprNode,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        if let Expression::Key(s) = name {
            match s.as_str() {
                "print" => {
                    // println!("DEBUG: value={:?}", value);
                    match value {
                        ExprNode::Call(n, args) => {
                            println!("{}", self.do_call(n, args, frame)?);
                        }
                        _ => {
                            let tmp = self.walk_tree(&value, frame)?;
                            // println!("DEBUG: tmp={:?}", tmp);
                            match tmp {
                                Value::EmString(r) => println!("{}", r),
                                // Value::Char(c) => println!("{}", c),
                                Value::Float(i) => println!("{}", i),
                                Value::Name(n) => println!("{}", frame.get_var(&n)),
                                Value::Null => println!("null"),
                                Value::Function(_, _, _) => {
                                    println!("{}", self.walk_tree(&value, frame)?)
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

        Ok(Value::Null)
    }

    /**Execute a keyword or function call*/
    fn do_call(
        &mut self,
        name: &Expression,
        param: &[ExprNode],
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        match name {
            Expression::Key(_) => return self.keyword(name, &param[0], frame),
            Expression::Ident(n) => {
                if let Some(func) = self.heap.get(n) {
                    //I'd really like to not have to borrow here
                    match &*func.clone().borrow() {
                        Value::Function(_, params, body) => {
                            if params.len() != param.len() {
                                Err(format!(
                                    "Expected {} arguments for {}, got {}",
                                    params.len(),
                                    n,
                                    param.len()
                                ))
                            } else {
                                let mut func_frame = StackFrame {
                                    stack: HashMap::new(),
                                };
                                for (i, e) in param.iter().enumerate() {
                                    if let Value::Name(arg) = &params[i] {
                                        let val = self.walk_tree(&e, frame)?;
                                        match val {
                                            Value::Name(n) => {
                                                let tmp = frame.get_var(&n).clone();
                                                func_frame.set_var(arg.to_string(), tmp);
                                                //I'd really like to not have to copy here
                                            }
                                            _ => func_frame.set_var(arg.to_string(), val),
                                        }
                                    }
                                }
                                let ret = self.walk_tree(&body, &mut func_frame);
                                //this shouldn't be necessary since Rust will destroy the old
                                //stack frame anyways when it goes out of  scope
                                // params.iter().for_each(|e| {
                                //     if let Value::Name(n) = e {
                                //         frame.free_var(n)
                                //     }
                                // });

                                return ret;
                            }
                        }
                        _ => Err(format!("Expected function, found {}", func.borrow())),
                    }
                } else {
                    Err(format!("Couldn't find identifier {}", n))
                }
            }
            _ => Err(format!("Expected keyword or identifier, found {:?}", name)),
        }
    }

    fn do_if(
        &mut self,
        condition: &ExprNode,
        body: &ExprNode,
        branches: &ExprNode,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        if self.walk_tree(condition, frame)? == Value::EmBool(true) {
            self.walk_tree(body, frame)
        } else {
            if let ExprNode::IfStatement(con, body, branch) = branches {
                self.do_if(con, body, branch, frame)
            } else {
                self.walk_tree(branches, frame)
            }
        }
    }
}

/**Keeps track of local variables for functions. Currently only created when a function is called. */
impl StackFrame {
    pub fn new() -> StackFrame {
        StackFrame {
            stack: HashMap::new(),
        }
    }

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

    //leaving this here for now in case I need it in the future
    // fn free_var(&mut self, name: &str) {
    //     if self.stack.contains_key(name) {
    //         self.stack.remove(name);
    //     }
    // }
}
