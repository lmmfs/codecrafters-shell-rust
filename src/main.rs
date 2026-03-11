#[allow(unused_imports)]
use std::io::{self, Write};

use regex::Regex;
use std::env;

use std::os::unix::fs::PermissionsExt;

fn main() {

    let echo_re = Regex::new(r"^(echo)").unwrap();
    let type_re = Regex::new(r"^(type)").unwrap();


    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        match command.trim() {
            s if echo_re.is_match(s) => echo_command(s.trim_start_matches("echo")),
            s if type_re.is_match(s) => type_command(s.trim_start_matches("type")),
            "exit" => break,
            "help" => println!("Available commands: help, exit, type, echo"),
            _ => print!("{}: command not found\n", command.trim()),
        }
    }
}


fn echo_command(args: &str) {
    let s = args.strip_prefix(' ').unwrap_or(args);
    println!("{}", s);
}

fn find_in_path(command: &str) -> Option<std::path::PathBuf> {
    env::var("PATH").ok()?.split(':')
        .filter_map(|dir| {
            let path = std::path::Path::new(dir).join(command);
           std::fs::metadata(&path)
           .ok()
            .filter(|meta| meta.permissions().mode() & 0o111 != 0)
            .map(|_| path)
        })
        .next()
}

fn type_command(args: &str) {
    let s = args.strip_prefix(' ').unwrap_or(args);
    match s {
        "exit" | "help" | "echo" | "type" => println!("{} is a shell builtin", s),
        _ => match find_in_path(s) {
            Some(path) => println!("{} is {}", s, path.display()),
            None => println!("{}: not found", s),
        }
    }

    
    
}
