use crate::{
    common::{literal::LiteralValue, position::FileSpan},
    lexer::{
        token::{Token, TokenKind},
        token_stream::TokenStream,
    },
};

use super::{
    ast_node::{ParsingResult, AstNodeKind, AstNode, NodeParent},
    expression::Expression,
    parser::FileAst,
};

pub struct Literal {
    pub value: LiteralValue,
    pub pos: FileSpan,
}

    pub(in crate::parser) fn parse(
        stream: &mut TokenStream,
        file_ast: &mut FileAst,
    ) -> ParsingResult {
        let lit = stream.take_if(|t| match t {
            token @ Token {
                kind: TokenKind::Literal(lit),
                ..
            } => Some(Literal {
                value: lit.clone(),
                pos: token.position 
            }),
            _ => None,
        });

        match lit {
            Some(l) => {
                file_ast.nodes.push(AstNode {
                    position: l.pos,
                    kind: AstNodeKind::Expression(Expression::Literal(l)),
                    parent: NodeParent::Unchecked
                });
                return ParsingResult::Ok;
            }
            None => return ParsingResult::Other,
        }
    }