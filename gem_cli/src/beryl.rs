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
    debug: bool,
}

impl Repl {
    pub fn new(debug: bool) -> Self {
        let mut cache = PathBuf::from(
            ProjectDirs::from("", "EmeraldScript", "Beryl")
                .expect("Unable to find cache directory")
                .cache_dir(),
        );
        cache.set_file_name("history.txt");
        let rt = Runtime::new();
        Repl {
            runtime: rt,
            glob_frame: StackFrame::new(),
            multiline: vec![],
            cache: cache,
            debug,
        }
    }

    pub fn run(&mut self) {
        Term::stdout()
            .clear_screen()
            .expect("Unable to clear screen on startup");
        let mut rl = Editor::<()>::new();
        if rl.load_history(&self.cache).is_err() {
            if self.debug {
                println!("Couldn't load history");
            }
        } else if self.debug {
            println!("Loaded history file {}", self.cache.display());
        }

        println!(
            "Welcome to Beryl, the interactive EmeraldScript interpreter v{}!\nGem version {}",
            env!("CARGO_PKG_VERSION"),
            gem::version()
        );
        println!("Type exit or stop to leave, or help for more info\n");
        let mut prompt = "<== ".to_owned();
        let mut data = String::new();
        loop {
            let input = rl.readline(&prompt);
            match input {
                Ok(raw) => {
                    match raw.as_str() {
                        "help" => {
                            println!(include_str!("help.txt"));
                        }
                        "exit" | "stop" => break,
                        "clear" => {
                            let buf = Term::stdout();
                            buf.clear_screen().expect("Unable to clear console");

                            println!(
                                "Welcome to Beryl, the interactive EmeraldScript interpreter v{}!",
                                env!("CARGO_PKG_VERSION")
                            );
                            println!("Type exit or stop to leave, or help for more info\n");
                        }

                        line => {
                            if line.starts_with("example") {
                                let parts = line.split_whitespace();
                                for p in parts {
                                    match p.parse::<usize>() {
                                        Ok(v) => {
                                            let ex = self.get_example(v - 1);
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

                            self.execute(&data);
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

            rl.save_history(&self.cache)
                .or_else(|er| {
                    use std::fs;
                    if fs::read(&self.cache).is_err() {
                        if self.debug {
                            print!("History file doesn't exist, creating it... ");
                        }
                        match fs::create_dir_all(&self.cache) {
                            Ok(_) => {
                                if self.debug {
                                    println!("Done!");
                                }
                                Ok(())
                            }
                            Err(e) => {
                                if self.debug {
                                    println!("Couldn't create file: {}", e);
                                }
                                Err(e)
                            }
                        }
                    } else {
                        if self.debug {
                            println!("File seems to exist, can't save to it because: {}", er);
                        }
                        Ok(())
                    }
                })
                .unwrap();
        }
        Term::stdout()
            .clear_screen()
            .expect("Unable to clear screen at exit");
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

    fn execute(&mut self, data: &str) {
        let tokens = lexer::run(&data);

        if self.debug {
            println!("Generated tokens: {:?}", tokens);
        }
        match parser::parse(tokens) {
            Ok(ast) => {
                if self.debug {
                    println!("Generated AST: {:?}", ast);
                }

                match repl_run(ast, &mut self.runtime, &mut self.glob_frame) {
                    Ok(v) => println!("\nout: {}", v),
                    Err(e) => println!("{}", e),
                }
                println!("");
            }
            Err(e) => println!("{}", e),
        }
    }
}
