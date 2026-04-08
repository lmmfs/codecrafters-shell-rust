pub use std::process::exit;
use std::{env::{self, split_paths}, fmt::Display, fs::DirEntry, io::{self, Write, stdin}, os::unix::{fs::PermissionsExt, process::CommandExt}, path::PathBuf, process::Command};

use anyhow::{Context, Result, bail};

use crate::{builtins::Builtin, custom_errors::CustomError};


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

pub fn get_path() -> Result<Vec<PathBuf>> {
    let path = env::var("PATH").context("Getting Path").unwrap();
    let split_paths = split_paths(&path).map(|path | {
        if path.is_file() {
            bail!("PATH variable is a file")
        } else {
            Ok(path)
        }
    } );

    split_paths.collect()
}


pub fn find_in_path(name: &str, paths: &[PathBuf]) ->Option<DirEntry> {
    for path in paths {
        let Ok(directory) = std::fs::read_dir(path) else {
            continue;
        };

        for entry in directory {
            let Ok(entry) = entry else {
                continue;
            };

            if entry.file_name().to_str().unwrap_or_default() != name {
                continue;
            }

            let Ok(metadada) = entry.metadata() else {
                continue;
            };

            if metadada.permissions().mode() & 0o111 == 0 {
                continue;
            }

            return Some(entry);
        }
    }
    None
}


pub fn run_external_command(command: &str, command_name: &str, args: &[String]) {
    let mut cmd = Command::new(command);
    cmd.arg0(command_name);
    cmd.args(args);
    match cmd.status() {
        Ok(status) => {
            if !status.success() {
                let error = CustomError::CommandInvalidExitStatus(status.to_string());
                print_error(error);
            }
        }
        Err(e) => {
            let error = CustomError::CommandExecutionFailed(e.to_string());
            print_error(error);

        },
    }
}


