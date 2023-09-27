use crate::{
    common::{
        literal::LiteralValue,
        position::{FilePosition, FileSpan},
        syntax_error::SyntaxError,
    },
    type_system::llc_type::Type,
};

use super::{
    file_stream::FileLine,
    token::{Token, TokenKind},
};

pub fn build_literal<'a>(line: &mut FileLine, errors: &mut Vec<SyntaxError>) -> Option<Token> {
    if let Some(token) = build_literal_str(line) {
        return Some(token);
    }

    if let Some(token) = build_literal_num(line, errors) {
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
                    TokenKind::Literal(LiteralValue {
                        llc_type: Type::String,
                        value: lit.into_boxed_str()
                    }),
                    line.number + 1,
                    from,
                    to,
                ))
            }
        };
    }
}

fn build_literal_num<'a>(line: &mut FileLine, errors: &mut Vec<SyntaxError>) -> Option<Token> {
    let mut lit = String::new();
    let from = line.current_index + 1;
    let mut has_decimal = false;
    loop {
        if let Some(i) = line.get_next() {
            if *i == '.' {
                if has_decimal {
                    let pos = FilePosition::new(line.number, line.current_index + 1);
                    errors.push(SyntaxError {
                        position: FileSpan::new(pos, pos),
                        reason: Box::from(
                            "Floating point literal cannot contain more than one decimal point.",
                        ),
                    });
                    return None;
                }

                has_decimal = true
            }
            if *i == '_' {
                continue;
            }
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
                let t = match has_decimal {
                    true => Type::Float {
                        signed: true,
                        size: 32,
                    },
                    false => Type::Integer {
                        signed: true,
                        size: 32,
                    },
                };
                Some(Token::new(
                    TokenKind::Literal(LiteralValue {
                        llc_type: t,
                        value: lit.into_boxed_str(),
                    }),
                    line.number + 1,
                    from,
                    to,
                ))
            }
        };
    }
}
