#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::{Context, Result};

use crate::{builtins::Builtin, custom_errors::CustomError, utils::{exit, get_command, print_error, prompt}};

pub mod utils;
mod custom_errors;
mod builtins;

pub fn run() -> Result<()>{
    loop {
        prompt();
        let command = get_command().context("Getting Command").unwrap();
        match  command{
            Builtin::Exit => exit(0),
            Builtin::NotFound(command_str) => {
                let error = CustomError::CommandNotFound(command_str);
                print_error(error);
            }
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}
