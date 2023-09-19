use std::{fs, vec};

use super::{
    file_stream::FileLine,
    literal_builder::build_literal,
    operator_builder::build_operator,
    token::{Token, TokenValue},
    token_stream::TokenStream,
};
use crate::{lexer::file_stream::FileStream, common::syntax_error::SyntaxError};

pub fn get_tokens(filename: &str) -> (TokenStream, Vec<SyntaxError>) {
    let mut file_stream = get_file_stream(filename);
    let mut result = TokenStream::new(vec![]);
    let mut errors = vec![];

    let mut current_line = match file_stream.get_next() {
        None => return (result, errors),
        Some(line) => line,
    };

    loop {
        if !current_line.can_read() {
            let line_number = current_line.number + 1;
            let char_number = current_line.current_index + 1;

            current_line = match file_stream.get_next() {
                None => {
                    result.tokens.push(Token {
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

        if let Some(mut ops) = build_operator(current_line) {
            result.tokens.append(&mut ops);
            continue;
        }

        if let Some(token) = build_literal(current_line) {
            result.tokens.push(token);
            continue;
        }

        if let Some(token) = build_single_char_token(current_line) {
            result.tokens.push(token);
            continue;
        }

        if let Some(token) = current_line.get_next() {
            let token = Token {
                value: TokenValue::Undefined(token.to_string().into_boxed_str()),
                line_number: current_line.number,
                from: current_line.current_index,
                to: current_line.current_index,
            }; 
            errors.push(SyntaxError::from_token(&token, Some(Box::from(format!("Undefined token. {token}")))))
        }
    }

    (result, errors)
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
            line_number: line.number + 1,
            from: line.current_index,
            to: line.current_index,
        });
    }
    None
}
