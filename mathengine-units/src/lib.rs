pub mod length;
pub mod temperature;

use std::fmt;

/// Common behavior for all unit types (m, cm, F, C, etc.)
pub trait UnitType: Copy + PartialEq + std::fmt::Debug + 'static {
    /// Get the canonical string representation (e.g., "m", "cm", "F")
    fn canonical_string(&self) -> &'static str;

    /// Parse a unit string into this unit type
    fn parse(s: &str) -> Result<Self, UnitError>;

    /// Get the dimension name for this unit type
    fn dimension_name() -> &'static str;
}

/// Conversion logic for each unit family
pub trait UnitConversion<U: UnitType> {
    /// Convert a value to the base unit for this dimension
    fn to_base_value(unit: U, value: f64) -> f64;

    /// Convert a base unit value to the target unit
    fn from_base_value(base_value: f64, unit: U) -> f64;

    /// Get the base unit for this dimension
    fn base_unit() -> U;

    /// Direct conversion between units (for precision), returns None if not available
    fn convert_direct(_from: U, _to: U, _value: f64) -> Option<f64> {
        None // Default: no direct conversion available
    }
}

/// Generic dimension type that eliminates all duplication
pub struct Dimension<U: UnitType> {
    value: f64,
    unit: U,
}

impl<U: UnitType> Dimension<U>
where
    Self: UnitConversion<U>
{
    /// Create a new dimension with the given value and unit
    pub fn new(value: f64, unit: U) -> Self {
        Self { value, unit }
    }

    /// Create a dimension from a unit string and value
    pub fn from_unit(unit_str: &str, value: f64) -> Result<Self, UnitError> {
        let unit = U::parse(unit_str)?;
        Ok(Self::new(value, unit))
    }

    /// Convert this dimension to a different unit
    pub fn convert_to(&self, target: U) -> Self {
        let new_value = if self.unit == target {
            self.value
        } else if let Some(direct_value) = Self::convert_direct(self.unit, target, self.value) {
            // Use direct conversion for precision when available
            direct_value
        } else {
            // Fall back to base unit conversion
            let base_value = Self::to_base_value(self.unit, self.value);
            Self::from_base_value(base_value, target)
        };

        Self::new(new_value, target)
    }

    /// Get the numeric value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get the unit
    pub fn unit(&self) -> U {
        self.unit
    }

    /// Parse a unit string into the unit type
    pub fn parse_unit(unit_str: &str) -> Result<U, UnitError> {
        U::parse(unit_str)
    }

    /// Convert a value between units (static method)
    pub fn convert_value(from_unit: U, to_unit: U, value: f64) -> f64 {
        if from_unit == to_unit {
            value
        } else if let Some(direct_value) = Self::convert_direct(from_unit, to_unit, value) {
            direct_value
        } else {
            let base_value = Self::to_base_value(from_unit, value);
            Self::from_base_value(base_value, to_unit)
        }
    }
}

impl<U: UnitType> fmt::Display for Dimension<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.unit.canonical_string())
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
