
pub mod echo;
pub mod builtin_type;


pub type CommandArguments = Vec<String>;

pub enum Builtin{
    Echo(CommandArguments),
    Exit,
    Type(CommandArguments),
    NotFound(String),
}


impl From<(String, CommandArguments)> for Builtin {
    fn from((command, arguments): (String, CommandArguments)) -> Self {
        match command.as_str() {
            "exit" => Self::Exit,
            "echo" => Self::Echo(arguments),
            "type" => Self::Type(arguments),
            _ => Self::NotFound(command.to_owned()),
        }
    }
}

impl From<String> for Builtin {
    fn from(command: String) -> Self {
        let arguments = vec![];
        Self::from((command, arguments))
    }

}
