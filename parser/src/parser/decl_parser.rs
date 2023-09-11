use llc_core::models::{
    ast_node::{Expression, Statement},
    token::{Operator, TokenValue},
};

use crate::lexer::token_stream::TokenStream;

use super::{
    errors::{CompileError, SyntaxError},
    parser::ParsingResult,
};

pub(crate) fn parse_declaration<'a>(
    stream: &'a TokenStream<'a>,
    index: usize,
) -> Option<ParsingResult<Statement<'a>>> {
    let id = match &stream.get(index).value {
        TokenValue::Identifier(id) => id,
        _ => return None,
    };

    let op = match stream.get(index + 1).value {
        TokenValue::Operator(op) => match op {
            Operator::Assignment | Operator::Declassignment => op,
            _ => return None,
        },
        _ => return None,
    };

    let mut current_index = index +2;

    let exp = match parse_expression(stream, index + 2) {
        Some(ParsingResult::Ok(exp, i)) => {
            current_index = i;
            exp
        }
        Some(ParsingResult::Err(err, i)) => return Some(ParsingResult::Err(err, i)),
        None => {
            let next = stream.get(current_index);
            let reason = format!("Expected expression after {op}").into_boxed_str();

            return Some(ParsingResult::Err(
                CompileError::Syntax(SyntaxError::from_token(next, Some(reason))),
                current_index,
            ));
        }
    };

    match op {
        Operator::Assignment => Some(ParsingResult::Ok(
            Statement::Assignement(id.to_owned(), exp),
            current_index,
        )),
        Operator::Declassignment => Some(ParsingResult::Ok(
            Statement::DeclAssignment(id.to_owned(), exp),
            current_index,
        )),
        _ => panic!("Expected = or := but found {op}"),
    }
}

pub(crate) fn parse_expression<'a>(
    stream: &'a TokenStream<'a>,
    index: usize,
) -> Option<ParsingResult<Expression<'a>>> {
    let first = &stream.get(index).value;
    let second = &stream.get(index + 1).value;

    match (first, second) {
        (TokenValue::Literal(val), TokenValue::EOI) => Some(ParsingResult::Ok(Expression::Literal(&val), index + 2)),
        _ => return None
    }
}