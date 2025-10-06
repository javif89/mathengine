use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    DivisionByZero,
    IncompatibleUnits {
        left_unit: String,
        right_unit: String,
        operation: String,
    },
    UnknownUnit {
        unit: String,
    },
    InvalidConversion {
        from_unit: String,
        to_unit: String,
    },
    UnsupportedOperation {
        operation: String,
        operand_type: String,
    },
    InvalidUnitExpression {
        message: String,
    },
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            EvalError::IncompatibleUnits {
                left_unit,
                right_unit,
                operation,
            } => {
                write!(
                    f,
                    "Cannot {} incompatible units: {} and {}",
                    operation, left_unit, right_unit
                )
            }
            EvalError::UnknownUnit { unit } => {
                write!(f, "Unknown unit: '{}'", unit)
            }
            EvalError::InvalidConversion { from_unit, to_unit } => {
                write!(f, "Cannot convert from '{}' to '{}'", from_unit, to_unit)
            }
            EvalError::UnsupportedOperation {
                operation,
                operand_type,
            } => {
                write!(
                    f,
                    "Unsupported operation '{}' for {}",
                    operation, operand_type
                )
            }
            EvalError::InvalidUnitExpression { message } => {
                write!(f, "Invalid unit expression: {}", message)
            }
        }
    }
}

impl std::error::Error for EvalError {}

impl From<mathengine_parser::types::ConversionError> for EvalError {
    fn from(err: mathengine_parser::types::ConversionError) -> Self {
        match err {
            mathengine_parser::types::ConversionError::UnknownUnit(unit) => {
                EvalError::UnknownUnit { unit }
            }
            mathengine_parser::types::ConversionError::CrossDimension => {
                EvalError::InvalidUnitExpression {
                    message: "Cannot convert between different dimensions".to_string(),
                }
            }
            mathengine_parser::types::ConversionError::Failed => {
                EvalError::InvalidUnitExpression {
                    message: "Conversion failed".to_string(),
                }
            }
        }
    }
}
