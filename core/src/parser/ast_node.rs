use super::{expression::Expression, parser::FileAst, statement::Statement};
use crate::{common::position::FileSpan, lexer::token_stream::TokenStream};
use std::fmt::Display;

pub enum ParsingResult {
    Ok,
    Error,
    Other,
}

pub struct AstNode {
    pub kind: AstNodeKind,
    pub position: FileSpan,
}

pub enum AstNodeKind {
    Expression(Expression),
    Statement(Statement),
}

impl AstNode {
    pub(in crate::parser) fn parse(
        stream: &mut TokenStream,
        file_ast: &mut FileAst,
    ) -> ParsingResult {
        Statement::parse(stream, file_ast)
    }

    fn print(&self, file_ast: &FileAst) -> String {
        match self.kind {
            AstNodeKind::Expression(exp) => exp.print(file_ast),
            AstNodeKind::Statement(stmt) => stmt.print(file_ast),
        }
    }
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self.kind {
            AstNodeKind::Expression(exp) => match exp {
                Expression::Function(_) => "Function_Body".to_string(),
                Expression::FunctionCall(_) => "Function_Call".to_string(),
                Expression::Identifier(id) => format!("Identifier({})", id.value),
                Expression::Literal(lit) => format!("Literal({})", lit.value),
            },
            AstNodeKind::Statement(stmt) => match stmt {
                Statement::Declaration { start } => "Declaration".to_string(),
                Statement::Expression { start } => "ExpressionStatement".to_string(),
                Statement::Return { start } => "Return".to_string(),
            },
        };

        write!(f, "{}", s)
    }
}
