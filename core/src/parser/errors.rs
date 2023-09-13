use std::fmt::Display;

use crate::lexer::token::Token;

#[derive(Debug)]
pub enum CompileError {
    Syntax(SyntaxError)
}

#[derive(Debug)]
pub struct SyntaxError {
    pub line_number: usize,
    pub ch_start: usize,
    pub ch_end: usize,
    pub reason: Box<str>
}

impl<'a> Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!(
            "At line {0}, chars {1}:{2}. {3}", 
            self.line_number, 
            self.ch_start, 
            self.ch_end, 
            self.reason
        ))
    }
}

impl SyntaxError {
    pub fn from_token(token: &Token, reason: Option<Box<str>>) -> Self {
        SyntaxError { 
            line_number: token.line_number, 
            ch_start: token.from,
            ch_end: token.to,
            reason: match reason {
                Some(str) => Box::from(format!("SyntaxError: {}", str)),
                None => Box::from(format!("SyntaxError: Unexpected token {}", token.value))
            }
        }
    }
}