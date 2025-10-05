use mathengine_lexer::Operation;
use mathengine_parser::{types::{Number, UnitValue, Value}, Expression};
use mathengine_units::{length::LengthDimension, temperature::TemperatureDimension};

pub fn evaluate(expr: &Expression) -> Value {
    match expr {
        Expression::Number(n) => Value::Number(Number::from(*n)),
        Expression::UnitValue { value, unit } => {
            Value::UnitValue(UnitValue::new(*value, unit.clone()))
        }
        Expression::Unit(_unit) => {
            // Units by themselves don't have a numeric value
            panic!("Cannot evaluate a unit without a value")
        }
        Expression::Binary { op, left, right } => match op {
            Operation::Convert => {
                let (value, from_unit) = match left.as_ref() {
                    Expression::UnitValue { value, unit } => (*value, unit),
                    _ => panic!("Invalid conversion"),
                };

                let to_unit = match right.as_ref() {
                    Expression::Unit(u) => u,
                    _ => panic!("Invalid conversion"),
                };

                match get_dimension_type(&from_unit) {
                    DimensionType::Length => {
                        let from = LengthDimension::from_unit(&from_unit, value).unwrap();
                        let to = LengthDimension::parse_unit(to_unit).unwrap();
                        let converted = from.convert_to(to);
                        Value::UnitValue(UnitValue::new(
                            converted.value(),
                            to.canonical_string().into(),
                        ))
                    }
                    DimensionType::Temperature => {
                        let from = TemperatureDimension::from_unit(&from_unit, value).unwrap();
                        let to = TemperatureDimension::parse_unit(to_unit).unwrap();
                        let converted = from.convert_to(to);
                        Value::UnitValue(UnitValue::new(
                            converted.value(),
                            to.canonical_string().into(),
                        ))
                    }
                    DimensionType::Unknown => panic!("Unknown dimension type"),
                }
            }
            _ => {
                let left_val = evaluate(left);
                let right_val = evaluate(right);

                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => {
                        let result = match op {
                            Operation::Add => l + r,
                            Operation::Subtract => l - r,
                            Operation::Multiply => l * r,
                            Operation::Divide => l / r,
                            Operation::Power => {
                                let l_val = l.0;
                                let r_val = r.0;
                                Number::from(l_val.powf(r_val))
                            }
                            _ => panic!("Unsupported operation in evaluation"),
                        };
                        Value::Number(result)
                    }
                    (Value::UnitValue(l), Value::UnitValue(r)) => {
                        let result = match op {
                            Operation::Add => l + r,
                            Operation::Subtract => l - r,
                            _ => panic!("Cannot multiply or divide unit values"),
                        };
                        Value::UnitValue(result)
                    }
                    (Value::UnitValue(l), Value::Number(r)) => {
                        let result = match op {
                            Operation::Add => l + r,
                            Operation::Subtract => l - r,
                            Operation::Multiply => l * r,
                            Operation::Divide => l / r,
                            _ => panic!("Unsupported operation"),
                        };
                        Value::UnitValue(result)
                    }
                    (Value::Number(l), Value::UnitValue(r)) => {
                        let result = match op {
                            Operation::Add => l + r,
                            Operation::Subtract => l - r,
                            Operation::Multiply => l * r,
                            _ => panic!("Unsupported operation"),
                        };
                        Value::UnitValue(result)
                    }
                }
            }
        },
        Expression::Unary { op, operand } => {
            let val = evaluate(operand);
            match op {
                Operation::Subtract => match val {
                    Value::Number(n) => Value::Number(-n),
                    _ => panic!("Cannot negate non-numeric values"),
                },
                _ => panic!("Unsupported unary operation"),
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
    if let Ok(_) = LengthDimension::parse_unit(unit) {
        return DimensionType::Length;
    } else if let Ok(_) = TemperatureDimension::parse_unit(unit) {
        return DimensionType::Temperature;
    }

    DimensionType::Unknown
}