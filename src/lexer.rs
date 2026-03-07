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

    // Identifiers and literals
    Identifier(String),
    Number(String),
    StringLit(String),

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

    EOF, // End Of File
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
            _ => {
                if self.token.chars().all(|c| c.is_numeric() || c == '.') {
                    Token::Number(String::from(&self.token))
                } else {
                    Token::Identifier(String::from(&self.token))
                }
            }
        });
        self.clear_token();
    }
    pub fn handle_string(&mut self) {
        self.increment();
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            self.read_char();
        }
        self.tokens
            .push(Token::StringLit(String::from(&self.token)));
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
}

pub fn lex(input: String) -> Vec<Token> {
    let mut lexer = Lexer::new(input.chars().collect());
    while lexer.pos < lexer.input.len() {
        let curr_char = lexer.input[lexer.pos];
        match curr_char {
            ' ' | '\t' | '\n' => lexer.flush_and_increment(),
            'a'..='z' | 'A'..='Z' => lexer.push_and_increment(curr_char),
            '0'..='9' => lexer.push_and_increment(curr_char),
            '"' | '\'' => lexer.handle_string(),
            '(' | ')' | '{' | '}' | '[' | ']' | ',' | ':' => {
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
    lexer.tokens
}
