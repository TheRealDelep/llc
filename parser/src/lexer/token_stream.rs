use llc_core::models::token::{Token, TokenValue};

use crate::parser::errors::{CompileError, SyntaxError};

pub struct TokenStream<'a> {
    pub tokens: Vec<Token<'a>>,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        TokenStream {
            tokens,
        }
    }

    pub fn get(&self, index: usize) -> &Token {
        match self.tokens.get(index) {
            Some(t) => t,
            None => {
                let eof = match self.tokens.last() {
                    Some(token) => match token.value {
                        TokenValue::EOF => token,
                        _ => panic!("Last token should be EOF")
                    },
                    None => panic!("Token stream is empty")
                };

                eof
            }
        }
    }

    pub fn can_read(&self, index: &usize) -> bool {
        *index < self.tokens.len() - 1
    }
}
