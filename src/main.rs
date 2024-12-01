#[allow(unused_imports)]
use std::env;
use std::{io::{self, Write}, path::PathBuf};

enum InternalCommand {
    Exit,
    Cd(String),
}

impl InternalCommand {
    fn parse_command(input: &str) -> Option<Self> {
        let args: Vec<&str> = input.split_whitespace().collect();
        match args.get(0) {
            Some(&"cd") => Some(InternalCommand::Cd(args.get(1).cloned().unwrap_or_default().to_string())),
            Some(&"exit") => Some(InternalCommand::Exit),
            _ => None,
        }
    }
}

struct Shell {
    history: Vec<String>,
}

impl Shell {
    fn new() -> Self {
        Shell {
            history: Vec::new(),
        }
    }

    fn execute_internal(&mut self, command: InternalCommand) {
        match command {
            InternalCommand::Cd(path) => {
                if path.is_empty() {
                    eprintln!("error: this path was not recognized");
                } else if let Err(e) = env::set_current_dir(path) {
                    eprintln!("fails to change directory: {}", e);
                }
            }
            InternalCommand::Exit => {
                println!("Exiting...");
                std::process::exit(0);
            }
        }
    }
}

fn get_current_directory() -> std::io::Result<PathBuf> {
     env::current_dir()
}

fn main() {
    loop {
        let working_dir = get_current_directory().unwrap();

        print!("~ {}> ", working_dir.display());
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut user_input = String::new();
        stdin.read_line(&mut user_input).unwrap();

        let input_user = user_input.trim().to_lowercase().to_string();
    }
}