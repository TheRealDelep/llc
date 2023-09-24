use super::token::{Token, TokenKind};

pub struct TokenStream {
    pub tokens: Vec<Token>,
    current_index: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream {
            tokens,
            current_index: 0,
        }
    }

    pub fn can_read(&self) -> bool {
        (self.current_index) < self.tokens.len() - 1
    }

    pub fn peek(&self, offset: isize) -> &Token {
        let index = (self.current_index as isize) + offset;
        match self.tokens.get(index as usize) {
            Some(t) => t,
            None => {
                let eof = match self.tokens.last() {
                    Some(token) => match token.value {
                        TokenKind::EOF => token,
                        _ => panic!("Last token should be EOF"),
                    },
                    None => panic!("Token stream is empty"),
                };

                eof
            }
        }
    }

    // take: consumes token anyway
    pub fn take(&mut self) -> &Token {
        self.current_index += 1;
        self.peek(-1)
    }

    // try_take: if match returns token and move cursor forward, else keeps cursor at position
    pub fn take_if<T>(&mut self, compare: impl Fn(&Token) -> Option<T>) -> Option<T> {
        let token = self.peek(0);
        match compare(token) {
            Some(val) => {
                self.current_index += 1;
                return Some(val);
            }
            None => None,
        }
    }

    pub fn skip(&mut self, count: usize) {
        self.current_index += count;
    }

    pub fn skip_if(&mut self, compare: impl Fn(&Token) -> bool) -> bool {
        if compare(self.peek(0)) {
            self.current_index += 1;
            return true;
        }
        false
    }

    // eat_until: consumes tokens until match, exclusive or inclusive
    pub fn skip_until(&mut self, compare: impl Fn(&Token) -> bool, inclusive: bool) {
        while !compare(self.peek(0)) {
            self.current_index += 1;
        }

        if inclusive {
            self.current_index += 1;
        }
    }
}
