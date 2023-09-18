use crate::{
    common::operator::Operator,
    lexer::{token::TokenValue, token_stream::TokenStream},
};

use super::{
    ast_node::{AstNode, AstNodeData, AstNodePos, ParsingResult},
    syntax_error::SyntaxError,
    expression::Expression,
    indentifier::Identifier, parser_buffer::ParserBuffer, parser::FileAst,
};

pub struct FunctionCall {
    pub identifier_id: usize,
    pub param_id: usize,
    pub pos: AstNodePos,
}

impl FunctionCall {
    pub(in crate::parser) fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
        if !stream.skip_if(|t| t.value == TokenValue::Operator(Operator::Into)) {
            return ParsingResult::Other
        }

        let identifier_id = match Identifier::parse(stream, buffer) {
            ParsingResult::Ok(id) => id,
            ParsingResult::Error => return ParsingResult::Error,
            ParsingResult::Other => {
                let token = stream.peek(0);
                let reason = Box::from(format!("Expected an expression after operator -> in function call expression but found {}.", token.value));
                buffer.errors.push(SyntaxError::from_token(token, Some(reason)));

                return ParsingResult::Error
            }
        };

        let param_id = buffer.get_node_id(buffer.len());
        let param = buffer.get(param_id);
        let exp = buffer.get(identifier_id);

        let pos = AstNodePos::from_nodes(param, exp);
        let node = AstNode::Expression(Expression::FunctionCall(FunctionCall { identifier_id, param_id, pos}));
        ParsingResult::Ok(buffer.push_node(node))
    }
}

impl AstNodeData for FunctionCall {
    fn print(&self, file_ast: &FileAst) -> String {
        let id = file_ast
            .nodes
            .get(self.identifier_id)
            .unwrap()
            .print(file_ast);

        let param = file_ast
            .nodes
            .get(self.param_id)
            .unwrap()
            .print(file_ast);

        format!(
            "Function Call (Identifier: {0}, Params: [{1}])",
            id, param
        )
    }

    fn get_pos(&self) -> &super::ast_node::AstNodePos {
        &self.pos
    }
}
