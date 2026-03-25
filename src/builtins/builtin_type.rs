use crate::{builtins::{Builtin, CommandArguments, echo::echo}, utils::find_in_path};


pub fn builtin_type(arguments: CommandArguments) {
    let command_input = arguments.first().cloned().unwrap_or_default();
    let _command = Builtin::from(command_input.clone());
    let mut message: Vec<String> = vec![];

    message.push(command_input.clone());

    if !matches!(_command, Builtin::NotFound(_)){
        message.push(format!("is a shell builtin"));
        echo(&message);
        return;

    }

    match find_in_path(command_input.as_str()) {
        Some(path) => {
            message.push(format!(" is {}", path.display()));
        }
            ,
        None => message.push(format!(": not found")),
    }


    let message = message.join("");
    echo(&[message]);

}




