use std::fmt::Display;

use phf::phf_map;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    Discard,
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

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Discard => "Discard",
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
    '_' => Operator::Discard
);

static COMPOSITE_OPERATOR_MAP: phf::Map<&str, Operator> = phf_map!(
    "->" => Operator::Into,
    "=>" => Operator::Return,
    "==" => Operator::Equality,
    ":=" => Operator::Declassignment,
    "::" => Operator::NameSpaceNav
);
