use crate::common::literal::LiteralValue;

use super::{
    file_stream::FileLine,
    token::{Token, TokenKind},
};

pub fn build_literal<'a>(line: &mut FileLine) -> Option<Token> {
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
            false => {
                let to = from + lit.len() - 1;
                Some(Token::new(
                    TokenKind::Literal(LiteralValue::String(lit.into_boxed_str())),
                    line.number + 1,
                    from,
                    to,
                ))
            }
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
            false => {
                let to = from + lit.len() - 1;
                Some(Token::new(
                    TokenKind::Literal(LiteralValue::Integer(lit.into_boxed_str())),
                    line.number + 1,
                    from,
                    to,
                ))
            }
        };
    }
}
