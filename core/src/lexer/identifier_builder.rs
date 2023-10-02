use std::collections::HashMap;

use crate::common::{identifier::{Identifier, TypeState}, keyword};

use super::{file_stream::FileLine, token::{Token, TokenKind}};

pub fn build_identifier<'a>(
    line: &mut FileLine,
    identifiers: &mut Vec<Identifier>,
    identifiers_index: &mut HashMap<Box<str>, usize>,
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
        break;
    }

    if identifier.is_empty() {
        return None;
    }

    if let Some(keyword) = keyword::parse_keyword(&identifier) {
        return Some(Token::new(TokenKind::Keyword(keyword), line.number, from, line.current_index))
    }

    let to = from + identifier.len() -1;
    let index = match identifiers_index.get(identifier.as_str()) {
        Some(index) => *index,
        None => {
            identifiers.push(Identifier::new(&identifier));
            let index = identifiers.len() - 1;
            identifiers_index.insert(identifier.into_boxed_str(), index);
            index
        }
    };

    Some(Token::new(
        TokenKind::Identifier { index },
        line.number,
        from,
        to
    ))
}