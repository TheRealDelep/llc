use llc_core::models::token::Token;

pub struct TokenStream<'a> {
    pub tokens: Vec<Token<'a>>,
    current_index: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        TokenStream { tokens, current_index: 0 }
    }

    pub fn backtrack(&mut self, steps: usize) {
        self.current_index = self.current_index.saturating_sub(steps)
    }

    pub fn get_next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current_index);
        self.current_index += 1;

        match token {
            None => None,
            Some(t) => Some(&t),
        }
    }
}
