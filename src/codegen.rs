use crate::{abstract_syntax_tree::*, type_checker};
use std::fs::write;

pub fn generate_rust(input: Vec<Expr>) {
    let generator = type_checker::type_check(input).unwrap();
    let mut code = String::new();
    for expr in generator.root.clone() {
        code += &generator.generate(expr)
    }
    write("output.rs", code).expect("Failed to write output file")
}

pub struct CodeGenerator {
    root: Vec<Expr>,
}

impl CodeGenerator {
    pub fn new(root: Vec<Expr>) -> Self {
        CodeGenerator { root }
    }
    pub fn generate(&self, expr: Expr) -> String {
        match expr {
            Expr::Literal(val) => self.generate_literal(val),
            Expr::BinaryOp { left, op, right } => self.generate_binaryop(left, op, right),
            Expr::Call { function, args } => self.generate_funccall(function, args),
            Expr::Var { name } => format!("{}", name),
            Expr::Let { name, value } => self.generate_let(name, value),
            Expr::FuncDef {
                name,
                params,
                body,
                return_type,
            } => self.generate_funcdef(name, params, body, return_type),
            Expr::Return { value } => self.generate_return(value),
            Expr::Block { statements } => self.generate_block(statements),
        }
    }

    fn generate_literal(&self, val: Value) -> String {
        match val {
            Value::Int(int) => format!("{}", int),
            Value::Float(flt) => format!("{}", flt),
            Value::Str(string) => format!("\"{}\"", string),
        }
    }

    fn generate_binaryop(&self, left: Box<Expr>, op: BinaryOp, right: Box<Expr>) -> String {
        let left = self.generate(*left);
        let right = self.generate(*right);
        let op = match op {
            BinaryOp::Add => String::from("+"),
            BinaryOp::Sub => String::from("-"),
            BinaryOp::Mul => String::from("*"),
            BinaryOp::Div => String::from("/"),
            BinaryOp::Mod => String::from("%"),
        };
        format!("{} {} {}", left, op, right)
    }

    fn generate_funccall(&self, function: String, args: Vec<Expr>) -> String {
        let mut funccall = function + "(";
        for (i, arg) in args.iter().enumerate() {
            funccall += &self.generate(arg.clone());
            if i < args.len() - 1 {
                funccall += ", ";
            }
        }
        funccall + ")"
    }

    fn generate_let(&self, name: String, value: Box<Expr>) -> String {
        let val = self.generate(*value);
        format!("let mut {} = {};", name, val)
    }

    fn generate_funcdef(
        &self,
        name: String,
        params: Vec<(String, Type)>,
        body: Box<Expr>,
        return_type: Option<Type>,
    ) -> String {
        let mut params_str = String::new();
        for (i, (n, t)) in params.iter().enumerate() {
            params_str += &format!("{}: {}", n, t.translate());
            if i < params.len() - 1 {
                params_str += ", ";
            }
        }
        let body = self.generate(*body);
        match return_type {
            Some(t) => format!(
                "fn {}({}) -> {} {{ {} }}",
                name,
                params_str,
                t.translate(),
                body
            ),
            None => format!("fn {}({}) {{ {} }}", name, params_str, body),
        }
    }

    fn generate_return(&self, val: Option<Box<Expr>>) -> String {
        match val {
            Some(v) => format!("return {};", self.generate(*v)),
            None => String::from(""),
        }
    }

    fn generate_block(&self, stmts: Vec<Expr>) -> String {
        let mut block = String::new();
        for stmt in stmts {
            block += &format!("{} \n", self.generate(stmt))
        }
        block
    }
}
