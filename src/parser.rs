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
        let mut is_num = true;
        let mut signed = false;
        let mut is_float = false;

        for (i, c) in token.literal.chars().enumerate() {
            match c {
                '-' => {
                    if i == 0 {
                        signed = true;
                        continue;
                    }
                    is_num = false;
                    break;
                }
                '.' => {
                    is_float = true;
                }
                _ => {
                    println!("{} {:b}", u32::from(c), u32::from(c));
                    let c_u32 = u32::from(c);
                    if c_u32 >= 48 && c_u32 <= 57 {
                        continue;
                    } else {
                        is_num = false;
                        break;
                    }
                }
            }
        }

        if is_num {
            if signed {
                if is_float {
                    let result = token.literal.parse::<f64>().unwrap_or_else(|_| {
                        panic!("String を float64 に parse できませんでした。")
                    });
                    return;
                } else {
                    let result = token
                        .literal
                        .parse::<i64>()
                        .unwrap_or_else(|_| panic!("String を int64 に parse できませんでした。"));
                    return;
                }
            } else {
                if is_float {
                    let result = token.literal.parse::<f64>().unwrap_or_else(|_| {
                        panic!("String を float64 に parse できませんでした。")
                    });
                    return;
                } else {
                    let result = token
                        .literal
                        .parse::<u64>()
                        .unwrap_or_else(|_| panic!("String を uint64 に parse できませんでした。"));
                    return;
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
        let test_tokens: Vec<Token> = vec![
            Token::new("0".to_string(), TokenType::VALUE),
            Token::new("-1".to_string(), TokenType::VALUE),
            Token::new("2.345".to_string(), TokenType::VALUE),
            Token::new("678.90123456789".to_string(), TokenType::VALUE),
            Token::new("-0.123456789".to_string(), TokenType::VALUE),
        ];

        let parser = Parser::new(input);

        for token in test_tokens {
            println!("aaaaaaaa");
            parser.parse_value(token);
        }
    }
}
