use std::{fs, vec};

use super::{
    file_stream::FileLine, literal_builder::build_literal, operator_builder::build_operator,
    token_stream::TokenStream,
};
use crate::lexer::file_stream::FileStream;
use llc_core::models::token::{Token, TokenValue};

pub fn get_tokens(filename: &str) -> Option<TokenStream> {
    let mut file_stream = get_file_stream(filename);

    let mut current_line = match file_stream.get_next() {
        None => return None,
        Some(line) => line,
    };

    let mut tokens = vec![];

    loop {
        if !current_line.can_read() {
            let line_number = current_line.number + 1;
            let char_number = current_line.current_index + 1;

            current_line = match file_stream.get_next() {
                None => {
                    tokens.push(Token {
                        filename,
                        value: TokenValue::EOF,
                        line_number,
                        from: char_number,
                        to: char_number,
                    });
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

        if let Some(mut ops) = build_operator(current_line, filename) {
            tokens.append(&mut ops);
            continue;
        }

        if let Some(token) = build_literal(current_line, filename) {
            tokens.push(token);
            continue;
        }

        if let Some(token) = build_single_char_token(current_line, filename) {
            tokens.push(token);
            continue;
        }

        current_line.get_next();

        tokens.push(Token {
            value: TokenValue::Undefined,
            filename,
            line_number: current_line.number,
            from: current_line.current_index,
            to: current_line.current_index,
        });
    }

    Some(TokenStream::new(tokens))
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
        Some(c) => *c == '\n',
    };

    let s = match line.get_next() {
        None => {
            line.backtrack(1);
            return false;
        }
        Some(c) => *c == '\n',
    };

    if !(f && s) {
        line.backtrack(2);
        return false;
    }

    true
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

fn build_single_char_token<'a>(line: &mut FileLine, filename: &'a str) -> Option<Token<'a>> {
    if let Some(c) = line.get_next() {
        let value = match c {
            '{' => TokenValue::OpenCurly,
            '}' => TokenValue::ClosingCurly,
            '(' => TokenValue::OpenParenthesis,
            ')' => TokenValue::ClosingParenthesis,
            ',' => TokenValue::Comma,
            ';' => TokenValue::EOI,
            _ => {
                line.backtrack(1);
                return None;
            }
        };

        return Some(Token {
            value,
            filename,
            line_number: line.number +1,
            from: line.current_index,
            to: line.current_index,
        });
    }
    None
}
