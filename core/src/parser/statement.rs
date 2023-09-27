use crate::lexer::{token::TokenKind, token_stream::TokenStream};

use super::{
    ast_node::ParsingResult, declaration, expression, parser::FileAst, return_stmt,
};

pub enum Statement {
    Declaration { ident_index: usize },
    Expression { exp_id: usize },
    Return 
}

pub(in crate::parser) fn parse(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    match declaration::parse(stream, file_ast) {
        ParsingResult::Ok => {
            stream.skip_if(|t| t.kind == TokenKind::EOI);
            return ParsingResult::Ok;
        }
        ParsingResult::Error => return ParsingResult::Error,
        ParsingResult::Other => {}
    }

    match expression::parse(stream, file_ast) {
        ParsingResult::Ok => {
            stream.skip_if(|t| t.kind == TokenKind::EOI);
            return ParsingResult::Ok;
        }
        ParsingResult::Error => return ParsingResult::Error,
        ParsingResult::Other => {}
    }

    match return_stmt::parse(stream, file_ast) {
        ParsingResult::Ok => {
            stream.skip_if(|t| t.kind == TokenKind::EOI);
            return ParsingResult::Ok;
        }
        ParsingResult::Error => return ParsingResult::Error,
        ParsingResult::Other => return ParsingResult::Other,
    }
}
