use std::fmt::Display;

/// Error type for unit conversions
#[derive(Debug, Clone, PartialEq)]
pub enum ConversionError {
    /// Attempted to convert between different dimensions (e.g., length to temperature)
    CrossDimension,
    /// Unknown unit string provided
    UnknownUnit(String),
    /// Conversion failed for other reasons
    Failed,
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::CrossDimension => write!(f, "Cannot convert between different dimensions"),
            ConversionError::UnknownUnit(unit) => write!(f, "Unknown unit: '{}'", unit),
            ConversionError::Failed => write!(f, "Conversion failed"),
        }
    }
}

impl std::error::Error for ConversionError {}