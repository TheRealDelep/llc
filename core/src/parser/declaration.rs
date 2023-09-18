use crate::{
    common::operator::Operator,
    lexer::{
        token::{Token, TokenValue},
        token_stream::TokenStream,
    },
};

use super::{
    ast_node::{AstNode, AstNodeData, AstNodePos, ParsingResult},
    expression::Expression,
    indentifier::Identifier,
    parser::FileAst,
    parser_buffer::ParserBuffer,
    statement::Statement,
    syntax_error::SyntaxError,
};

pub struct Declaration {
    identifier_id: usize,
    expression_id: usize,
    pos: AstNodePos,
}

impl Declaration {
    pub(in crate::parser) fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
        let id_id = match Identifier::parse(stream, buffer) {
            ParsingResult::Ok(i) => i,
            _ => return ParsingResult::Other,
        };

        if !stream.skip_if(|t| match t {
            Token {
                value: TokenValue::Operator(Operator::Declassignment),
                ..
            } => true,
            _ => false,
        }) {
            return ParsingResult::Other;
        }

        let exp_id = match Expression::parse(stream, buffer) {
            ParsingResult::Ok(id) => id,
            ParsingResult::Error => return ParsingResult::Error,
            ParsingResult::Other => {
                let token = stream.take();
                let tmp = (token.line_number, token.from);
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
                let eoi = stream.take();

                let err = SyntaxError {
                    ln_start: tmp.0,
                    ln_end: eoi.line_number,
                    ch_start: tmp.1,
                    ch_end: eoi.to,
                    reason: Box::from("Expected a value after operator :="),
                };

                buffer.errors.push(err);
                return ParsingResult::Error;
            }
        };

        let id = buffer.get(id_id);
        let exp = buffer.get(exp_id);

        return ParsingResult::Ok(buffer.push_node(AstNode::Statement(Statement::Declaration(
            Declaration {
                identifier_id: id_id,
                expression_id: exp_id,
                pos: AstNodePos::from_nodes(id, exp),
            },
        ))));
    }
}

impl AstNodeData for Declaration {
    fn print(&self, file_ast: &FileAst) -> String {
        let identifier = file_ast.nodes.get(self.identifier_id).unwrap();
        let expression = file_ast.nodes.get(self.expression_id).unwrap();

        format!(
            "Declaration. (Identifier ({0}), Expression ({1}))",
            identifier.print(file_ast),
            expression.print(file_ast)
        )
    }

    fn get_pos(&self) -> &AstNodePos {
        &self.pos
    }
}
