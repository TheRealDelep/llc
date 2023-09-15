use crate::{
    lexer::{
        token::TokenValue,
        token_stream::TokenStream,
    },
    type_system::llc_type::Type,
};

use super::{
    ast_node::{AstNode, AstNodeData, AstNodePos, Parsable, ParsingResult},
    errors::SyntaxError,
    expression::Expression,
    parser::{ParserBuffer, FileAst},
    statement::Statement,
};

pub struct Function {
    pub pos: AstNodePos,
    params: Option<Vec<FunctionParam>>,
    body: Vec<usize>,
}

pub struct FunctionParam {
    pub llc_type: Type,
    pub name: Option<Box<str>>,
}

impl Parsable for Function {
    fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
        let begin = match stream.take_if(|t| match t.value {
            TokenValue::OpenCurly => Some((t.line_number, t.from)),
            _ => None,
        }) {
            Some(b) => b,
            None => return ParsingResult::Other,
        };

        let mut statements = vec![];

        loop {
            if !stream.can_read() {
                let eof = stream.peek(0);
                buffer.errors.push(SyntaxError::from_token(
                    eof,
                    Some(Box::from("Missing closing curly brace }")),
                ));
                return ParsingResult::Error;
            }

            if let Some(end) = stream.take_if(|t| match t.value {
                TokenValue::ClosingCurly => Some((t.line_number, t.to)),
                _ => None,
            }) {
                return ParsingResult::Ok(buffer.push(AstNode::Expression(Expression::Function(
                    Self {
                        body: statements,
                        params: None,
                        pos: AstNodePos {
                            ln_start: begin.0,
                            ln_end: end.0,
                            ch_start: begin.1,
                            ch_end: end.1,
                        },
                    },
                ))));
            }

            if let ParsingResult::Ok(stmt) = Statement::parse(stream, buffer) {
                statements.push(stmt)
            }
        }
    }
}

impl AstNodeData for Function {
    fn print(&self, file_ast: &FileAst) -> String {
        let statements: Vec<String> = self.body
            .iter()
            .map(|i| file_ast.nodes.get(*i).unwrap().print(file_ast))
            .collect();

        format!("Function body \n\t {}", statements.join("\n\t")) 
    }

    fn get_pos(&self) -> &AstNodePos {
        &self.pos
    }
}
