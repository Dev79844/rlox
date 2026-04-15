use crate::{token::{Literal, Token}, token_type::TokenType};

pub struct Scanner {
    tokens: Vec<Token>,
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, String::from(""), None, self.line));
        std::mem::take(&mut self.tokens)
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let tt = if self.match_char('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(tt);
            }
            '=' => {
                let tt = if self.match_char('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(tt);
            }
            '<' => {
                let tt = if self.match_char('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(tt);
            }
            '>' => {
                let tt = if self.match_char('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(tt);
            }
            '/' => {
                if self.match_char('/') {
                    // Line comment — consume until end of line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_alphabetic() || c == '_' => self.identifier(),
            _ => eprintln!("[line {}] Error: Unexpected character '{}'.", self.line, c),
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }

    // Consume current char only if it matches `expected`
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    // Look at current char without consuming it
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    // Look one char ahead without consuming
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            return;
        }

        self.advance(); // closing "

        // Trim surrounding quotes to get the value
        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token_with_literal(TokenType::StringLit, Some(Literal::String(value)));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume '.'
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = lexeme.parse().unwrap();
        self.add_token_with_literal(TokenType::Number, Some(Literal::Number(value)));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = Self::keyword(&text).unwrap_or(TokenType::Identifier);
        self.add_token(token_type);
    }

    fn keyword(text: &str) -> Option<TokenType> {
        match text {
            "and"    => Some(TokenType::And),
            "class"  => Some(TokenType::Class),
            "else"   => Some(TokenType::Else),
            "false"  => Some(TokenType::False),
            "for"    => Some(TokenType::For),
            "fun"    => Some(TokenType::Fun),
            "if"     => Some(TokenType::If),
            "nil"    => Some(TokenType::Nil),
            "or"     => Some(TokenType::Or),
            "print"  => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super"  => Some(TokenType::Super),
            "this"   => Some(TokenType::This),
            "true"   => Some(TokenType::True),
            "var"    => Some(TokenType::Var),
            "while"  => Some(TokenType::While),
            _        => None,
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Literal;
    use crate::token_type::TokenType;

    // Helper: scan source and return all tokens except the trailing EOF.
    fn scan(source: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(source.to_string());
        let mut tokens = scanner.scan_tokens();
        tokens.pop(); // remove EOF
        tokens
    }

    // Helper: scan source and return every token including EOF.
    fn scan_with_eof(source: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(source.to_string());
        scanner.scan_tokens()
    }

    fn types(tokens: &[Token]) -> Vec<&TokenType> {
        tokens.iter().map(|t| &t.token_type).collect()
    }

    // ── EOF ───────────────────────────────────────────────────────────────────

    #[test]
    fn empty_source_produces_only_eof() {
        let tokens = scan_with_eof("");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
        assert_eq!(tokens[0].lexeme, "");
        assert_eq!(tokens[0].line, 1);
    }

    // ── Single-character tokens ───────────────────────────────────────────────

    #[test]
    fn single_char_tokens() {
        let tokens = scan("(){},.-+;*");
        assert_eq!(
            types(&tokens),
            vec![
                &TokenType::LeftParen,
                &TokenType::RightParen,
                &TokenType::LeftBrace,
                &TokenType::RightBrace,
                &TokenType::Comma,
                &TokenType::Dot,
                &TokenType::Minus,
                &TokenType::Plus,
                &TokenType::Semicolon,
                &TokenType::Star,
            ]
        );
    }

    #[test]
    fn single_char_lexemes_are_preserved() {
        let tokens = scan("(");
        assert_eq!(tokens[0].lexeme, "(");
    }

    // ── One-or-two-character tokens ───────────────────────────────────────────

    #[test]
    fn bang_alone() {
        let tokens = scan("!");
        assert_eq!(tokens[0].token_type, TokenType::Bang);
        assert_eq!(tokens[0].lexeme, "!");
    }

    #[test]
    fn bang_equal() {
        let tokens = scan("!=");
        assert_eq!(tokens[0].token_type, TokenType::BangEqual);
        assert_eq!(tokens[0].lexeme, "!=");
    }

    #[test]
    fn equal_alone() {
        let tokens = scan("=");
        assert_eq!(tokens[0].token_type, TokenType::Equal);
    }

    #[test]
    fn equal_equal() {
        let tokens = scan("==");
        assert_eq!(tokens[0].token_type, TokenType::EqualEqual);
    }

    #[test]
    fn less_alone() {
        let tokens = scan("<");
        assert_eq!(tokens[0].token_type, TokenType::Less);
    }

    #[test]
    fn less_equal() {
        let tokens = scan("<=");
        assert_eq!(tokens[0].token_type, TokenType::LessEqual);
    }

    #[test]
    fn greater_alone() {
        let tokens = scan(">");
        assert_eq!(tokens[0].token_type, TokenType::Greater);
    }

    #[test]
    fn greater_equal() {
        let tokens = scan(">=");
        assert_eq!(tokens[0].token_type, TokenType::GreaterEqual);
    }

    // ── Slash and comments ────────────────────────────────────────────────────

    #[test]
    fn slash_produces_slash_token() {
        let tokens = scan("/");
        assert_eq!(tokens[0].token_type, TokenType::Slash);
    }

    #[test]
    fn line_comment_produces_no_tokens() {
        let tokens = scan("// this is a comment");
        assert!(tokens.is_empty());
    }

    #[test]
    fn line_comment_does_not_consume_next_line() {
        let tokens = scan("// comment\n+");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Plus);
    }

    #[test]
    fn slash_followed_by_non_slash_is_slash_token() {
        let tokens = scan("/+");
        assert_eq!(tokens[0].token_type, TokenType::Slash);
        assert_eq!(tokens[1].token_type, TokenType::Plus);
    }

    // ── Whitespace ────────────────────────────────────────────────────────────

    #[test]
    fn whitespace_is_ignored() {
        let tokens = scan("  \t\r  ");
        assert!(tokens.is_empty());
    }

    #[test]
    fn newline_increments_line_number() {
        let tokens = scan("\n+");
        assert_eq!(tokens[0].line, 2);
    }

    #[test]
    fn multiple_newlines_tracked() {
        let tokens = scan("\n\n\n+");
        assert_eq!(tokens[0].line, 4);
    }

    // ── String literals ───────────────────────────────────────────────────────

    #[test]
    fn simple_string() {
        let tokens = scan("\"hello\"");
        assert_eq!(tokens[0].token_type, TokenType::StringLit);
        assert_eq!(tokens[0].lexeme, "\"hello\"");
        assert_eq!(tokens[0].literal, Some(Literal::String("hello".to_string())));
    }

    #[test]
    fn empty_string() {
        let tokens = scan("\"\"");
        assert_eq!(tokens[0].token_type, TokenType::StringLit);
        assert_eq!(tokens[0].literal, Some(Literal::String("".to_string())));
    }

    #[test]
    fn multiline_string_increments_line() {
        let tokens = scan("\"line1\nline2\"");
        assert_eq!(tokens[0].token_type, TokenType::StringLit);
        assert_eq!(tokens[0].line, 2);
    }

    #[test]
    fn unterminated_string_produces_no_token() {
        // Should print an error to stderr but not panic or produce a token.
        let tokens = scan("\"unterminated");
        assert!(tokens.is_empty());
    }

    // ── Number literals ───────────────────────────────────────────────────────

    #[test]
    fn integer_number() {
        let tokens = scan("42");
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "42");
        assert_eq!(tokens[0].literal, Some(Literal::Number(42.0)));
    }

    #[test]
    fn decimal_number() {
        let tokens = scan("3.14");
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].literal, Some(Literal::Number(3.14)));
    }

    #[test]
    fn number_followed_by_dot_without_fraction() {
        // "1." — the dot has no digit after it, so it should be Number(1) then Dot.
        let tokens = scan("1.");
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].literal, Some(Literal::Number(1.0)));
        assert_eq!(tokens[1].token_type, TokenType::Dot);
    }

    #[test]
    fn zero() {
        let tokens = scan("0");
        assert_eq!(tokens[0].literal, Some(Literal::Number(0.0)));
    }

    // ── Identifiers ───────────────────────────────────────────────────────────

    #[test]
    fn simple_identifier() {
        let tokens = scan("foo");
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "foo");
    }

    #[test]
    fn identifier_with_underscore_and_digits() {
        let tokens = scan("my_var2");
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "my_var2");
    }

    // ── Keywords ──────────────────────────────────────────────────────────────

    #[test]
    fn all_keywords() {
        let cases = [
            ("and",    TokenType::And),
            ("class",  TokenType::Class),
            ("else",   TokenType::Else),
            ("false",  TokenType::False),
            ("for",    TokenType::For),
            ("fun",    TokenType::Fun),
            ("if",     TokenType::If),
            ("nil",    TokenType::Nil),
            ("or",     TokenType::Or),
            ("print",  TokenType::Print),
            ("return", TokenType::Return),
            ("super",  TokenType::Super),
            ("this",   TokenType::This),
            ("true",   TokenType::True),
            ("var",    TokenType::Var),
            ("while",  TokenType::While),
        ];
        for (src, expected_type) in cases {
            let tokens = scan(src);
            assert_eq!(tokens.len(), 1, "Expected one token for keyword '{src}'");
            assert_eq!(tokens[0].token_type, expected_type, "Wrong type for keyword '{src}'");
        }
    }

    #[test]
    fn keyword_prefix_is_identifier_not_keyword() {
        // "iffy" starts with "if" but is an identifier.
        let tokens = scan("iffy");
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
    }

    // ── Line numbers ──────────────────────────────────────────────────────────

    #[test]
    fn tokens_carry_correct_line_numbers() {
        let tokens = scan("var\n  foo\n  =\n  1;");
        //                  ^1   ^2     ^3   ^4
        let lines: Vec<usize> = tokens.iter().map(|t| t.line).collect();
        assert_eq!(lines, vec![1, 2, 3, 4, 4]);
    }

    // ── Mixed / expression-level ──────────────────────────────────────────────

    #[test]
    fn simple_expression() {
        let tokens = scan("1 + 2");
        assert_eq!(
            types(&tokens),
            vec![&TokenType::Number, &TokenType::Plus, &TokenType::Number]
        );
    }

    #[test]
    fn equality_expression() {
        let tokens = scan("a == b");
        assert_eq!(
            types(&tokens),
            vec![&TokenType::Identifier, &TokenType::EqualEqual, &TokenType::Identifier]
        );
    }

    #[test]
    fn print_statement() {
        let tokens = scan("print \"hello\";");
        assert_eq!(
            types(&tokens),
            vec![&TokenType::Print, &TokenType::StringLit, &TokenType::Semicolon]
        );
    }

    #[test]
    fn var_declaration() {
        let tokens = scan("var x = 10;");
        assert_eq!(
            types(&tokens),
            vec![
                &TokenType::Var,
                &TokenType::Identifier,
                &TokenType::Equal,
                &TokenType::Number,
                &TokenType::Semicolon,
            ]
        );
    }

    #[test]
    fn if_else_block() {
        let tokens = scan("if (x) { } else { }");
        assert_eq!(
            types(&tokens),
            vec![
                &TokenType::If,
                &TokenType::LeftParen,
                &TokenType::Identifier,
                &TokenType::RightParen,
                &TokenType::LeftBrace,
                &TokenType::RightBrace,
                &TokenType::Else,
                &TokenType::LeftBrace,
                &TokenType::RightBrace,
            ]
        );
    }
}
