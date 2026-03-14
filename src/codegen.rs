use crate::{abstract_syntax_tree::*, type_checker};
use std::{collections::HashMap, fs::write};

pub fn generate_rust(input: Vec<Expr>, filename: &str) -> Result<(), Vec<String>> {
    let generator = type_checker::type_check(input)?;
    let mut code = String::new();
    for expr in generator.root.clone() {
        code += &generator.generate(expr)
    }
    write(filename, code).expect("Failed to write output file");
    Ok(())
}

pub struct CodeGenerator {
    root: Vec<Expr>,
    func_env: HashMap<String, FuncSignature>,
}

impl CodeGenerator {
    pub fn new(root: Vec<Expr>, func_env: HashMap<String, FuncSignature>) -> Self {
        CodeGenerator { root, func_env }
    }
    pub fn generate(&self, expr: Expr) -> String {
        match expr {
            Expr::Literal(val) => self.generate_literal(val),
            Expr::BinaryOp {
                left,
                op,
                right,
                r#type,
            } => self.generate_binaryop(left, op, right, r#type),
            Expr::BooleanOp { left, op, right } => self.generate_booleanop(left, op, right),
            Expr::Call { function, args } => self.generate_funccall(function, args),
            Expr::Var { name } => name,
            Expr::Let { name, value } => self.generate_let(name, value),
            Expr::FuncDef {
                name,
                params,
                body,
                return_type,
            } => self.generate_funcdef(name, params, body, return_type),
            Expr::Return { value } => self.generate_return(value),
            Expr::Value { value } => self.generate_block_return(value),
            Expr::Block {
                statements,
                returns,
            } => self.generate_block(statements, returns),
            Expr::If {
                condition,
                body,
                r#else,
            } => self.generate_if_else(condition, body, r#else),
            Expr::NotExist => String::new(),
        }
    }

    fn generate_literal(&self, val: Value) -> String {
        match val {
            Value::Int(int) => format!("{}", int),
            Value::Float(flt) => format!("{}f64", flt),
            Value::Str(string) => format!("\"{}\"", string),
            Value::Bool(b) => format!("{}", b),
        }
    }

    fn generate_booleanop(&self, left: Box<Expr>, op: BooleanOp, right: Box<Expr>) -> String {
        let left = self.generate(*left);
        let right = self.generate(*right);
        let op = match op {
            BooleanOp::And => String::from("&&"),
            BooleanOp::Or => String::from("||"),
        };
        format!("{} {} {}", left, op, right)
    }

    fn generate_binaryop(
        &self,
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        expr_type: Type,
    ) -> String {
        let left = format!("({} as {})", self.generate(*left), expr_type.translate());
        let right = format!("({} as {})", self.generate(*right), expr_type.translate());
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
        let return_type = self.func_env[&name].return_type.clone();
        match return_type {
            Type::Void => format!("fn {}({}) {{ {} }}", name, params_str, body),
            _ => format!(
                "fn {}({}) -> {} {{ {} }}",
                name,
                params_str,
                return_type.translate(),
                body
            ),
        }
    }

    fn generate_return(&self, val: Option<Box<Expr>>) -> String {
        match val {
            Some(v) => format!("return {};", self.generate(*v)),
            None => String::from(""),
        }
    }

    fn generate_block_return(&self, val: Option<Box<Expr>>) -> String {
        match val {
            Some(v) => format!("{}", self.generate(*v)),
            None => String::from(""),
        }
    }

    fn generate_block(&self, stmts: Vec<Expr>, returns: Box<Expr>) -> String {
        let mut block = String::new();
        for stmt in stmts {
            if stmt == *returns {
                block += &format!("{} \n", self.generate(stmt));
                break;
            }
            block += &format!("{} \n", self.generate(stmt))
        }
        block
    }

    fn generate_if_else(&self, cond: Box<Expr>, body: Box<Expr>, else_body: Box<Expr>) -> String {
        if *else_body == Expr::NotExist {
            format!("if {} {{ {} }}", self.generate(*cond), self.generate(*body))
        } else {
            format!(
                "if {} {{ {} }} else {{ {} }}",
                self.generate(*cond),
                self.generate(*body),
                self.generate(*else_body)
            )
        }
    }
}
