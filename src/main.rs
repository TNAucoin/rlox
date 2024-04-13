use std::io::{Error, Write};
use std::{env, fs, io};
struct RLox {
    had_error: bool,
}
impl RLox {
    fn new() -> Self {
        RLox {
            had_error: false,
        }
    }
    fn run_prompt(&mut self) {
        let mut input = String::new();
        loop {
            print!("> ");
            io::stdout().flush().expect("flush failed");
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().is_empty() {
                break;
            }
            self.run(&input);
            self.had_error = false;
            input.clear();
        }
    }
    fn run_file(&self, path: &String) -> io::Result<()> {
        let contents = fs::read_to_string(path)?;
        self.run(&contents);
        if self.had_error {
            std::process::exit(64);
        }
        Ok(())
    }

    fn run(&self, source: &String) {
        let scanner = Scanner::new();
        let tokens = scanner.scan_tokens();
        println!("input: {}", source);
    }
}

fn main() {
    let mut rlox = RLox::new();
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-h".to_string()) || args.contains(&"help".to_string()) {
        print_help();
        return;
    }
    match args.len() {
        2 => rlox.run_file(&args[1]).unwrap_or_else(|e| println!("{}", e)),
        _ => rlox.run_prompt(),
    }
}
fn print_help() {
    println!("Usage: rlox [script]");
    println!("If a script is provided, rlox will run it.");
    println!("If no script is provided, rlox will enter interactive mode.");
}





struct Scanner {}

impl Scanner {
    fn new() -> Self {
        Scanner {}
    }
    fn scan_tokens(&self) {}
}
