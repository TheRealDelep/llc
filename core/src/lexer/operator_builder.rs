use crate::common::operator::{self, Operator};

use super::{
    file_stream::FileLine,
    token::{Token, TokenKind},
};

pub fn build_operator<'a>(line: &mut FileLine) -> Option<Vec<Token>> {
    let mut result = Vec::new();

    match get_operator(line) {
        OperatorBuilderResult::None => return None,
        OperatorBuilderResult::One(op) => result.push(Token::new (
            TokenKind::Operator(op),
            line.number + 1,
            match op.is_composite() {
                true => line.current_index - 1,
                false => line.current_index,
            },
            line.current_index,
        )),
        OperatorBuilderResult::Two(op1, op2) => {
            result.push(Token::single_char (
                TokenKind::Operator(op1),
                line.number + 1,
                line.current_index,
            ));
            result.push(Token::single_char(
                TokenKind::Operator(op2),
                line.number + 1,
                line.current_index + 1,
            ));
        }
    };
    Some(result)
}

enum OperatorBuilderResult {
    None,
    One(Operator),
    Two(Operator, Operator),
}

fn get_operator(line: &mut FileLine) -> OperatorBuilderResult {
    let mut get_op = || -> Option<(char, Operator)> {
        let c = match line.get_next() {
            Some(c) => c,
            None => return None,
        };

        let op = match operator::parse_operator(*c) {
            Some(op) => op,
            None => {
                line.backtrack(1);
                return None;
            }
        };

        Some((*c, op))
    };

    if let Some(res1) = get_op() {
        if let Some(res2) = get_op() {
            let s = &String::from_iter(&[res1.0, res2.0]);
            if let Some(comp_op) = operator::parse_comp_operator(s.as_str()) {
                return OperatorBuilderResult::One(comp_op);
            }
            return OperatorBuilderResult::Two(res1.1, res2.1);
        }
        return OperatorBuilderResult::One(res1.1);
    }

    OperatorBuilderResult::None
}
