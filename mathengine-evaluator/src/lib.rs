use mathengine_lexer::Operation;
use mathengine_parser::{
    Expression,
    types::{Number, UnitValue, Value},
};
use mathengine_units::{length::LengthDimension, temperature::TemperatureDimension};

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
                let (value, from_unit) = match left.as_ref() {
                    Expression::UnitValue { value, unit } => (*value, unit),
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

                match get_dimension_type(from_unit) {
                    DimensionType::Length => {
                        let from = LengthDimension::from_unit(from_unit, value).map_err(|_| {
                            EvalError::UnknownUnit {
                                unit: from_unit.clone(),
                            }
                        })?;
                        let to = LengthDimension::parse_unit(to_unit).map_err(|_| {
                            EvalError::UnknownUnit {
                                unit: to_unit.clone(),
                            }
                        })?;
                        let converted = from.convert_to(to);
                        Ok(Value::UnitValue(UnitValue::new(
                            converted.value(),
                            to.canonical_string().into(),
                        )))
                    }
                    DimensionType::Temperature => {
                        let from =
                            TemperatureDimension::from_unit(from_unit, value).map_err(|_| {
                                EvalError::UnknownUnit {
                                    unit: from_unit.clone(),
                                }
                            })?;
                        let to = TemperatureDimension::parse_unit(to_unit).map_err(|_| {
                            EvalError::UnknownUnit {
                                unit: to_unit.clone(),
                            }
                        })?;
                        let converted = from.convert_to(to);
                        Ok(Value::UnitValue(UnitValue::new(
                            converted.value(),
                            to.canonical_string().into(),
                        )))
                    }
                    DimensionType::Unknown => Err(EvalError::InvalidConversion {
                        from_unit: from_unit.clone(),
                        to_unit: to_unit.clone(),
                    }),
                }
            }
            _ => {
                let left_val = evaluate(left)?;
                let right_val = evaluate(right)?;

                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => {
                        let result = match op {
                            Operation::Add => l + r,
                            Operation::Subtract => l - r,
                            Operation::Multiply => l * r,
                            Operation::Divide => {
                                if r.0 == 0.0 {
                                    return Err(EvalError::DivisionByZero);
                                }
                                l / r
                            }
                            Operation::Power => {
                                let l_val = l.0;
                                let r_val = r.0;
                                Number::from(l_val.powf(r_val))
                            }
                            Operation::Convert => {
                                return Err(EvalError::UnsupportedOperation {
                                    operation: "convert".to_string(),
                                    operand_type: "numbers".to_string(),
                                });
                            }
                        };
                        Ok(Value::Number(result))
                    }
                    (Value::UnitValue(l), Value::UnitValue(r)) => {
                        let result = match op {
                            Operation::Add => l + r,
                            Operation::Subtract => l - r,
                            Operation::Multiply | Operation::Divide => {
                                return Err(EvalError::UnsupportedOperation {
                                    operation: format!("{:?}", op),
                                    operand_type: "unit values".to_string(),
                                });
                            }
                            Operation::Power => {
                                return Err(EvalError::UnsupportedOperation {
                                    operation: "power".to_string(),
                                    operand_type: "unit values".to_string(),
                                });
                            }
                            Operation::Convert => {
                                return Err(EvalError::UnsupportedOperation {
                                    operation: "convert".to_string(),
                                    operand_type: "unit values".to_string(),
                                });
                            }
                        };
                        Ok(Value::UnitValue(result))
                    }
                    (Value::UnitValue(l), Value::Number(r)) => {
                        let result = match op {
                            Operation::Add => l + r,
                            Operation::Subtract => l - r,
                            Operation::Multiply => l * r,
                            Operation::Divide => {
                                if r.0 == 0.0 {
                                    return Err(EvalError::DivisionByZero);
                                }
                                l / r
                            }
                            Operation::Power => {
                                return Err(EvalError::UnsupportedOperation {
                                    operation: "power".to_string(),
                                    operand_type: "unit value and number".to_string(),
                                });
                            }
                            Operation::Convert => {
                                return Err(EvalError::UnsupportedOperation {
                                    operation: "convert".to_string(),
                                    operand_type: "unit value and number".to_string(),
                                });
                            }
                        };
                        Ok(Value::UnitValue(result))
                    }
                    (Value::Number(l), Value::UnitValue(r)) => {
                        let result = match op {
                            Operation::Add => l + r,
                            Operation::Subtract => l - r,
                            Operation::Multiply => l * r,
                            Operation::Divide => {
                                return Err(EvalError::UnsupportedOperation {
                                    operation: "divide".to_string(),
                                    operand_type: "number by unit value".to_string(),
                                });
                            }
                            Operation::Power => {
                                return Err(EvalError::UnsupportedOperation {
                                    operation: "power".to_string(),
                                    operand_type: "number and unit value".to_string(),
                                });
                            }
                            Operation::Convert => {
                                return Err(EvalError::UnsupportedOperation {
                                    operation: "convert".to_string(),
                                    operand_type: "number and unit value".to_string(),
                                });
                            }
                        };
                        Ok(Value::UnitValue(result))
                    }
                }
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

enum DimensionType {
    Length,
    Temperature,
    Unknown,
}

fn get_dimension_type(unit: &str) -> DimensionType {
    if LengthDimension::parse_unit(unit).is_ok() {
        return DimensionType::Length;
    } else if TemperatureDimension::parse_unit(unit).is_ok() {
        return DimensionType::Temperature;
    }

    DimensionType::Unknown
}
