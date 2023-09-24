use crate::lexer::{
    token::{Token, TokenKind},
    token_stream::TokenStream,
};

use super::{
    ast_node::{AstNode, ParsingResult, AstNodeKind},
    expression::Expression,
    parser::FileAst,
};

pub(in crate::parser) fn parse(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    let identifier = stream.take_if(|t| match t.value {
        TokenKind::Identifier(id) => Some(t),
        _ => None
    });

    match identifier {
        Some(id) => {
            let node = AstNode {
                position : id.position,
                kind: AstNodeKind::Expression::Identifier
            };
            file_ast.nodes.push(node);
            ParsingResult::Ok
        }
        None => ParsingResult::Other,
    }
}
