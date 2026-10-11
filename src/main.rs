use std::fs::read_to_string;

pub mod core;

fn main() {
    let s = read_to_string("example.masm").expect("Failed to read from file.");
    println!("Original content: {s}");

    // Lexer
    let mut lexer = core::Lexer::new(&s);
    lexer.tokenize().unwrap();
    println!("Tokens: {:?}", lexer.tokens);

    // Parser
    let mut parser = core::Parser::new(lexer.tokens);
    parser.parse().unwrap();
    println!("Parser: {:?}", parser);
    parser.resolve().unwrap();
    println!("Resolved parser: {:?}", parser);
}
