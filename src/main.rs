use std::fs::read_to_string;

mod abstract_syntax_tree;
mod codegen;
mod lexer;
mod parser;
mod type_checker;

fn main() {
    let input = read_to_string("test.txt").unwrap();
    let tokens = lexer::lex(input);
    println!("lexed result : {:?} \n", tokens);
    let parsed = parser::parse(tokens);
    println!("parsed result : {:?} \n", parsed);
    match type_checker::type_check(parsed) {
        Ok(_) => println!("type checking passed \n"),
        Err(errors) => {
            for error in errors {
                eprintln!("Error: {}", error);
            }
        }
    };
}
