use console::Term;
use directories::ProjectDirs;
use gem::interpreter::*;
use gem::{lexer, parser};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::PathBuf;

pub struct Repl {
    runtime: Runtime,
    glob_frame: StackFrame,
    multiline: Vec<bool>,
    cache: PathBuf,
}

impl Repl {
    pub fn new() -> Self {
        let mut cache = PathBuf::from(
            ProjectDirs::from("", "EmeraldScript", "Beryl")
                .expect("Unable to find cache directory")
                .cache_dir(),
        );
        cache.set_file_name("history.txt");
        Repl {
            runtime: Runtime::new(),
            glob_frame: StackFrame::new(),
            multiline: vec![],
            cache: cache,
        }
    }

    pub fn run(&mut self, debug: bool) {
        let mut rl = Editor::<()>::new();
        if rl.load_history(&self.cache).is_err() {
            println!("Couldn't load history");
        }

        println!(
            "Welcome to Beryl, the interactive EmeraldScript interpreter v{}!",
            env!("CARGO_PKG_VERSION")
        );
        println!("do \"gem --help\" to see other commands");
        println!("Type exit or stop to leave, or help for more info\n");
        let mut prompt = "<== ".to_owned();
        let mut data = String::new();
        loop {
            let input = rl.readline(&prompt);
            match input {
                Ok(raw) => {
                    match raw.as_str() {
                        "help" => {
                            println!("Possible commands are:\t\n'help'         => display this message\t\n'exit', 'stop' => leaves Beryl\t\n'clear'        => clears the screen\n");
                        }
                        "exit" | "stop" => break,
                        "clear" => {
                            let buf = Term::stdout();
                            buf.clear_screen().expect("Unable to clear console");

                            println!(
                                "Welcome to Beryl, the interactive EmeraldScript interpreter v{}!",
                                env!("CARGO_PKG_VERSION")
                            );
                            println!("do \"gem --help\" to see other commands");
                            println!("Type exit or stop to leave\n");
                        }

                        line => {
                            if line.starts_with("example") {
                                let parts = line.split_whitespace();
                                for p in parts {
                                    match p.parse::<usize>() {
                                        Ok(v) => {
                                            let ex = self.get_example(v - 1);
                                            // println!("ex: {}", ex);
                                            // data = String::new();
                                            // data.push_str(ex);
                                            data = rl
                                                .readline_with_initial("ex: ", (&ex, ""))
                                                .unwrap();
                                            //very useful library
                                            Term::stdout()
                                                .move_cursor_down(
                                                    data.chars().filter(|v| *v == '\n').count(),
                                                )
                                                .expect("unable to move cursor");
                                            break;
                                        }
                                        Err(_) => {
                                            continue;
                                        }
                                    }
                                }
                            }

                            data.push_str(&raw);
                            //do repl stuf here
                            if line.ends_with("{") {
                                self.multiline.push(true);
                            } else if line.ends_with("}") {
                                self.multiline.pop();
                            }

                            if self.multiline.contains(&true) {
                                let mut tmp = "... ".to_owned();
                                for _ in self.multiline.iter() {
                                    tmp.push('\t');
                                }
                                prompt = tmp;
                                rl.add_history_entry(line);
                                continue;
                            } else {
                                prompt = "<== ".to_owned();
                            }

                            self.execute(&data, debug);
                            data = String::new();

                            rl.add_history_entry(line);
                        }
                    }
                }
                Err(ReadlineError::Eof) => break,
                Err(ReadlineError::Interrupted) => break,
                Err(err) => {
                    println!("{}", err.to_string());
                    break;
                }
            }

            rl.save_history(&self.cache).unwrap();
        }
    }

    fn get_example(&self, ex: usize) -> String {
        let examples = vec![
            "i = 0;\nprint i;",
            "for(i = 0; i < 10; i++){ \n\tprint i; \n\t}",
            "fn hello_world(){ \n\tprint \"hello world\"; \n\t} \n\nhello_world();",
        ];

        if let Some(v) = examples.get(ex) {
            [
                v,
                "//press enter to run example, or use the arrow keys to go back and edit it",
            ]
            .join("\n")
        } else {
            "Couldn't find a example for that number!".to_owned()
        }
    }

    fn execute(&mut self, data: &str, debug: bool) {
        let tokens = lexer::run(&data);

        if debug {
            println!("Generated tokens: {:?}", tokens);
        }
        match parser::parse(tokens) {
            Ok(ast) => {
                if debug {
                    println!("Generated AST: {:?}", ast);
                }

                println!("");
                match repl_run(ast, &mut self.runtime, &mut self.glob_frame) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                }
                println!("");
            }
            Err(e) => println!("{}", e),
        }
    }
}
