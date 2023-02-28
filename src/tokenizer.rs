use std::{
    iter::{Enumerate, Peekable},
    slice::Iter,
};

const CHARACTER_REFERENCE: [char; 6] = ['*', '/', '+', '-', '(', ')'];

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    Multiplication,
    Division,
    Addition,
    Subtraction,
    Number(f32),
}
impl TokenType {
    const fn as_f32(&self) -> Result<f32, bool> {
        match self {
            Self::Number(n) => Ok(*n),
            _ => Err(false),
        }
    }
}

impl TryFrom<&char> for TokenType {
    type Error = &'static str;
    fn try_from(c: &char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Self::LeftParenthesis),
            ')' => Ok(Self::RightParenthesis),
            '*' => Ok(Self::Multiplication),
            '/' => Ok(Self::Division),
            '+' => Ok(Self::Addition),
            '-' => Ok(Self::Subtraction),
            _ => Err("that is a strange character you have there"),
        }
    }
}
impl TryFrom<String> for TokenType {
    type Error = &'static str;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse::<f32>()
            .map_or(Err("problem parsing string into f32"), |f| {
                Ok(Self::Number(f))
            })
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub token_precedence: u8,
}
impl Token {
    pub const fn new(token_type: TokenType) -> Self {
        Self {
            token_precedence: match token_type {
                TokenType::Multiplication | TokenType::Division => 2,
                TokenType::Addition | TokenType::Subtraction => 1,
                _ => 0,
            },
            token_type,
        }
    }
    pub fn value(&self) -> Result<f32, bool> {
        self.token_type.as_f32().map_or(Err(false), Ok)
    }
}

fn build_multidigit(
    string_seed: char,
    char_stream: &mut Peekable<Enumerate<Iter<char>>>,
    iterate_stream: bool,
) -> String {
    let mut concatenated_numeric = String::from(string_seed.to_owned());
    if iterate_stream {
        char_stream.next();
    }
    while let Some(x) = char_stream.next_if(|&x| x.1.is_numeric() || x.1 == &'.') {
        concatenated_numeric.push(x.1.to_owned());
    }
    concatenated_numeric
}

fn push_token(v: &mut Vec<Token>, c: Option<&char>, s: Option<String>) {
    if let Some(char) = c {
        if let Ok(token_type) = TokenType::try_from(char) {
            v.push(Token::new(token_type));
        }
    } else if let Some(string) = s {
        if let Ok(token_type) = TokenType::try_from(string) {
            v.push(Token::new(token_type));
        }
    } else {
        panic!("couldn't push token to vector");
    }
}

pub fn tokenizer(expression: &str) -> Vec<Token> {
    let mut tokens_vector: Vec<Token> = vec![];
    let mut characters_vector: Vec<char> = vec![];
    for character in expression.chars() {
        characters_vector.push(character);
    }
    let mut char_stream = characters_vector.iter().enumerate().peekable();
    // parse characters and convert to tokens
    while let Some(character) = char_stream.next() {
        // Always push right parens and continue
        if *character.1 == ')' {
            push_token(&mut tokens_vector, Some(character.1), None);
            continue;
        }
        // while there are still characters in the string
        if let Some(next_character) = char_stream.peek() {
            // check for negative numbers at the beginning
            if character.0 == 0 && *character.1 == '-' && next_character.1.is_numeric() {
                push_token(
                    &mut tokens_vector,
                    None,
                    Some(build_multidigit('-', &mut char_stream, false)),
                );
            } else if CHARACTER_REFERENCE.iter().any(|&i| i == *character.1) {
                // look at next character for minus char
                push_token(&mut tokens_vector, Some(character.1), None);
                if *next_character.1 == '-' {
                    push_token(
                        &mut tokens_vector,
                        None,
                        Some(build_multidigit(
                            next_character.1.to_owned(),
                            &mut char_stream,
                            true,
                        )),
                    );
                }
            } else if character.1.is_numeric() {
                push_token(
                    &mut tokens_vector,
                    None,
                    Some(build_multidigit(
                        character.1.to_owned(),
                        &mut char_stream,
                        false,
                    )),
                );
            } else {
                push_token(&mut tokens_vector, Some(character.1), None);
            }
            // TODO check your understanding of how this works, may need to remove!
        } else if character.1.is_numeric() {
            push_token(
                &mut tokens_vector,
                None,
                Some(build_multidigit(
                    character.1.to_owned(),
                    &mut char_stream,
                    false,
                )),
            );
        } else {
            push_token(&mut tokens_vector, Some(character.1), None);
        }
    }
    tokens_vector
}

