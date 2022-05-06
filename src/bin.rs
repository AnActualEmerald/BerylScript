use std::{
    fs,
    io::{self, Read},
};

#[macro_use]
extern crate clap;

use atty::Stream;
use clap::{Arg, Command};

use gem::run;

fn main() {
    let matches = Command::new("Beryl")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Emerald <@Emerald#6666>")
        .about("Parses and runs beryl script")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Display debugging information"),
        )
        .arg(
            Arg::new("PATH")
                .takes_value(true)
                .help("Path to the file to run"),
        )
        .arg(
            Arg::new("ARGS")
                .takes_value(true)
                .value_delimiter(' ')
                .help("Arguments to pass to the script"),
        )
        .get_matches();

    let debug = matches.is_present("debug");

    let (file, args) = if let Some(path) = matches.value_of("PATH") {
        let data = fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!("Couldn't read file {}: {}", path, e);
        });
        let args = if let Some(tmp) = matches.values_of("ARGS") {
            tmp.map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join(",")
        } else {
            "".to_string()
        };
        (data, args)
    } else if !atty::is(Stream::Stdin) {
        let mut buff = String::new();
        match io::stdin().read_to_string(&mut buff) {
            Ok(n) => println!("Read {} bytes from input", n),
            Err(e) => println!("Error reading stdin: {}", e),
        }
        (buff, String::default())
    } else {
        //run the repl
        (String::default(), String::default())
    };

    run(file, &args, debug);
}
