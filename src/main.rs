#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        match command.trim() {
            "exit" => break,
            "help" => println!("Available commands: help, exit"),
            _ => print!("{}: command not found\n", command.trim()),
        }
    }
}
