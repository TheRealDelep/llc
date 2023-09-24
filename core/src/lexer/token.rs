use std::fmt::Display;

use crate::common::{
    keyword::Keyword,
    literal::LiteralValue,
    operator::Operator,
    position::{FilePosition, FileSpan},
};

#[derive(Debug)]
pub struct Token {
    pub value: TokenKind,
    pub position: FileSpan,
}

pub trait TokenType {}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum TokenKind {
    #[default]
    Null,
    Undefined(Box<str>),
    Literal(LiteralValue),
    Operator(Operator),
    Identifier {index: usize},
    Keyword(Keyword),
    OpenParenthesis,
    ClosingParenthesis,
    OpenCurly,
    ClosingCurly,
    Comma,
    EOI,
    EOF,
}

impl Token {
    pub fn new(kind: TokenKind, row: usize, start_col: usize, end_col: usize) -> Self {
        Self {
            value: kind,
            position: FileSpan::new(
                FilePosition::new(row, start_col),
                FilePosition::new(row, end_col),
            ),
        }
    }

    pub fn single_char(kind: TokenKind, row: usize, col: usize) -> Self {
        Self::new(kind, row, col, col)
    }
}

impl<'a> Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}.", self.position, self.value)
    }
}

impl Display for TokenKind {
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
