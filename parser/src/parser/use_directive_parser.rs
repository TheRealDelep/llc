use llc_core::models::{
    ast_node::{AstNode, AstNodeKind, Statement},
    token::{Keyword, Operator, Token, TokenValue},
    use_directive::UseDirective,
};

use super::{errors::SyntaxError, parser::ParsingResult};
use crate::lexer::token_stream::TokenStream;

pub(crate) fn parse_use_directive(stream: &mut TokenStream) -> Option<ParsingResult> {
    let use_keyword = match stream.get(0) {
        token @ Token {
            value: TokenValue::Keyword(Keyword::Use),
            ..
        } => token,
        _ => return None,
    };

    let mut offset = 0;
    let mut path: Vec<Box<str>> = vec![];

    'a: loop {
        offset += 1;

        match stream.get(offset) {
            Token {
                value: TokenValue::Identifier(id),
                ..
            } => path.push(id.to_owned()),
            token @ _ => {
                let previous = stream.get(offset - 1);
                let reason = format!(
                    "Expected identifier after {0} in use directive but found {1}.",
                    &previous.value, token.value
                ).into_boxed_str();

                let err = SyntaxError::from_token(token, Some(reason));
                stream.move_index(offset);
                return Some(ParsingResult::Err(err));
            }
        };

        offset += 1;

        match stream.get(offset) {
            Token {
                value: TokenValue::Operator(Operator::NameSpaceNav),
                ..
            } => continue 'a,
            Token {
                value: TokenValue::EOI,
                ..
            } => {
                let previous = stream.get(offset - 1);
                let res = Some(ParsingResult::Ok(AstNode {
                    ln_start: use_keyword.line_number,
                    ln_end: previous.line_number,
                    ch_start: use_keyword.from,
                    ch_end: previous.to,
                    kind: AstNodeKind::Statement(Statement::UseDirective(UseDirective { path })),
                }));
                stream.move_index(offset + 1);
                return res
            }
            token @ _ => {
                let reason = format!(
                    "Expected :: or ; after identifier in use directive but found {}.",
                    token.value
                );

                return Some(ParsingResult::Err(SyntaxError::from_token(
                    token,
                    Some(reason.into_boxed_str()),
                )));
            }
        };
    }
}
