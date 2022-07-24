use crate::ast_printer::AstPrinter;
use crate::expr::{Expr, LiteralType};
use crate::scanner::Scanner;
use crate::token::Token;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;
pub mod ast_printer;
pub mod expr;
pub mod scanner;
pub mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(1);
    } else if args.len() == 2 {
        let script_path: &String = &args[1];
        run_file(script_path);
        println!("Ran: {}", script_path);
    } else {
        run_prompt();
    }
}

fn run_file(script_path: &String) {
    println!("To run: {}", script_path);
    let file_text: String = fs::read_to_string(script_path).expect("Error reading script");
    run(&file_text);
}

fn run_prompt() {
    loop {
        let mut line = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        let _num_bytes = io::stdin()
            .read_line(&mut line)
            .expect("Error reading user input line");
        run(&line);
    }
}

fn run(input: &String) {
    let mut scanner: Scanner = Scanner::new(input);

    let token_results: &Vec<Result<Token, String>> = scanner.scan_tokens();
    for token_result in token_results {
        match token_result {
            Ok(token) => println!("{:?}", token),
            Err(message) => println!("Error: {}", message),
        }
    }

    let my_ast_printer = AstPrinter;
    my_ast_printer.print(Expr::Literal(LiteralType::Number(0.0)));
}
