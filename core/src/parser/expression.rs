use crate::{
    common::{operator::Operator, syntax_error::SyntaxError},
    lexer::{token::TokenKind, token_stream::TokenStream},
};

use super::{
    ast_node::ParsingResult,
    function::Function,
    function_call::FunctionCall,
    literal::Literal,
    parser::FileAst,
};

pub enum Expression {
    Identifier {id: usize},
    Literal(Literal),
    Function(Function),
    FunctionCall(FunctionCall),
}

pub(in crate::parser) fn parse(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    match stream.peek(0).value {
        TokenKind::Identifier(_)
        | TokenKind::Literal(_)
        | TokenKind::OpenCurly
        | TokenKind::Operator(Operator::Into) => {}
        _ => return ParsingResult::Other,
    }

    if let ParsingResult::Ok = parse_left_side(stream, file_ast) {
        match stream.peek(0).value {
            TokenKind::EOI => return ParsingResult::Ok,
            _ => {}
        }
    }

    loop {
        if let ParsingResult::Ok = parse_right_side(stream, file_ast) {
            match stream.peek(0).value {
                TokenKind::EOI => return ParsingResult::Ok,
                _ => continue,
            }
        }

        let first = stream.peek(0);
        let second = stream.peek(1);
        let reason = format!(
            "Unexpected token encountered while trying to parse expression. Expected one of(;, function call, function body, operator) after {0} but found {1}", 
            first.value, second.value);

        file_ast.errors.push(SyntaxError::from_token(
            stream.peek(1),
            Some(reason.into_boxed_str()),
        ));
        return ParsingResult::Error;
    }
}

fn parse_right_side(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    if let res @ ParsingResult::Ok = FunctionCall::parse(stream, file_ast) {
        return res;
    }

    ParsingResult::Other
}

fn parse_left_side(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    if let ParsingResult::Ok = parse_single_token_exp(stream, file_ast) {
        return ParsingResult::Ok;
    }

    if let ParsingResult::Ok = Function::parse(stream, file_ast) {
        return ParsingResult::Ok;
    }

    ParsingResult::Other
}

fn parse_single_token_exp(stream: &mut TokenStream, file_ast: &mut FileAst) -> ParsingResult {
    if let ParsingResult::Ok = Identifier::parse(stream, file_ast) {
        return ParsingResult::Ok;
    }

    if let ParsingResult::Ok = Literal::parse(stream, file_ast) {
        return ParsingResult::Ok;
    }

    ParsingResult::Other
}
