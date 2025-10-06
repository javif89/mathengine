use crate::{UnitError, UnitType, UnitConversion, Dimension};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureUnit {
    Kelvin,
    Celcius,
    Farenheit,
}


impl UnitType for TemperatureUnit {
    fn canonical_string(&self) -> &'static str {
        match self {
            TemperatureUnit::Kelvin => "K",
            TemperatureUnit::Celcius => "C",
            TemperatureUnit::Farenheit => "F",
        }
    }

    fn parse(s: &str) -> Result<Self, UnitError> {
        match s.to_lowercase().as_str() {
            "c" | "celcius" => Ok(TemperatureUnit::Celcius),
            "f" | "farenheit" => Ok(TemperatureUnit::Farenheit),
            "k" | "kelvin" => Ok(TemperatureUnit::Kelvin),
            _ => Err(UnitError::UnknownUnit(s.to_string())),
        }
    }

    fn dimension_name() -> &'static str {
        "Temperature"
    }
}


impl UnitConversion<TemperatureUnit> for Dimension<TemperatureUnit> {
    fn to_base_value(unit: TemperatureUnit, value: f64) -> f64 {
        match unit {
            TemperatureUnit::Kelvin => value,
            TemperatureUnit::Celcius => value + 273.15,
            TemperatureUnit::Farenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,
        }
    }

    fn from_base_value(base_value: f64, unit: TemperatureUnit) -> f64 {
        match unit {
            TemperatureUnit::Kelvin => base_value,
            TemperatureUnit::Celcius => base_value - 273.15,
            TemperatureUnit::Farenheit => (base_value - 273.15) * 9.0 / 5.0 + 32.0,
        }
    }

    fn base_unit() -> TemperatureUnit {
        TemperatureUnit::Kelvin
    }
}


/// Type alias for the concrete temperature dimension
pub type TemperatureDimension = Dimension<TemperatureUnit>;

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
        assert_eq!(format!("{}", temp), "25.5C");
    }

    #[test]
    fn test_unknown_unit() {
        let result = TemperatureDimension::from_unit("xyz", 10.0);
        assert!(result.is_err());
    }
}
