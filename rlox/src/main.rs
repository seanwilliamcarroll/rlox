use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

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
        let num_bytes = io::stdin()
            .read_line(&mut line)
            .expect("Error reading user input line");
        run(&line);
    }
}

struct Scanner {
    original_text: String,
}

impl Scanner {
    fn new(text: &String) -> Scanner {
        Scanner {
            original_text: text.clone(),
        }
    }

    fn scan_tokens(&self) -> Vec<Token> {
        vec![]
    }
}

#[derive(Debug)]
enum Token {
    LeftParen,
    RightParen,
}

fn run(input: &String) {
    let scanner: Scanner = Scanner::new(input);

    let mut tokens: Vec<Token> = scanner.scan_tokens();
    for token in &tokens {
        println!("{:?}", token);
    }
}
