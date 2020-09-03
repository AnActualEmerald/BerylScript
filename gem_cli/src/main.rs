// use gem;
use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    // println!("got args: {:?}", args);
    let file = args.nth(1).unwrap_or_else(|| "".to_owned());
    // println!("Path to read: {}", file);
    let data = fs::read_to_string(&file).unwrap_or_else(|e| {
        panic!("Couldn't read file {}: {}", file, e);
    });
    gem::run(data);
}
