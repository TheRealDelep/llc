use crate::{
    common::literal::LiteralValue,
    lexer::{
        token::{Token, TokenValue},
        token_stream::TokenStream,
    },
};

use super::{
    ast_node::{AstNode, AstNodeData, AstNodePos, Parsable, ParsingResult},
    expression::Expression,
    parser::{FileAst, ParserBuffer},
};

pub struct Literal {
    pub value: LiteralValue,
    pub pos: AstNodePos,
}

impl Parsable for Literal {
    fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
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
                return ParsingResult::Ok(buffer.push(node));
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
