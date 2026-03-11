#[allow(unused_imports)]
use std::io::{self, Write};

use regex::Regex;

fn main() {

    let echo_re = Regex::new(r"^(echo)").unwrap();


    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        match command.trim() {
            s if echo_re.is_match(s) => echo_command(s.trim_start_matches("echo")),
            "exit" => break,
            "help" => println!("Available commands: help, exit"),
            _ => print!("{}: command not found\n", command.trim()),
        }
    }
}


fn echo_command(args: &str) {
    let s = args.strip_prefix(' ').unwrap_or(args);
    println!("{}", s);
}
