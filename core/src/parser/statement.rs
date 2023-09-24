use crate::lexer::{token::TokenKind, token_stream::TokenStream};

use super::{
    ast_node::ParsingResult, declaration, expression::Expression, parser::FileAst, return_stmt,
};

pub enum Statement {
    Declaration { start: usize },
    Expression { start: usize },
    Return {start: usize },
}

pub(in crate::parser) fn parse(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    match declaration::parse(stream, file_ast) {
        ParsingResult::Ok => {
            stream.skip_if(|t| t.value == TokenKind::EOI);
            return ParsingResult::Ok;
        }
        ParsingResult::Error => return ParsingResult::Error,
        ParsingResult::Other => {}
    }

    match Expression::parse(stream, file_ast) {
        ParsingResult::Ok => {
            stream.skip_if(|t| t.value == TokenKind::EOI);
            return ParsingResult::Ok;
        }
        ParsingResult::Error => return ParsingResult::Error,
        ParsingResult::Other => {}
    }

    match return_stmt::parse(stream, file_ast) {
        ParsingResult::Ok => {
            stream.skip_if(|t| t.value == TokenKind::EOI);
            return ParsingResult::Ok;
        }
        ParsingResult::Error => return ParsingResult::Error,
        ParsingResult::Other => return ParsingResult::Other,
    }
}
