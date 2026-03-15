use std::env;
use std::fs;

mod abstract_syntax_tree;
mod codegen;
mod lexer;
mod parser;
mod type_checker;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: scorie compile <file> <output_file>");
        std::process::exit(1)
    }
    let cmd = &args[1];
    let mut file = args[2].clone();
    let mut output = args[3].clone();
    if !file.ends_with(".scorie") {
        file.push_str(".scorie");
    };
    if !output.ends_with(".rs") {
        output.push_str(".rs");
    };
    match cmd.as_str() {
        "compile" => compile(&file, &output),
        _ => eprintln!("invalid command {}", cmd),
    }
}

fn compile(file: &str, output: &str) {
    let input = fs::read_to_string(file).unwrap();
    match codegen::generate_rust(parser::parse(lexer::lex(input)), output) {
        Ok(_) => println!("Successfully compiled"),
        Err(e) => {
            for err in e {
                eprintln!("{}", err);
            }
        }
    };
}

fn test_step_by_step() {
    let input = fs::read_to_string("test.txt").unwrap();
    let tokens = lexer::lex(input);
    println!("lexed result : {:?} \n", tokens);
    let parsed = parser::parse(tokens);
    println!("parsed result : {:?} \n", parsed);
    let codegenerator = codegen::generate_rust(parsed, "output.rs");
}
