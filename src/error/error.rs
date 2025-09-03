pub enum ErrorType {
    UnknownCharacterError,
    SyntaxError,
    CompilerError,
}

#[allow(dead_code)]
pub struct Error {
    pub r#type: ErrorType,
    pub message: string,
}

impl Error {
    pub fn call(self) {
        println!(self::r#type + ": " + self::message);
    }
}