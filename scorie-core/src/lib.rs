mod abstract_syntax_tree;
mod codegen;
mod lexer;
mod objects;
mod parser;
mod stdlib;
mod type_checker;
mod types;
pub use abstract_syntax_tree::*;
pub use lexer::Token;

pub fn compile(source: String, output: &str) {
    match codegen::generate_rust(parser::parse(lexer::lex(source)), output) {
        Ok(_) => println!("Successfully compiled"),
        Err(e) => {
            for err in e {
                eprintln!("{}", err);
            }
        }
    };
}
