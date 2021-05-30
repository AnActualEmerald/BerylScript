mod builtins;
// #[cfg(test)]
// mod tests;
mod types;

use crate::interpreter::types::EmObject;
use crate::interpreter::types::Indexable;
use crate::parser::{Node, Operator};

use regex::Regex;
use std::fmt;
use std::{cell::RefCell, collections::HashMap};

///Represents everything that exists in the language currently
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Null,
    Float(f32),
    EmString(String),
    EmBool(bool),
    EmArray(Vec<Box<Value>>),
    //Char(u8),
    Name(String),
    Function(Node, Vec<Value>, Node),
    Object(EmObject),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Float(s) => write!(f, "{}", s),
            Value::EmString(s) => {
                let re = Regex::new(r"esc_QUOTE").unwrap(); // this is terrible and I hate it
                let unescaped = re.replace_all(s, "\"");
                write!(f, "{}", unescaped)
            }
            // Value::Char(c) => write!(f, "{}", c),
            Value::Name(n) => write!(f, "{}", n),
            Value::Null => write!(f, "null"),
            Value::Function(n, p, _) => write!(f, "{:?}({:?})", n, p),
            Value::EmBool(b) => write!(f, "{}", b),
            Value::EmArray(v) => {
                let mut tmp = String::new();
                for val in v.iter() {
                    if let Value::EmString(_) = **val {
                        tmp = format!("{}\"{}\", ", tmp, val);
                    } else {
                        tmp = format!("{}{}, ", tmp, val);
                    }
                }
                tmp.pop();
                tmp.pop();
                write!(f, "[{}]", tmp)
            }
            Value::Object(e) => {
                if let Some(Value::Function(_, _, t)) = e.get_prop("~display") {
                    let mut rt = Runtime::new();
                    let mut gf = StackFrame::new();
                    gf.set_var(String::from("self"), self.clone());
                    let res = repl_run(t.clone(), &mut rt, &mut gf).unwrap_or_default();
                    write!(f, "{}", res)
                } else {
                    write!(f, "{:?}", e.members)
                }
            }
        }
    }
}

impl types::Indexable<Value> for Value {
    fn index<'a>(&'a self, index: usize) -> Result<&'a Value, String> {
        match self {
            Value::EmArray(v) => {
                if let Some(val) = v.get(index) {
                    Ok(val)
                } else {
                    Err(format!(
                        "Index {} out of bounds (I hope I can include line numbers some day)",
                        index
                    ))
                }
            }
            _ => Err(format!("Type {} isn't indexable", self)),
        }
    }

    fn index_mut<'a>(&'a mut self, index: usize) -> Result<&'a mut Value, String> {
        match self {
            Value::EmArray(v) => {
                if let Some(val) = v.get_mut(index) {
                    Ok(val)
                } else {
                    Err(format!(
                        "Index {} out of bounds (I hope I can include line numbers some day)",
                        index
                    ))
                }
            }
            _ => Err(format!("Type {} isn't indexable", self)),
        }
    }
}

///Stores variables in a hashmap for a given function block. Only created on function call, with the exception of the global frame
pub struct StackFrame {
    stack: HashMap<String, Value>,
}

///Handles all of the interpretation, and keeps track of things like function definitions
pub struct Runtime {
    // tree: Node,
    // stack: Vec<StackFrame>,
    heap: HashMap<String, RefCell<Value>>,
    functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> Value>>,
    returning: bool,
}

///A run function that accepts a runtime and global frame, mostly for use with the REPL
pub fn repl_run(
    tree: Node,
    runtime: &mut Runtime,
    glob_frame: &mut StackFrame,
) -> Result<String, String> {
    match runtime.walk_tree(&tree, glob_frame) {
        Ok(val) => Ok(format!("{}", val)),
        Err(e) => Err(e),
    }
}

///Walks through the provided tree and executes all the nodes
pub fn run(tree: Node, args: Node) {
    let mut r = Runtime::new();
    // r.find_global_vars();
    let mut glob_frame = StackFrame::new();

    //define all functions and any global variables
    if let Err(e) = r.walk_tree(&tree, &mut glob_frame) {
        println!("Interpreter crashed because: {}", e);
    }

    if let Err(e) = r.do_call(
        &Node::Name("main".to_owned()),
        &vec![Box::new(args)],
        &mut glob_frame,
    ) {
        println!("Interpreter crashed because: {}", e);
    }
    // println!("{:?}", glob_frame.stack);
}

