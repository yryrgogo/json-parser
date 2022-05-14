use crate::token::{Token, TokenType};

pub struct Lexer {
	input: Vec<char>,
	position: usize,
}

impl Lexer {
	pub fn new(input: String) -> Lexer {
		let chars = input.chars().collect::<Vec<char>>();
		Lexer {
			input: chars,
			position: 0,
		}
	}

	pub fn next_token(&mut self) -> Option<Token> {
		if let Some(ch) = self.next_char() {
			let token = match ch {
				'{' => Token::new(ch.to_string(), TokenType::LBRACE),
				'}' => Token::new(ch.to_string(), TokenType::RBRACE),
				':' => Token::new(ch.to_string(), TokenType::COLON),
				',' => Token::new(ch.to_string(), TokenType::COMMA),
				'"' => {
					let mut values: Vec<char> = vec![];
					loop {
						self.position += 1;
						if let Some(value_char) = self.next_char() {
							if value_char == '"' {
								break;
							} else {
								values.push(value_char);
							}
						} else {
							panic!(r#"`"`が閉じられていません。"#)
						}
					}
					let value: String = values.iter().collect();
					Token::new(value, TokenType::VALUE)
				}
				_ => todo!("{}", ch),
			};
			self.position += 1;
			Some(token)
		} else {
			println!("end");
			None
		}
	}

	pub fn next_char(&self) -> Option<char> {
		Some(*self.input.get(self.position).unwrap())
	}

	pub fn peek_char(&self) -> Option<&char> {
		self.input.get(self.position + 1)
	}
}

struct TestInput {
	input: String,
	expected_tokens: Vec<Token>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_next_token() {
		let tests: Vec<TestInput> = vec![TestInput {
			input: String::from(r#"{"key":"0"}"#),
			expected_tokens: vec![
				Token::new(String::from("{"), TokenType::LBRACE),
				Token::new(String::from("key"), TokenType::VALUE),
				Token::new(String::from(":"), TokenType::COLON),
				Token::new(String::from("0"), TokenType::VALUE),
				Token::new(String::from("}"), TokenType::RBRACE),
			],
		}];

		for test in tests {
			let mut lexer = Lexer::new(test.input);
			for expected_token in test.expected_tokens {
				if let Some(token) = lexer.next_token() {
					assert_eq!(token.literal, expected_token.literal);
					assert_eq!(token.token_type, expected_token.token_type);
				}
			}
		}
	}
}
