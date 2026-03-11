#[allow(unused_imports)]
use std::io::{self, Write};

use std::env;

use std::os::unix::fs::PermissionsExt;

use std::process::Command;

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command_with_args = String::new();
        io::stdin().read_line(&mut command_with_args).unwrap();

        let input = command_with_args.trim();
        let (command, args) = input.split_once(' ').unwrap_or((input, ""));

        match command {
            "echo" => echo_command(args),
            "type" => type_command(args),
            "exit" => break,
            "help" => println!("Available built-in commands: help, exit, type, echo"),
            _ => match find_in_path(command) {
                Some(path) => run_external_command(path, args),
                None => print!("{}: command not found\n", command),
        }
        }
    }
}


fn echo_command(args: &str) {
    println!("{}", args);
}

fn find_in_path(command: &str) -> Option<std::path::PathBuf> {
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

fn type_command(args: &str) {
    match args {
        "exit" | "help" | "echo" | "type" => println!("{} is a shell builtin", args),
        _ => match find_in_path(args) {
            Some(path) => println!("{} is {}", args, path.display()),
            None => println!("{}: not found", args),
        }
    }
}

fn run_external_command(path: std::path::PathBuf, args: &str) {
    let args_vec: Vec<&str> = args.split_whitespace().collect();
    match Command::new(path).args(&args_vec).status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("Command exited with status {}", status);
            }
        }
        Err(e) => eprintln!("Failed to execute: {}", e),
    }
}