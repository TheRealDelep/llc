use crate::{
    common::{syntax_error::SyntaxError, operator::Operator},
    lexer::{token::TokenValue, token_stream::TokenStream},
};

use super::{
    ast_node::{ParsingResult, AstNode, AstNodePos, AstNodeData}, expression::Expression, parser::FileAst, statement::Statement,
};

pub struct Return {
    pub exp_id: Option<usize>,
    pub pos: AstNodePos
}

impl Return {
    pub(in crate::parser) fn parse(
        stream: &mut TokenStream,
        file_ast: &mut FileAst,
    ) -> ParsingResult {
        let return_pos = match stream.take_if(|t| match t.value {
            TokenValue::Operator(Operator::Return) => Some(AstNodePos::from_token(t)),
            _ => None
        }) {
            Some(t) => t,
            None => return ParsingResult::Other 
        };

        if stream.skip_if(|t| t.value == TokenValue::EOI) {}

        let exp_id = match Expression::parse(stream, file_ast) {
            ParsingResult::Ok => Some(file_ast.nodes.len() - 1),
            ParsingResult::Error => return ParsingResult::Error,
            ParsingResult::Other => {
                let first = stream.take();

                match first.value {
                    TokenValue::EOI => None,
                    _ => {
                        let ln_start = first.line_number;
                        let ch_start = first.from;

                        stream.skip_until(
                            |t| t.value == TokenValue::EOI || t.value == TokenValue::EOF,
                            false,
                        );
                        let last = stream.take();
                        let err = SyntaxError {
                            ln_start,
                            ln_end: last.line_number,
                            ch_start,
                            ch_end: last.to,
                            reason: Box::from("Expected ; or a value after return keyword"),
                        };

                        file_ast.errors.push(err);
                        return ParsingResult::Error;
                    }
                }
            }
        };

        let pos = match exp_id {
            Some(id) => {
                let exp = &file_ast.nodes[id];
                AstNodePos {
                    ln_start: return_pos.ln_start,
                    ln_end: exp.get_pos().ln_end,
                    ch_start: return_pos.ch_start,
                    ch_end: exp.get_pos().ch_end
                }
            },
            None => {
                return_pos
            }
        };

        file_ast.nodes.push(AstNode::Statement(Statement::Return(Return { exp_id, pos })));
        ParsingResult::Ok
    }
}

impl AstNodeData for Return {
    fn print(&self, file_ast: &FileAst) -> String {
        format!("Return Statement:{0};", match self.exp_id {
            Some(id) => file_ast.nodes[id].print(file_ast),
            None => "".to_string()
        })
    }

    fn get_pos(&self) -> &AstNodePos {
        &self.pos
    }
}