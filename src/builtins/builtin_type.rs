use std::path::PathBuf;

use crate::{builtins::{Builtin, CommandArguments, echo::echo}, utils::find_in_path};


pub fn builtin_type(arguments: CommandArguments, paths: &[PathBuf]) {
    let type_input = arguments.first().cloned().unwrap_or_default();
    let _command = Builtin::from(type_input.clone());
    let mut message: Vec<String> = vec![];

    message.push(type_input.clone());

    if !matches!(_command, Builtin::NotFound(_)){
        message.push(format!("is a shell builtin"));
        echo(&message);
        return;
    }

    match find_in_path(type_input.as_str(), paths) {
        Some(entry) => {
            let path_buf = entry.path();
            let path = path_buf.into_os_string().into_string().unwrap_or("unknown path".to_owned());
            message.push(format!(" is {}", path));
        },
        None => message.push(format!(": not found"))
    }

    let message = message.join("");
    echo(&[message]);
}
