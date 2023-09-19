use std::fmt::Display;

use super::ast_node::ParsingResult;
use super::ast_node::{AstNode, AstNodeData};
use crate::common::syntax_error::SyntaxError;
use crate::lexer::token::{Token, TokenValue};
use crate::lexer::lexer;

pub struct FileAst {
    pub file_name: Box<str>,
    pub nodes: Vec<AstNode>,
    pub errors: Vec<SyntaxError>,
    root_nodes: Vec<usize>,
}

impl FileAst {
    pub fn parse_file<'a>(file_name: &'a str) -> Self {
        let (mut stream, errors) = lexer::get_tokens(file_name);

        let mut file_ast = FileAst {
            file_name: Box::from(file_name),
            nodes: vec![],
            errors,
            root_nodes: vec![],
        };

        if !file_ast.errors.is_empty() {
            return file_ast
        }

        'parse: loop {
            if !stream.can_read() {
                break 'parse;
            }

            match AstNode::parse(&mut stream, &mut file_ast) {
                ParsingResult::Ok => {
                    file_ast.root_nodes.push(file_ast.nodes.len() - 1);
                },
                ParsingResult::Error => {
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
                },
                ParsingResult::Other => {
                    let t = stream.peek(0);
                    let first = (t.line_number, t.from);
                    stream.skip_until(
                        |t| match t {
                            Token {
                                value: TokenValue::EOI | TokenValue::EOF,
                                ..
                            } => true,
                            _ => false,
                        },
                        false,
                    );
                    let last = stream.take();
                    let error = SyntaxError {
                        ln_start: first.0,
                        ln_end: last.line_number,
                        ch_start: first.1,
                        ch_end: last.to,
                        reason: Box::from("Node cannot be parsed with your shit, morron!")
                    }; 
                    file_ast.errors.push(error);
                }
            };
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