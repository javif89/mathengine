use crate::ast::Expression;
use crate::error::ParseError;
use crate::types::DimensionType;
use mathengine_lexer::{Operation, Token};
use mathengine_units::{length::LengthDimension, temperature::TemperatureDimension};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    // Entry point for parsing - parses the entire token stream and ensures all tokens are consumed
    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        if self.tokens.is_empty() {
            return Err(ParseError::EmptyTokenStream);
        }

        let expr = self.parse_expression(0)?;
        if self.pos < self.tokens.len() {
            return Err(ParseError::UnexpectedToken {
                expected: "end of input".to_string(),
                found: self.tokens[self.pos].clone(),
                position: self.pos,
            });
        }
        Ok(expr)
    }

    // Pratt parsing algorithm - handles binary operators with correct precedence and associativity
    // min_precedence determines the minimum operator precedence this call will handle
    fn parse_expression(&mut self, min_precedence: u8) -> Result<Expression, ParseError> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Operation(op) => {
                    let precedence = self.get_precedence(op);
                    if precedence < min_precedence {
                        break;
                    }

                    let op = match self.advance() {
                        Some(Token::Operation(o)) => o.clone(),
                        _ => unreachable!(),
                    };

                    let right_precedence = if self.is_right_associative(&op) {
                        precedence
                    } else {
                        precedence + 1
                    };

                    let right = self.parse_expression(right_precedence)?;
                    left = Expression::Binary {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Parses primary expressions: numbers, parenthesized expressions, and unary operators
    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        let start_pos = self.pos;
        match self.advance() {
            Some(Token::Number(n)) => Ok(Expression::Number(*n)),
            Some(Token::UnitValue { value, unit }) => Ok(Expression::UnitValue {
                value: *value,
                unit: canonicalize_unit(unit),
            }),
            Some(Token::Unit(unit)) => Ok(Expression::Unit(unit.clone())),
            Some(Token::Lparen) => {
                let expr = self.parse_expression(0)?;
                match self.advance() {
                    Some(Token::Rparen) => Ok(expr),
                    Some(other) => Err(ParseError::UnexpectedToken {
                        expected: "')'".to_string(),
                        found: other.clone(),
                        position: self.pos - 1,
                    }),
                    None => Err(ParseError::UnexpectedEndOfInput {
                        expected: "')'".to_string(),
                    }),
                }
            }
            Some(Token::Operation(Operation::Subtract)) => {
                let operand = self.parse_primary()?;
                Ok(Expression::Unary {
                    op: Operation::Subtract,
                    operand: Box::new(operand),
                })
            }
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: "number, unit value, '(', or unary operator".to_string(),
                found: token.clone(),
                position: start_pos,
            }),
            None => Err(ParseError::UnexpectedEndOfInput {
                expected: "expression".to_string(),
            }),
        }
    }

    // Returns the current token without consuming it
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    // Consumes and returns the current token, advancing the position
    fn advance(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    // Returns the precedence level for each operator (higher number = higher precedence)
    fn get_precedence(&self, op: &Operation) -> u8 {
        match op {
            Operation::Add | Operation::Subtract => 1,
            Operation::Multiply | Operation::Divide => 2,
            Operation::Power => 3,
            Operation::Convert => 5,
        }
    }

    // Determines if an operator is right-associative (currently all ops are left-associative)
    fn is_right_associative(&self, op: &Operation) -> bool {
        match op {
            Operation::Power => true, // Power is right-associative: 2^3^4 = 2^(3^4)
            _ => false,
        }
    }
}

fn canonicalize_unit(unit: &str) -> String {
    match DimensionType::from_unit(unit) {
        DimensionType::Length => LengthDimension::parse_unit(unit)
            .unwrap()
            .canonical_string()
            .into(),
        DimensionType::Temperature => TemperatureDimension::parse_unit(unit)
            .unwrap()
            .canonical_string()
            .into(),
        DimensionType::Unknown => "unknown".into(),
    }
}
