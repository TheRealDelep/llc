use crate::{
    common::{operator::Operator, position::FileSpan, syntax_error::SyntaxError, identifier::TypeState},
    lexer::{
        token::TokenKind,
        token_stream::TokenStream,
    },
};

use super::{
    ast_node::{AstNode, AstNodeKind, ParsingResult, NodeParent},
    expression::{self, Expression},
    parser::FileAst,
    statement::Statement, identifier
};

pub struct Declaration {
    id_idx: usize,
    exp_idx: usize,
    name: Box<str>,
    llc_type: TypeState,
}

pub(in crate::parser) fn parse(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    let ident_index = match identifier::parse(stream, file_ast) {
        ParsingResult::Ok => file_ast.nodes.len() - 1,
        _ => return ParsingResult::Other,
    };

    if !stream.skip_if(|t| t.kind == TokenKind::Operator(Operator::Declassignment)) {
        return ParsingResult::Other;
    }

    let exp_node = match expression::parse(stream, file_ast) {
        ParsingResult::Ok => &file_ast.nodes[file_ast.nodes.len() - 1],
        ParsingResult::Error => return ParsingResult::Error,
        ParsingResult::Other => {
            let begin = stream.take().position;
            stream.skip_until(
                |t| t.kind == TokenKind::EOI || t.kind == TokenKind::EOF,
                false,
            );

            let end = stream.take().position;
            file_ast.errors.push(SyntaxError {
                position: FileSpan::combine(&begin, &end),
                reason: Box::from("Expected a value after operator :="),
            });

            return ParsingResult::Error;
        }
    };

    if let AstNodeKind::Expression(Expression::Literal(lit)) = &exp_node.kind {
        file_ast.identifiers[ident_index].type_state = TypeState::Ok(lit.value.llc_type.clone());
    }

    file_ast.nodes.push(AstNode {
        kind: AstNodeKind::Statement(Statement::Declaration { ident_index }),
        position: FileSpan::combine(
            &file_ast.nodes[ident_index].position,
            &exp_node.position,
        ),
        parent: NodeParent::Unchecked
    });

    match file_ast.nodes[ident_index].kind {
        AstNodeKind::Expression(Expression::Identifier { index }) => {
            file_ast.identifiers[index].declaration_idx = file_ast.nodes.len()
        },
        _ => panic!("Expected and identifier")
    };

    return ParsingResult::Ok;
}
