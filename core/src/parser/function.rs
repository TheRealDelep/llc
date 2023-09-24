use crate::{
    common::syntax_error::SyntaxError,
    lexer::{token::TokenKind, token_stream::TokenStream},
    type_system::llc_type::Type,
};

use super::{
    ast_node::{AstNode, AstNodeData, AstNodePos, ParsingResult},
    expression::Expression,
    parser::FileAst,
    statement::Statement,
};

pub struct Function {
    pub pos: AstNodePos,
    body: Vec<usize>,
}

pub struct FunctionParam {
    pub llc_type: Type,
    pub name: Option<Box<str>>,
}

impl Function {
    pub(in crate::parser) fn parse(
        stream: &mut TokenStream,
        file_ast: &mut FileAst,
    ) -> ParsingResult {
        let begin = match stream.take_if(|t| match t.value {
            TokenKind::OpenCurly => Some((t.line_number, t.from)),
            _ => None,
        }) {
            Some(b) => b,
            None => return ParsingResult::Other,
        };

        let mut statements = vec![];

        loop {
            if !stream.can_read() {
                let eof = stream.peek(0);
                file_ast.errors.push(SyntaxError::from_token(
                    eof,
                    Some(Box::from("Missing scope end }")),
                ));
                return ParsingResult::Error;
            }

            if let Some(end) = stream.take_if(|t| match t.value {
                TokenKind::ClosingCurly => Some((t.line_number, t.to)),
                _ => None,
            }) {
                let node = AstNode::Expression(Expression::Function(Self {
                    body: statements,
                    pos: AstNodePos {
                        ln_start: begin.0,
                        ln_end: end.0,
                        ch_start: begin.1,
                        ch_end: end.1,
                    },
                }));
                file_ast.nodes.push(node);
                return ParsingResult::Ok;
            }

            if let ParsingResult::Ok = Statement::parse(stream, file_ast) {
                statements.push(file_ast.nodes.len() - 1)
            }
        }
    }
}

impl AstNodeData for Function {
    fn print(&self, file_ast: &FileAst) -> String {
        let statements: Vec<String> = self
            .body
            .iter()
            .map(|i| file_ast.nodes.get(*i).unwrap().print(file_ast))
            .collect();

        format!("Function body \n\t {}", statements.join("\n\t"))
    }

    fn get_pos(&self) -> &AstNodePos {
        &self.pos
    }
}
