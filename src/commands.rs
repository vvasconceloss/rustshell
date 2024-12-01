pub enum InternalCommand {
  Cd(String),
  Exit,
}

impl InternalCommand {
  pub fn parse_command(input: &str) -> Option<Self> {
      let args: Vec<&str> = input.split_whitespace().collect();
      match args.get(0) {
          Some(&"cd") => Some(InternalCommand::Cd(args.get(1).cloned().unwrap_or_default().to_string())),
          Some(&"exit") => Some(InternalCommand::Exit),
          _ => None,
      }
  }
}