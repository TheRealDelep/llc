use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LiteralValue {
    String(Box<str>),
    Char(char),
    Integer(Box<str>),
    Float(Box<str>),
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::String(str) => format!("String (\"{}\")", str),
            Self::Char(c) => format!("Char (\"{}\")", c),
            Self::Integer(i) => format!("Integer (\"{}\")", i),
            Self::Float(f) => format!("Float (\"{}\")", f),
        };

        write!(f, "{}", format!("Literal value ({val})"))
    }
}
