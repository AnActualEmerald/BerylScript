use directories::ProjectDirs;
use gem::interpreter::*;
use gem::{lexer, parser};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs;
use std::path::PathBuf;

pub struct Repl {
    runtime: Runtime,
    glob_frame: StackFrame,
    multiline: Vec<bool>,
    cache: String,
}

impl Repl {
    pub fn new() -> Self {
        let cache = ProjectDirs::from("", "EmeraldScript", "Beryl")
            .expect("Unable to find cache directory");
        Repl {
            runtime: Runtime::new(),
            glob_frame: StackFrame::new(),
            multiline: vec![],
            cache: format!("{}/{}", cache.cache_dir().to_str().unwrap(), "history.txt"),
        }
    }

    pub fn run(&mut self, debug: bool) {
        let mut rl = Editor::<()>::new();
        if rl.load_history(&self.cache).is_err() {
            fs::create_dir_all(PathBuf::from(&self.cache)).expect("Unable to create history file");
        }

        println!(
            "Welcome to Beryl, the interactive EmeraldScript interpreter v{}!",
            env!("CARGO_PKG_VERSION")
        );
        println!("do \"gem --help\" to see other commands");
        println!("Type exit or stop to leave\n");

        loop {
            let input = rl.readline("---> ");
            match input {
                Ok(line) => {
                    //do repl stuf here
                    if line.ends_with("{\n") {
                        self.multiline.push(true);
                    } else if line.ends_with("}\n") {
                        self.multiline.pop();
                    }
                    rl.add_history_entry(line.as_str());
                    println!("Line: {}", line);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                _ => println!("{:?}", input),
            }

            rl.save_history(&self.cache).unwrap();
        }
    }
}
