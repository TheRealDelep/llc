use std::fmt::Display;

use super::ast_node;
use super::ast_node::ParsingResult;
use super::ast_node::AstNode;
use crate::common::identifier::Identifier;
use crate::common::position::FileSpan;
use crate::common::syntax_error::SyntaxError;
use crate::lexer::lexer;
use crate::lexer::token::TokenKind;

pub struct FileAst {
    pub file_name: Box<str>,
    pub nodes: Vec<AstNode>,
    pub errors: Vec<SyntaxError>,
    pub identifiers: Vec<Identifier>,
    pub root_nodes: Vec<usize>,
}

impl FileAst {
    pub fn new<'a>(file_name: &'a str) -> Self {
        let mut lexer = lexer::get_tokens(file_name);

        let mut file_ast = FileAst {
            file_name: Box::from(file_name),
            nodes: vec![],
            errors: lexer.errors,
            identifiers: lexer.identifiers,
            root_nodes: vec![],
        };

        if !file_ast.errors.is_empty() {
            return file_ast;
        }

        'parse: loop {
            if !lexer.stream.can_read() {
                break 'parse;
            }

            match ast_node::parse(&mut lexer.stream, &mut file_ast) {
                ParsingResult::Ok => {
                    file_ast.root_nodes.push(file_ast.nodes.len() - 1);
                }
                ParsingResult::Error => {
                    lexer.stream.skip_until(
                        |t| t.kind == TokenKind::EOI || t.kind == TokenKind::EOF,
                        false,
                    );
                }
                ParsingResult::Other => {
                    let begin = lexer.stream.take().position;
                    lexer.stream.skip_until(
                        |t| t.kind == TokenKind::EOI || t.kind == TokenKind::EOF,
                        false,
                    );

                    let end = lexer.stream.take().position;
                    file_ast.errors.push(SyntaxError {
                        position: FileSpan::combine(&begin, &end),
                        reason: Box::from("Node cannot be parsed with your shit, morron!"),
                    });
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
            .map(|i| self.nodes[*i].to_string())
            .collect();

        write!(f, "{}", s.join("\n"))
    }
}
