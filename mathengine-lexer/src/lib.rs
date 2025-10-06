use std::{iter::Peekable, str::Chars};

pub mod error;
pub use error::LexError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operation(Operation),
    Number(f64),
    UnitValue { value: f64, unit: String },
    Unit(String),
    Lparen,
    Rparen,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
    Power,
    Convert,
}

pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Self {
        let source = input.into();

        Self { source }
    }

    pub fn tokenize(self) -> Result<Vec<Token>, LexError> {
        if self.source.trim().is_empty() {
            return Err(LexError::EmptyInput);
        }

        let mut tokens = Vec::new();
        let mut chars = self.source.chars().peekable();
        let mut position = 0;

        while let Some(ch) = chars.next() {
            match ch {
                '0'..='9' => {
                    let start_pos = position;
                    let num = self.lex_number(ch, &mut chars);
                    position += num.len();

                    // Skip whitespace after number
                    while let Some(&c) = chars.peek() {
                        if c.is_whitespace() {
                            chars.next();
                            position += 1;
                        } else {
                            break;
                        }
                    }
                    // Check if there's a unit attached (with or without space)
                    if let Some(&c) = chars.peek() {
                        if c.is_alphabetic() {
                            let unit = self.lex_identifier(chars.next().unwrap(), &mut chars);
                            position += unit.len();
                            let value =
                                num.parse::<f64>().map_err(|_| LexError::InvalidNumber {
                                    input: num.clone(),
                                    position: start_pos,
                                })?;
                            tokens.push(Token::UnitValue { value, unit });
                        } else {
                            let value =
                                num.parse::<f64>().map_err(|_| LexError::InvalidNumber {
                                    input: num.clone(),
                                    position: start_pos,
                                })?;
                            tokens.push(Token::Number(value));
                        }
                    } else {
                        let value = num.parse::<f64>().map_err(|_| LexError::InvalidNumber {
                            input: num.clone(),
                            position: start_pos,
                        })?;
                        tokens.push(Token::Number(value));
                    }
                }
                c if c.is_alphabetic() => {
                    let ident = self.lex_identifier(c, &mut chars);
                    position += ident.len();

                    let tok: Token = match ident.to_lowercase().as_ref() {
                        "to" => Token::Operation(Operation::Convert),
                        v => Token::Unit(v.into()),
                    };

                    tokens.push(tok);
                }
                '+' => {
                    tokens.push(Token::Operation(Operation::Add));
                    position += 1;
                }
                '-' => {
                    tokens.push(Token::Operation(Operation::Subtract));
                    position += 1;
                }
                '*' => {
                    tokens.push(Token::Operation(Operation::Multiply));
                    position += 1;
                }
                '/' => {
                    tokens.push(Token::Operation(Operation::Divide));
                    position += 1;
                }
                '^' => {
                    tokens.push(Token::Operation(Operation::Power));
                    position += 1;
                }
                '(' => {
                    tokens.push(Token::Lparen);
                    position += 1;
                }
                ')' => {
                    tokens.push(Token::Rparen);
                    position += 1;
                }
                c if c.is_whitespace() => {
                    position += 1;
                    continue;
                }
                _ => {
                    return Err(LexError::UnexpectedCharacter { char: ch, position });
                }
            }
        }
        Ok(tokens)
    }

    fn lex_number(&self, first_digit: char, chars: &mut Peekable<Chars<'_>>) -> String {
        let mut s = first_digit.to_string();
        while let Some(&next) = chars.peek() {
            if next.is_ascii_digit() || next == '.' {
                s.push(next);
                chars.next();
            } else {
                break;
            }
        }

        s
    }

    fn lex_identifier(&self, first_char: char, chars: &mut Peekable<Chars<'_>>) -> String {
        let mut ident = String::new();
        ident.push(first_char);

        while let Some(&next) = chars.peek() {
            if next.is_alphanumeric() {
                ident.push(next);
                chars.next();
            } else {
                break;
            }
        }

        ident
    }
}
