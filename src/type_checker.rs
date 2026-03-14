use crate::abstract_syntax_tree::*;
use crate::codegen::CodeGenerator;
use std::collections::HashMap;

pub fn type_check(input: Vec<Expr>) -> Result<CodeGenerator, Vec<String>> {
    let mut type_checker = TypeChecker::new(input);
    type_checker.enter_scope();
    type_checker.get_func_env();
    let mut root = std::mem::take(&mut type_checker.root);
    for expr in &mut root {
        type_checker.check_statement(expr);
    }
    type_checker.root = root;
    if type_checker.errors.is_empty() {
        Ok(CodeGenerator::new(type_checker.root, type_checker.func_env))
    } else {
        Err(type_checker.errors)
    }
}

struct TypeChecker {
    root: Vec<Expr>,
    var_env: Vec<HashMap<String, Type>>,
    func_env: HashMap<String, FuncSignature>,
    errors: Vec<String>,
    current_return_type: Option<Type>,
}

impl TypeChecker {
    fn new(input: Vec<Expr>) -> Self {
        TypeChecker {
            root: input,
            var_env: Vec::new(),
            func_env: HashMap::new(),
            errors: Vec::new(),
            current_return_type: None,
        }
    }
    fn enter_scope(&mut self) {
        self.var_env.push(HashMap::new());
    }
    fn exit_scope(&mut self) {
        self.var_env.pop();
    }
    fn current_scope(&mut self) -> &mut HashMap<String, Type> {
        self.var_env.last_mut().unwrap()
    }
    fn global_scope(&mut self) -> &HashMap<String, Type> {
        &self.var_env[0]
    }

    fn get_func_env(&mut self) {
        for expr in &self.root {
            if let Expr::FuncDef {
                name,
                params,
                body,
                return_type,
            } = expr
            {
                let sig = FuncSignature {
                    param_types: vec![Type::Unknown; params.len()],
                    return_type: Type::Unknown,
                };
                self.func_env.insert(String::from(name), sig);
            }
        }
    }

    fn update_var(&mut self, var: &str, n_type: &Type) {
        for scope in self.var_env.iter_mut().rev() {
            if scope.contains_key(var) {
                scope.insert(var.to_string(), n_type.clone());
                return;
            }
        }
    }

    fn check_expression(&mut self, expr: &mut Expr) -> Type {
        match expr {
            Expr::Literal(v) => v.parse_type(),
            Expr::BinaryOp {
                left,
                op,
                right,
                r#type,
            } => {
                let left_t = self.check_expression(left);
                let right_t = self.check_expression(right);
                if left_t == Type::Unknown && right_t != Type::Unknown {
                    if let Expr::Var { name } = left.as_ref() {
                        self.update_var(name, &right_t);
                    }
                }
                if right_t == Type::Unknown && left_t != Type::Unknown {
                    if let Expr::Var { name } = right.as_ref() {
                        self.update_var(name, &left_t);
                    }
                }
                *r#type = self.resolve_type_priority(left_t, right_t);
                r#type.clone()
            }
            Expr::BooleanOp { left, op, right } => {
                let left_t = self.check_expression(left);
                let right_t = self.check_expression(right);
                if left_t != Type::Bool || right_t != Type::Bool {
                    self.error(format!(
                        "Boolean operations should have 2 bools not {} and {}",
                        left_t.translate(),
                        right_t.translate()
                    ));
                }
                Type::Bool
            }
            Expr::Call { function, args } => self.check_call_type(function, args),
            Expr::Var { name } => self.check_var(name),
            Expr::Return { value } => self.check_return(value),
            Expr::Value { value } => self.check_return(value),
            Expr::Block {
                statements,
                returns,
            } => {
                for stmt in statements {
                    self.check_statement(stmt);
                }
                self.check_statement(&mut **returns)
            }
            Expr::If {
                condition,
                body,
                r#else,
            } => self.check_if(condition, body, r#else),
            Expr::NotExist => Type::Void,
            _ => {
                self.error(format!(
                    "Non expression {:?} entered check_expression()",
                    expr
                ));
                Type::Unknown
            }
        }
    }

    fn check_statement(&mut self, stmt: &mut Expr) -> Type {
        match stmt {
            Expr::Let { name, value } => {
                let vartype = self.check_expression(value);
                self.current_scope().insert(name.clone(), vartype);
                Type::Void
            }
            Expr::FuncDef {
                name,
                params,
                body,
                return_type,
            } => {
                self.enter_scope();
                for param in &mut *params {
                    self.current_scope()
                        .insert(param.0.clone(), param.1.clone());
                }
                self.check_expression(body);
                let mut param_types: Vec<Type> = Vec::new();
                for param in params {
                    param_types.push(self.current_scope().get(&param.0).unwrap().clone());
                }
                let new_return_type = self.current_return_type.take().unwrap_or(Type::Void);
                let new_sig = FuncSignature {
                    param_types,
                    return_type: new_return_type,
                };
                self.func_env.insert(name.clone(), new_sig);
                self.exit_scope();
                Type::Void
            }
            _ => self.check_expression(stmt),
        }
    }

    fn check_var(&mut self, var: &str) -> Type {
        for scope in self.var_env.iter().rev() {
            if let Some(t) = scope.get(var) {
                return t.clone();
            }
        }
        self.error(format!("Undefined variable: {}", var));
        Type::Unknown
    }

    fn check_return(&mut self, value: &mut Option<Box<Expr>>) -> Type {
        let ret_type = match value {
            Some(e) => self.check_expression(e),
            None => Type::Void,
        };
        self.current_return_type = Some(ret_type.clone());
        ret_type
    }

    fn check_call_type(&mut self, function: &String, args: &mut Vec<Expr>) -> Type {
        let sig = self.func_env.get(function).unwrap();
        let wanted_types = sig.param_types.clone();
        let return_type = sig.return_type.clone();
        let mut called_types: Vec<Type> = Vec::new();
        for arg in args {
            called_types.push(self.check_expression(arg));
        }
        match wanted_types == called_types {
            true => (),
            false => self.error(format!(
                "mismatched call types from expected types of function {}",
                function
            )),
        }
        return_type
    }

    fn resolve_type_priority(&mut self, left: Type, right: Type) -> Type {
        match (left.clone(), right.clone()) {
            (Type::Int, Type::Int) => Type::Int,
            (Type::Str, Type::Str) => Type::Str,
            (Type::Float, Type::Float) => Type::Float,
            (Type::Int, Type::Float) | (Type::Float, Type::Int) => Type::Float,
            (Type::Unknown, Type::Unknown) => Type::Unknown,
            (Type::Unknown, t) => t,
            _ => {
                self.error(format!(
                    "Type mismatch: cannot combine {:?} and {:?}",
                    left, right
                ));
                Type::Unknown
            }
        }
    }

    fn check_if(&mut self, cond: &mut Expr, body: &mut Expr, else_stmt: &mut Expr) -> Type {
        match self.check_statement(cond) {
            Type::Bool => (),
            _ => self.error(String::from("'if' condition should evaluate to a boolean")),
        };
        self.check_statement(body);
        if match else_stmt {
            Expr::NotExist => false,
            _ => true,
        } {
            self.check_statement(else_stmt)
        } else {
            Type::Void
        }
    }

    fn error(&mut self, err: String) {
        self.errors.push(err);
    }
}
