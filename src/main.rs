#[allow(unused_imports)]
use std::env;
use std::{io::{self, Write}, path::PathBuf};

fn get_current_directory() -> std::io::Result<PathBuf> {
     env::current_dir()
}

fn main() {
    loop {
        let working_dir = get_current_directory().unwrap();

        print!("~ {}>", working_dir.display());
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