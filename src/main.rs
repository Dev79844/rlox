use std::{env, process::exit, fs, io, io::Write};

mod token_type;
mod token;
mod scanner;

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

fn run_file(path: String) -> String {
    let contents = fs::read_to_string(path).expect("Error reading the file");
    contents
}

fn run_prompt(){
    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");
        let mut source = String::new();
        io::stdin().read_line(&mut source).expect("Error reading the user input");
        run(source);
    }
}

fn run(source: String) {
    println!("{}", source);
}

fn error(line: i32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: i32, location: String, message: String) {
    println!("[line {} ] Error {} :{}", line, location, message);
}