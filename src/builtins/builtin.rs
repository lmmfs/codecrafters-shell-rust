pub enum Builtin{
    Echo(Vec<String>),
    Exit,
    NotFound(String)
}


impl From<(String, Vec<String>)> for Builtin {
    fn from((command, arguments): (String, Vec<String>)) -> Self {
        match command.as_str() {
            "exit" => Self::Exit,
            "echo" => Self::Echo(arguments),
            _ => Self::NotFound(command.to_owned()),
        }
    }
}
