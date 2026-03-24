#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::Result;

use crate::{custom_errors::CustomError, utils::{get_user_input, print_error, prompt}};

mod utils;
mod custom_errors;

pub fn run() -> Result<()>{
    prompt();
    let input = get_user_input().unwrap();
    //println!("{}", input);

    //let message = format!("{input}: command not found");
    let error = CustomError::CommandNotFound(input);
    print_error(error);

    Ok(())
}
