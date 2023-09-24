use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{BuildHasher, Hash, Hasher},
};

use crate::common::{
    identifier::{
        Identifier,
        TypeState::{self, Unchecked},
    },
    keyword,
    literal::LiteralValue,
};

use super::{
    file_stream::FileLine,
    token::{Token, TokenKind},
};

pub fn build_literal<'a>(line: &mut FileLine, ) -> Option<Token> {
    if let Some(token) = build_literal_str(line) {
        return Some(token);
    }

    if let Some(token) = build_literal_num(line) {
        return Some(token);
    }

    None
}

fn build_literal_str<'a>(line: &mut FileLine) -> Option<Token> {
    let mut lit = String::new();
    let from = line.current_index + 1;

    if let Some(c) = line.get_next() {
        if *c != '\"' {
            line.backtrack(1);
            return None;
        }
    }

    loop {
        if let Some(c) = line.get_next() {
            if *c != '\"' {
                lit.push(*c);
                continue;
            }
        }

        return match lit.is_empty() {
            true => None,
            false => Some(Token::new(
                TokenKind::Literal(LiteralValue::String(lit.into_boxed_str())),
                line.number + 1,
                from,
                from + lit.len() - 1,
            )),
        };
    }
}

fn build_literal_num<'a>(line: &mut FileLine) -> Option<Token> {
    let mut lit = String::new();
    let from = line.current_index + 1;
    loop {
        if let Some(i) = line.get_next() {
            if i.is_numeric() {
                lit.push(*i);
                continue;
            }
        }

        line.backtrack(1);

        return match lit.is_empty() {
            true => None,
            false => Some(Token::new(
                TokenKind::Literal(LiteralValue::Integer(lit.into_boxed_str())),
                line.number + 1,
                from,
                from + lit.len() - 1,
            )),
        };
    }
}

pub fn build_identifier<'a>(
    line: &mut FileLine,
    identifiers: &mut Vec<Identifier>,
    identifiers_index: &mut HashMap<&str, usize>,
) -> Option<Token> {
    let mut identifier = String::new();
    let from = line.current_index + 1;
    loop {
        if let Some(c) = line.get_next() {
            if c.is_alphabetic() || *c == '_' {
                identifier.push(*c);
                continue;
            }
        }

        line.backtrack(1);
    }

    if identifier.is_empty() {
        return None;
    }

    if let keyword = keyword::parse_keyword(&identifier) {
        return Some(keyword);
    }

    let index = match identifiers_index.get(&identifier) {
        Some(index) => *index,
        None => {
            identifiers.push(Identifier {
                name: identifier,
                type_state: TypeState::Unchecked,
            });

            let index = identifiers.len() - 1;
            identifiers_index.insert(&identifier, index);
            index
        }
    };

    Some(Token::new(
        TokenKind::Identifier(index),
        line.number,
        from,
        from + identifier.len(),
    ))
}
