

pub enum ErrorType {
    UnknownCharacterError,
    SyntaxError,
    CompilerError,
}

pub struct Error {
    pub r#type: ErrorType,
    pub message: string,
}

impl Error {
    fn call(self) {
        println!(self::r#type + ": " + self::message);
    }
}