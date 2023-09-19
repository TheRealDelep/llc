use crate::lexer::{token::Token, token_stream::TokenStream};

use super::{expression::Expression, parser::FileAst, statement::Statement};

pub trait AstNodeData {
    fn print(&self, file_ast: &FileAst) -> String;
    fn get_pos(&self) -> &AstNodePos;
}

pub enum ParsingResult {
    Ok,
    Error,
    Other,
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

impl AstNode {
    pub(in crate::parser) fn parse(
        stream: &mut TokenStream,
        file_ast: &mut FileAst,
    ) -> ParsingResult {
        Statement::parse(stream, file_ast)
    }
}

impl AstNodeData for AstNode {
    fn print(&self, file_ast: &FileAst) -> String {
        match self {
            Self::Expression(exp) => exp.print(file_ast),
            Self::Statement(stmt) => stmt.print(file_ast),
        }
    }

    fn get_pos(&self) -> &AstNodePos {
        match self {
            Self::Expression(exp) => exp.get_pos(),
            Self::Statement(stmt) => stmt.get_pos(),
        }
    }
}

impl AstNodePos {
    pub(crate) fn from_nodes(first: &AstNode, last: &AstNode) -> Self {
        let first_pos = first.get_pos();
        let last_pos = last.get_pos();

        Self::combine(first_pos, last_pos) 
    }

    pub(crate) fn from_token(token: &Token) -> Self {
        Self {
            ln_start: token.line_number,
            ln_end: token.line_number,
            ch_start: token.from,
            ch_end: token.to,
        }
    }

    pub (crate) fn combine(begin: &AstNodePos, end: &AstNodePos) -> Self {
        Self {
            ln_start: begin.ln_start,
            ln_end: end.ln_end,
            ch_start: begin.ch_start,
            ch_end: end.ch_end,
        }
    }
}
