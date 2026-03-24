pub enum Builtin{
    Exit,
    NotFound(String)
}


impl From<&str> for Builtin {
    fn from(user_input: &str) -> Self {
        match user_input {
            "exit" => Self::Exit,
            _ => Self::NotFound(user_input.to_owned()),
        }
    }
}
