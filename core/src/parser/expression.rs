use crate::lexer::token_stream::{TokenStream};

use super::{indentifier::Identifier, ast_node::{AstNodeData, ParsingResult, Parsable, AstNodePos}, parser::{ParserBuffer, FileAst}, literal::{Literal, self}};

pub enum Expression {
    Identifier(Identifier),
    Literal(Literal)
}

impl Parsable for Expression {
    fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
        if let ParsingResult::Ok(id) = Identifier::parse(stream, buffer) {
            return ParsingResult::Ok(id) 
        }

        if let ParsingResult::Ok(id) = Literal::parse(stream, buffer) {
            return ParsingResult::Ok(id) 
        }

        ParsingResult::Other
    }
}

impl AstNodeData for Expression {
    fn print(&self, file_ast: &FileAst) -> String {
        match self {
            Self::Identifier(id) => id.print(file_ast),
            Self::Literal(lit) => lit.print(file_ast)
        }
    }

    fn get_pos(&self) -> &AstNodePos {
        match self {
            Self::Identifier(id) => id.get_pos(),
            Self::Literal(lit) => lit.get_pos()
        }
    }
}