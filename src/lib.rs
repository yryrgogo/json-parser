mod lexer;
mod parser;
mod token;

use crate::lexer::Lexer;

pub fn tokenize() {
	let input = "aaa".to_string();
	let lexer = Lexer::new(input);
}
