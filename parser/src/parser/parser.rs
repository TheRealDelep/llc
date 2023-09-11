use llc_core::models::{
    ast_node::{AstNode, Statement, SyntaxNode},
    token::TokenValue,
};

use super::{errors::CompileError, use_directive_parser::parse_use_directive};
use crate::lexer::{lexer, token_stream::TokenStream};

pub struct ParserResult<'a> {
    pub nodes: Vec<Statement<'a>>,
    pub errors: Vec<CompileError>,
    pub tokens: TokenStream<'a>
}

pub fn parse_file<'a>(filename: &'a str) -> ParserResult<'a> {
    let tokens = get_tokens(filename);

    let mut index = 0;

    let mut nodes = vec![];
    let mut errors = vec![];

    'a: loop {
        if !tokens.can_read(&index) {
            break 'a;
        }

        if let Some(res) = parse_use_directive(&tokens, index) {
            index = handle_result(res, &mut nodes, &mut errors);
            continue;
        }

        'b: loop {
            match tokens.get(index).value {
                TokenValue::EOI => {
                    index += 1;
                    break 'b;
                }
                _ => index += 1,
            };
        }
    }

    ParserResult { nodes, errors, tokens }
}

pub fn get_tokens(filename: &str) -> TokenStream {
    match lexer::get_tokens(filename) {
        None => panic!("no tokens"),
        Some(tokens) => tokens,
    }
}

fn handle_result<'a>(
    res: ParsingResult<Statement<'a>>,
    nodes: &mut Vec<Statement<'a>>,
    errors: &mut Vec<CompileError>,
) -> usize {
    match res {
        ParsingResult::Ok(node, index) => {
            nodes.push(node);
            index
        }
        ParsingResult::Err(err, index) => {
            errors.push(err);
            index
        }
    }
}

pub enum ParsingResult<T: AstNode> {
    Ok(T, usize),
    Err(CompileError, usize),
}
