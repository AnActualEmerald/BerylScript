use super::compiler::*;

//implement a trait on ExprNode to make compiling easier
trait Comp {
    fn compile(&self, stack: &mut Vec<Box<instr>>);
}

trait Runner {
    fn run<T>(&self, stack: Vec<T>);
}

impl Comp for super::compiler::ExprNode {
    fn compile(&self, stack: &mut Vec<Box<instr>>) {
        match self {
            ExprNode::Block(v) => {
                for t in v {
                    t.compile(stack);
                }
            }
            ExprNode::Call(e, n) => match &**e {
                Expression::Key(k) => match k.trim() {
                    "print" => {
                        // if let ExprNode::Literal(v) = **n {
                        //     match &*v {
                        //         Expression::Number(num) => stack.push(Box::new(instr {
                        //             func: &|| print!("{}", num.clone()),
                        //         })),
                        //         _ => {}
                        //     }
                        // }
                    }
                    _ => {}
                },
                _ => {}
            },
            ExprNode::Operation(o, l, r) => {}
            _ => panic!("Uhhhhh this shouldn't be possible, what did you do"),
        }
    }
}

pub struct instr {
    func: &'static (dyn Fn() + 'static),
}

pub fn compile(root: ExprNode) -> Vec<Box<instr>> {
    let mut stack = vec![];
    root.compile(&mut stack);
    stack
}

pub fn run() {}
