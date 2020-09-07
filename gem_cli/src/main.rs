use std::fs;

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
        let data = fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!("Couldn't read file {}: {}", path, e);
        });
        gem::run(data);
    }
}
