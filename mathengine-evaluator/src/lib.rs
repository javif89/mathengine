use mathengine_lexer::Operation;
use mathengine_parser::{
    Expression,
    types::{Number, UnitValue, Value},
};

pub mod error;
pub use error::EvalError;

pub fn evaluate(expr: &Expression) -> Result<Value, EvalError> {
    match expr {
        Expression::Number(n) => Ok(Value::Number(Number::from(*n))),
        Expression::UnitValue { value, unit } => {
            Ok(Value::UnitValue(UnitValue::new(*value, unit.clone())))
        }
        Expression::Unit(_unit) => Err(EvalError::InvalidUnitExpression {
            message: "Cannot evaluate a unit without a value".to_string(),
        }),
        Expression::Binary { op, left, right } => match op {
            Operation::Convert => {
                let left_val = evaluate(left)?;
                let (value, from_unit) = match left_val {
                    Value::UnitValue(uv) => (uv.value(), uv.unit().to_string()),
                    _ => {
                        return Err(EvalError::InvalidUnitExpression {
                            message: "Left side of conversion must be a unit value".to_string(),
                        });
                    }
                };

                let to_unit = match right.as_ref() {
                    Expression::Unit(u) => u,
                    _ => {
                        return Err(EvalError::InvalidUnitExpression {
                            message: "Right side of conversion must be a unit".to_string(),
                        });
                    }
                };

                // Use the new UnitValue conversion method
                let unit_value = UnitValue::new(value, from_unit.clone());
                let converted = unit_value.convert_to(to_unit)?;

                Ok(Value::UnitValue(converted))
            }
            _ => {
                let left_val = evaluate(left)?;
                let right_val = evaluate(right)?;

                // Check for division by zero before delegating to operators
                if let Operation::Divide = op {
                    match &right_val {
                        Value::Number(n) if n.0 == 0.0 => return Err(EvalError::DivisionByZero),
                        _ => {}
                    }
                }

                let result = match op {
                    Operation::Add => left_val + right_val,
                    Operation::Subtract => left_val - right_val,
                    Operation::Multiply => left_val * right_val,
                    Operation::Divide => left_val / right_val,
                    Operation::Power => {
                        // Power is not implemented via operators yet, handle specially
                        match (left_val, right_val) {
                            (Value::Number(l), Value::Number(r)) => {
                                Value::Number(Number::from(l.0.powf(r.0)))
                            }
                            _ => {
                                return Err(EvalError::UnsupportedOperation {
                                    operation: "power".to_string(),
                                    operand_type: "non-numeric values".to_string(),
                                });
                            }
                        }
                    }
                    Operation::Convert => {
                        return Err(EvalError::UnsupportedOperation {
                            operation: "convert".to_string(),
                            operand_type: "binary operation".to_string(),
                        });
                    }
                };

                Ok(result)
            }
        },
        Expression::Unary { op, operand } => {
            let val = evaluate(operand)?;
            match op {
                Operation::Subtract => match val {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    Value::UnitValue(_) => Err(EvalError::UnsupportedOperation {
                        operation: "negate".to_string(),
                        operand_type: "unit value".to_string(),
                    }),
                },
                _ => Err(EvalError::UnsupportedOperation {
                    operation: format!("{:?}", op),
                    operand_type: "unary operand".to_string(),
                }),
            }
        }
    }
}

