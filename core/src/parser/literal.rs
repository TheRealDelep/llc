use crate::{
    common::literal::LiteralValue,
    lexer::{
        token::{Token, TokenValue},
        token_stream::TokenStream,
    },
};

use super::{
    ast_node::{AstNode, AstNodeData, AstNodePos, ParsingResult},
    expression::Expression,
    parser::FileAst,
    parser_buffer::ParserBuffer,
};

pub struct Literal {
    pub value: LiteralValue,
    pub pos: AstNodePos,
}

impl Literal {
    pub(in crate::parser) fn parse(
        stream: &mut TokenStream,
        buffer: &mut ParserBuffer,
    ) -> ParsingResult {
        let lit = stream.take_if(|t| match t {
            token @ Token {
                value: TokenValue::Literal(lit),
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
                return ParsingResult::Ok(buffer.push_node(node));
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
