use std::{
    iter::{Enumerate, Peekable},
    slice::Iter,
};

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

#[derive(Debug)]
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
    while let Some(x) = char_stream.next_if(|&x| x.1.is_numeric()) {
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
    println!("{expression:?}");
    let mut tokens_vector: Vec<Token> = vec![];
    let mut characters_vector: Vec<char> = vec![];
    let character_reference: Vec<char> = vec!['*', '/', '+', '-', '(', ')', '^'];
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
            } else if character_reference.iter().any(|&i| i == *character.1) {
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
