use std::fmt::Display;

use super::token::{Token, LiteralValue};

pub trait AstNode {}

pub enum SyntaxNode<'a> {
    Statement(Statement<'a>),
    Expression(Expression<'a>),
}

pub enum Statement<'a> {
    UseDirective(Vec<Box<str>>),
    Assignement(Box<str>, Expression<'a>),
    DeclAssignment(Box<str>, Expression<'a>),
}

pub enum Expression<'a> {
    Literal(),
}

pub struct UseDirectiveNode {
    pub tokens: Vec<Token>,
}

pub struct AssignementNode {
    pub tokens: Vec<Token>
}

impl<'a> AstNode for SyntaxNode<'a> {}
impl<'a> AstNode for Statement<'a> {}
impl<'a> AstNode for Expression<'a> {}

impl<'a> Display for Statement<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UseDirective(ids) =>
                    format!("Use directive. Module path: {0}", ids.join("::")),
                _ => todo!("Not Implemented"),
            }
        )
    }
}

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
