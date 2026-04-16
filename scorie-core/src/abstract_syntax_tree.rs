use crate::types::Type;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        r#type: Type,
    },
    BooleanOp {
        left: Box<Expr>,
        op: BooleanOp,
        right: Box<Expr>,
    },
    Call {
        function: String,
        args: Vec<Expr>,
    },
    Var {
        name: String,
    },
    Let {
        name: String,
        value: Box<Expr>,
        //r#type: Option<Type>,
    },
    FuncDef {
        name: String,
        params: Vec<(String, Type)>,
        body: Box<Expr>,
        return_type: Option<Type>,
    },
    Return {
        value: Option<Box<Expr>>,
    },
    Value {
        value: Option<Box<Expr>>,
    },
    If {
        condition: Box<Expr>,
        body: Box<Expr>,
        r#else: Box<Expr>,
    },
    Block {
        statements: Vec<Expr>,
        returns: Box<Expr>,
    },
    NotExist,
    EOL,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    Str(String),
    FStr(String),
    Bool(bool),
}

impl Value {
    pub fn parse_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::Str(_) | Value::FStr(_) => Type::Str,
            Value::Bool(_) => Type::Bool,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BooleanOp {
    And,
    Or,
}
#[derive(Debug, Clone)]
pub struct FuncSignature {
    pub param_types: Vec<Type>,
    pub return_type: Type,
}

impl FuncSignature {
    pub fn new(param_types: Vec<Type>, return_type: Type) -> Self {
        FuncSignature {
            param_types,
            return_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectDef {
    attributes: HashMap<String, (String, Type)>, // Name, Value, Type
    methods: HashMap<String, FuncSignature>,
}

// pub fn stdlib() -> Vec<(String, FuncSignature)> {
//     vec![
//         (
//             String::from("print"),
//             FuncSignature {
//                 param_types: vec![Type::Str],
//                 return_type: Type::Void,
//             },
//         ),
//         (
//             String::from("panic"),
//             FuncSignature {
//                 param_types: vec![Type::Str],
//                 return_type: Type::Void,
//             },
//         ),
//         (
//             String::from("len"),
//             FuncSignature {
//                 param_types: vec![Type::Unknown],
//                 return_type: Type::Int,
//             },
//         ),
//     ]
// }
