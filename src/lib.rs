#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::{Context, Result};

use crate::{
    builtins::{builtin::Builtin, echo::echo},
    custom_errors::CustomError,
    utils::{get_command, print_error, prompt}};

pub mod utils;
mod custom_errors;
mod builtins;

pub fn run() -> Result<()>{
    loop {
        prompt();
        let command = get_command().context("Getting Command").unwrap();
        match  command{
            Builtin::Echo(arguments) => echo(arguments),
            Builtin::Exit => break,
            Builtin::NotFound(command_str) => {
                let error = CustomError::CommandNotFound(command_str);
                print_error(error);
            }
        }
    }
    Ok(())
}



