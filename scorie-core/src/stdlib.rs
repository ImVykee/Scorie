use crate::{FuncSignature, objects::Class, types::Type};
use std::collections::HashMap;

pub struct stdlib {
    pub functions: HashMap<String, FuncSignature>,
    pub classes: HashMap<String, Class>,
}

impl stdlib {
    pub fn init() -> Self {
        stdlib {
            functions: stdlib::init_functions(),
            classes: stdlib::init_classes(),
        }
    }

    fn init_functions() -> HashMap<String, FuncSignature> {
        let printlnsig = FuncSignature::new(vec![Type::Str], Type::Void);
        let panicsig = FuncSignature::new(vec![Type::Str], Type::Void);
        let lensig = FuncSignature::new(vec![Type::OneOf(vec![Type::Str])], Type::Int);
        HashMap::from([
            (String::from("println"), printlnsig),
            (String::from("panic"), panicsig),
            (String::from("len"), lensig),
        ])
    }

    pub fn get_function(&self, function: &str) -> &FuncSignature {
        match self.functions.get(function) {
            Some(function) => function,
            None => panic!("function '{}' does not exist", function),
        }
    }

    fn init_classes() -> HashMap<String, Class> {
        HashMap::new()
    }
}
