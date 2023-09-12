use std::fmt::Display;

use super::{token::Token, use_directive::UseDirective};

pub struct AstNode {
    pub ln_start: usize,
    pub ln_end: usize,
    pub ch_start: usize,
    pub ch_end: usize,
    pub kind: AstNodeKind
}

pub enum AstNodeKind {
    Statement(Statement),
    Expression(Expression),
}

pub enum Statement {
    UseDirective(UseDirective),
}

pub enum Expression {
    Literal(Literal),
}

pub struct Assignement {
    pub tokens: Vec<Token>
}

pub struct DeclAssignment {

}

pub struct Literal {

}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UseDirective(dir) =>
                    format!("Use directive. Module path: {0}", dir.path.join("::")),
                _ => todo!("Not Implemented"),
            }
        )
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for AstNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            Self::Expression(exp) => format!("{}", exp),
            Self::Statement(stmt) => format!("{}", stmt)
        })
    }
}