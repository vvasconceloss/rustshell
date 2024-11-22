#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();
}