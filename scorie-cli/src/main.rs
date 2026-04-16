use scorie::compile;
use std::env;
use std::fs;

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
    let source = fs::read_to_string(file).unwrap();
    match cmd.as_str() {
        "compile" => compile(source, &output),
        _ => eprintln!("invalid command {}", cmd),
    }
}
