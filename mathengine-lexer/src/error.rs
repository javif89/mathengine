use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnexpectedCharacter { char: char, position: usize },
    InvalidNumber { input: String, position: usize },
    EmptyInput,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::UnexpectedCharacter { char, position } => {
                write!(
                    f,
                    "Unexpected character '{}' at position {}",
                    char, position
                )
            }
            LexError::InvalidNumber { input, position } => {
                write!(f, "Invalid number '{}' at position {}", input, position)
            }
            LexError::EmptyInput => {
                write!(f, "Empty input provided")
            }
        }
    }
}

impl std::error::Error for LexError {}
