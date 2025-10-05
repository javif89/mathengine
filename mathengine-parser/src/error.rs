use std::fmt;
use mathengine_lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: Token,
        position: usize,
    },
    UnexpectedEndOfInput {
        expected: String,
    },
    InvalidExpression {
        message: String,
        position: usize,
    },
    EmptyTokenStream,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found, position } => {
                write!(f, "Expected {} but found {:?} at position {}", expected, found, position)
            }
            ParseError::UnexpectedEndOfInput { expected } => {
                write!(f, "Expected {} but reached end of input", expected)
            }
            ParseError::InvalidExpression { message, position } => {
                write!(f, "Invalid expression at position {}: {}", position, message)
            }
            ParseError::EmptyTokenStream => {
                write!(f, "Cannot parse empty token stream")
            }
        }
    }
}

impl std::error::Error for ParseError {}