use std::fs;

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(app =>
    (name: "BerylScript")
    (version:env!("CARGO_PKG_VERSION"))
    (author: "Emerald <@Emerald#6666>")
    (about: "Parses and runs BerylScript")
    (@arg debug: -d --debug "Display debugging information")
    (@arg PATH: +required "Path of the file to run")
    (@arg ARGS: ... +use_delimiter "Arguments to pass to the script")
    )
    .get_matches();

    let debug = matches.is_present("debug");

    if let Some(path) = matches.value_of("PATH") {
        let data = fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!("Couldn't read file {}: {}", path, e);
        });
        let args = if let Some(tmp) = matches.values_of("ARGS") {
            tmp.collect::<Vec<&str>>()
        } else {
            vec![]
        };
        beryl_lib::run(data, &args, debug);
        return;
    }
}
