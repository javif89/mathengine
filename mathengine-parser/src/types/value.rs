use std::fmt::Display;
use crate::types::{Number, UnitValue};

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
#[derive(Debug, Clone)]
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

impl std::ops::Add for Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
            (Value::UnitValue(l), Value::Number(r)) => Value::UnitValue(l + r),
            (Value::Number(l), Value::UnitValue(r)) => Value::UnitValue(l + r),
            (Value::UnitValue(l), Value::UnitValue(r)) => Value::UnitValue(l + r),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Value;
    fn sub(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l - r),
            (Value::UnitValue(l), Value::Number(r)) => Value::UnitValue(l - r),
            (Value::Number(l), Value::UnitValue(r)) => Value::UnitValue(l - r),
            (Value::UnitValue(l), Value::UnitValue(r)) => Value::UnitValue(l - r),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l * r),
            (Value::UnitValue(l), Value::Number(r)) => Value::UnitValue(l * r),
            (Value::Number(l), Value::UnitValue(r)) => Value::UnitValue(l * r),
            (Value::UnitValue(l), Value::UnitValue(_r)) => {
                // UnitValue * UnitValue would create area/volume units which we don't support yet
                // For now, return the left operand (same as current behavior for unsupported ops)
                // TODO: Implement compound units (area, volume, etc.)
                Value::UnitValue(l)
            }
        }
    }
}

impl std::ops::Div for Value {
    type Output = Value;
    fn div(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l / r),
            (Value::UnitValue(l), Value::Number(r)) => Value::UnitValue(l / r),
            (Value::Number(l), Value::UnitValue(_r)) => {
                // Number / UnitValue would create inverse units (like 1/time = frequency)
                // We don't support this yet, so return the left operand
                // TODO: Implement inverse units
                Value::Number(l)
            }
            (Value::UnitValue(l), Value::UnitValue(_r)) => {
                // UnitValue / UnitValue could create ratios or dimensionless quantities
                // We don't support this yet, so return the left operand
                // TODO: Implement unit ratios and dimensionless quantities
                Value::UnitValue(l)
            }
        }
    }
}
