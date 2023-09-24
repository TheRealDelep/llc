use crate::{
    common::{operator::Operator, position::FileSpan, syntax_error::SyntaxError},
    lexer::{
        token::TokenKind,
        token_stream::TokenStream,
    },
};

use super::{
    ast_node::{AstNode, AstNodeKind, ParsingResult},
    expression::Expression,
    parser::FileAst,
    statement::Statement, indentifier,
};

pub(in crate::parser) fn parse(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    let identifier = match indentifier::parse(stream, file_ast) {
        ParsingResult::Ok => file_ast.nodes.len() - 1,
        _ => return ParsingResult::Other,
    };

    if !stream.skip_if(|t| t.value == TokenKind::Operator(Operator::Declassignment)) {
        return ParsingResult::Other;
    }

    let exp = match Expression::parse(stream, file_ast) {
        ParsingResult::Ok => file_ast.nodes.len() - 1,
        ParsingResult::Error => return ParsingResult::Error,
        ParsingResult::Other => {
            let begin = stream.take().position;
            stream.skip_until(
                |t| t.value == TokenKind::EOI || t.value == TokenKind::EOF,
                false,
            );

            let end = stream.take().position;
            file_ast.errors.push(SyntaxError {
                position: FileSpan::from_file_spans(&begin, &end),
                reason: Box::from("Expected a value after operator :="),
            });

            return ParsingResult::Error;
        }
    };

    file_ast.nodes.push(AstNode {
        kind: AstNodeKind::Statement(Statement::Declaration { start: identifier }),
        position: FileSpan::from_file_spans(
            &file_ast.nodes[identifier].position,
            &file_ast.nodes[exp].position,
        ),
    });

    return ParsingResult::Ok;
}
