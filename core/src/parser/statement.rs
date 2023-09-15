use crate::lexer::{token::TokenValue, token_stream::TokenStream};

use super::{
    ast_node::{AstNodeData, AstNodePos, Parsable, ParsingResult},
    declaration::Declaration,
    parser::{FileAst, ParserBuffer},
};

pub enum Statement {
    Declaration(Declaration),
}

impl Parsable for Statement {
    fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
        match Declaration::parse(stream, buffer) {
            ParsingResult::Ok(id) => {
                stream.skip_if(|t| t.value == TokenValue::EOI);
                return ParsingResult::Ok(id);
            }
            ParsingResult::Error => return ParsingResult::Error,
            ParsingResult::Other => return ParsingResult::Other,
        }
    }
}

impl AstNodeData for Statement {
    fn print(&self, file_ast: &FileAst) -> String {
        match self {
            Self::Declaration(decl) => decl.print(file_ast),
        }
    }

    fn get_pos(&self) -> &AstNodePos {
        match self {
            Self::Declaration(decl) => decl.get_pos(),
        }
    }
}
