use super::{
    expression::Expression,
    parser::FileAst,
    statement::{self, Statement},
};
use crate::{common::position::FileSpan, lexer::token_stream::TokenStream};
use std::{fmt::Display, default};

pub enum ParsingResult {
    Ok,
    Error,
    Other,
}

pub struct AstNode {
    pub kind: AstNodeKind,
    pub parent: NodeParent,
    pub position: FileSpan,
}

pub enum AstNodeKind {
    Expression(Expression),
    Statement(Statement),
}

#[derive(Default)]
pub enum NodeParent {
    #[default]
    Unchecked,
    Root,
    Node {index: usize}
}

pub(in crate::parser) fn parse(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    statement::parse(stream, file_ast)
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match &self.kind {
            AstNodeKind::Statement(stmt) => match stmt {
                Statement::Declaration { .. } => "Declaration".to_string(),
                Statement::Expression { .. } => "ExpressionStatement".to_string(),
                Statement::Return => "Return".to_string(),
            },
            AstNodeKind::Expression(exp) => match exp {
                Expression::Block => "BlockExpression".to_string(),
                Expression::FunctionCall => "FunctionCall".to_string(),
                Expression::Identifier { index } => format!("Identifier({})", index),
                Expression::Literal(lit) => format!("Literal({})", lit.value),
            },
        };

        write!(f, "{}", s)
    }
}
