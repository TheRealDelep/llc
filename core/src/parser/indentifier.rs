use crate::lexer::{
    token::{Token, TokenValue},
    token_stream::TokenStream,
};

use super::{
    ast_node::{AstNode, AstNodePos, ParsingResult, AstNodeData, Parsable},
    parser::{ParserBuffer, FileAst}, expression::Expression,
};

pub struct Identifier {
    pub value: Box<str>,
    pub pos: AstNodePos,
}

impl Parsable for Identifier {
    fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
        let identifier = stream.take_if(|t| match t {
            Token {
                value: TokenValue::Identifier(id),
                ..
            } => {
                let id = Identifier {
                    value: id.to_owned(),
                    pos: AstNodePos {
                        ln_start: t.line_number,
                        ln_end: t.line_number,
                        ch_start: t.from,
                        ch_end: t.to,
                    },
                };
                Some(AstNode::Expression(Expression::Identifier(id)))
            }
            _ => None,
        });

        match identifier {
            Some(node) => ParsingResult::Ok(buffer.push(node)),
            None => ParsingResult::Other,
        }
    }
}

impl AstNodeData for Identifier {
    fn print(&self, file_ast: &super::parser::FileAst) -> String {
        format!("Identifier({})", self.value)
    }

    fn get_pos(&self) -> &AstNodePos {
        &self.pos
    }
}