use std::fmt::Display;

use llc_core::models::token::Token;

#[derive(Debug)]
pub enum CompileError {
    Syntax(SyntaxError)
}

#[derive(Debug)]
pub struct SyntaxError {
    pub line_number: usize,
    pub from: usize,
    pub to: usize,
    pub reason: Box<str>
}

impl<'a> Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::Syntax(data) => format!(
                "At line {0}, chars {1}:{2}. {3}", 
                data.line_number, 
                data.from, 
                data.to, 
                data.reason
            ),
        };

        write!(f, "{}", desc)
    }
}

impl SyntaxError {
    pub fn from_token(token: &Token, reason: Option<Box<str>>) -> Self {
        SyntaxError { 
            line_number: token.line_number, 
            from: token.from,
            to: token.to,
            reason: match reason {
                Some(str) => Box::from(format!("SyntaxError: {}", str)),
                None => Box::from(format!("SyntaxError: Unexpected token {}", token.value))
            }
        }
    }
}