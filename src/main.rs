#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut user_input = String::new();
        stdin.read_line(&mut user_input).unwrap();

        match user_input.trim().to_lowercase().as_str() {
            "" => continue,
            "exit" => break,
            "exit 0" => break,
            _ => println!("{}: command not found", user_input.trim()),
        }   
    }
}