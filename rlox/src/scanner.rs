use crate::token::Token;
use crate::token::TokenType;
use crate::util;

pub struct Scanner {
    original_text: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
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

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new_simple_token(
            TokenType::EOF,
            &"".to_owned(),
            self.line,
        ));
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
            '\n' => self.line += 1,
            '\r' => (),
            ' ' => (),
            '\t' => (),
            _ => {
                let mut message: String = "Unexpected character '".to_owned();
                message.push(next_char);
                message.push('\'');
                util::error(self.line, &message)
            }
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

    fn advance(&mut self) -> char {
        let out_char = self.current_char();
        self.current += 1;
        out_char
    }

    fn add_simple_token(&mut self, token_type: TokenType) {
        let text = self
            .original_text
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens
            .push(Token::new_simple_token(token_type, &text, self.line));
    }
}
