use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Token {
    Operation(Operation),
    Number(f64),
    Unit(Unit),
    Lparen,
    Rparen,
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
    Convert,
}

#[derive(Debug)]
pub enum Unit {
    Temperature(TemperatureUnit),
    Measurement(MeasurementUnit),
}

#[derive(Debug)]
pub enum TemperatureUnit {
    Farenheit,
    Celcius,
}

#[derive(Debug)]
pub enum MeasurementUnit {
    Feet,
    Inches,
    Meters,
    Centimeters,
    Milimiters,
}

pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Self {
        let source = input.into();

        Self { source }
    }

    pub fn tokenize(self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = self.source.chars().peekable();
        while let Some(ch) = chars.next() {
            match ch {
                '0'..='9' => {
                    let num = self.lex_number(ch, &mut chars);
                    tokens.push(Token::Number(num.parse::<f64>().unwrap()));
                }
                c if c.is_alphabetic() => {
                    let ident = self.lex_identifier(c, &mut chars);

                    let tok: Token = match ident.to_lowercase().as_ref() {
                        "to" => Token::Operation(Operation::Convert),
                        "c" => Token::Unit(Unit::Temperature(TemperatureUnit::Celcius)),
                        "f" => Token::Unit(Unit::Temperature(TemperatureUnit::Farenheit)),
                        "feet" => Token::Unit(Unit::Measurement(MeasurementUnit::Feet)),
                        "in" => Token::Unit(Unit::Measurement(MeasurementUnit::Feet)),
                        "m" => Token::Unit(Unit::Measurement(MeasurementUnit::Meters)),
                        "cm" => Token::Unit(Unit::Measurement(MeasurementUnit::Centimeters)),
                        "mm" => Token::Unit(Unit::Measurement(MeasurementUnit::Feet)),
                        _ => panic!("Crash :("),
                    };

                    tokens.push(tok);
                }
                '+' => tokens.push(Token::Operation(Operation::Add)),
                '-' => tokens.push(Token::Operation(Operation::Subtract)),
                '*' => tokens.push(Token::Operation(Operation::Multiply)),
                '/' => tokens.push(Token::Operation(Operation::Divide)),
                '(' => tokens.push(Token::Lparen),
                ')' => tokens.push(Token::Rparen),
                c if c.is_whitespace() => continue,
                _ => panic!("Unexpected character: {}", ch),
            }
        }
        tokens
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
