use crate::{
    common::{operator::Operator, position::FileSpan, syntax_error::SyntaxError},
    lexer::{token::TokenKind, token_stream::TokenStream},
};

use super::{
    ast_node::{AstNode, AstNodeKind, ParsingResult},
    expression::Expression,
    identifier,
    parser::FileAst,
};

pub(in crate::parser) fn parse(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    let op_pos = match stream.take_if(|t| match t.kind {
        TokenKind::Operator(Operator::Into) => Some(t.position),
        _ => None,
    }) {
        Some(pos) => pos,
        None => return ParsingResult::Other,
    };

    let identifier_id = match identifier::parse(stream, file_ast) {
        ParsingResult::Ok => file_ast.nodes.len() - 1,
        ParsingResult::Error => return ParsingResult::Error,
        ParsingResult::Other => {
            let token = stream.peek(0);
            let reason = Box::from(format!("Expected an expression after operator -> in function call expression but found {}.", token.kind));
            file_ast
                .errors
                .push(SyntaxError::from_token(token, Some(reason)));
            return ParsingResult::Error;
        }
    };

    file_ast.nodes.push(AstNode {
        kind: AstNodeKind::Expression(Expression::FunctionCall),
        position: FileSpan::combine(&op_pos, &file_ast.nodes[identifier_id].position),
    });

    ParsingResult::Ok
}
