use std::fmt::Display;

use super::ast_node::ParsingResult;
use super::ast_node::{AstNode, AstNodeData};
use crate::common::syntax_error::SyntaxError;
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

        let mut file_ast = FileAst {
            file_name: Box::from(file_name),
            nodes: vec![],
            errors: vec![],
            root_nodes: vec![],
        };

        'parse: loop {
            if !stream.can_read() {
                break 'parse;
            }

            match AstNode::parse(&mut stream, &mut file_ast) {
                ParsingResult::Ok => {
                    file_ast.root_nodes.push(file_ast.nodes.len() - 1);
                }
                _ => {
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
        }

        file_ast
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
