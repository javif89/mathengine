use mathengine_units::{length::LengthDimension, temperature::TemperatureDimension};
use std::fmt::Display;

#[derive(Debug)]
pub struct Number(pub f64);

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Self::Output {
        Number(self.0 - rhs.0)
    }
}

impl std::ops::Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}

impl std::ops::Div for Number {
    type Output = Number;
    fn div(self, rhs: Number) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}

impl std::ops::Rem for Number {
    type Output = Number;
    fn rem(self, rhs: Number) -> Self::Output {
        Number(self.0 % rhs.0)
    }
}

impl std::ops::Neg for Number {
    type Output = Number;
    fn neg(self) -> Self::Output {
        Number(-self.0)
    }
}

/// Represents the dimension type of a unit
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DimensionType {
    Length,
    Temperature,
    Unknown,
}

impl DimensionType {
    /// Determine the dimension type from a unit string
    pub fn from_unit(unit: &str) -> Self {
        if LengthDimension::parse_unit(unit).is_ok() {
            DimensionType::Length
        } else if TemperatureDimension::parse_unit(unit).is_ok() {
            DimensionType::Temperature
        } else {
            DimensionType::Unknown
        }
    }
}

#[derive(Debug)]
pub struct UnitValue {
    value: f64,
    unit: String,
    dimension: DimensionType,
}

impl UnitValue {
    pub fn new(value: f64, unit: String) -> Self {
        let dimension = DimensionType::from_unit(&unit);
        Self {
            value,
            unit,
            dimension,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn unit(&self) -> &str {
        &self.unit
    }

    pub fn dimension(&self) -> DimensionType {
        self.dimension
    }

    /// Get the "canonical" name for a unit for standardizing
    /// display.
    /// For example
    /// "meters" -> "m"
    /// "inches" -> "in"
    /// "celcius" -> "C"
    pub fn canonical_unit_name(&self) -> String {
        match self.dimension {
            DimensionType::Length => LengthDimension::parse_unit(&self.unit)
                .map_or(self.unit.to_string(), |u| u.canonical_string().to_string()),
            DimensionType::Temperature => TemperatureDimension::parse_unit(&self.unit)
                .map_or(self.unit.to_string(), |u| u.canonical_string().to_string()),
            _ => self.unit.to_string(),
        }
    }

    /// Convert this unit value to base units for its dimension
    fn to_base_value(&self) -> f64 {
        match self.dimension {
            DimensionType::Length => {
                // Convert to meters (base unit)
                if let Ok(dim) = LengthDimension::from_unit(&self.unit, self.value) {
                    dim.to_meters()
                } else {
                    self.value // Fallback if conversion fails
                }
            }
            DimensionType::Temperature => {
                // Convert to Kelvin (base unit)
                if let Ok(dim) = TemperatureDimension::from_unit(&self.unit, self.value) {
                    dim.to_kelvin()
                } else {
                    self.value // Fallback if conversion fails
                }
            }
            DimensionType::Unknown => self.value,
        }
    }

    /// Get the base unit string for this dimension
    fn base_unit(&self) -> String {
        match self.dimension {
            DimensionType::Length => "m".to_string(),
            DimensionType::Temperature => "K".to_string(),
            DimensionType::Unknown => self.unit.clone(),
        }
    }
}

impl Display for UnitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.canonical_unit_name())
    }
}

impl std::ops::Add for UnitValue {
    type Output = UnitValue;
    fn add(self, rhs: Self) -> Self::Output {
        // Only add if dimensions match
        if self.dimension != rhs.dimension {
            // For now, just return the left side if dimensions don't match
            // In the future, this should be an error
            return self;
        }

        // Convert both to base units
        let base_value = self.to_base_value() + rhs.to_base_value();

        // Return result in base units
        UnitValue {
            value: base_value,
            unit: self.base_unit(),
            dimension: self.dimension,
        }
    }
}

impl std::ops::Add<Number> for UnitValue {
    type Output = UnitValue;
    fn add(self, rhs: Number) -> Self::Output {
        // When adding a number to a unit value, treat the number as having the same unit
        UnitValue {
            value: self.value + rhs.0,
            unit: self.unit,
            dimension: self.dimension,
        }
    }
}

impl std::ops::Add<UnitValue> for Number {
    type Output = UnitValue;
    fn add(self, rhs: UnitValue) -> Self::Output {
        UnitValue {
            value: self.0 + rhs.value,
            unit: rhs.unit,
            dimension: rhs.dimension,
        }
    }
}

impl std::ops::Sub for UnitValue {
    type Output = UnitValue;
    fn sub(self, rhs: Self) -> Self::Output {
        // Only subtract if dimensions match
        if self.dimension != rhs.dimension {
            // For now, just return the left side if dimensions don't match
            // In the future, this should be an error
            return self;
        }

        // Convert both to base units
        let base_value = self.to_base_value() - rhs.to_base_value();

        // Return result in base units
        UnitValue {
            value: base_value,
            unit: self.base_unit(),
            dimension: self.dimension,
        }
    }
}

impl std::ops::Sub<Number> for UnitValue {
    type Output = UnitValue;
    fn sub(self, rhs: Number) -> Self::Output {
        UnitValue {
            value: self.value - rhs.0,
            unit: self.unit,
            dimension: self.dimension,
        }
    }
}

impl std::ops::Sub<UnitValue> for Number {
    type Output = UnitValue;
    fn sub(self, rhs: UnitValue) -> Self::Output {
        UnitValue {
            value: self.0 - rhs.value,
            unit: rhs.unit,
            dimension: rhs.dimension,
        }
    }
}

impl std::ops::Mul<Number> for UnitValue {
    type Output = UnitValue;
    fn mul(self, rhs: Number) -> Self::Output {
        UnitValue {
            value: self.value * rhs.0,
            unit: self.unit,
            dimension: self.dimension,
        }
    }
}

impl std::ops::Mul<UnitValue> for Number {
    type Output = UnitValue;
    fn mul(self, rhs: UnitValue) -> Self::Output {
        UnitValue {
            value: self.0 * rhs.value,
            unit: rhs.unit,
            dimension: rhs.dimension,
        }
    }
}

impl std::ops::Div<Number> for UnitValue {
    type Output = UnitValue;
    fn div(self, rhs: Number) -> Self::Output {
        UnitValue {
            value: self.value / rhs.0,
            unit: self.unit,
            dimension: self.dimension,
        }
    }
}

/// Unified value type for evaluation results
#[derive(Debug)]
pub enum Value {
    Number(Number),
    UnitValue(UnitValue),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::UnitValue(uv) => write!(f, "{}", uv),
        }
    }
}

impl From<Number> for Value {
    fn from(n: Number) -> Self {
        Value::Number(n)
    }
}

impl From<UnitValue> for Value {
    fn from(uv: UnitValue) -> Self {
        Value::UnitValue(uv)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Number(Number(f))
    }
}
