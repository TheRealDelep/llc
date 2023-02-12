use std::fmt::{format, Display};

use phf::phf_map;

#[derive(Debug)]
pub struct Token<'a> {
    pub value: TokenValue,
    pub filename: &'a str,
    pub line_number: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Debug)]
pub enum TokenValue {
    Undefined,
    Literal(LiteralValue),
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

#[derive(Debug)]
pub enum LiteralValue {
    String(Box<str>),
    Char(char),
    Integer(Box<str>),
    Float(Box<str>),
}

#[derive(Copy, Clone, Debug)]
pub enum Operator {
    Minus,
    Plus,
    Times,
    Divide,
    Modulus,
    Assignment,
    Declaration,
    Declassignment,
    GreaterThan,
    LessThan,
    Equality,
    Into,
    Return,
    NameSpaceNav,
}

#[derive(Clone, Copy, Debug)]
pub enum Keyword {
    Use,
}

impl<'a> Display for Token<'a> {
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
                Self::Undefined => String::from("Undefined"),
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

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(str) => str.to_owned(),
                Self::Char(c) => c.to_string().into_boxed_str(),
                Self::Integer(i) => i.to_owned(),
                Self::Float(f) => f.to_owned(),
            }
        )
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Minus => "Minus",
                Self::Plus => "Plus",
                Self::Times => "Times",
                Self::Divide => "Divide",
                Self::Modulus => "Modulus",
                Self::Assignment => "Assignment",
                Self::Declaration => "Declaration",
                Self::Declassignment => "Declassignment",
                Self::GreaterThan => "GreaterThan",
                Self::LessThan => "LessThan",
                Self::Equality => "Equality",
                Self::Into => "Into",
                Self::Return => "Return",
                Self::NameSpaceNav => "NameSpaceNav",
            }
        )
    }
}

impl Operator {
    pub fn is_composite(&self) -> bool {
        match self {
            Self::Declassignment
            | Self::NameSpaceNav
            | Self::Return
            | Self::Into
            | Self::LessThan
            | Self::Equality
            | Self::GreaterThan => true,
            _ => false,
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Use => "use",
            }
        )
    }
}

pub fn parse_operator(c: char) -> Option<Operator> {
    match CHAR_OPERATOR_MAP.get(&c) {
        None => None,
        Some(op) => Some(*op),
    }
}

pub fn parse_comp_operator(s: &str) -> Option<Operator> {
    match COMPOSITE_OPERATOR_MAP.get(s) {
        None => None,
        Some(op) => Some(*op),
    }
}

pub fn parse_keyword(s: &str) -> Option<Keyword> {
    match KEYWORD_MAP.get(s) {
        None => None,
        Some(k) => Some(*k),
    }
}

static CHAR_OPERATOR_MAP: phf::Map<char, Operator> = phf_map!(
    '-' => Operator::Minus,
    '+' => Operator::Plus,
    '/' => Operator::Divide,
    '*' => Operator::Times,
    '%' => Operator::Modulus,
    '=' => Operator::Assignment,
    ':' => Operator::Declaration,
    '>' => Operator::GreaterThan,
    '<' => Operator::LessThan,
);

static COMPOSITE_OPERATOR_MAP: phf::Map<&str, Operator> = phf_map!(
    "->" => Operator::Into,
    "=>" => Operator::Return,
    "==" => Operator::Equality,
    ":=" => Operator::Declassignment,
    "::" => Operator::NameSpaceNav
);

static KEYWORD_MAP: phf::Map<&str, Keyword> = phf_map! {
    "use" => Keyword::Use
};
