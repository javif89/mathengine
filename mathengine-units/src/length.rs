use crate::UnitError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LengthUnit {
    Meter,
    Centimeter,
    Millimeter,
    Kilometer,
    Foot,
    Inch,
    Yard,
    Mile,
}

#[derive(Debug, Clone)]
pub struct LengthDimension {
    value: f64,
    unit: LengthUnit,
}

impl LengthDimension {
    // Unit constants for clean conversion API
    pub const METERS: LengthUnit = LengthUnit::Meter;
    pub const CENTIMETERS: LengthUnit = LengthUnit::Centimeter;
    pub const MILLIMETERS: LengthUnit = LengthUnit::Millimeter;
    pub const KILOMETERS: LengthUnit = LengthUnit::Kilometer;
    pub const FEET: LengthUnit = LengthUnit::Foot;
    pub const INCH: LengthUnit = LengthUnit::Inch;
    pub const INCHES: LengthUnit = LengthUnit::Inch;
    pub const YARDS: LengthUnit = LengthUnit::Yard;
    pub const MILES: LengthUnit = LengthUnit::Mile;

    /// Create a LengthDimension from a unit string and value
    /// Example: LengthDimension::from_unit("cm", 10.0)
    pub fn from_unit(unit_str: &str, value: f64) -> Result<Self, UnitError> {
        let unit = Self::parse_unit(unit_str)?;
        Ok(Self { value, unit })
    }

    /// Create a LengthDimension directly with a LengthUnit
    pub fn new(value: f64, unit: LengthUnit) -> Self {
        Self { value, unit }
    }

    /// Parse a string into a LengthUnit
    pub fn parse_unit(s: &str) -> Result<LengthUnit, UnitError> {
        match s.to_lowercase().as_str() {
            "m" | "meter" | "meters" => Ok(LengthUnit::Meter),
            "cm" | "centimeter" | "centimeters" => Ok(LengthUnit::Centimeter),
            "mm" | "millimeter" | "millimeters" => Ok(LengthUnit::Millimeter),
            "km" | "kilometer" | "kilometers" => Ok(LengthUnit::Kilometer),
            "ft" | "foot" | "feet" => Ok(LengthUnit::Foot),
            "in" | "inch" | "inches" => Ok(LengthUnit::Inch),
            "yd" | "yard" | "yards" => Ok(LengthUnit::Yard),
            "mi" | "mile" | "miles" => Ok(LengthUnit::Mile),
            _ => Err(UnitError::UnknownUnit(s.to_string())),
        }
    }

    /// Convert this length to meters (base unit)
    fn to_meters(&self) -> f64 {
        match self.unit {
            LengthUnit::Meter => self.value,
            LengthUnit::Centimeter => self.value / 100.0,
            LengthUnit::Millimeter => self.value / 1000.0,
            LengthUnit::Kilometer => self.value * 1000.0,
            LengthUnit::Foot => self.value * 0.3048,
            LengthUnit::Inch => self.value * 0.0254,
            LengthUnit::Yard => self.value * 0.9144,
            LengthUnit::Mile => self.value * 1609.344,
        }
    }

    /// Convert meters to the specified unit
    fn from_meters(meters: f64, unit: LengthUnit) -> f64 {
        match unit {
            LengthUnit::Meter => meters,
            LengthUnit::Centimeter => meters * 100.0,
            LengthUnit::Millimeter => meters * 1000.0,
            LengthUnit::Kilometer => meters / 1000.0,
            LengthUnit::Foot => meters / 0.3048,
            LengthUnit::Inch => meters / 0.0254,
            LengthUnit::Yard => meters / 0.9144,
            LengthUnit::Mile => meters / 1609.344,
        }
    }

