use crate::{abstract_syntax_tree::*, stdlib::stdlib, type_checker, types::Type};
use std::{collections::HashMap, fs::write};

pub fn generate_rust(input: Vec<Expr>, filename: &str) -> Result<(), Vec<String>> {
    let generator = type_checker::type_check(input)?;
    let mut code = String::new();
    for expr in generator.root.clone() {
        code += &generator.generate(expr)
    }
    write(filename, code).expect("Failed to write output file");
    // println!("Codegen successfull");
    Ok(())
}

pub struct CodeGenerator {
    root: Vec<Expr>,
    func_env: HashMap<String, FuncSignature>,
    std: stdlib,
}

impl CodeGenerator {
    pub fn new(root: Vec<Expr>, func_env: HashMap<String, FuncSignature>, std: stdlib) -> Self {
        CodeGenerator {
            root,
            func_env,
            std,
        }
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
                name, params, body, ..
            } => self.generate_funcdef(name, params, body),
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
            Expr::EOL => String::from(";"),
        }
    }

    fn generate_literal(&self, val: Value) -> String {
        match val {
            Value::Int(int) => format!("{}", int),
            Value::Float(flt) => format!("{}f64", flt),
            Value::Str(string) => format!("\"{}\"", string),
            Value::Bool(b) => format!("{}", b),
            Value::FStr(fs) => self.format_fstring(&fs),
        }
    }

    fn format_fstring(&self, input: &str) -> String {
        if !input.contains('{') || !input.contains('}') {
            String::from(input)
        } else {
            let mut format_str = String::new();
            let mut vars = Vec::new();
            let mut current_var = String::new();
            let mut in_braces = false;
            for c in input.chars() {
                match c {
                    '{' => {
                        in_braces = true;
                        format_str.push_str("{}");
                    }
                    '}' => {
                        in_braces = false;
                        if !current_var.is_empty() {
                            vars.push(current_var.clone());
                            current_var.clear();
                        }
                    }
                    _ => {
                        if in_braces {
                            current_var.push(c);
                        } else {
                            format_str.push(c);
                        }
                    }
                }
            }
            format!("format!(\"{}\", {})", format_str, vars.join(", "))
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
        if self.std.functions.contains_key(&function) {
            return self.generate_stdcall(function, args);
        }
        function + "(" + self.generate_args(args).as_str() + ")"
    }

    fn generate_args(&self, args: Vec<Expr>) -> String {
        let mut new_args = String::from("");
        for (i, arg) in args.iter().enumerate() {
            new_args += &self.generate(arg.clone());
            if i < args.len() - 1 {
                new_args += ", ";
            }
        }
        new_args
    }

    fn generate_stdcall(&self, function: String, args: Vec<Expr>) -> String {
        match function.as_str() {
            "println" => {
                String::from("println!(\"{}\", ") + self.generate_args(args).as_str() + ")"
            }
            "panic" => String::from("panic!(\"{}\"") + self.generate_args(args).as_str() + ")",
            "len" => format!("{}.len()", self.generate(args[0].clone())),
            _ => unreachable!(),
        }
        // if function != "len" {
        //     funccall + "(" + self.generate_args(args).as_str() + ")"
        // } else {
        //     funccall
        // }
    }

    fn generate_let(&self, name: String, value: Box<Expr>) -> String {
        let val = self.generate(*value);
        format!("let mut {} = {}", name, val)
    }

    fn generate_funcdef(
        &self,
        name: String,
        params: Vec<(String, Type)>,
        body: Box<Expr>,
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
            Some(v) => format!("return {}", self.generate(*v)),
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
