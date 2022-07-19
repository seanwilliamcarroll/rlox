use crate::token::Token;
use crate::token::TokenType;

pub struct Scanner {
    original_text: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
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
            '\n' => self.line += 1,
            '\r' => (),
            ' ' => (),
            '\t' => (),
            _ => panic!("Unexpected character '{}'", next_char),
        }
    }

    fn advance(&mut self) -> char {
        let out_char = self.original_text.chars().nth(self.current).unwrap();
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
