
use std::{fmt::Display, io::{self, Write, stdin}, process};

use anyhow::{Context, Result};

use crate::builtins::Builtin;


pub fn get_user_input() -> Result<String> {
    let mut user_input = String::new();
    stdin().read_line(&mut user_input)
        .context("Reading User Input")
        .unwrap();
    Ok(user_input.trim().to_owned())
}

pub fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

pub fn print_error(message: impl Display) {
    eprintln!("{message}")
}

pub fn get_command() -> Result<Builtin> {
    let input = get_user_input().unwrap();
    let command = Builtin::from(input.as_str());
    Ok(command)
}

pub fn exit(code: i32) {
    process::exit(code)
}
