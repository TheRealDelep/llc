use crate::lexer::token_stream::TokenStream;

use super::{expression::Expression, parser::{ParserBuffer, FileAst}, statement::Statement};

pub trait AstNodeData {
    fn print(&self, file_ast: &FileAst) -> String;
    fn get_pos(&self) -> &AstNodePos;
}

pub(crate) trait Parsable : AstNodeData {
    fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult;
}

pub enum ParsingResult {
    Ok(usize),
    Error,
    Other
}

pub struct AstNodePos {
    pub ln_start: usize,
    pub ln_end: usize,
    pub ch_start: usize,
    pub ch_end: usize,
}

pub enum AstNode {
    Statement(Statement),
    Expression(Expression),
}

impl Parsable for AstNode {
    fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
        Statement::parse(stream, buffer)
    }
}

impl AstNodeData for AstNode {
    fn print(&self, file_ast: &FileAst) -> String {
        match self {
            Self::Expression(exp) => exp.print(file_ast),
            Self::Statement(stmt) => stmt.print(file_ast)
        }
    }

    fn get_pos(&self) -> &AstNodePos {
        match self {
            Self::Expression(exp) => exp.get_pos(),
            Self::Statement(stmt) => stmt.get_pos()
        }
    }
}

impl AstNodePos {
    pub(crate) fn from_nodes(first: &AstNode, last: &AstNode) -> Self{
        let first_pos = first.get_pos();
        let last_pos = last.get_pos();

        Self {
            ln_start: first_pos.ln_start,
            ln_end: last_pos.ln_end,
            ch_start: first_pos.ch_start,
            ch_end: last_pos.ch_end
        }
    }
}