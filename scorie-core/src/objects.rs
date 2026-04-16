use crate::{Expr, Value, types::Type};
use std::collections::HashMap;

pub struct Class {
    pub identifier: String,
    pub attributes: HashMap<String, Attribute>,
    pub methods: HashMap<String, Method>,
}

struct Attribute {
    identifier: String,
    r#type: Type,
    val: Value,
}

impl Attribute {
    pub fn new(identifier: &str, val: Value) -> Self {
        Attribute {
            identifier: String::from(identifier),
            r#type: val.parse_type(),
            val,
        }
    }
}

struct Method {
    class: String,
    identifier: String,
    params: Vec<(String, Type)>,
    body: Vec<Expr>,
    return_type: Type,
}

impl Method {
    pub fn new(
        class: &str,
        identifier: &str,
        params: Vec<(String, Type)>,
        body: Vec<Expr>,
        return_type: Type,
    ) -> Self {
        Method {
            class: String::from(class),
            identifier: String::from(identifier),
            params,
            body,
            return_type,
        }
    }
}
