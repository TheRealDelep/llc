use std::default;

use crate::{
    common::{syntax_error::SyntaxError, position::FileSpan},
    lexer::{token::TokenKind, token_stream::TokenStream},
};

use super::{
    ast_node::{AstNode, ParsingResult, AstNodeKind, NodeParent},
    expression::Expression,
    parser::FileAst,
    statement
};

pub(in crate::parser) fn parse(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    let begin = match stream.take_if(|t| match t.kind {
        TokenKind::OpenCurly => Some(t.position),
        _ => None,
    }) {
        Some(b) => b,
        None => return ParsingResult::Other,
    };

    let mut statements = vec![];

    loop {
        if !stream.can_read() {
            let eof = stream.peek(0);
            file_ast.errors.push(SyntaxError::from_token(
                eof,
                Some(Box::from("Missing scope end }")),
            ));
            return ParsingResult::Error;
        }

        if let Some(end) = stream.take_if(|t| match t.kind {
            TokenKind::ClosingCurly => Some(t.position),
            _ => None,
        }) {

            let node = AstNode {
                kind: AstNodeKind::Expression(Expression::Block),
                position: FileSpan::combine(&begin, &end),
                parent: NodeParent::Unchecked
            };

            file_ast.nodes.push(node);
            return ParsingResult::Ok;
        }

        if let ParsingResult::Ok = statement::parse(stream, file_ast) {
            statements.push(file_ast.nodes.len() - 1)
        }
    }
}
