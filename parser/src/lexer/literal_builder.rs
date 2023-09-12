use llc_core::models::token::{self, Token, TokenValue};

use super::file_stream::FileLine;

pub fn build_literal<'a>(line: &mut FileLine, filename: &'a str) -> Option<Token> {
    if let Some(token) = build_literal_str(line, filename) {
        return Some(token);
    }

    if let Some(token) = build_literal_num(line, filename) {
        return Some(token);
    }

    if let Some(token) = build_identifier(line, filename) {
        return Some(token);
    }

    None
}

fn build_literal_str<'a>(line: &mut FileLine, filename: &'a str) -> Option<Token> {
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
            false => Some(Token {
                line_number: line.number + 1,
                from,
                to: from + lit.len() - 1,
                value: TokenValue::Literal(token::LiteralValue::String(lit.into_boxed_str())),
            }),
        };
    }
}

fn build_literal_num<'a>(line: &mut FileLine, filename: &'a str) -> Option<Token> {
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
            false => Some(Token {
                from,
                to: from + lit.len() - 1,
                line_number: line.number + 1,
                value: TokenValue::Literal(token::LiteralValue::Integer(lit.into_boxed_str())),
            }),
        };
    }
}

fn build_identifier<'a>(line: &mut FileLine, filename: &'a str) -> Option<Token> {
    let mut identifier = String::new();
    let from = line.current_index + 1;
    loop {
        if let Some(c) = line.get_next() {
            if c.is_alphabetic() {
                identifier.push(*c);
                continue;
            }
        }

        line.backtrack(1);

        return match identifier.is_empty() {
            true => None,
            false => Some(Token {
                from,
                to: from + identifier.len() - 1,
                value: match token::parse_keyword(&identifier) {
                    Some(k) => TokenValue::Keyword(k),
                    None => TokenValue::Identifier(identifier.into_boxed_str()),
                },
                line_number: line.number + 1,
            }),
        };
    }
}