#[cfg(test)]
mod token_type {
    use super::TokenType;
    use super::CHARACTER_REFERENCE;

    #[test]

    fn as_f32_given_number_variant_return_f32() {
        let token = TokenType::Number(548.125);
        assert!(
            (token.as_f32().unwrap() - 548.125).abs() < f32::EPSILON,
            "it should return an f32 from a number variant"
        );
    }
    #[test]

    fn as_f32_given_non_number_variant_return_error() {
        let token = TokenType::Subtraction;
        assert_eq!(
            token.as_f32(),
            Err(false),
            "it should return an error when attempting to convert a non number variant to f32"
        );
    }
    #[test]

    fn try_from_char_given_predefined_char_return_variant() {
        for character in CHARACTER_REFERENCE {
            match character {
                '(' => assert_eq!(
                    TokenType::try_from(&character),
                    Ok(TokenType::LeftParenthesis),
                    "it should take a predefined char and return a variant of TokenType"
                ),
                ')' => assert_eq!(
                    TokenType::try_from(&character),
                    Ok(TokenType::RightParenthesis),
                    "it should take a predefined char and return a variant of TokenType"
                ),
                '*' => assert_eq!(
                    TokenType::try_from(&character),
                    Ok(TokenType::Multiplication),
                    "it should take a predefined char and return a variant of TokenType"
                ),
                '/' => assert_eq!(
                    TokenType::try_from(&character),
                    Ok(TokenType::Division),
                    "it should take a predefined char and return a variant of TokenType"
                ),
                '+' => assert_eq!(
                    TokenType::try_from(&character),
                    Ok(TokenType::Addition),
                    "it should take a predefined char and return a variant of TokenType"
                ),
                '-' => assert_eq!(
                    TokenType::try_from(&character),
                    Ok(TokenType::Subtraction),
                    "it should take a predefined char and return a variant of TokenType"
                ),
                _ => unreachable!(),
            }
        }
    }
    #[test]

    fn try_from_char_given_undefined_char_return_error() {
        assert_eq!(
            TokenType::try_from(&'%'),
            Err("that is a strange character you have there"),
            "it should return an error when you send an undefined char"
        );
    }
    #[test]

    fn try_from_string_given_string_of_numeric_chars_return_number_variant() {
        assert_eq!(
            TokenType::try_from("-456715.54".to_string()),
            Ok(TokenType::Number(-456_715.54)),
            "it should take a string of num chars this and return a number variant"
        );
    }
    #[test]

    fn try_from_string_given_string_of_non_numeric_or_mixed_chars_return_number_variant() {
        assert_eq!(
            TokenType::try_from("-4h56a715#.54".to_string()),
            Err("problem parsing string into f32"),
            "it should return an error when you send non numeric chars in the string"
        );
    }
}

#[cfg(test)]
mod token {
    use super::Token;
    use super::TokenType;

    #[test]

    fn token_new_given_token_type_return_token() {
        assert!(
            matches!(
                Token::new(TokenType::Multiplication),
                Token {
                    token_type: TokenType::Multiplication,
                    token_precedence: 2
                }
            ),
            "it should create a new token given a token type"
        );
    }
    #[test]

    fn token_new_check_precedence() {
        assert_eq!(
            Token::new(TokenType::Multiplication).token_precedence,
            2,
            "it should apply the correct token precedence when given a token type"
        );
        assert_eq!(
            Token::new(TokenType::Addition).token_precedence,
            1,
            "it should apply the correct token precedence when given a token type"
        );
        assert_eq!(
            Token::new(TokenType::Number(76.0)).token_precedence,
            0,
            "it should apply the correct token precedence when given a token type"
        );
    }
    #[test]

    fn token_value_given_number_token_type_return_f32() {
        assert!(
            (Token::new(TokenType::Number(57.0)).value().unwrap() - 57.0).abs() < f32::EPSILON,
            "it should return an f32 when value method is call on a token with token type number"
        );
    }

    #[test]

    fn token_value_given_non_number_token_type_return_bool() {
        assert!(
            (Token::new(TokenType::Subtraction).value() == Err(false)),
            "it should return false when given a non numeric token type"
        );
    }
}

