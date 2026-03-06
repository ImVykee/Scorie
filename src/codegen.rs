use crate::abstract_syntax_tree::*;

fn main() {
    println!("codegen");
}

pub struct CodeGenerator {
    root: Vec<Expr>,
}

impl CodeGenerator {
    pub fn new(root: Vec<Expr>) -> Self {
        CodeGenerator { root }
    }
}
