use std::fmt::Display;

use crate::lexer::token::Token;

use super::position::FileSpan;

#[derive(Debug)]
pub enum CompileError {
    Syntax(SyntaxError),
}

#[derive(Debug)]
pub struct SyntaxError {
    pub position: FileSpan,
    pub reason: Box<str>,
}

impl<'a> Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = if self.position.begin.row == self.position.end.row {
            format!(
                "At line {0}, col {1}:{2}. {3}",
                self.position.begin.row,
                self.position.begin.col,
                self.position.end.col,
                self.reason
            )
        } else {
            format!("{0}. {1}", self.position, self.reason)
        };

        write!(f, "{}", msg)
    }
}

impl SyntaxError {
    pub(crate) fn from_token(token: &Token, reason: Option<Box<str>>) -> Self {
        SyntaxError {
            position: token.position, 
            reason: match reason {
                Some(str) => Box::from(format!("SyntaxError: {}", str)),
                None => Box::from(format!("SyntaxError. {}", token.kind)),
            },
        }
    }

    pub(crate) fn from_tokens(first: &Token, last: &Token, reason: Option<Box<str>>) -> Self {
        SyntaxError {
            position: FileSpan::combine(&first.position, &last.position),
            reason: match reason {
                Some(str) => Box::from(format!("SyntaxError: {}", str)),
                None => Box::from(format!(
                    "SyntaxError. from {0}, to {1}",
                    first.kind, last.kind
                )),
            },
        }
    }
}
