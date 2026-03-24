#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::Result;

use crate::utils::{get_user_input, print_error, prompt};

mod utils;

pub fn run() -> Result<()>{
    prompt();
    let input = get_user_input().unwrap();
    //println!("{}", input);

    let message = format!("{input}: command not found");
    print_error(message);

    Ok(())
}
