use super::ast_node::{Parsable, ParsingResult};
use super::declaration::{self, Declaration};
use super::errors::SyntaxError;
use super::{ast_node::AstNode, indentifier::Identifier};
use crate::lexer::token::{Operator, Token, TokenValue};
use crate::lexer::{lexer, token_stream::TokenStream};

pub struct FileAst {
    pub file_name: Box<str>,
    pub nodes: Vec<AstNode>,
    pub errors: Vec<SyntaxError>,
    root_nodes: Vec<usize>,
}

pub fn parse_file<'a>(file_name: &'a str) -> FileAst {
    let mut stream = get_tokens(file_name);
    let mut result = FileAst {
        file_name: Box::from(file_name),
        nodes: vec![],
        errors: vec![],
        root_nodes: vec![],
    };

    let mut buffer = ParserBuffer::new();

    'parse: loop {
        if !stream.can_read() {
            break 'parse;
        }

        match AstNode::parse(&mut stream, &mut buffer) {
            ParsingResult::Ok(id) => {
                result.nodes.append(&mut buffer.get_raw_nodes());
                result.root_nodes.push(id);
            },
            ParsingResult::Error => {
                result.errors.append(buffer.get_raw_errors());
            },
            ParsingResult::Other => {
                stream.skip_until(|t| match t {
                    Token {value: TokenValue::EOI | TokenValue::EOF, ..} => true, 
                    _ => false
                }, true);
            }
        }

        buffer.reset(Some(result.nodes.len()));
    }

    result
}

pub fn get_tokens(filename: &str) -> TokenStream {
    match lexer::get_tokens(filename) {
        None => panic!("no tokens"),
        Some(tokens) => tokens,
    }
}

pub(crate) struct ParserBuffer {
    pub errors: Vec<SyntaxError>,
    nodes: Vec<AstNode>,
    nodes_offset: usize,
}

impl ParserBuffer {
    pub fn new() -> Self {
        ParserBuffer {
            errors: vec![],
            nodes: vec![],
            nodes_offset: 0,
        }
    }

    pub fn push(&mut self, node: AstNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1 + self.nodes_offset
    }

    pub fn reset(&mut self, offset: Option<usize>) {
        self.nodes.clear();
        match offset {
            Some(i) => self.nodes_offset = i,
            None => {}
        }
    }

    pub fn get_raw_nodes(&mut self) -> &mut Vec<AstNode> {
        &mut self.nodes
    }

    pub fn get_raw_errors(&mut self) -> &mut Vec<SyntaxError> {
        &mut self.errors
    }

    pub fn get(&self, id: usize) -> &AstNode {
        self.nodes.get(id - self.nodes_offset).unwrap()
    }
}
