use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Parser {
    lexer: Lexer,
    keys: Vec<Token>,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let mut lexer = Lexer::new(input);
        Parser {
            lexer: lexer,
            keys: vec![],
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.lexer.next_token() {
            match token.token_type {
                TokenType::LBRACE => self.parse_pair(),
                TokenType::COMMA => {
                    // FIXME:
                    self.parse_pair();
                }
                TokenType::VALUE => {
                    self.parse_value(token);
                    // HashMap::new();
                }
                TokenType::RBRACE => {
                    // FIXME:
                }
                _ => {}
            };
        }
    }

    fn parse_pair(&mut self) {
        let key_token = self
            .lexer
            .next_token()
            .unwrap_or_else(|| panic!("{:#?} に続く値がありません", TokenType::LBRACE));

        assert!(key_token.token_type == TokenType::VALUE);

        let colon_token = self
            .lexer
            .next_token()
            .unwrap_or_else(|| panic!("{:#?} に続く値がありません", key_token));

        assert!(colon_token.token_type == TokenType::COLON);

        self.keys.push(key_token);
    }

    fn parse_value(&self, token: Token) {
        let mut signed = false;
        let mut is_float = false;

        for (i, c) in token.literal.chars().enumerate() {
            match c {
                '-' => {
                    if i == 0 {
                        signed = true;
                        continue;
                    }
                    break;
                }
                '.' => {
                    is_float = true;
                }
                _ => {
                    println!("{} {:b}", u32::from(c), u32::from(c));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        let input = "".to_string();

        let mut lexer = Lexer::new(input);
        // Parser::new()
    }
}
