use crate::UnitError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureUnit {
    Kelvin,
    Celcius,
    Farenheit,
}

#[derive(Debug, Clone)]
pub struct TemperatureDimension {
    value: f64,
    unit: TemperatureUnit,
}

impl TemperatureDimension {
    // Unit constants for clean conversion API
    pub const KELVIN: TemperatureUnit = TemperatureUnit::Kelvin;
    pub const CELCIUS: TemperatureUnit = TemperatureUnit::Celcius;
    pub const FARENHEIR: TemperatureUnit = TemperatureUnit::Farenheit;

    /// Create a TemperatureDimension from a unit string and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathengine_units::temperature::TemperatureDimension;
    ///
    /// let temp = TemperatureDimension::from_unit("C", 25.0).unwrap();
    /// assert_eq!(temp.value(), 25.0);
    /// ```
    pub fn from_unit(unit_str: &str, value: f64) -> Result<Self, UnitError> {
        let unit = Self::parse_unit(unit_str)?;
        Ok(Self { value, unit })
    }

    /// Create a TemperatureDimension directly with a TemperatureUnit.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathengine_units::temperature::{TemperatureDimension, TemperatureUnit};
    ///
    /// let temp = TemperatureDimension::new(32.0, TemperatureUnit::Farenheit);
    /// assert_eq!(temp.value(), 32.0);
    /// ```
    pub fn new(value: f64, unit: TemperatureUnit) -> Self {
        Self { value, unit }
    }

    /// Parse a string into a TemperatureUnit
    pub fn parse_unit(s: &str) -> Result<TemperatureUnit, UnitError> {
        match s.to_lowercase().as_str() {
            "c" | "celcius" => Ok(TemperatureUnit::Celcius),
            "f" | "farenheit" => Ok(TemperatureUnit::Farenheit),
            "k" | "kelvin" => Ok(TemperatureUnit::Kelvin),
            _ => Err(UnitError::UnknownUnit(s.to_string())),
        }
    }

    /// Convert this temperature to Kelvin (base unit)
    pub fn to_kelvin(&self) -> f64 {
        match self.unit {
            TemperatureUnit::Kelvin => self.value,
            TemperatureUnit::Celcius => self.value + 273.15,
            TemperatureUnit::Farenheit => (self.value - 32.0) * 5.0 / 9.0 + 273.15,
        }
    }

    /// Convert Kelvin to the specified unit
    fn from_kelvin(kelvin: f64, unit: TemperatureUnit) -> f64 {
        match unit {
            TemperatureUnit::Kelvin => kelvin,
            TemperatureUnit::Celcius => kelvin - 273.15,
            TemperatureUnit::Farenheit => (kelvin - 273.15) * 9.0 / 5.0 + 32.0,
        }
    }

    /// Convert this temperature to a different unit.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathengine_units::temperature::{TemperatureDimension, TemperatureUnit};
    ///
    /// let celsius = TemperatureDimension::new(0.0, TemperatureUnit::Celcius);
    /// let fahrenheit = celsius.convert_to(TemperatureUnit::Farenheit);
    /// assert_eq!(fahrenheit.value(), 32.0);
    /// ```
    pub fn convert_to(&self, target: TemperatureUnit) -> Self {
        if self.unit == target {
            return self.clone();
        }

        // All conversions go through Kelvin (base unit)
        let kelvin = self.to_kelvin();
        let converted_value = Self::from_kelvin(kelvin, target);

        Self {
            value: converted_value,
            unit: target,
        }
    }

    /// Get the numeric value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get the unit.
    pub fn unit(&self) -> TemperatureUnit {
        self.unit
    }

    /// Get value as Kelvin
    pub fn as_kelvin(&self) -> f64 {
        self.to_kelvin()
    }
}

impl TemperatureUnit {
    /// Get the canonical string representation for this unit
    pub fn canonical_string(&self) -> &'static str {
        match self {
            TemperatureUnit::Kelvin => "K",
            TemperatureUnit::Celcius => "C",
            TemperatureUnit::Farenheit => "F",
        }
    }
}

impl fmt::Display for TemperatureDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}°{}", self.value, self.unit.canonical_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_creation() {
        let temp = TemperatureDimension::from_unit("C", 25.0).unwrap();
        assert_eq!(temp.value(), 25.0);
        assert_eq!(temp.unit(), TemperatureUnit::Celcius);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        let temp = TemperatureDimension::from_unit("C", 0.0).unwrap();
        let in_f = temp.convert_to(TemperatureUnit::Farenheit);
        assert!((in_f.value() - 32.0).abs() < 1e-10);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        let temp = TemperatureDimension::new(212.0, TemperatureUnit::Farenheit);
        let in_c = temp.convert_to(TemperatureUnit::Celcius);
        assert!((in_c.value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        let temp = TemperatureDimension::new(0.0, TemperatureUnit::Celcius);
        let in_k = temp.convert_to(TemperatureUnit::Kelvin);
        assert!((in_k.value() - 273.15).abs() < 1e-10);
    }

    #[test]
    fn test_display() {
        let temp = TemperatureDimension::from_unit("C", 25.5).unwrap();
        assert_eq!(format!("{}", temp), "25.5°C");
    }

    #[test]
    fn test_unknown_unit() {
        let result = TemperatureDimension::from_unit("xyz", 10.0);
        assert!(result.is_err());
    }
}
