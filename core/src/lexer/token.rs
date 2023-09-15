use std::fmt::Display;

use crate::common::{literal::Literal, operator::Operator, keyword::Keyword};

#[derive(Debug, Default)]
pub struct Token {
    pub value: TokenValue,
    pub line_number: usize,
    pub from: usize,
    pub to: usize,
}

pub trait TokenType {}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum TokenValue {
    #[default]
    Null,
    Undefined(Box<str>),
    Literal(Literal),
    Operator(Operator),
    Identifier(Box<str>),
    Keyword(Keyword),
    OpenParenthesis,
    ClosingParenthesis,
    OpenCurly,
    ClosingCurly,
    Comma,
    EOI,
    EOF,
}

impl<'a> Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let col = match self.from == self.to {
            true => self.from.to_string(),
            false => format!("{}-{}", self.from, self.to),
        };
        writeln!(
            f,
            "line: {}, col: {}, value: {}",
            self.line_number, col, self.value
        )
    }
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Null => String::from("Null token"),
                Self::Undefined(t) => format!("Undefined: {t}"),
                Self::Literal(lit) => format!("Literal value: {lit}"),
                Self::Operator(op) => format!("Operator: {op}"),
                Self::Identifier(id) => format!("Identifier: {id}"),
                Self::Keyword(k) => format!("Keyword: {k}"),
                Self::OpenParenthesis => String::from("Opening parenthesis"),
                Self::ClosingParenthesis => String::from("Closing parenthesis"),
                Self::OpenCurly => String::from("Opening curly brace"),
                Self::ClosingCurly => String::from("Closing curly brace"),
                Self::Comma => String::from("Comma"),
                Self::EOI => String::from("End of instruction"),
                Self::EOF => String::from("End of file"),
            }
        )
    }
}