use crate::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize
}

pub enum Literal {
    Nil,
    Bool(bool),
    Number(usize),
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