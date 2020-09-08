use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

// use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(app =>
    (name: "Gem CLI")
    (version: "0.0.2")
    (author: "Emerald <@Emerald#6666>")
    (about: "Runs emerald script programs and other helpful stuff")
    (@subcommand examples =>
        (about: "Generates some example files")
        (@arg PATH: "Where to generate the files, defaults to current directory"))
    (@subcommand run =>
        (about: "Run an emerald script program")
        (@arg PATH: +required "Path of file to run"))
    )
    .get_matches();

    if let Some(sub) = matches.subcommand_matches("run") {
        let path = sub.value_of("PATH").expect("Path should not be NONE");
        if path.ends_with(".em") {
            let data = fs::read_to_string(&path).unwrap_or_else(|e| {
                panic!("Couldn't read file {}: {}", path, e);
            });
            gem::run(data);
        } else {
            println!("Not a valid .em file!");
        }
    }

    if let Some(sub) = matches.subcommand_matches("examples") {
        let path = if let Some(tmp) = sub.value_of("PATH") {
            PathBuf::from(tmp)
        } else {
            let tmp = env::current_dir().expect("Couldn't get current directory");
            PathBuf::from(format!("{}{}", tmp.display(), "/examples/"))
        };

        create_examples(&path);
    }

    //if there aren't any subcommands we should start a REPL, though that will require modifying how the gem
    //works as of v0.3
    repl().expect("REPL encountered an issue: ");
}

fn create_examples(path: &PathBuf) {
    let examples = [
        "fn main() {\n\tprint \"Hello world!\"; \n}",
        "fn main() {\n\tfor(i = 0; i < 10; i = i + 1){\n\t\tprint \"EmeraldScript rules!\";\n\t}\n}",
    ];

    println!("Generating example files at {}", path.display());

    //check if the directory exists first, create it if not
    if let Err(_) = fs::read_dir(&path) {
        fs::create_dir_all(&path)
            .expect(format!("Unable to create target directory {}", path.display()).as_str());
    }

    let mut count = 1;
    for ex in examples.iter() {
        let expath = path.join(format!("example{}.em", count));
        fs::write(&expath, ex)
            .expect(format!("Error generating example file {}", expath.display()).as_str());
        count += 1;
    }
}

fn repl() -> io::Result<usize> {
    use gem::interpreter::*;
    use gem::{lexer, parser};
    let mut runtime = Runtime::new();
    let mut glob_frame = StackFrame::new();
    let mut input = String::new();
    let mut multiline = false;

    println!("Welcome to the EmeraldScript REPL!");
    println!("Type exit or stop to leave\n");

    print!(">>> ");
    io::stdout().flush().expect("Couldn't flush stdout");

    //while input isn't "exit" or "stop"
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read input");

        if input.ends_with("{\n") {
            multiline = true;
        } else if input.ends_with("}\n") {
            multiline = false;
        }

        if ["exit\n", "stop\n"].contains(&input.as_str()) {
            break;
        } else if multiline {
            print!("\t");
            io::stdout().flush().expect("Couldn't flush stdout");
            continue;
        } else {
            let ast = parser::parse(lexer::run(format!("{}", input).as_str()));
            if let Err(e) = repl_run(ast, &mut runtime, &mut glob_frame) {
                println!("{}", e);
            }
            input = String::new();
            print!(">>> ");
            io::stdout().flush().expect("Couldn't flush stdout");
        }
    }

    Ok(0)
}