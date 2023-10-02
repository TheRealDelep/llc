use crate::lexer::{
    token::TokenKind,
    token_stream::TokenStream,
};

use super::{
    ast_node::{AstNode, ParsingResult, AstNodeKind, NodeParent},
    expression::Expression,
    parser::FileAst,
};

pub(in crate::parser) fn parse(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    let pos_id = stream.take_if(|t| match t.kind {
        TokenKind::Identifier { index } => Some((t.position, index)),
        _ => None
    });

    match pos_id {
        Some((pos, id)) => {
            let node = AstNode {
                position : pos, 
                kind: AstNodeKind::Expression(Expression::Identifier {index: id}),
                parent: NodeParent::Unchecked
            };
            file_ast.nodes.push(node);
            ParsingResult::Ok
        }
        None => ParsingResult::Other,
    }
}