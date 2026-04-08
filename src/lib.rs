#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::{Context, Result};

use crate::{
    builtins::{Builtin, builtin_type::builtin_type, echo::echo},
    custom_errors::CustomError,
    utils::{find_in_path, get_command, get_path, print_error, prompt, run_external_command}};

pub mod utils;
mod custom_errors;
pub mod builtins;

pub fn run() -> Result<()>{

    let path = get_path().context("Retriving Path").unwrap();

    loop {
        prompt();
        let command = get_command().context("Getting Command").unwrap();
        match command {
            Builtin::Echo(arguments) => echo(&arguments),
            Builtin::Exit => break,
            Builtin::Type(arguments) => builtin_type(arguments, &path),
            Builtin::NotFound(command_str, arguments) => {
                match find_in_path(command_str.as_str(), &path) {
                    Some(entry) => {
                        let path_buf = entry.path();
                        let path = path_buf.into_os_string().into_string().unwrap_or("unknown path".to_owned());
                        run_external_command(path.as_str(), arguments);
                    },
                    None => {
                        let error = CustomError::CommandNotFound(command_str);
                        print_error(error);
                    }
                }
            }
        }
    }
    Ok(())
}



