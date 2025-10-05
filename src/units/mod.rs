use std::fmt;

pub mod length;
pub mod temperature;

#[derive(Debug, Clone, PartialEq)]
pub enum UnitError {
    UnknownUnit(String),
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitError::UnknownUnit(unit) => write!(f, "Unknown unit: '{}'", unit),
        }
    }
}

impl std::error::Error for UnitError {}
