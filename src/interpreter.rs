use super::compiler::{ExprNode, Expression};
use std::collections::HashMap;

enum Value {
    Null,
    Int(i32),
    EmString(String),
    Char(u8),
    Function,
}

struct StackFrame {
    stack: HashMap<String, Value>,
}

struct Runtime {
    tree: ExprNode,
    stack: Vec<StackFrame>,
}

pub fn run(tree: ExprNode) {
    let mut r = Runtime {
        tree: tree.clone(),
        stack: vec![],
    };
    r.find_global_vars();
    // r.walk_tree(&tree);
}

impl Runtime {
    fn find_global_vars(&mut self) {
        if let ExprNode::Block(v) = &self.tree {
            let mut globFrame = StackFrame {
                stack: HashMap::new(),
            };
            v.iter().for_each(|e| match e {
                ExprNode::Operation(oper, left, right) => {
                    if let Expression::Equal = **oper {
                        if let ExprNode::Literal(l) = &**left {
                            if let Expression::Ident(s) = &**l {
                                let value = self.walk_tree(right); // this is all very ugly but idk how to fix it
                                globFrame.stack.insert(String::from(s), value);
                            }
                        }
                    }
                }
                _ => {}
            });
            self.stack.push(globFrame);
        }
    }

    fn walk_tree(&self, node: &ExprNode) -> Value {
        Value::Null
    }
}
