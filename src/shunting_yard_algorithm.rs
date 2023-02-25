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

#[cfg(test)]
mod operator_instructions {
    use super::operator_instructions;
    use crate::tokenizer::Token;

    #[test]
    fn it_should_push_to_operator_stack_when_empty() {
        let mut operator_stack: Vec<Token> = vec![];
        let mut output: Vec<Token> = vec![];
        let sample_token: Token = Token::new(crate::tokenizer::TokenType::Addition);
        assert!(operator_stack.is_empty());
        operator_instructions(&mut operator_stack, &mut output, sample_token);
        assert_eq!(
            operator_stack[0],
            Token::new(crate::tokenizer::TokenType::Addition)
        );
        assert_eq!(operator_stack.len(), 1);
    }

    #[test]
    fn it_should_push_to_operator_stack_when_left_paren_at_top_of_stack() {
        let mut operator_stack: Vec<Token> = vec![];
        operator_stack.push(Token::new(crate::tokenizer::TokenType::LeftParenthesis));
        let mut output: Vec<Token> = vec![];
        let sample_token: Token = Token::new(crate::tokenizer::TokenType::Addition);
        assert_eq!(
            operator_stack[0],
            Token::new(crate::tokenizer::TokenType::LeftParenthesis)
        );
        operator_instructions(&mut operator_stack, &mut output, sample_token);
        assert_eq!(
            operator_stack[0],
            Token::new(crate::tokenizer::TokenType::LeftParenthesis)
        );
        assert_eq!(
            operator_stack[1],
            Token::new(crate::tokenizer::TokenType::Addition)
        );
        assert_eq!(operator_stack.len(), 2);
    }

    #[test]
    fn it_should_push_to_operator_stack_when_top_of_stack_has_less_precedence() {
        let mut operator_stack: Vec<Token> = vec![];
        operator_stack.push(Token::new(crate::tokenizer::TokenType::Subtraction));
        let mut output: Vec<Token> = vec![];
        let sample_token: Token = Token::new(crate::tokenizer::TokenType::Division);
        assert_eq!(
            operator_stack[0],
            Token::new(crate::tokenizer::TokenType::Subtraction)
        );
        operator_instructions(&mut operator_stack, &mut output, sample_token);
        assert_eq!(
            operator_stack[0],
            Token::new(crate::tokenizer::TokenType::Subtraction)
        );
        assert_eq!(
            operator_stack[1],
            Token::new(crate::tokenizer::TokenType::Division)
        );
        assert_eq!(operator_stack.len(), 2);
    }

    #[test]
    fn it_should_pop_to_output_and_push_to_stack_when_top_has_greater_precedence() {
        let mut operator_stack: Vec<Token> = vec![];
        operator_stack.push(Token::new(crate::tokenizer::TokenType::Division));
        let mut output: Vec<Token> = vec![];
        let sample_token: Token = Token::new(crate::tokenizer::TokenType::Subtraction);
        assert_eq!(
            operator_stack[0],
            Token::new(crate::tokenizer::TokenType::Division)
        );
        operator_instructions(&mut operator_stack, &mut output, sample_token);
        assert_eq!(output[0], Token::new(crate::tokenizer::TokenType::Division));
        assert_eq!(
            operator_stack[0],
            Token::new(crate::tokenizer::TokenType::Subtraction)
        );
        assert_eq!(operator_stack.len(), 1);
        assert_eq!(output.len(), 1);
    }
    #[test]
    fn it_should_pop_to_output_and_push_to_stack_when_top_has_equal_precedence() {
        let mut operator_stack: Vec<Token> = vec![];
        operator_stack.push(Token::new(crate::tokenizer::TokenType::Addition));
        let mut output: Vec<Token> = vec![];
        let sample_token: Token = Token::new(crate::tokenizer::TokenType::Subtraction);
        assert_eq!(
            operator_stack[0],
            Token::new(crate::tokenizer::TokenType::Addition)
        );
        operator_instructions(&mut operator_stack, &mut output, sample_token);
        assert_eq!(output[0], Token::new(crate::tokenizer::TokenType::Addition));
        assert_eq!(
            operator_stack[0],
            Token::new(crate::tokenizer::TokenType::Subtraction)
        );
        assert_eq!(operator_stack.len(), 1);
        assert_eq!(output.len(), 1);
    }
}

// #[cfg(test)]
// mod right_paren_instructions{
//     use super::right_paren_instructions;

//     #[test]
//     fn it_should_discard_left_paren_when_on_top_of_operator_stack(){

//     }

//     #[test]
//     fn it_should_push_all_other_operators_to_output_and_then_discard_itself_and_left_paren(){

//     }
// }
