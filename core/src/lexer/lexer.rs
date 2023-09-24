use std::{fs, vec, collections::HashMap};

use super::{
    file_stream::FileLine,
    literal_builder::{build_literal, build_identifier},
    operator_builder::build_operator,
    token::{Token, TokenKind},
    token_stream::TokenStream,
};
use crate::{common::{syntax_error::SyntaxError, identifier::Identifier}, lexer::file_stream::FileStream};

pub struct Lexer {
    file: FileStream,
    tokens: TokenStream,
    errors: Vec<SyntaxError>,
    identifiers: Vec<Identifier>
}

pub fn get_tokens(filename: &str) -> (TokenStream, Vec<SyntaxError>) {
    let mut lexer = Lexer {
        file: get_file_stream(filename),
        tokens: vec![],
        errors: vec![],
        identifiers: vec![]
    };

    let mut indentifiers_index: HashMap<str, usize> = HashMap::new();

    let mut current_line = match lexer.file.get_next() {
        None => return (lexer.tokens, lexer.errors),
        Some(line) => line,
    };

    loop {
        if !current_line.can_read() {
            let line_number = current_line.number + 1;
            let char_number = current_line.current_index + 1;

            current_line = match lexer.file.get_next() {
                None => {
                    lexer.tokens.push(Token::single_char(
                        TokenKind::EOF,
                        line_number,
                        char_number,
                    ));
                    break;
                }
                Some(line) => line,
            };
        }

        if is_comment_line(&mut current_line) {
            continue;
        }

        if eat_white_spaces(&mut current_line) {
            continue;
        }

        if let Some(mut ops) = build_operator(current_line) {
            lexer.tokens.append(&mut ops);
            continue;
        }

        if let Some(token) = build_literal(current_line) {
            lexer.tokens.push(token);
            continue;
        }

        if let Some(token) = build_identifier(current_line, &mut lexer.identifiers, indentifiers_index) {
            lexer.tokens.push(token);
            continue;
        }

        if let Some(token) = build_single_char_token(current_line) {
            lexer.tokens.push(token);
            continue;
        }

        if let Some(token) = current_line.get_next() {
            let token = Token::single_char(
                TokenKind::Undefined(token.to_string().into_boxed_str()),
                current_line.number,
                current_line.current_index,
            );

            lexer.errors.push(SyntaxError::from_token(
                &token,
                Some(Box::from(format!("Undefined token. {token}"))),
            ))
        }
    }

    (lexer.tokens, lexer.errors)
}

fn get_file_stream(filename: &str) -> FileStream {
    let bytes = match fs::read(filename) {
        Ok(data) => data,
        Err(error) => panic!("Error happened opening the file: {}", error),
    };

    match std::str::from_utf8(&bytes) {
        Ok(data) => FileStream::new(data),
        Err(error) => panic!("Error happend while reading the file: {}", error),
    }
}

fn is_comment_line(line: &mut FileLine) -> bool {
    let f = match line.get_next() {
        None => false,
        Some(c) => *c == '/',
    };

    let s = match line.get_next() {
        None => {
            line.backtrack(1);
            return false;
        }
        Some(c) => *c == '/',
    };

    if !(f && s) {
        line.backtrack(2);
        return false;
    }

    loop {
        match line.get_next() {
            Some(_) => continue,
            None => return true,
        };
    }
}

fn eat_white_spaces(line: &mut FileLine) -> bool {
    match line.get_next() {
        Some(c) => match c {
            c if !c.is_whitespace() => {
                line.backtrack(1);
                return false;
            }
            _ => loop {
                eat_white_spaces(line);
                return true;
            },
        },
        None => false,
    }
}

fn build_single_char_token<'a>(line: &mut FileLine) -> Option<Token> {
    if let Some(c) = line.get_next() {
        let value = match c {
            '{' => TokenKind::OpenCurly,
            '}' => TokenKind::ClosingCurly,
            '(' => TokenKind::OpenParenthesis,
            ')' => TokenKind::ClosingParenthesis,
            ',' => TokenKind::Comma,
            ';' => TokenKind::EOI,
            _ => {
                line.backtrack(1);
                return None;
            }
        };

        return Some(Token::single_char(
            value,
            line.number + 1,
            line.current_index,
        ));
    }
    None
}
