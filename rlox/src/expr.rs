// Generated using ast_generator. DO NOT EDIT
use crate::token::Token;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal(Option<crate::token::Literal>),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
