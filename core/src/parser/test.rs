use crate::common::{identifier::TypeState, literal::LiteralValue, position::FileSpan};

pub struct ASTNode {
    position: FileSpan,
    kind: AstNodeKind,
}

pub enum AstNodeKind {
    Statement(StatementKind),
    Expression(ExpressionKind),
}

pub enum StatementKind {
    Declaration { idx: usize },
    Expression(ExpressionKind),
    Return(ExpressionKind),
}

pub enum ExpressionKind {
    Identifier { decl_idx: usize },
    Literal { idx: usize },
    FunctionCall { idx: usize },
}

pub struct Identifier {
    decl_idx: usize,
}

pub struct DeclarationStmt {
    node: ASTNode,
    id_name: Box<str>,
    id_type: TypeState,
    exp_idx: usize,
}

pub struct ReturnStmt {

}

struct AstArray {
    stmt_array: StmtArray,
    exp_array: ExpArray,
}

struct StmtArray {
    decl_array: Vec<Declaration>,
}

struct ExpArray {}

pub struct DeclarationId {
    id: usize,
}