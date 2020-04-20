mod gem;

use std::env;

fn main() {
    gem::run(env::args().nth(1).unwrap().as_str());
}
