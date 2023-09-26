use std::{collections::HashMap, fs, vec};

use super::{
    file_stream::FileLine,
    identifier_builder::build_identifier,
    literal_builder::build_literal,
    operator_builder::build_operator,
    token::{Token, TokenKind},
    token_stream::TokenStream,
};
use crate::{
    common::{identifier::Identifier, syntax_error::SyntaxError},
    lexer::file_stream::FileStream,
};

pub struct LexedFile {
    pub file_name: Box<str>,
    pub stream: TokenStream,
    pub errors: Vec<SyntaxError>,
    pub identifiers: Vec<Identifier>,
}

pub fn get_tokens(file_name: &str) -> LexedFile {
    let mut file = get_file_stream(file_name);
    let mut identifiers_index: HashMap<Box<str>, usize> = HashMap::new();

    let mut lexer = LexedFile {
        file_name: file_name.to_owned().into_boxed_str(),
        stream: TokenStream::new(vec![]),
        errors: vec![],
        identifiers: vec![],
    };

    let mut current_line = match file.get_next() {
        None => return lexer,
        Some(line) => line,
    };

    loop {
        if !current_line.can_read() {
            let line_number = current_line.number + 1;
            let char_number = current_line.current_index + 1;

            current_line = match file.get_next() {
                None => {
                    lexer
                        .stream
                        .tokens
                        .push(Token::single_char(TokenKind::EOF, line_number, char_number));
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
            lexer.stream.tokens.append(&mut ops);
            continue;
        }

        if let Some(token) = build_literal(current_line) {
            lexer.stream.tokens.push(token);
            continue;
        }

        if let Some(token) =
            build_identifier(current_line, &mut lexer.identifiers, &mut identifiers_index)
        {
            lexer.stream.tokens.push(token);
            continue;
        }

        if let Some(token) = build_single_char_token(current_line) {
            lexer.stream.tokens.push(token);
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
    lexer
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
