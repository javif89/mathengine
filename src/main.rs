use mathengine::{
    lexer::{Lexer, Operation, Token},
    types::{Number, UnitValue, Value},
    units::{length::LengthDimension, temperature::TemperatureDimension},
};

fn main() {
    let expressions = vec![
        "2 + 3 * (100.50 - 4)",
        "10m to feet",
        "10m + 2",
        // "20lbs to kg",
        "10 feet to in",
        "2^10",
        "23C to f",
        "1m to miles",
    ];

    for e in expressions {
        println!("\nExpression: {}", e);
        let l = Lexer::new(e);
        let tokens = l.tokenize();

        // println!("Tokens: {:?}", tokens);

        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(expr) => {
                // println!("AST: {:#?}", expr);
                println!("Result: {}", evaluate(&expr));
            }
            Err(err) => println!("Parse error: {}", err),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    UnitValue {
        value: f64,
        unit: String,
    },
    Unit(String),
    Binary {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        op: Operation,
        operand: Box<Expression>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    // Entry point for parsing - parses the entire token stream and ensures all tokens are consumed
    pub fn parse(&mut self) -> Result<Expression, String> {
        let expr = self.parse_expression(0)?;
        if self.pos < self.tokens.len() {
            return Err(format!(
                "Unexpected token at position {}: {:?}",
                self.pos, self.tokens[self.pos]
            ));
        }
        Ok(expr)
    }

    // Pratt parsing algorithm - handles binary operators with correct precedence and associativity
    // min_precedence determines the minimum operator precedence this call will handle
    fn parse_expression(&mut self, min_precedence: u8) -> Result<Expression, String> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Operation(op) => {
                    let precedence = self.get_precedence(op);
                    if precedence < min_precedence {
                        break;
                    }

                    let op = match self.advance() {
                        Some(Token::Operation(o)) => o.clone(),
                        _ => unreachable!(),
                    };

                    let right_precedence = if self.is_right_associative(&op) {
                        precedence
                    } else {
                        precedence + 1
                    };

                    let right = self.parse_expression(right_precedence)?;
                    left = Expression::Binary {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Parses primary expressions: numbers, parenthesized expressions, and unary operators
    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(Expression::Number(*n)),
            Some(Token::UnitValue { value, unit }) => Ok(Expression::UnitValue {
                value: *value,
                unit: canonicalize_unit(unit),
            }),
            Some(Token::Unit(unit)) => Ok(Expression::Unit(unit.clone())),
            Some(Token::Lparen) => {
                let expr = self.parse_expression(0)?;
                match self.advance() {
                    Some(Token::Rparen) => Ok(expr),
                    Some(other) => Err(format!("Expected ')', found {:?}", other)),
                    None => Err("Expected ')', found end of input".to_string()),
                }
            }
            Some(Token::Operation(Operation::Subtract)) => {
                let operand = self.parse_primary()?;
                Ok(Expression::Unary {
                    op: Operation::Subtract,
                    operand: Box::new(operand),
                })
            }
            Some(token) => Err(format!("Unexpected token: {:?}", token)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    // Returns the current token without consuming it
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    // Consumes and returns the current token, advancing the position
    fn advance(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    // Returns the precedence level for each operator (higher number = higher precedence)
    fn get_precedence(&self, op: &Operation) -> u8 {
        match op {
            Operation::Add | Operation::Subtract => 1,
            Operation::Multiply | Operation::Divide => 2,
            Operation::Power => 3,
            Operation::Convert => 5,
        }
    }

    // Determines if an operator is right-associative (currently all ops are left-associative)
    fn is_right_associative(&self, op: &Operation) -> bool {
        match op {
            Operation::Power => true, // Power is right-associative: 2^3^4 = 2^(3^4)
            _ => false,
        }
    }
}

fn evaluate(expr: &Expression) -> Value {
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

fn canonicalize_unit(unit: &str) -> String {
    match get_dimension_type(unit) {
        DimensionType::Length => LengthDimension::parse_unit(unit)
            .unwrap()
            .canonical_string()
            .into(),
        DimensionType::Temperature => TemperatureDimension::parse_unit(unit)
            .unwrap()
            .canonical_string()
            .into(),
        DimensionType::Unknown => "unknown".into(),
    }
}
