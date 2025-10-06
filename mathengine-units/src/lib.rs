pub mod length;
pub mod temperature;

use std::fmt;

/// Core trait for all dimension types, providing unified conversion and parsing operations.
pub trait Dimension {
    /// The unit enum type for this dimension (e.g., LengthUnit, TemperatureUnit)
    type Unit: Copy + PartialEq + std::fmt::Debug;

    /// Parse a unit string into the concrete unit type
    fn parse_unit_str(unit_str: &str) -> Result<Self::Unit, UnitError>;

    /// Convert a value to the base unit for this dimension
    fn to_base_value(unit: Self::Unit, value: f64) -> f64;

    /// Convert a base unit value to the target unit
    fn from_base_value(base_value: f64, target_unit: Self::Unit) -> f64;

    /// Get the base unit for this dimension
    fn base_unit() -> Self::Unit;

    /// Convert between units within the same dimension (compile-time safe)
    fn convert_value(from_unit: Self::Unit, to_unit: Self::Unit, value: f64) -> f64 {
        if from_unit == to_unit {
            value
        } else {
            let base_value = Self::to_base_value(from_unit, value);
            Self::from_base_value(base_value, to_unit)
        }
    }
}

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
