use hug_lib::value::HugValue;

use crate::tokenizer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct TokenPair<'a> {
    pub text: &'a str,
    pub token: Token<'a>,
}

impl<'a> TokenPair<'a> {
    pub const fn null() -> Self {
        Self {
            text: "",
            token: Token {
                len: 0,
                kind: TokenKind::Unknown,
            },
        }
    }

    pub fn parse_literal(&self) -> Option<HugValue> {
        if let Some(_) = self.token.kind.expect_literal() {
            if let Ok(int) = self.text.parse::<i32>() {
                Some(HugValue::from(int))
            } else if let Ok(float) = self.text.parse::<f32>() {
                Some(HugValue::from(float))
            } else if self.text.len() > 2 {
                Some(HugValue::from(String::from(
                    &self.text[1..self.text.len() - 1],
                )))
            } else {
                None
            }
        } else {
            None
        }
    }
}

// pub fn generate_pairs<'a>(program: &'a str, tokens: Vec<Token>) -> Vec<TokenPair<'a>> {
//     let mut pairs = Vec::new();

//     let mut chars = program.chars();
//     for token in tokens {
//         let mut buffer = String::new();
//         for _i in 0..token.len {
//             buffer.push(chars.next().unwrap());
//         }

//         pairs.push(TokenPair {
//             text: buffer,
//             token,
//         })
//     }

//     pairs
// }
