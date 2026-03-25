use crate::builtins::{Builtin, CommandArguments, echo::echo};



pub fn builtin_type(arguments: CommandArguments) {
    let command_input = arguments.first().cloned().unwrap_or_default();
    let _command = Builtin::from(command_input.clone());
    let mut message = String::new();
    message.push_str(&command_input);

    if matches!(_command, Builtin::NotFound(_)){
        message.push_str(": not found");
    } else {
        message.push_str(" is a shell builtin");
    }

    echo(vec![message]);

}
