#[allow(unused_imports)]
use std::io::{self, Write};

use std::env;

use std::os::unix::fs::PermissionsExt;

use std::process::Command;

const BUILTINS: &[&str] = &["help", "exit", "type", "echo", "pwd", "cd"];

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command_with_args = String::new();
        io::stdin().read_line(&mut command_with_args).unwrap();

        let input = command_with_args.trim();
        let (command, args) = input.split_once(' ').unwrap_or((input, ""));

        let args_vec = arg_parse(args);

        match command {
            "echo" => echo_command(args_vec),
            "type" => type_command(args),
            "pwd" => match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("Error getting current directory: {}", e),
            },
            "cd" => cd_command(args),
            "exit" => break,
            "help" => println!("Available built-in commands: {}", BUILTINS.join(", ")),
            _ => match find_in_path(command) {
                Some(_) => run_external_command(command, args_vec),
                None => print!("{}: command not found\n", command),
        }
        }
    }
}

fn arg_parse(args:&str) -> Vec<String> {
    let mut arg_vec: Vec<String> = Vec::new();
    let mut in_quotes = false;
    let mut in_double_quotes = false;
    let mut escape_next = false;
    let mut current_token = String::new();

    for c in args.chars() {
        
        match c {
            '\\' if !in_quotes && !escape_next => escape_next = true,
            '"' if !escape_next => {
                in_double_quotes = !in_double_quotes;
                escape_next = false;
            },
            '\'' if !in_double_quotes && !escape_next => {
                in_quotes = !in_quotes;
                escape_next = false;
            },
            ' ' if escape_next => {
                current_token.push(' ');
                escape_next = false;
            }
            ' ' if !in_quotes && !in_double_quotes => {
                if !current_token.is_empty() {
                    arg_vec.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {  
                current_token.push(c);
                escape_next = false;
            },
        }
    }

    if !current_token.is_empty() {
        arg_vec.push(current_token);
    }

    
    
    arg_vec
}


fn echo_command(args: Vec<String>) {
    println!("{}", args.join(" "));
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
    if BUILTINS.contains(&args) {
        println!("{} is a shell builtin", args);
        return;
    } 

    match find_in_path(args) {
        Some(path) => println!("{} is {}", args, path.display()),
        None => println!("{}: not found", args),
    }
}

fn cd_command(args: &str) {
    let target = if args.is_empty() || args == "~" {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        args.to_string()
    };

    if let Err(e) = env::set_current_dir(&target) {
        match e.kind() {
            std::io::ErrorKind::NotFound => eprintln!("cd: {}: No such file or directory", target),
            std::io::ErrorKind::PermissionDenied => eprintln!("cd: {}: Permission denied", target),
            _ => eprintln!("cd: {}: {}", target, e),
        }
    }
}

fn run_external_command(command: &str, args: Vec<String>) {
    match Command::new(command).args(&args).status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("Command exited with status {}", status);
            }
        }
        Err(e) => eprintln!("Failed to execute: {}", e),
    }
}