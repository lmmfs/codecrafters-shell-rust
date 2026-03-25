pub use std::process::exit;
use std::{env, fmt::Display, io::{self, Write, stdin}, os::unix::fs::PermissionsExt};

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
    let mut user_input_iter = input.split_whitespace();
    let command = user_input_iter.next().unwrap_or(" ");
    let arguments: Vec<String> = user_input_iter.map(|s| s.to_string()).collect();
    let command = Builtin::from((command.to_owned(), arguments));
    Ok(command)
}


pub fn find_in_path(command: &str) -> Option<std::path::PathBuf> {
    env::var("PATH").ok()?.split(':')
    .filter_map(|dir| {
        let path = std::path::Path::new(dir).join(command);
        std::fs::metadata(&path)
        .ok()
        // Check if path as permissions and is executable
        .filter(|meta| meta.permissions().mode() & 0o111 != 0)
        .map(|_| path)
    })
    .next()
}