#[cfg(test)]
mod build_multidigit {
    use super::build_multidigit;
    #[test]

    fn build_multidigit_given_stream_returns_string() {
        let mut characters_vector: Vec<char> = vec![];
        for character in String::from("451321").chars() {
            characters_vector.push(character);
        }
        let mut char_stream = characters_vector.iter().enumerate().peekable();
        assert_eq!(
            build_multidigit('-', &mut char_stream, false),
            "-451321",
            "it should return a string from a character stream"
        );
    }

    #[test]

    fn build_multidigit_given_stream_with_non_numeric_mixed_returns_numeric_string() {
        let mut characters_vector: Vec<char> = vec![];
        for character in String::from("451$32%1").chars() {
            characters_vector.push(character);
        }
        let mut char_stream = characters_vector.iter().enumerate().peekable();
        assert_eq!(
            build_multidigit('-', &mut char_stream, false),
            "-451",
            "it should return a string from a non numeric mixed character stream"
        );
    }

    #[test]

    fn build_multidigit_given_stream_iteration_returns_string_that_skips_first_character() {
        let mut characters_vector: Vec<char> = vec![];
        for character in String::from("451321").chars() {
            characters_vector.push(character);
        }
        let mut char_stream = characters_vector.iter().enumerate().peekable();
        assert_eq!(
            build_multidigit('-', &mut char_stream, true),
            "-51321",
            "it should return a string from a character stream with the first character removed"
        );
    }

    #[test]
    fn it_should_create_multidigit_strings_from_floats() {
        let mut characters_vector: Vec<char> = vec![];
        for character in String::from("64.325").chars() {
            characters_vector.push(character);
        }
        let mut char_stream = characters_vector.iter().enumerate().peekable();
        assert_eq!(build_multidigit('6', &mut char_stream, true), "64.325");
    }
}

#[cfg(test)]
mod push_token {
    use super::push_token;
    use super::Token;
    use super::TokenType;

    #[test]
    fn push_token_given_char_successfully_pushes_to_vector() {
        let mut token_vector: Vec<Token> = vec![];
        push_token(&mut token_vector, Some(&')'), None);
        assert!(
            token_vector.contains(&Token {
                token_type: TokenType::RightParenthesis,
                token_precedence: 0
            }),
            "it should push to a vector when passed a char"
        );
    }
    #[test]
    fn push_token_given_string_successfully_pushes_to_vector() {
        let mut token_vector: Vec<Token> = vec![];
        push_token(&mut token_vector, None, Some("-574".to_string()));
        assert!(
            token_vector.contains(&Token {
                token_type: TokenType::Number(-574.0),
                token_precedence: 0
            }),
            "it should push to a vector when given a string"
        );
    }
    #[test]
    #[should_panic(expected = "couldn't push token to vector")]
    fn push_token_given_nothing_panics() {
        push_token(&mut vec![], None, None);
    }
}

#[cfg(test)]
mod tokenizer_test {

    use super::{tokenizer, Token, TokenType};

    #[test]
    fn tokenizer_given_expression_returns_vector_of_tokens() {
        assert_eq!(
            tokenizer("-20(-50-39)*-5/9+46"),
            vec![
                Token {
                    token_type: TokenType::Number(-20.0),
                    token_precedence: 0
                },
                Token {
                    token_type: TokenType::LeftParenthesis,
                    token_precedence: 0
                },
                Token {
                    token_type: TokenType::Number(-50.0),
                    token_precedence: 0
                },
                Token {
                    token_type: TokenType::Subtraction,
                    token_precedence: 1
                },
                Token {
                    token_type: TokenType::Number(39.0),
                    token_precedence: 0
                },
                Token {
                    token_type: TokenType::RightParenthesis,
                    token_precedence: 0
                },
                Token {
                    token_type: TokenType::Multiplication,
                    token_precedence: 2
                },
                Token {
                    token_type: TokenType::Number(-5.0),
                    token_precedence: 0
                },
                Token {
                    token_type: TokenType::Division,
                    token_precedence: 2
                },
                Token {
                    token_type: TokenType::Number(9.0),
                    token_precedence: 0
                },
                Token {
                    token_type: TokenType::Addition,
                    token_precedence: 1
                },
                Token {
                    token_type: TokenType::Number(46.0),
                    token_precedence: 0
                }
            ]
        );
    }
}
