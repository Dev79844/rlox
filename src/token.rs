use std::fmt;
use crate::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = match &self.literal {
            Some(Literal::String(s)) => s.clone(),
            Some(Literal::Number(n)) => n.to_string(),
            Some(Literal::Bool(b))   => b.to_string(),
            Some(Literal::Nil)       => "nil".to_string(),
            None                     => "null".to_string(),
        };
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal)
    }
}