use crate::tokenizer;

fn operator_instructions(
    stack: &mut Vec<tokenizer::Token>,
    output: &mut Vec<tokenizer::Token>,
    token: tokenizer::Token,
) {
    while !stack.is_empty() {
        let top_of_stack = stack
            .last()
            .expect("while loop is ensuring stack is not empty");
        if top_of_stack.token_type != tokenizer::TokenType::LeftParenthesis
            && top_of_stack.token_precedence >= token.token_precedence
        {
            output.push(
                stack
                    .pop()
                    .expect("while loop is still protecting us from an empty vector"),
            );
        } else {
            break;
        }
    }
    stack.push(token);
}
fn right_paren_instructions(stack: &mut Vec<tokenizer::Token>, output: &mut Vec<tokenizer::Token>) {
    while !stack.is_empty() {
        let top_of_stack = stack
            .last()
            .expect("while loop is ensuring stack is not empty");
        if top_of_stack.token_type == tokenizer::TokenType::LeftParenthesis {
            stack.pop();
            break;
        }

        output.push(
            stack
                .pop()
                .expect("while loop is still protecting us from an empty vector"),
        );
    }
}
pub fn convert_into_rpn(tokenized_expression: Vec<tokenizer::Token>) -> Vec<tokenizer::Token> {
    let input = tokenized_expression;
    let mut output: Vec<tokenizer::Token> = vec![];
    let mut operator_stack: Vec<tokenizer::Token> = vec![];
    for t in input {
        match t.token_type {
            tokenizer::TokenType::Number(_) => output.push(t),
            tokenizer::TokenType::Multiplication
            | tokenizer::TokenType::Division
            | tokenizer::TokenType::Addition
            | tokenizer::TokenType::Subtraction => {
                operator_instructions(&mut operator_stack, &mut output, t);
            }
            tokenizer::TokenType::LeftParenthesis => operator_stack.push(t),
            tokenizer::TokenType::RightParenthesis => {
                right_paren_instructions(&mut operator_stack, &mut output);
            }
        }
    }
    while !operator_stack.is_empty() {
        output.push(
            operator_stack
                .pop()
                .expect("while loop condition should protect us here"),
        );
    }
    output
}
