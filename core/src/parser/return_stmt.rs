use crate::{
    common::{operator::Operator, position::FileSpan, syntax_error::SyntaxError},
    lexer::{token::TokenKind, token_stream::TokenStream},
};

use super::{
    ast_node::{AstNode, AstNodeKind, ParsingResult},
    expression::Expression,
    parser::FileAst,
    statement::Statement,
};

pub(in crate::parser) fn parse(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    let begin = match stream.take_if(|t| match t.value {
        TokenKind::Operator(Operator::Return) => Some(t.position),
        _ => None,
    }) {
        Some(t) => t,
        None => return ParsingResult::Other,
    };

    let end = match stream.skip_if(|t| t.value == TokenKind::EOI) {
        true => None,
        false => match Expression::parse(stream, file_ast) {
            ParsingResult::Ok => Some(stream.peek(-1).position),
            ParsingResult::Error => return ParsingResult::Error,
            ParsingResult::Other => {
                let begin = stream.take().position;
                stream.skip_until(
                    |t| t.value == TokenKind::EOI || t.value == TokenKind::EOF,
                    false,
                );
                let end = stream.take().position;

                let err = SyntaxError {
                    position: FileSpan::from_file_spans(&begin, &end),
                    reason: Box::from("Expected ; or a value after return keyword"),
                };

                file_ast.errors.push(err);
                return ParsingResult::Error;
            }
        },
    };

    let position = match end {
        Some(pos) => FileSpan::from_file_spans(&begin, &end),
        None => begin,
    };

    file_ast.nodes.push(AstNode {
        position,
        kind: AstNodeKind::Statement(Statement::Return()),
    });

    ParsingResult::Ok
}
