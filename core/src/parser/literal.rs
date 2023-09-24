use crate::{
    common::literal::LiteralValue,
    lexer::{
        token::{Token, TokenKind},
        token_stream::TokenStream,
    },
};

use super::{
    ast_node::{AstNode, AstNodeData, AstNodePos, ParsingResult},
    expression::Expression,
    parser::FileAst,
};

pub struct Literal {
    pub value: LiteralValue,
    pub pos: AstNodePos,
}

impl Literal {
    pub(in crate::parser) fn parse(
        stream: &mut TokenStream,
        file_ast: &mut FileAst,
    ) -> ParsingResult {
        let lit = stream.take_if(|t| match t {
            token @ Token {
                value: TokenKind::Literal(lit),
                ..
            } => Some(Literal {
                value: lit.clone(),
                pos: AstNodePos::from_token(token),
            }),
            _ => None,
        });

        match lit {
            Some(l) => {
                let node = AstNode::Expression(Expression::Literal(l));
                file_ast.nodes.push(node);
                return ParsingResult::Ok;
            }
            None => return ParsingResult::Other,
        }
    }
}

impl AstNodeData for Literal {
    fn print(&self, _: &FileAst) -> String {
        format!("{}", self.value)
    }

    fn get_pos(&self) -> &AstNodePos {
        &self.pos
    }
}
