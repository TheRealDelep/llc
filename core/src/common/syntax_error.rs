use std::fmt::Display;

use crate::lexer::token::Token;

#[derive(Debug)]
pub enum CompileError {
    Syntax(SyntaxError),
}

#[derive(Debug)]
pub struct SyntaxError {
    pub ln_start: usize,
    pub ln_end: usize,
    pub ch_start: usize,
    pub ch_end: usize,
    pub reason: Box<str>,
}

impl<'a> Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = if self.ln_start == self.ln_end {
            format!(
                "At line {0}, chars {1}:{2}. {3}",
                self.ln_start, self.ch_start, self.ch_end, self.reason
            )
        } else {
            format!(
                "From (line {0}, char {1}) to (line {2}, char {3}). {4}",
                self.ln_start, self.ch_start, self.ln_end, self.ch_end, self.reason
            )
        };

        write!(f, "{}", msg)
    }
}

impl SyntaxError {
    pub(crate) fn from_token(token: &Token, reason: Option<Box<str>>) -> Self {
        SyntaxError {
            ln_start: token.line_number,
            ln_end: token.line_number,
            ch_start: token.from,
            ch_end: token.to,
            reason: match reason {
                Some(str) => Box::from(format!("SyntaxError: {}", str)),
                None => Box::from(format!("SyntaxError. {}", token.value)),
            },
        }
    }
}
