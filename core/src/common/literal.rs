use std::fmt::Display;

use crate::type_system::llc_type::Type;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiteralValue {
    pub llc_type: Type,
    pub value: Box<str>,
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match &self.llc_type {
            Type::Char => "char".to_string(),
            Type::String => "string".to_string(),
            Type::Float { signed, size } => format!(
                "{}f{}",
                match signed {
                    true => "s",
                    false => "u",
                },
                size
            ),
            Type::Integer { signed, size } => format!(
                "{}i{}",
                match signed {
                    true => "s",
                    false => "u",
                },
                size
            ),
            Type::Block { input, output } => format!(
                "{} -> {}",
                match input {
                    Some(params) => "",
                    None => "()",
                },
                match output {
                    Some(ret) => todo!(),
                    None => "{}",
                }
            ),
        };

        write!(f, "{}", format!("Literal {} ({})", t, self.value))
    }
}
