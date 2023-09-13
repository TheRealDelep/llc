use super::{file_stream::FileLine, token::{Token, TokenValue, Operator, self}};

pub fn build_operator<'a>(line: &mut FileLine, filename: &'a str) -> Option<Vec<Token>> {
    let mut result = Vec::new();

    match get_operator(line) {
        OperatorBuilderResult::None => return None,
        OperatorBuilderResult::One(op) => result.push(Token {
            line_number: line.number + 1,
            from: match op.is_composite(){
                true => line.current_index - 1,
                false => line.current_index,
            },
            to: line.current_index,
            value: TokenValue::Operator(op),
        }),
        OperatorBuilderResult::Two(op1, op2) => {
            result.push(Token {
                line_number: line.number + 1,
                from: line.current_index,
                to: line.current_index,
                value: TokenValue::Operator(op1),
            });
            result.push(Token {
                line_number: line.number + 1,
                from: line.current_index + 1,
                to: line.current_index + 1,
                value: TokenValue::Operator(op2),
            });
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

        let op = match token::parse_operator(*c) {
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
            if let Some(comp_op) = token::parse_comp_operator(s.as_str()) {
                return OperatorBuilderResult::One(comp_op);
            }
            return OperatorBuilderResult::Two(res1.1, res2.1);
        }
        return OperatorBuilderResult::One(res1.1);
    }

    OperatorBuilderResult::None
}
