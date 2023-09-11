use core::panic;

use llc_core::models::{
    ast_node::Statement,
    token::{Keyword, Operator, TokenValue},
};

use crate::lexer::token_stream::TokenStream;

use super::{errors::{CompileError, SyntaxError}, parser::ParsingResult};

pub(crate) fn parse_use_directive<'a>(
    stream: &'a TokenStream<'a>,
    index: usize,
) -> Option<ParsingResult<Statement<'a>>> {
    let mut new_index = index.clone();

    match stream.get(new_index).value {
        TokenValue::Keyword(Keyword::Use) => {},
        _ => return None,
    };
    
    let mut path: Vec<Box<str>> = vec![];

    'a :loop {
        new_index += 1;
        match get_identifier(stream, new_index) {
            Ok(id) => path.push(id),
            Err(err) => return Some(ParsingResult::Err(err, new_index)),
        }

        new_index += 1;
        match get_separator_or_eoi(stream, new_index) {
            Ok(t) => match t {
                TokenValue::EOI => {
                    return Some(ParsingResult::Ok(
                        Statement::UseDirective(path),
                        new_index + 1,
                    ))
                }
                TokenValue::Operator(Operator::NameSpaceNav) => continue 'a,
                _ => panic!("Expected EOI or Namesapce navigation but found {t}"),
            },
            Err(err) => return Some(ParsingResult::Err(err, new_index + 1)),
        }
    }
}

fn get_identifier(stream: &TokenStream, index: usize) -> Result<Box<str>, CompileError> {
    let token = stream.get(index);
    match &token.value {
        TokenValue::Identifier(id) => Ok(id.to_owned()),
        _ => {
            let previous = stream.get(index - 1);
            let reason = format!(
                "Expected identifier after {0} in use directive but found {1}.",
                &previous.value, token.value
            );
            return Err(CompileError::Syntax(SyntaxError::from_token(
                token,
                Some(reason.into_boxed_str()),
            )));
        } 
    }
}

fn get_separator_or_eoi<'a>(
    stream: &'a TokenStream,
    index: usize,
) -> Result<&'a TokenValue, CompileError> {
    let token = stream.get(index + 1);
    match token.value {
        TokenValue::EOI | TokenValue::Operator(Operator::NameSpaceNav) => Ok(&token.value),
        _ => {
            let reason = format!(
                "Expected :: or ; after identifier in use directive but found {}.",
                token.value
            );
            return Err(CompileError::Syntax(SyntaxError::from_token(
                token,
                Some(reason.into_boxed_str()),
            )));
        }
    }
}