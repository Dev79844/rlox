use std::{env, process::exit, fs, io, io::Write};

mod token_type;
mod token;
mod scanner;
mod expr;
mod ast_printer;

use ast_printer::AstPrinter;
use expr::Expr;
use scanner::Scanner;
use token::{Literal, Token};
use token_type::TokenType;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(1);
    } else if args.len() == 2 {
        run_file(args[1].to_string());
    } else {
        run_prompt();
    }
}

fn run_file(path: String) {
    let contents = fs::read_to_string(path).expect("Error reading the file");
    run(contents);
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");
        let mut source = String::new();
        let bytes_read = io::stdin().read_line(&mut source).expect("Error reading the user input");
        if bytes_read == 0 {
            break;
        }
        run(source);
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in &tokens {
        println!("{}", token);
    }

    // No parser yet, so print a hardcoded AST for now — book's chapter 5 example.
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal(Some(Literal::Number(123.0)))),
        }),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal(Some(Literal::Number(45.67)))),
        }),
    };
    println!("{}", AstPrinter.print(&expr));
}

#[allow(dead_code)]
fn error(line: i32, message: String) {
    report(line, "".to_string(), message);
}

#[allow(dead_code)]
fn report(line: i32, location: String, message: String) {
    println!("[line {} ] Error {} :{}", line, location, message);
}
