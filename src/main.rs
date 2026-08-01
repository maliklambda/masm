use std::fs::read_to_string;

pub mod constants;
pub mod identifier;
pub mod lexer;
pub mod parser;
pub mod section;
pub mod tokens;

fn main() {
    let s = read_to_string("example.masm").expect("Failed to read from file.");
    println!("Original content: {s}");

    // Lexer
    let mut lexer = lexer::Lexer::new(&s);
    lexer.tokenize().unwrap();
    println!("Tokens: {:?}", lexer.tokens);

    // Parser
    let mut parser = parser::Parser::new(lexer.tokens);
    parser.parse().unwrap();
    println!("Parser: {:?}", parser);
    parser.resolve().unwrap();
    println!("Resolved parser: {:?}", parser);
}
