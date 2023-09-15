use std::fmt::Display;

use phf::phf_map;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    Use,
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

pub fn parse_keyword(s: &str) -> Option<Keyword> {
    match KEYWORD_MAP.get(s) {
        None => None,
        Some(k) => Some(*k),
    }
}

static KEYWORD_MAP: phf::Map<&str, Keyword> = phf_map! {
    "use" => Keyword::Use
};
