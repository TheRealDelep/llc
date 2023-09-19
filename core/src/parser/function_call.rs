use crate::{
    common::{operator::Operator, syntax_error::SyntaxError},
    lexer::{token::TokenValue, token_stream::TokenStream},
};

use super::{
    ast_node::{AstNode, AstNodeData, AstNodePos, ParsingResult},
    expression::Expression,
    indentifier::Identifier,
    parser::FileAst,
};

pub struct FunctionCall {
    pub identifier_id: usize,
    pub param_id: usize,
    pub pos: AstNodePos,
}

impl FunctionCall {
    pub(in crate::parser) fn parse(
        stream: &mut TokenStream,
        file_ast: &mut FileAst,
    ) -> ParsingResult {
        if !stream.skip_if(|t| t.value == TokenValue::Operator(Operator::Into)) {
            return ParsingResult::Other;
        }

        let param_id = file_ast.nodes.len() - 1;

        let identifier_id = match Identifier::parse(stream, file_ast) {
            ParsingResult::Ok => file_ast.nodes.len() - 1,
            ParsingResult::Error => return ParsingResult::Error,
            ParsingResult::Other => {
                let token = stream.peek(0);
                let reason = Box::from(format!("Expected an expression after operator -> in function call expression but found {}.", token.value));
                file_ast
                    .errors
                    .push(SyntaxError::from_token(token, Some(reason)));
                return ParsingResult::Error;
            }
        };

        let param = &file_ast.nodes[param_id];
        let exp = &file_ast.nodes[identifier_id];

        let pos = AstNodePos::from_nodes(&param, &exp);
        let node = AstNode::Expression(Expression::FunctionCall(FunctionCall {
            identifier_id,
            param_id,
            pos,
        }));
        file_ast.nodes.push(node);
        ParsingResult::Ok
    }
}

impl AstNodeData for FunctionCall {
    fn print(&self, file_ast: &FileAst) -> String {
        let id = file_ast
            .nodes
            .get(self.identifier_id)
            .unwrap()
            .print(file_ast);

        let param = file_ast.nodes.get(self.param_id).unwrap().print(file_ast);

        format!("Function Call (Identifier: {0}, Params: [{1}])", id, param)
    }

    fn get_pos(&self) -> &super::ast_node::AstNodePos {
        &self.pos
    }
}