    /// Convert this length to a different unit
    pub fn convert_to(&self, target: LengthUnit) -> Self {
        if self.unit == target {
            return self.clone();
        }

        // Try direct conversion first (for exact imperial conversions)
        if let Some(direct_value) = Self::convert_direct(self.unit, target, self.value) {
            return Self {
                value: direct_value,
                unit: target,
            };
        }

        // Fall back to conversion through meters (base unit)
        let meters = self.to_meters();
        let converted_value = Self::from_meters(meters, target);

        Self {
            value: converted_value,
            unit: target,
        }
    }

    /// Direct conversions for exact relationships (primarily imperial units)
    fn convert_direct(from: LengthUnit, to: LengthUnit, value: f64) -> Option<f64> {
        match (from, to) {
            // Inch <-> Foot
            (LengthUnit::Inch, LengthUnit::Foot) => Some(value / 12.0),
            (LengthUnit::Foot, LengthUnit::Inch) => Some(value * 12.0),

            // Foot <-> Yard
            (LengthUnit::Foot, LengthUnit::Yard) => Some(value / 3.0),
            (LengthUnit::Yard, LengthUnit::Foot) => Some(value * 3.0),

            // Inch <-> Yard (12 * 3 = 36)
            (LengthUnit::Inch, LengthUnit::Yard) => Some(value / 36.0),
            (LengthUnit::Yard, LengthUnit::Inch) => Some(value * 36.0),

            // Foot <-> Mile (5280 feet per mile)
            (LengthUnit::Foot, LengthUnit::Mile) => Some(value / 5280.0),
            (LengthUnit::Mile, LengthUnit::Foot) => Some(value * 5280.0),

            // Inch <-> Mile (5280 * 12 = 63360)
            (LengthUnit::Inch, LengthUnit::Mile) => Some(value / 63360.0),
            (LengthUnit::Mile, LengthUnit::Inch) => Some(value * 63360.0),

            // Yard <-> Mile (5280 / 3 = 1760)
            (LengthUnit::Yard, LengthUnit::Mile) => Some(value / 1760.0),
            (LengthUnit::Mile, LengthUnit::Yard) => Some(value * 1760.0),

            // No direct conversion available
            _ => None,
        }
    }

    /// Get the numeric value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get the unit
    pub fn unit(&self) -> LengthUnit {
        self.unit
    }

    /// Get value as meters
    pub fn as_meters(&self) -> f64 {
        self.to_meters()
    }
}

impl LengthUnit {
    /// Get the canonical string representation for this unit
    pub fn canonical_string(&self) -> &'static str {
        match self {
            LengthUnit::Meter => "m",
            LengthUnit::Centimeter => "cm",
            LengthUnit::Millimeter => "mm",
            LengthUnit::Kilometer => "km",
            LengthUnit::Foot => "ft",
            LengthUnit::Inch => "in",
            LengthUnit::Yard => "yd",
            LengthUnit::Mile => "mi",
        }
    }
}

impl fmt::Display for LengthDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.unit.canonical_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_creation() {
        let length = LengthDimension::from_unit("cm", 10.0).unwrap();
        assert_eq!(length.value(), 10.0);
        assert_eq!(length.unit(), LengthUnit::Centimeter);
    }

    #[test]
    fn test_length_conversion() {
        let length = LengthDimension::from_unit("cm", 100.0).unwrap();
        let in_meters = length.convert_to(LengthDimension::METERS);
        assert_eq!(in_meters.value(), 1.0);
        assert_eq!(in_meters.unit(), LengthUnit::Meter);
    }

    #[test]
    fn test_feet_to_inches() {
        let length = LengthDimension::new(1.0, LengthDimension::FEET);
        let in_inches = length.convert_to(LengthDimension::INCHES);
        assert!((in_inches.value() - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_display() {
        let length = LengthDimension::from_unit("km", 5.5).unwrap();
        assert_eq!(format!("{}", length), "5.5km");
    }

    #[test]
    fn test_unknown_unit() {
        let result = LengthDimension::from_unit("xyz", 10.0);
        assert!(result.is_err());
    }
}
