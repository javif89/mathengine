use mathengine_units::{
    length::LengthDimension,
    temperature::TemperatureDimension,
    Dimension
};
use std::fmt::Display;

/// Represents a numeric value in mathematical expressions.
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

/// Unified enum for any unit type in the system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Length(mathengine_units::length::LengthUnit),
    Temperature(mathengine_units::temperature::TemperatureUnit),
}

impl Unit {
    /// Get the canonical string for this unit
    pub fn canonical_string(&self) -> &'static str {
        match self {
            Unit::Length(u) => u.canonical_string(),
            Unit::Temperature(u) => u.canonical_string(),
        }
    }

    /// Get the dimension type for this unit
    pub fn dimension_type(&self) -> DimensionType {
        match self {
            Unit::Length(_) => DimensionType::Length,
            Unit::Temperature(_) => DimensionType::Temperature,
        }
    }
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

    /// Parse a unit string into a Unit
    pub fn parse_unit_str(&self, unit_str: &str) -> Result<Unit, mathengine_units::UnitError> {
        match self {
            DimensionType::Length => {
                LengthDimension::parse_unit_str(unit_str)
                    .map(Unit::Length)
            }
            DimensionType::Temperature => {
                TemperatureDimension::parse_unit_str(unit_str)
                    .map(Unit::Temperature)
            }
            DimensionType::Unknown => Err(mathengine_units::UnitError::UnknownUnit(unit_str.to_string())),
        }
    }

    /// Get the canonical string for a unit (with dimension validation)
    pub fn canonical_string(&self, unit: &Unit) -> Option<&'static str> {
        if unit.dimension_type() == *self {
            Some(unit.canonical_string())
        } else {
            None
        }
    }

    /// Convert a value to the base unit for this dimension (with validation)
    pub fn to_base_value(&self, unit: &Unit, value: f64) -> Option<f64> {
        match (self, unit) {
            (DimensionType::Length, Unit::Length(u)) => {
                Some(LengthDimension::to_base_value(*u, value))
            }
            (DimensionType::Temperature, Unit::Temperature(u)) => {
                Some(TemperatureDimension::to_base_value(*u, value))
            }
            _ => None,
        }
    }

    /// Convert a value between units within this dimension (with validation)
    pub fn convert_value(&self, from_unit: &Unit, to_unit: &Unit, value: f64) -> Option<f64> {
        match (self, from_unit, to_unit) {
            (DimensionType::Length, Unit::Length(from), Unit::Length(to)) => {
                Some(LengthDimension::convert_value(*from, *to, value))
            }
            (DimensionType::Temperature, Unit::Temperature(from), Unit::Temperature(to)) => {
                Some(TemperatureDimension::convert_value(*from, *to, value))
            }
            _ => None, // Cross-dimension conversion rejected
        }
    }

    /// Get the base unit string for this dimension
    pub fn base_unit_string(&self) -> &'static str {
        match self {
            DimensionType::Length => LengthDimension::base_unit().canonical_string(),
            DimensionType::Temperature => TemperatureDimension::base_unit().canonical_string(),
            DimensionType::Unknown => "unknown",
        }
    }
}

/// Represents a value with an associated unit (e.g., "5 meters", "32 fahrenheit").
///
/// UnitValues automatically track their dimension type (Length, Temperature, etc.)
/// and support arithmetic operations with automatic unit conversion to base units.
#[derive(Debug)]
pub struct UnitValue {
    value: f64,
    unit: String,
    dimension: DimensionType,
}

impl UnitValue {
    /// Create a new UnitValue with the given value and unit string.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathengine_parser::types::UnitValue;
    ///
    /// let length = UnitValue::new(5.0, "meters".to_string());
    /// let temp = UnitValue::new(32.0, "F".to_string());
    /// ```
    pub fn new(value: f64, unit: String) -> Self {
        let dimension = DimensionType::from_unit(&unit);
        Self {
            value,
            unit,
            dimension,
        }
    }

    /// Get the numeric value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get the unit string.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Get the dimension type of this unit value.
    pub fn dimension(&self) -> DimensionType {
        self.dimension
    }

    /// Get the canonical unit name for display purposes.
    ///
    /// Converts unit names to their standard abbreviated forms:
    /// - "meters" → "m"
    /// - "inches" → "in"
    /// - "celsius" → "C"
    ///
    /// # Examples
    ///
    /// ```
    /// use mathengine_parser::types::UnitValue;
    ///
    /// let length = UnitValue::new(5.0, "meters".to_string());
    /// assert_eq!(length.canonical_unit_name(), "m");
    /// ```
    pub fn canonical_unit_name(&self) -> String {
        self.dimension.parse_unit_str(&self.unit)
            .ok()
            .and_then(|unit| self.dimension.canonical_string(&unit).map(|s| s.to_string()))
            .unwrap_or_else(|| self.unit.clone())
    }

    /// Convert this unit value to base units for its dimension
    fn to_base_value(&self) -> f64 {
        self.dimension.parse_unit_str(&self.unit)
            .ok()
            .and_then(|unit| self.dimension.to_base_value(&unit, self.value))
            .unwrap_or(self.value)
    }

    /// Get the base unit string for this dimension
    fn base_unit(&self) -> String {
        self.dimension.base_unit_string().to_string()
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

/// Unified value type for evaluation results.
///
/// This enum represents the result of evaluating a mathematical expression,
/// which can be either a plain number or a value with a unit.
///
/// # Examples
///
/// ```
/// use mathengine_parser::types::{Value, Number, UnitValue};
///
/// let num_result = Value::Number(Number::from(42.0));
/// let unit_result = Value::UnitValue(UnitValue::new(5.0, "m".to_string()));
/// ```
#[derive(Debug)]
pub enum Value {
    /// A plain numeric value
    Number(Number),
    /// A value with an associated unit
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
