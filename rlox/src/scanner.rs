use crate::token::Token;
use crate::token::TokenType;

use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Scanner {
    original_text: String,
    tokens: Vec<Result<Token, String>>,
    start: usize,
    current: usize,
    line: usize,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("fun", TokenType::Fun),
        ("for", TokenType::For),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ]);
}

impl Scanner {
    // SWC: Maybe nice to one day eliminate side effects from a lot of this code

    pub fn new(text: &String) -> Scanner {
        Scanner {
            original_text: text.clone(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Result<Token, String>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Ok(Token::new(TokenType::EOF, &"".to_owned(), self.line)));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.original_text.len()
    }

    fn scan_token(&mut self) {
        let next_char = self.advance();

        match next_char {
            '(' => self.add_simple_token(TokenType::LeftParen),
            ')' => self.add_simple_token(TokenType::RightParen),
            '{' => self.add_simple_token(TokenType::LeftBrace),
            '}' => self.add_simple_token(TokenType::RightBrace),
            ',' => self.add_simple_token(TokenType::Comma),
            '.' => self.add_simple_token(TokenType::Dot),
            '-' => self.add_simple_token(TokenType::Minus),
            '+' => self.add_simple_token(TokenType::Plus),
            ';' => self.add_simple_token(TokenType::Semicolon),
            '*' => self.add_simple_token(TokenType::Star),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_simple_token(token_type)
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_simple_token(token_type)
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_simple_token(token_type)
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_simple_token(token_type)
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        let _ = self.advance();
                    }
                } else {
                    self.add_simple_token(TokenType::Slash)
                }
            }
            '"' => self.string(),
            '\n' => self.line += 1,
            '\r' => (),
            ' ' => (),
            '\t' => (),
            _ => {
                if self.is_digit(next_char) {
                    self.number();
                } else if self.is_alpha(next_char) {
                    self.identifier();
                } else {
                    let mut message: String = "Unexpected character '".to_owned();
                    message.push(next_char);
                    message.push('\'');
                    self.add_error(self.line, &message)
                }
            }
        }
    }

    fn is_digit(&self, next_char: char) -> bool {
        next_char >= '0' && next_char <= '9'
    }

    fn is_alpha(&self, next_char: char) -> bool {
        (next_char >= 'a' && next_char <= 'z')
            || (next_char >= 'A' && next_char <= 'Z')
            || (next_char == '_')
    }

    fn is_alpha_numeric(&self, next_char: char) -> bool {
        self.is_digit(next_char) || self.is_alpha(next_char)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            let _ = self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            let _ = self.advance();
            while self.is_digit(self.peek()) {
                let _ = self.advance();
            }
        }

        let number_string = self.current_lexeme();
        let number: f32 = number_string.parse().unwrap();
        self.add_simple_token(TokenType::Number(number));
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            let _ = self.advance();
        }

        let lexeme = self.current_lexeme();
        let possible_entry = KEYWORDS.get(&lexeme.as_str());
        match possible_entry {
            Some(token_type) => self.add_simple_token((*token_type).clone()),
            _ => self.add_simple_token(TokenType::Identifier(lexeme)),
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            let _ = self.advance();
        }
        if self.is_at_end() {
            self.add_error(self.line, &"Unterminated string".to_owned());
            return;
        }

        // Last " char
        let _ = self.advance();

        let string_start = self.start + 1;
        let string_end = self.current - 1;
        let string = self
            .original_text
            .chars()
            .skip(string_start)
            .take(string_end - string_start)
            .collect();
        self.add_simple_token(TokenType::String(string));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current_char()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.original_text.len() {
            '\0'
        } else {
            self.next_char()
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.current_char() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn current_char(&self) -> char {
        self.original_text.chars().nth(self.current).unwrap()
    }

    fn next_char(&self) -> char {
        self.original_text.chars().nth(self.current + 1).unwrap()
    }

    fn advance(&mut self) -> char {
        let out_char = self.current_char();
        self.current += 1;
        out_char
    }

    fn current_lexeme(&self) -> String {
        self.original_text
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect()
    }

    fn add_simple_token(&mut self, token_type: TokenType) {
        let text = self.current_lexeme();
        self.tokens
            .push(Ok(Token::new(token_type, &text, self.line)));
    }

    fn add_error(&mut self, line: usize, message: &String) {
        self.add_report(line, &"".to_owned(), message);
    }

    fn add_report(&mut self, line: usize, where_at: &String, message: &String) {
        let mut output = "[line ".to_owned();
        output.push_str(&line.to_string());
        output.push_str(&"] Error".to_owned());
        output.push_str(where_at);
        output.push_str(&": ".to_owned());
        output.push_str(message);
        self.tokens.push(Err(output));
    }
}
