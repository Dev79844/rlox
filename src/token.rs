use crate::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Nil,
    Bool(bool),
    Number(f64),
    String(String)
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self{
            token_type,
            lexeme,
            line,
            literal
        }
    }
}