mod lexer;

fn main() {

    let tokens = lexer::tokenize("fn main {print \"hello world\"}");
    println!("{{print \"hello world\"}}");
    println!("{:?}", tokens);
}
