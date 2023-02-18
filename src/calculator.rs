use crate::shunting_yard_algorithm;
use crate::tokenizer;

pub fn evaluate_expression(formula: &str, variable: f32) -> f32 {
    let expression = if formula.contains('x') {
        formula.replace('x', &variable.to_string())
    } else {
        formula.to_string()
    };
    let tokenized_expression: Vec<tokenizer::Token> = tokenizer::tokenizer(&expression);
    let rpn_ordered_expression: Vec<tokenizer::Token> =
        shunting_yard_algorithm::convert_into_rpn(tokenized_expression);
    let mut stack = vec![];
    for token in rpn_ordered_expression {
        if token.value().is_ok() {
            stack.push(token);
            continue;
        }

        let rhs = stack.pop();
        let lhs = stack.pop();

        match (lhs, rhs) {
            (Some(a), Some(b)) => {
                let a_f32 = a.value().unwrap();
                let b_f32 = b.value().unwrap();
                let r = match token.token_type {
                    tokenizer::TokenType::Multiplication => a_f32 * b_f32,
                    tokenizer::TokenType::Division => a_f32 / b_f32,
                    tokenizer::TokenType::Addition => a_f32 + b_f32,
                    tokenizer::TokenType::Subtraction => a_f32 - b_f32,
                    _ => panic!("calculation not implemented for this token type"),
                };
                stack.push(tokenizer::Token::new(tokenizer::TokenType::Number(r)));
            }
            (None, Some(b)) => return b.value().unwrap(),
            _ => panic!("the stack should never look like this"),
        }
    }
    stack.pop().map_or_else(
        || {
            panic!("there should be a result at the end of the stack");
        },
        |t| {
            t.value().map_or_else(
                |_| {
                    panic!("how did a operator get here");
                },
                |f| f,
            )
        },
    )
}
