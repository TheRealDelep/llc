use super::token::{Token, TokenValue};


pub struct TokenStream {
    pub tokens: Vec<Token>,
    current_index: usize
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream {
            tokens,
            current_index: 0
        }
    }

    pub fn get(&self, offset: usize) -> &Token {
        match self.tokens.get(self.current_index + offset) {
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

    pub fn move_index(&mut self, offset: usize) {
        self.current_index += offset;
    }

    pub fn can_read(&self, offset: usize) -> bool {
        (self.current_index + offset) < self.tokens.len() - 1
    }
}