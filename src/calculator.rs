use crate::shunting_yard_algorithm;
use crate::tokenizer;

pub fn evaluate_expression(formula: &str, variable: f32) -> f32 {
    let expression = formula.replace('x', &variable.to_string());
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
                    _ => unreachable!("calculation not implemented for this token type"),
                };
                stack.push(tokenizer::Token::new(tokenizer::TokenType::Number(r)));
            }
            (None, Some(b)) => return b.value().unwrap(),
            _ => unreachable!("the stack should never look like this"),
        }
    }
    stack.pop().map_or_else(
        || {
            panic!("there should be a result at the end of the stack");
        },
        |t| {
            t.value().map_or_else(
                |_| {
                    panic!("how did an operator get here");
                },
                |f| f,
            )
        },
    )
}

#[cfg(test)]
mod evaluate_expression {
    use super::evaluate_expression;
    #[test]
    fn it_should_evaluate_expressions() {
        assert!((evaluate_expression("x+65.2", 20.5) - 85.7).abs() < f32::EPSILON);
        assert!((evaluate_expression("x-65.2", 20.5) - -44.699_997).abs() < f32::EPSILON);
        assert!((evaluate_expression("x*65.2", 20.5) - 1336.6).abs() < f32::EPSILON);
        assert!((evaluate_expression("x/65.2", 20.5) - 0.314_417_18).abs() < f32::EPSILON);
        assert!((evaluate_expression("(x-15)+65.2", 20.5) - 70.7).abs() < f32::EPSILON);
        assert!(
            (evaluate_expression("((x-15)*65.2/150) + x", 10.0) - 7.826_667).abs() < f32::EPSILON
        );
        assert!(
            (evaluate_expression("((x-15)*-65.2/150) + x", 10.0) - 12.173_333).abs() < f32::EPSILON
        );
    }
}
