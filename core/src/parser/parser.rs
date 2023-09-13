
use super::{errors::{SyntaxError}, use_directive_parser::parse_use_directive, ast_node::AstNode};
use crate::lexer::{lexer, token_stream::TokenStream, token::{Token, TokenValue}};

pub struct ParserResult {
    pub nodes: Vec<AstNode>,
    pub errors: Vec<SyntaxError>,
    pub tokens: TokenStream,
}

pub fn parse_file<'a>(filename: &'a str) -> ParserResult {
    let mut tokens = get_tokens(filename);

    let mut nodes: Vec<AstNode> = vec![];
    let mut errors = vec![];

    'a: loop {
        if !tokens.can_read(0) {
            break 'a;
        }

        if let Some(res) = parse_use_directive(&mut tokens) {
            match res {
                ParsingResult::Ok(node) => nodes.push(node),
                ParsingResult::Err(err) => {
                    errors.push(err);
                    eat_until_eoi(&mut tokens);
                },
            }
            continue;
        }

        eat_until_eoi(&mut tokens);
    }

    ParserResult {
        nodes,
        errors,
        tokens,
    }
}

pub fn get_tokens(filename: &str) -> TokenStream {
    match lexer::get_tokens(filename) {
        None => panic!("no tokens"),
        Some(tokens) => tokens,
    }
}

fn eat_until_eoi(tokens: &mut TokenStream) {
    let mut offset = 0;
    loop {
        match tokens.get(offset) {
            Token {value: TokenValue::EOI | TokenValue::EOF, ..} => {
                offset+=1;
            },
            _ => {
                break;
            }
        }
    }
    tokens.move_index(offset);
}

pub enum ParsingResult {
    Ok(AstNode),
    Err(SyntaxError),
}
