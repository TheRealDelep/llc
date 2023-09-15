use std::fmt::Display;

use super::ast_node::{AstNode, AstNodeData};
use super::ast_node::{Parsable, ParsingResult};
use super::errors::SyntaxError;
use crate::lexer::token::{Token, TokenValue};
use crate::lexer::{lexer, token_stream::TokenStream};

pub struct FileAst {
    pub file_name: Box<str>,
    pub nodes: Vec<AstNode>,
    pub errors: Vec<SyntaxError>,
    root_nodes: Vec<usize>,
}

impl FileAst {
    pub fn parse_file<'a>(file_name: &'a str) -> Self {
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
                }
                ParsingResult::Error => {
                    result.errors.append(buffer.get_raw_errors());
                }
                ParsingResult::Other => {
                    stream.skip_until(
                        |t| match t {
                            Token {
                                value: TokenValue::EOI | TokenValue::EOF,
                                ..
                            } => true,
                            _ => false,
                        },
                        true,
                    );
                }
            }

            buffer.reset(Some(result.nodes.len()));
        }

        result
    }
}

impl Display for FileAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: Vec<String> = self
            .root_nodes
            .iter()
            .map(|i| self.nodes.get(*i).unwrap().print(self))
            .collect();

        write!(f, "{}", s.join("\n"))
    }
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
