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
    pub param_id: Option<usize>,
    pub pos: AstNodePos,
}

impl FunctionCall {
    pub(in crate::parser) fn parse(
        stream: &mut TokenStream,
        file_ast: &mut FileAst,
    ) -> ParsingResult {
        let op_pos = match stream.take_if(|t| match t.value {
            TokenValue::Operator(Operator::Return) => Some(AstNodePos::from_token(&t)),
            _ => None
        }) {
            Some(pos) => pos,
            None => return ParsingResult::Other
        };

        let param_id = match &file_ast.nodes[file_ast.nodes.len() - 1] {
            AstNode::Expression(exp) => Some(file_ast.nodes.len() - 1),
            AstNode::Statement(_) => None
        };

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

        

        let exp = &file_ast.nodes[identifier_id];

        let pos = match param_id {
            Some(id) => {
                let param = &file_ast.nodes[id];
                AstNodePos::from_nodes(param, exp)
            },
            None => AstNodePos::combine(&op_pos, exp.get_pos())
        };

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
        let id = file_ast.nodes[self.identifier_id].print(file_ast);
        let param = match self.param_id {
            Some(id) => file_ast.nodes[id].print(file_ast),
            None => "".to_string()
        };

        format!("Function Call (Identifier: {0}, Params: [{1}])", id, param)
    }

    fn get_pos(&self) -> &super::ast_node::AstNodePos {
        &self.pos
    }
}
