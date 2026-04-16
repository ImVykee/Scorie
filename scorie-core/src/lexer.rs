pub struct Lexer {
    input: Vec<char>,
    pub pos: usize,
    pub token: String,
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Fn,
    Return,
    Value,
    And,
    Or,
    If,
    Else,

    // Identifiers and literals
    Identifier(String),
    Number(String),
    StringLit(String),
    FStringLit(String),
    Boolean(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    Equals,

    // Punctuation
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftSqBrace,  // [
    RightSqBrace, // ]
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;

    EOF, // End Of File

    // Utils
    F_STRING_START,
}

impl Lexer {
    fn new(input: Vec<char>) -> Self {
        Lexer {
            input,
            pos: 0,
            token: String::new(),
            tokens: Vec::new(),
        }
    }
    pub fn read_char(&mut self) {
        self.token.push(self.input[self.pos]);
        self.pos += 1;
    }
    fn clear_token(&mut self) {
        self.token = String::new();
    }
    fn flush(&mut self) {
        if self.token.is_empty() {
            return;
        }
        self.tokens.push(match self.token.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "value" => Token::Value,
            "and" => Token::And,
            "or" => Token::Or,
            "if" => Token::If,
            "else" => Token::Else,
            _ => {
                if self.token.chars().all(|c| c.is_numeric() || c == '.') {
                    Token::Number(String::from(&self.token))
                } else if self.token == "True" || self.token == "False" {
                    Token::Boolean(String::from(&self.token))
                } else {
                    Token::Identifier(String::from(&self.token))
                }
            }
        });
        self.clear_token();
    }

    pub fn handle_string(&mut self) {
        let mut is_fstring = false;
        if self.tokens.last().unwrap() == &Token::F_STRING_START {
            is_fstring = true;
        }
        self.increment();
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            self.read_char();
        }
        if !is_fstring {
            self.tokens
                .push(Token::StringLit(String::from(&self.token)));
        } else {
            self.tokens
                .push(Token::FStringLit(String::from(&self.token)));
        }
        self.clear_token();
        self.increment();
    }

    pub fn increment(&mut self) {
        self.pos += 1;
    }
    pub fn push_to_buffer(&mut self, c: char) {
        self.token.push(c);
    }
    pub fn push_and_increment(&mut self, c: char) {
        self.push_to_buffer(c);
        self.increment();
    }
    pub fn flush_and_increment(&mut self) {
        self.flush();
        self.increment();
    }

    fn decrement(&mut self) {
        self.pos -= 1;
    }

    fn peek(&mut self) -> char {
        self.increment();
        let ret = self.input[self.pos];
        self.decrement();
        ret
    }
}

pub fn lex(input: String) -> Vec<Token> {
    let mut lexer = Lexer::new(input.chars().collect());
    while lexer.pos < lexer.input.len() {
        let curr_char = lexer.input[lexer.pos];
        if curr_char == 'f' {
            let peeked = lexer.peek();
            if peeked == '"' || peeked == '\'' {
                lexer.tokens.push(Token::F_STRING_START);
                lexer.increment();
                continue;
            }
        }
        match curr_char {
            ' ' | '\t' | '\n' => lexer.flush_and_increment(),
            'a'..='z' | 'A'..='Z' | '_' => lexer.push_and_increment(curr_char),
            '0'..='9' => lexer.push_and_increment(curr_char),
            '"' | '\'' => lexer.handle_string(),
            '(' | ')' | '{' | '}' | '[' | ']' | ',' | ':' | ';' => {
                lexer.flush();
                lexer.tokens.push(match curr_char {
                    '(' => Token::LeftParen,
                    ')' => Token::RightParen,
                    '{' => Token::LeftBrace,
                    '}' => Token::RightBrace,
                    '[' => Token::LeftSqBrace,
                    ']' => Token::RightSqBrace,
                    ',' => Token::Comma,
                    ':' => Token::Colon,
                    ';' => Token::Semicolon,
                    _ => unreachable!(),
                });
                lexer.increment();
            }
            '=' | '+' | '-' | '*' | '/' | '%' => {
                lexer.flush();
                lexer.tokens.push(match curr_char {
                    '=' => Token::Equals,
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Star,
                    '/' => Token::Slash,
                    '%' => Token::Modulo,
                    _ => unreachable!(),
                });
                lexer.increment();
            }
            '.' => {
                if !lexer.token.is_empty()
                    && lexer.token.chars().all(|c| c.is_numeric())
                    && lexer.pos + 1 < lexer.input.len()
                    && lexer.input[lexer.pos + 1].is_numeric()
                {
                    lexer.push_and_increment('.');
                } else {
                    panic!("Unexpected '.' character")
                }
            }
            _ => panic!("Unrecognized character : {}", curr_char),
        }
    }
    lexer.tokens.push(Token::EOF);
    lexer.tokens.retain(|t| t != &Token::F_STRING_START);

    // println!("Tokenized successfully : {:?}", lexer.tokens);
    lexer.tokens
}
