use crate::token::Token;

#[derive(Debug)]
pub enum JsonError {
    UnexpectedToken(String),
    UnexpectedCharacter(char),
    UnexpectedEndOfJson,
    CantCastCodePointToCharacter(u32),
    ArrayIndexOutOfBounds,
    WrongType(String),
    UndefinedField(String),
}

impl JsonError {
    pub fn unexpected_toke(token: Token) -> Self {
        JsonError::UnexpectedToken(format!("{:?}", token))
    }
}