// Basically *is* the interpreter, walks through the AST and executes the nodes as needed
impl Runtime {
    //TODO: Reduce the number of copies ins this code

    ///Creates a new Runtime with an empty heap
    pub fn new() -> Runtime {
        Runtime {
            heap: HashMap::new(),
            returning: false,
            functions: builtins::get_functions(),
        }
    }

    ///Matches the provided node and dispatches functions to handle it
    fn walk_tree(&mut self, node: &Node, frame: &mut StackFrame) -> Result<Value, String> {
        // println!(
        //     "Walking tree: \n    Current node: {:?}\n     Current stack: {:?}",
        //     node, frame.stack
        // );
        let res: Value;
        match node {
            Node::Block(v) => {
                let mut ret = Value::Null;
                for e in v.iter() {
                    match &**e {
                        /*When we run into a ReturnVal, it needs special treatment so we know to stop executing the
                         *current block once we get whatever the value is
                         **/
                        Node::ReturnVal(v) => {
                            ret = self.walk_tree(&*v, frame)?;
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
            Node::Operation(l, o, r) => res = self.do_operation(&*o, &**l, &**r, frame)?,
            Node::Call(ex, n) => res = self.do_call(&**ex, &*n, frame)?,
            // Node::MethodCall(n, args) => res = self.do_method(n, args, frame)?,
            Node::StrLiteral(s) => res = Value::EmString(s.to_string()),
            Node::NumLiteral(n) => res = Value::Float(*n),
            Node::BoolLiteral(b) => res = Value::EmBool(*b),
            Node::Name(n) => res = frame.get_var_copy(n),
            Node::Func(n, p, b) => res = self.def_func(n, p, b)?, //don't need the stackframe here because functions are stored on the heap
            Node::Statement(e) => res = self.walk_tree(&**e, frame)?,
            Node::Loop(ty, con, block) => res = self.do_loop(&**ty, &**con, &**block, frame)?,
            Node::IfStatement(con, body, branch) => res = self.do_if(con, body, branch, frame)?,
            Node::Array(v) => res = self.create_array(v, frame)?,
            Node::Index(ident, index) => res = self.index_array(ident, index, frame)?,
            Node::New(name, args) => res = self.do_init(name, args, frame)?,
            Node::Class(name, body) => res = self.define_class(&**name, &**body, frame)?,
            _ => res = Value::Null,
        }
        //Reset the returning flag, since we're returning whatever value we got anyways
        self.returning = false;
        Ok(res)
    }

    ///Executes both varieties of loop and walks through the nodes in the loop blocks
    fn do_loop(
        &mut self,
        ty: &str,
        condition: &Node,
        block: &Node,
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
                if let Node::ForLoopDec(dec, con, inc) = condition {
                    if let Node::None = **dec {
                        self.walk_tree(&dec, frame)?;
                    }
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

                Ok(ret)
            }
            _ => Ok(Value::Null),
        }
    }

    ///Defines a function and saves it as a variable in the heap
    fn def_func(
        &mut self,
        name: &Node,
        params: &Vec<Box<Node>>,
        body: &Vec<Box<Node>>,
    ) -> Result<Value, String> {
        if let Node::Name(n) = name {
            let mut args = vec![];
            params.iter().for_each(|e| {
                if let Node::Name(n) = &**e {
                    args.push(Value::Name(n.to_string()));
                }
            });
            let f = Value::Function(name.clone(), args, Node::Block(body.clone()));
            self.heap.insert(n.to_owned(), RefCell::new(f.clone()));
            Ok(f)
        } else {
            Err(format!("Unexpected node: {}", name))
        }
    }

    ///Performs arithmatic and boolean operations and returns their results
    fn do_operation(
        &mut self,
        opr: &Operator,
        left: &Node,
        right: &Node,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        match opr {
            Operator::Equals => match left {
                Node::Name(n) => {
                    let v = self.walk_tree(&right, frame)?;
                    // println!("Assigning variable: {:?}", v);
                    frame.set_var(n.to_string(), v.clone());
                    Ok(v)
                }
                Node::Index(n, i) => {
                    let name = if let Node::Name(s) = *n.clone() {
                        s
                    } else {
                        return Err(format!("Error getting name {:?}", n));
                    };
                    let index = self.walk_tree(i, frame)?;
                    let val = self.walk_tree(right, frame)?;
                    frame.update_array_index(&name, index, val.clone());

                    Ok(val)
                }
                Node::Operation(l, o, r) => match *o {
                    // Operator::Lbracket => {
                    //     let val = self.walk_tree(right, frame)?;
                    //     frame.update_nested_array(l, r, Some(val.clone()), true);
                    //     Ok(val)
                    // }
                    Operator::Dot => {
                        let name = if let Node::Name(ref n) = &**l {
                            n
                        } else {
                            return Err(format!("Expected name, got {:?}", l));
                        };
                        let val = self.walk_tree(right, frame)?;

                        if let Some(Value::Object(e)) = frame.get_var_mut(&name.to_string()) {
                            let prop = if let Node::Name(ref n) = &**r {
                                n
                            } else {
                                return Err(format!("Unexpected symbol {:?}", r));
                            };

                            e.set_prop(prop.to_string(), Box::new(val.clone()));
                            Ok(val)
                        } else {
                            Err(format!("Unexpected {:?}", name))
                        }
                    }
                    _ => Err(format!("Unexpected symbol {:?}", o)),
                },
                _ => Err(format!("Error assigning to variable {:?}", left)),
            },

            Operator::Dot => {
                // let val = self.walk_tree(&left, frame)?;
                return if let Value::Object(obj) = self.walk_tree(&left, frame)? {
                    match right {
                        Node::Name(n) => {
                            if let Some(v) = obj.get_prop(n) {
                                Ok(v.clone())
                            } else {
                                Err(format!("{} has no property {}", obj, n))
                            }
                        }
                        Node::Call(n, p) => self.do_method(&obj, right, frame),
                        _ => Err(format!("Expected a member or method call, got {}", right)),
                    }
                    // if let Some(v) = obj.get_prop(&right) {
                    //     Ok(v.clone())
                    // } else {
                    //     Err(format!("{} has no property {}", obj, right.inner()))
                    // }
                } else {
                    Err(format!("{:?} is not an object", left))
                };
            }
            _ => {
                let l_p = self.walk_tree(&left, frame)?;
                let r_p = self.walk_tree(&right, frame)?;

                let f = match l_p {
                    Value::Float(f) => f,
                    Value::Name(ref n) => {
                        if let Value::Float(f) = frame.get_var(&n) {
                            *f
                        } else {
                            0.0 as f32
                        }
                    }
                    Value::EmString(s) => return Ok(Value::EmString(format!("{}{}", s, r_p))),
                    _ => 0.0 as f32,
                };

                let r = match r_p {
                    Value::Float(f) => f,
                    Value::Name(ref n) => {
                        if let Value::Float(f) = frame.get_var(&n) {
                            *f
                        } else {
                            0.0 as f32
                        }
                    }
                    _ => 0.0 as f32,
                };

                match opr {
                    Operator::Add => Ok(Value::Float(f + r)),
                    Operator::Sub => Ok(Value::Float(f - r)),
                    Operator::Mul => Ok(Value::Float(f * r)),
                    Operator::Div => Ok(Value::Float(f / r)),
                    Operator::EqualTo => Ok(Value::EmBool(l_p == r_p)),
                    Operator::NotEqualTo => Ok(Value::EmBool(l_p != r_p)),
                    Operator::Greater => Ok(Value::EmBool(l_p > r_p)),
                    Operator::Less => Ok(Value::EmBool(l_p < r_p)),
                    Operator::GreaterOrEq => Ok(Value::EmBool(l_p >= r_p)),
                    Operator::LessOrEq => Ok(Value::EmBool(l_p <= r_p)),
                    _ => Err(format!("Invalid Operator: {:?}", opr)),
                }
            }
        }
    }

    fn keyword(
        &mut self,
        name: &Node,
        value: &Node,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        // if let Expression::Key(s) = name {
        //     let tmp = match value {
        //         Node::Call(n, args) => self.do_call(n, args, frame)?,

        //         _ => self.walk_tree(&value, frame)?,
        //     };
        //     match s.as_str() {
        //         "return" => {
        //             self.returning = true;
        //             return Ok(tmp);
        //         }
        //         _ => {}
        //     }
        // }

        Ok(Value::Null)
    }

    ///Executes a keyword or function call
    fn do_call(
        &mut self,
        name: &Node,
        args: &Vec<Box<Node>>,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        match name {
            // Node::Key(_) => self.keyword(name, &args[0], frame),
            Node::Name(n) => {
                //check if there is a built-in function to use
                if self.functions.contains_key(n) {
                    let tmp = args
                        .iter()
                        .map(|e| self.walk_tree(e, frame).unwrap())
                        .collect();
                    let func = self.functions.get(n).unwrap();
                    return Ok(func(tmp));
                }

                if let Some(func) = self.heap.get(n) {
                    //I'd really like to not have to borrow here
                    match &*func.clone().borrow() {
                        Value::Function(_, params, body) => {
                            if params.len() != args.len() {
                                Err(format!(
                                    "Expected {} arguments for {}, got {}",
                                    params.len(),
                                    n,
                                    args.len()
                                ))
                            } else {
                                let mut func_frame = StackFrame::new();
                                for (i, e) in args.iter().enumerate() {
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
                                self.walk_tree(&body, &mut func_frame)
                                //this shouldn't be necessary since Rust will destroy the old
                                //stack frame anyways when it goes out of  scope
                                // params.iter().for_each(|e| {
                                //     if let Value::Name(n) = e {
                                //         frame.free_var(n)
                                //     }
                                // });
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

    fn do_method(
        &mut self,
        obj: &EmObject,
        method: &Node,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        if let Node::Call(member, args) = method {
            let func = obj.get_prop(&*member.inner());
            match func {
                Some(Value::Function(n, p, body)) => {
                    if args.len() != p.len() - 1 {
                        Err(format!(
                            "Method {} for {} takes {} arguments, found {}",
                            n,
                            obj.get_prop("~name").unwrap(),
                            p.len(),
                            args.len()
                        ))
                    } else {
                        let mut func_frame = StackFrame::new();
                        func_frame.set_var(String::from("self"), Value::Object(obj.clone()));
                        for (i, e) in args.iter().enumerate() {
                            if let Value::Name(arg) = &p[i + 1] {
                                let val = self.walk_tree(&**e, frame)?;
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
                        self.walk_tree(body, &mut func_frame)
                    }
                }
                _ => Err(format!("Expected function, got {:?}", func)),
            }
        } else {
            Err(format!("Expected a method call"))
        }
    }
    ///Performs an if statement and any of its relevant branches
    fn do_if(
        &mut self,
        condition: &Node,
        body: &Node,
        branches: &Node,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        if self.walk_tree(condition, frame)? == Value::EmBool(true) {
            self.walk_tree(body, frame)
        } else if let Node::IfStatement(con, body, branch) = branches {
            self.do_if(con, body, branch, frame)
        } else {
            self.walk_tree(branches, frame)
        }
    }

    fn do_init(
        &mut self,
        name: &Node,
        init_args: &Vec<Box<Node>>,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        if let Node::Name(n) = name {
            let class = match self.heap.get(n) {
                Some(val) => {
                    if let Value::Object(e) = val.borrow().clone() {
                        e
                    } else {
                        return Err(format!("Expected class, got {}", val.borrow()));
                    }
                }
                None => return Err(format!("Class {} is not defined", name)),
            };
            if let Some(Value::Function(_, params, body)) = class.get_prop("~init") {
                if init_args.len() != params.len() - 1 {
                    Err(format!(
                        "Contrsuctor for {} takes {} arguments, found {}",
                        class.get_prop("~name").unwrap(),
                        params.len() - 1,
                        init_args.len()
                    ))
                } else {
                    let mut func_frame = StackFrame::new();
                    func_frame.set_var(String::from("self"), Value::Object(class.clone()));
                    for (i, e) in init_args.iter().enumerate() {
                        if let Value::Name(arg) = &params[i + 1] {
                            let val = self.walk_tree(&**e, frame)?;
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
                    self.walk_tree(body, &mut func_frame)?;

                    //should figure out a way to get ownership from a stackframe
                    Ok(func_frame.get_var("self").clone())
                }
            } else {
                Ok(Value::Object(class))
            }
        } else {
            Err(format!("Expected object, found {:?}", name))
        }
    }

    ///Defines an array and saves it to the current stackframe
    fn create_array(
        &mut self,
        raw: &Vec<Box<Node>>,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        let mut tmp = vec![];
        for val in raw.iter() {
            tmp.push(Box::new(self.walk_tree(&**val, frame)?));
        }

        Ok(Value::EmArray(tmp))
    }

    ///Returns the value at a given array index
    fn index_array(
        &mut self,
        ident: &Node,
        index: &Node,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        let array = self.walk_tree(ident, frame)?;
        if let Value::Float(f) = self.walk_tree(index, frame)? {
            Ok(array.index(f as usize)?.clone())
        } else {
            Err(format!("Index was not a numeber"))
        }
    }

    fn define_class(
        &mut self,
        name: &Node,
        body: &Node,
        frame: &mut StackFrame,
    ) -> Result<Value, String> {
        let mut members = HashMap::new();
        let class = if let Node::Name(s) = name {
            s
        } else {
            return Err("Expected an identifier".to_string());
        };

        //the name property will be the name of the class for now, this might change in the future
        members.insert(
            "~name".to_string(),
            Box::new(Value::EmString(class.clone())),
        );

        if let Node::Block(v) = body {
            for node in v {
                let val = self.walk_tree(node, frame)?;
                match &val {
                    Value::Function(n, _, _) => {
                        let fn_name = if let Node::Name(s) = n {
                            s
                        } else {
                            return Err("Expected identifier".to_owned());
                        };
                        members.insert(fn_name.clone(), Box::new(val.clone()));
                    }
                    er => {
                        return Err(format!("Unexpected {:?} in class definition", er));
                    }
                }
            }
        }

        let tmp = Value::Object(EmObject { members: members });
        self.heap.insert(class.clone(), RefCell::new(tmp.clone()));

        Ok(tmp)
    }
}

///Keeps track of local variables for functions. Currently only created when a function is called
impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StackFrame {
    fn default() -> Self {
        Self::new()
    }
}

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

    fn get_var_mut(&mut self, name: &str) -> Option<&mut Value> {
        if self.stack.contains_key(name) {
            self.stack.get_mut(name)
        } else {
            None
        }
    }

    fn update_array_index(&mut self, name: &str, index: Value, val: Value) {
        let var = self
            .stack
            .get_mut(name)
            .expect(format!("Unable to find variable {}", name).as_str());

        if let Value::Float(f) = index {
            match var {
                Value::EmArray(v) => {
                    v[f as usize] = Box::new(val);
                }
                _ => panic!("Expected array, found {}", var),
            }
        }
    }

    fn update_nested_array(
        &mut self,
        ident: &Node,
        index: &Node,
        val: Option<Value>,
        first: bool,
    ) -> Option<&mut Box<Value>> {
        match ident {
            Node::Operation(l, o, r) => {
                if *o != Operator::Equals {
                    panic!("Found operation when assigning to array: {:?}", ident);
                } else {
                    let var = self.update_nested_array(l, r, None, false)?;
                    let i = match index {
                        Node::NumLiteral(f) => *f as usize,
                        _ => panic!("Expected number literal, found {:?}", index),
                    };
                    match &mut **var {
                        Value::EmArray(v) => {
                            if first {
                                v[i] = Box::new(val.unwrap());
                                None
                            } else {
                                v.get_mut(i)
                            }
                        }
                        n => panic!("Expected array, found {}", n),
                    }
                }
            }
            Node::Name(n) => {
                let i = match index {
                    Node::NumLiteral(f) => *f as usize,
                    _ => panic!("Expected number literal, found {:?}", index),
                };

                let var = self
                    .stack
                    .get_mut(&**n)
                    .expect(format!("Unable to find variable {}", n).as_str());

                match var {
                    Value::EmArray(v) => v.get_mut(i),
                    _ => panic!("Expected array, found {}", var),
                }
            }

            _ => panic!("Unexpected node: {:?}", ident),
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
