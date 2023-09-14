use crate::lexer::token_stream::{TokenStream};

use super::{indentifier::Identifier, ast_node::{AstNodeData, ParsingResult, Parsable, AstNodePos}, parser::{ParserBuffer, FileAst}};

pub enum Expression {
    Identifier(Identifier),
}

impl Parsable for Expression {
    fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
        if let ParsingResult::Ok(id) = Identifier::parse(stream, buffer) {
            return ParsingResult::Ok(id) 
        }

        ParsingResult::Other
    }
}

impl AstNodeData for Expression {
    fn print(&self, file_ast: &FileAst) -> String {
        match self {
            Self::Identifier(id) => id.print(file_ast),
        }
    }

    fn get_pos(&self) -> &AstNodePos {
        match self {
            Self::Identifier(id) => id.get_pos(),
        }
    }
}