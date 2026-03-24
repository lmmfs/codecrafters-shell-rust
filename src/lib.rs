#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::Result;

use crate::{builtins::Builtin, custom_errors::CustomError, utils::{get_user_input, print_error, prompt}};

mod utils;
mod custom_errors;
mod builtins;

pub fn run() -> Result<()>{
    loop {
        prompt();
        let input = get_user_input().unwrap();
        let command = Builtin::from(input.as_str());
        match  command{
            Builtin::Exit => break,
            Builtin::NotFound(command_str) => {
                let error = CustomError::CommandNotFound(command_str);
                print_error(error);
            }
        }
    }
    Ok(())
}
