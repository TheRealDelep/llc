use crate::lexer::token_stream::TokenStream;

use super::{declaration::Declaration, ast_node::{Parsable, AstNodeData, ParsingResult, AstNodePos}, parser::{ParserBuffer, FileAst}};

pub enum Statement {
    Declaration(Declaration)
}

impl Parsable for Statement {
    fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
        Declaration::parse(stream, buffer)
    }
}

impl AstNodeData for Statement {
    fn print(&self, file_ast: &FileAst) -> String {
        match self {
            Self::Declaration(decl) => decl.print(file_ast)
        }
    }

    fn get_pos(&self) -> &AstNodePos {
        match self {
            Self::Declaration(decl) => decl.get_pos()
        }
    }
}