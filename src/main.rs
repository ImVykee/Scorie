use std::fs::read_to_string;

mod abstract_syntax_tree;
mod codegen;
mod lexer;
mod parser;
mod type_checker;

fn main() {
    compile();
}

fn compile() {
    let input = read_to_string("test.scorie").unwrap();
    codegen::generate_rust(parser::parse(lexer::lex(input)));
}

fn test_step_by_step() {
    let input = read_to_string("test.txt").unwrap();
    let tokens = lexer::lex(input);
    println!("lexed result : {:?} \n", tokens);
    let parsed = parser::parse(tokens);
    println!("parsed result : {:?} \n", parsed);
    let codegenerator = codegen::generate_rust(parsed);
}
