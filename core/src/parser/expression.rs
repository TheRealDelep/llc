use crate::lexer::{token::TokenValue, token_stream::TokenStream};

use super::{
    ast_node::{AstNodeData, AstNodePos, Parsable, ParsingResult},
    errors::SyntaxError,
    function::Function,
    indentifier::Identifier,
    literal::Literal,
    parser::{FileAst, ParserBuffer},
};

pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Function(Function),
}

impl Parsable for Expression {
    fn parse(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
        match stream.peek(0).value {
            TokenValue::Identifier(_) | TokenValue::Literal(_) | TokenValue::OpenCurly => {}
            _ => return ParsingResult::Other,
        }

        if let ParsingResult::Ok(id) = parse_single_token_exp(stream, buffer) {
            return ParsingResult::Ok(id);
        }

        if let ParsingResult::Ok(id) = Function::parse(stream, buffer) {
            return ParsingResult::Ok(id);
        }

        let first = stream.peek(0);
        let second = stream.peek(1);
        let reason = format!(
            "Unexpected token encountered while trying to parse expression. Expected one of(;, function call, function body, operator) after {0} but found {1}", 
            first.value, second.value);

        buffer.errors.push(SyntaxError::from_token(
            stream.peek(1),
            Some(reason.into_boxed_str()),
        ));
        ParsingResult::Error
    }
}

fn parse_single_token_exp(stream: &mut TokenStream, buffer: &mut ParserBuffer) -> ParsingResult {
    if let ParsingResult::Ok(id) = Identifier::parse(stream, buffer) {
        return ParsingResult::Ok(id);
    }

    if let ParsingResult::Ok(id) = Literal::parse(stream, buffer) {
        return ParsingResult::Ok(id);
    }

    ParsingResult::Other
}

impl AstNodeData for Expression {
    fn print(&self, file_ast: &FileAst) -> String {
        match self {
            Self::Identifier(id) => id.print(file_ast),
            Self::Literal(lit) => lit.print(file_ast),
            Self::Function(func) => func.print(file_ast),
        }
    }

    fn get_pos(&self) -> &AstNodePos {
        match self {
            Self::Identifier(id) => id.get_pos(),
            Self::Literal(lit) => lit.get_pos(),
            Self::Function(func) => func.get_pos(),
        }
    }
}
