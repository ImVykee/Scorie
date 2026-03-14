use crate::{abstract_syntax_tree::*, lexer::Token};

pub fn parse(input: Vec<Token>) -> Vec<Expr> {
    let mut parser = Parser::new(input);
    let mut statements: Vec<Expr> = Vec::new();
    while parser.current() != &Token::EOF {
        statements.push(parser.parse_statement());
    }
    statements
}

struct Parser {
    input: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(input: Vec<Token>) -> Self {
        Parser { input, pos: 0 }
    }

    fn increment(&mut self) {
        self.pos += 1;
    }

    fn decrement(&mut self) {
        self.pos -= 1;
    }

    fn current(&self) -> &Token {
        &self.input[self.pos]
    }

    fn parse_statement(&mut self) -> Expr {
        match self.current() {
            Token::Let => self.parse_let(),
            Token::Fn => self.parse_fn(),
            Token::Return => self.parse_return(),
            Token::Value => self.parse_expression_return(),
            _ => self.parse_expression(),
        }
    }

    fn parse_expression(&mut self) -> Expr {
        // an expression produces a value
        let left = self.parse_primary();
        match self.current() {
            Token::Plus | Token::Minus | Token::Slash | Token::Star | Token::Modulo => {
                let op = match self.current() {
                    Token::Plus => BinaryOp::Add,
                    Token::Minus => BinaryOp::Sub,
                    Token::Slash => BinaryOp::Div,
                    Token::Star => BinaryOp::Mul,
                    Token::Modulo => BinaryOp::Mod,
                    _ => unreachable!(),
                };
                self.increment();
                let right = self.parse_expression();
                Expr::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                    r#type: Type::Unknown,
                }
            }
            Token::And | Token::Or => {
                let op = match self.current() {
                    Token::And => BooleanOp::And,
                    Token::Or => BooleanOp::Or,
                    _ => unreachable!(),
                };
                self.increment();
                let right = self.parse_expression();
                Expr::BooleanOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                }
            }
            _ => left,
        }
    }

    fn parse_primary(&mut self) -> Expr {
        let token = self.current().clone();
        match token {
            Token::Number(n) => {
                if n.chars().filter(|c| *c == '.').count() == 1 {
                    let val = n.parse::<f64>().unwrap();
                    self.increment();
                    return Expr::Literal(Value::Float(val));
                }
                let val = n.parse::<i32>().unwrap();
                self.increment();
                Expr::Literal(Value::Int(val))
            }
            Token::StringLit(s) => self.parse_stringlit(s),
            Token::LeftParen => self.parse_paren_operations(),
            Token::Identifier(elem) => self.parse_identifier(elem),
            Token::Boolean(b) => self.parse_boolean(b),
            Token::If => self.parse_if(),
            _ => panic!("Unknown token {:?}", self.input[self.pos]),
        }
    }

    fn parse_if(&mut self) -> Expr {
        self.increment();
        let conditions = self.parse_expression();
        let expressions = self.parse_block();
        let else_stmt = if self.current() == &Token::Else {
            self.increment();
            self.parse_block()
        } else {
            Expr::NotExist
        };
        Expr::If {
            condition: Box::new(conditions),
            body: Box::new(expressions),
            r#else: Box::new(else_stmt),
        }
    }

    fn parse_boolean(&mut self, b: String) -> Expr {
        self.increment();
        match b.as_str() {
            "True" => Expr::Literal(Value::Bool(true)),
            "False" => Expr::Literal(Value::Bool(false)),
            _ => unreachable!(),
        }
    }

    fn parse_stringlit(&mut self, s: String) -> Expr {
        self.increment();
        Expr::Literal(Value::Str(s))
    }

    fn parse_identifier(&mut self, elem: String) -> Expr {
        self.increment();
        match self.current() {
            Token::LeftParen => self.parse_call(elem),
            _ => Expr::Var { name: elem },
        }
    }

    fn parse_call(&mut self, elem: String) -> Expr {
        self.increment();
        let parsed_args = self.get_func_params_value();
        self.increment();
        Expr::Call {
            function: elem,
            args: parsed_args,
        }
    }

    fn parse_paren_operations(&mut self) -> Expr {
        self.increment();
        let expr = self.parse_expression();
        match self.current() {
            Token::RightParen => self.increment(),
            _ => panic!("Expected '('"),
        };
        expr
    }

    fn parse_return(&mut self) -> Expr {
        self.increment();
        let value = self.parse_statement();
        match value {
            Expr::FuncDef { .. } => panic!("Cannot return a function definition"),
            Expr::Let { .. } => panic!("Cannot return a variable assignment"),
            Expr::Value { .. } => panic!("Cannot return a 'Value' statement"),
            Expr::Return { .. } => panic!("Cannot return a 'Return' statement"),
            _ => Expr::Return {
                value: Some(Box::new(value)),
            },
        }
    }

    fn parse_expression_return(&mut self) -> Expr {
        self.increment();
        let value = self.parse_statement();
        match value {
            Expr::FuncDef { .. } => panic!("Cannot return a function definition"),
            Expr::Let { .. } => panic!("Cannot return a variable assignment"),
            Expr::Value { .. } => panic!("Cannot return a 'Value' statement"),
            Expr::Return { .. } => panic!("Cannot return a 'Return' statement"),
            _ => Expr::Value {
                value: Some(Box::new(value)),
            },
        }
    }

    fn get_func_params_value(&mut self) -> Vec<Expr> {
        let mut args: Vec<Expr> = Vec::new();
        while self.current() != &Token::RightParen {
            args.push(self.parse_expression());
            if self.current() == &Token::Comma {
                self.increment();
            }
        }
        args
    }

    fn get_func_params_identifier(&mut self) -> Vec<(String, Type)> {
        let mut params: Vec<(String, Type)> = Vec::new();
        while self.current() != &Token::RightParen {
            let mut curr_param = (String::new(), Type::Unknown);
            curr_param.0 = match self.current() {
                Token::Identifier(n) => n.clone(),
                _ => panic!("Expected parameter"),
            };
            self.increment();
            if self.current() != &Token::Colon {
                panic!("Expected ':' after parameter declaration");
            };
            self.increment();
            curr_param.1 = match self.current() {
                Token::Identifier(t) => match t.as_str() {
                    "Int" => Type::Int,
                    "Float" => Type::Float,
                    "String" => Type::Str,
                    _ => panic!("Unknown type {}", t),
                },
                _ => panic!("Expected type annotation"),
            };
            self.increment();
            params.push(curr_param);
            if self.current() == &Token::Comma {
                self.increment();
            } else if self.current() != &Token::RightParen {
                panic!("Expected ',' as param separator or ')' as end of params declaration")
            }
        }
        params
    }

    fn parse_fn(&mut self) -> Expr {
        self.increment();
        let name = match self.current() {
            Token::Identifier(n) => n.clone(),
            _ => panic!("Expected Identifier after 'fn'"),
        };
        self.increment();
        match self.current() {
            Token::LeftParen => self.increment(),
            _ => panic!("Expected '(' after function name"),
        };
        let params: Vec<(String, Type)> = self.get_func_params_identifier();
        self.increment();

        let body = Box::new(self.parse_block());
        Expr::FuncDef {
            name,
            params,
            body,
            return_type: None,
        }
    }

    fn parse_block(&mut self) -> Expr {
        // recursive
        let mut block: Vec<Expr> = Vec::new();
        let mut returns = Expr::NotExist;
        match self.current() {
            Token::LeftBrace => self.increment(),
            _ => panic!("Expected '{{' after function parameters"),
        }
        while self.current() != &Token::RightBrace {
            if self.current() == &Token::Value {
                returns = self.parse_statement();
                block.push(returns.clone());
            } else {
                block.push(self.parse_statement());
            }
        }
        self.increment();
        Expr::Block {
            statements: block,
            returns: Box::from(returns),
        }
    }

    fn parse_let(&mut self) -> Expr {
        self.increment();
        let name = match self.current() {
            Token::Identifier(n) => n.clone(),
            _ => panic!("Expected Identifier after 'let'"),
        };
        self.increment();
        match self.current() {
            Token::Equals => self.increment(),
            _ => panic!("Expected '=' after 'let'"),
        };
        let val = self.parse_expression();
        Expr::Let {
            name,
            value: Box::new(val),
        }
    }
}
