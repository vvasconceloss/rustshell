mod shell;
mod commands;

use std::env;
use shell::Shell;
use std::{io::{self, Write}, path::PathBuf};

fn get_current_directory() -> std::io::Result<PathBuf> {
     env::current_dir()
}

fn main() {
    let mut shell = Shell::new();

    loop {
        let working_dir = get_current_directory().unwrap();

        print!("~ {}> ", working_dir.display());
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut user_input = String::new();
        stdin.read_line(&mut user_input).unwrap();

        let input_user = user_input.trim().to_lowercase().to_string();

        shell.execute_command(input_user);
    }
}