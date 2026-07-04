use crate::expr::Expr;
use crate::token::Literal;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => {
                self.parenthesize(&operator.lexeme, &[left, right])
            }
            Expr::Grouping { expression } => {
                self.parenthesize("group", &[expression])
            }
            Expr::Literal(value) => match value {
                Some(Literal::Number(n)) => n.to_string(),
                Some(Literal::String(s)) => s.clone(),
                Some(Literal::Bool(b)) => b.to_string(),
                Some(Literal::Nil) | None => "nil".to_string(),
            },
            Expr::Unary { operator, right } => {
                self.parenthesize(&operator.lexeme, &[right])
            }
        }
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut s = String::from("(");
        s.push_str(name);
        for expr in exprs {
            s.push(' ');
            s.push_str(&self.print(expr));
        }
        s.push(')');
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::Expr;
    use crate::token::{Literal, Token};
    use crate::token_type::TokenType;

    fn token(token_type: TokenType, lexeme: &str) -> Token {
        Token::new(token_type, lexeme.to_string(), None, 1)
    }

    // (* (- 123) (group 45.67))  — book's example from chapter 5
    #[test]
    fn book_example() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: token(TokenType::Minus, "-"),
                right: Box::new(Expr::Literal(Some(Literal::Number(123.0)))),
            }),
            operator: token(TokenType::Star, "*"),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal(Some(Literal::Number(45.67)))),
            }),
        };

        assert_eq!(AstPrinter.print(&expr), "(* (- 123) (group 45.67))");
    }
}
