use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    String(Box<str>),
    Char(char),
    Integer(Box<str>),
    Float(Box<str>),
}

impl Display for Literal {
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