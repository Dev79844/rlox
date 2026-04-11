use crate::{token::{Literal, Token}, token_type::TokenType};

pub struct Scanner {
    tokens: Vec<Token>,
    source: String,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source, tokens: Vec::new(), start: 0, current: 0, line: 1 }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, String::from(""), None, self.line));
        self.tokens
    }

    fn scan_token(&self) {
        let c = &self.advance();

        match c {

        }
    }

    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).expect("error getting the character");
        &self.current + 1;

        ch
    }

    fn add_token(&self, token_type: TokenType) {
        &self._add_token(token_type, None);
    }

    fn _add_token(&self, token_type: TokenType, literal: Option<Literal>) {}

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}