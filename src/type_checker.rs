use crate::abstract_syntax_tree::*;
use crate::codegen::CodeGenerator;
use std::collections::HashMap;

pub fn type_check(input: Vec<Expr>) -> Result<CodeGenerator, Vec<String>> {
    let mut type_checker = TypeChecker::new(input);
    type_checker.enter_scope();
    type_checker.get_func_env();
    for expr in type_checker.root.clone() {
        type_checker.check_statement(&expr);
    }
    if type_checker.errors.is_empty() {
        Ok(CodeGenerator::new(type_checker.root))
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

struct FuncSignature {
    param_types: Vec<Type>,
    return_type: Type,
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

    fn check_expression(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Literal(e) => self.get_literal_type(e),
            Expr::BinaryOp { left, op, right } => {
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
                self.resolve_type_priority(left_t, right_t)
            }
            Expr::Call { function, args } => self.check_call_type(function, args),
            Expr::Var { name } => self.check_var(name),
            Expr::Return { value } => {
                let ret_type = match value {
                    Some(e) => self.check_expression(e),
                    None => Type::Void,
                };
                self.current_return_type = Some(ret_type.clone());
                ret_type
            }
            Expr::Block { statements } => {
                let mut return_type = Type::Void;
                for stmt in statements {
                    return_type = self.check_statement(stmt);
                }
                return_type
            }
            _ => {
                self.error(format!(
                    "Non expression {:?} entered check_expression()",
                    expr
                ));
                Type::Unknown
            }
        }
    }

    fn check_statement(&mut self, stmt: &Expr) -> Type {
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
                for param in params {
                    self.current_scope().insert(param.0.clone(), Type::Unknown);
                }
                self.check_expression(body);
                let mut param_types: Vec<Type> = Vec::new();
                for param in params {
                    param_types.push(self.current_scope().get(&param.0).unwrap().clone());
                }
                let return_type = self.current_return_type.take().unwrap_or(Type::Void);
                let new_sig = FuncSignature {
                    param_types,
                    return_type,
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

    fn check_call_type(&mut self, function: &String, args: &Vec<Expr>) -> Type {
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

    fn get_literal_type(&self, literal: &Value) -> Type {
        match literal {
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::Str(_) => Type::Str,
        }
    }

    fn error(&mut self, err: String) {
        self.errors.push(err);
    }
}
