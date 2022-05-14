mod lexer;
mod parser;
mod token;
mod value;

use crate::lexer::Lexer;

pub type JsonResult<T> = Result<T, JsonError>;

pub fn tokenize() {
	let input = "aaa".to_string();
	let lexer = Lexer::new(input);
}
