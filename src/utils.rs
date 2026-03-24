
use std::{fmt::Display, io::{self, Write, stdin}};

use anyhow::{Context, Result};

pub fn get_user_input() -> Result<String> {
    let mut user_input = String::new();
    stdin().read_line(&mut user_input)
        .context("Reading User Input")
        .unwrap();
    Ok(user_input.trim_end().to_string())
}

pub fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

pub fn print_error(message: impl Display) {
    eprintln!("{message}")
}
