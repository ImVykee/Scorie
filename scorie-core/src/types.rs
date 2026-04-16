#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    Unknown,
    Void,
    OneOf(Vec<Type>),
}

impl Type {
    pub fn translate(&self) -> String {
        match self {
            Type::Int => String::from("i32"),
            Type::Float => String::from("f64"),
            Type::Str => String::from("String"),
            Type::Unknown | Type::OneOf(_) => {
                eprintln!("Debug info : {:?}", self);
                panic!("Unknown type made it into code generation phase");
            }
            Type::Void => String::from("()"),
            Type::Bool => String::from("bool"),
        }
    }
}
