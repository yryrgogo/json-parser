pub struct Token {
	pub literal: String,
	pub token_type: TokenType,
}

impl Token {
	pub fn new(literal: String, token_type: TokenType) -> Token {
		Token {
			literal: literal,
			token_type: token_type,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
	// Block
	LBRACE,
	RBRACE,

	// Delimiter
	COMMA,
	COLON,

	// Value
	VALUE,
}
