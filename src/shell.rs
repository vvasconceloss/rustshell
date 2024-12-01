use std::env;
use crate::commands::InternalCommand;

pub struct Shell {
  history: Vec<String>,
}

impl Shell {
  pub fn new() -> Self {
      Shell {
          history: Vec::new(),
      }
  }

  pub fn execute_internal(&mut self, command: InternalCommand) {
      match command {
          InternalCommand::Cd(path) => {
              if path.is_empty() {
                  eprintln!("error: this path was not recognized");
              } else if let Err(e) = env::set_current_dir(path) {
                  eprintln!("fails to change directory: {}", e);
              }
          }
          InternalCommand::Exit => {
              std::process::exit(0);
          }
      }
  }

  pub fn execute_command(&mut self, input: String) {
      self.history.push(input.clone());

      if let Some(internal_command) = InternalCommand::parse_command(&input) {
          self.execute_internal(internal_command);
      } else {
          let args: Vec<&str> = input.split_whitespace().collect();

          match std::process::Command::new(args[0]).args(&args[1..]).spawn() {
              Ok(mut child) => {
                  child.wait().unwrap();
              }
              Err(e) => {
                  eprintln!("error when executing this command: {}", e);
              } 
          }
      }
  }
}