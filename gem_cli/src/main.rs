mod beryl;

use std::env;
use std::fs;
use std::path::PathBuf;

// use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate clap;

///Starts the REPL by default, also has `run` and `examples` subcommands
fn main() {
    let matches = clap_app!(app =>
    (name: "Beryl")
    (version: env!("CARGO_PKG_VERSION"))
    (author: "Emerald <@Emerald#6666>")
    (about: "Runs emerald script programs and other helpful stuff")
    (@arg debug: -d --debug "Display debugging information")
    (@subcommand examples =>
        (about: "Generates some example files")
        (@arg PATH: "Where to generate the files, defaults to current directory"))
    (@subcommand run =>
        (about: "Run an emerald script program")
        (@arg PATH: +required "Path of file to run")
        (@arg ARGS: ... +use_delimiter "Arguments to pass to the script"))
    )
    .get_matches();

    let debug = matches.is_present("debug");

    if let Some(sub) = matches.subcommand_matches("run") {
        let path = sub.value_of("PATH").expect("Path should not be NONE");
        if path.ends_with(".em") {
            let data = fs::read_to_string(&path).unwrap_or_else(|e| {
                panic!("Couldn't read file {}: {}", path, e);
            });
            let args = if let Some(tmp) = sub.values_of("ARGS"){
                tmp.map(|e| format!("\"{}\"", e)).collect::<Vec<String>>().join(",")
            }else {
                "".to_string()
            };
            gem::run(data, &args, debug);
            return;
        } else {
            println!("Not a valid .em file!");
            return;
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
        return;
    }

    let mut b = beryl::Repl::new(debug);
    b.run();
    // repl(debug).expect("REPL encountered an issue: ");
}

///Generates example files in the target directory or one provided by the user
fn create_examples(path: &PathBuf) {
    //big fan of this macro, makes it easy to include files in the binary
    let examples = [
        include_str!("examples/example1.em"),
        include_str!("examples/example2.em"),
        include_str!("examples/example3.em"),
        include_str!("examples/example4.em"),
        // include_str!("examples/example4.em"),
    ];

    println!("Generating example files at {}", path.display());

    //check if the directory exists first, create it if not
    if fs::read_dir(&path).is_err() {
        fs::create_dir_all(&path).unwrap_or_else(|_| {
            println!("Unable to create target directory {}", path.display());
        });
    }

    let mut count = 1;
    for ex in examples.iter() {
        let expath = path.join(format!("example{}.em", count));
        fs::write(&expath, ex).unwrap_or_else(|_| {
            println!("Error generating example file {}", expath.display());
        });
        count += 1;
    }
}
