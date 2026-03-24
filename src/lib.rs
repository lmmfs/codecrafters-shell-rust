#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::Result;

use crate::{custom_errors::CustomError, utils::{get_user_input, print_error, prompt}};

mod utils;
mod custom_errors;

pub fn run() -> Result<()>{
    loop {
        prompt();
        let input = get_user_input().unwrap();
        let error = CustomError::CommandNotFound(input);
        print_error(error);
    }
    #[allow(unreachable_code)]
    Ok(())
}
