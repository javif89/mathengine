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

#[derive(Debug)]
pub struct UnitValue {
    value: f64,
    unit: String,
}

impl Display for UnitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

impl std::ops::Add for UnitValue {
    type Output = UnitValue;
    fn add(self, rhs: Self) -> Self::Output {
        // TODO: When I implement conversions, we have to convert
        // to the base unit and that will be the result
        UnitValue {
            unit: self.unit,
            value: self.value + rhs.value,
        }
    }
}

impl std::ops::Add<Number> for UnitValue {
    type Output = UnitValue;
    fn add(self, rhs: Number) -> Self::Output {
        UnitValue {
            unit: self.unit,
            value: self.value + rhs.0,
        }
    }
}

impl std::ops::Add<UnitValue> for Number {
    type Output = UnitValue;
    fn add(self, rhs: UnitValue) -> Self::Output {
        UnitValue {
            unit: rhs.unit,
            value: self.0 + rhs.value,
        }
    }
}

impl std::ops::Sub for UnitValue {
    type Output = UnitValue;
    fn sub(self, rhs: Self) -> Self::Output {
        // TODO: Check units match
        UnitValue {
            unit: self.unit,
            value: self.value - rhs.value,
        }
    }
}

impl std::ops::Sub<Number> for UnitValue {
    type Output = UnitValue;
    fn sub(self, rhs: Number) -> Self::Output {
        UnitValue {
            unit: self.unit,
            value: self.value - rhs.0,
        }
    }
}

impl std::ops::Sub<UnitValue> for Number {
    type Output = UnitValue;
    fn sub(self, rhs: UnitValue) -> Self::Output {
        UnitValue {
            unit: rhs.unit,
            value: self.0 - rhs.value,
        }
    }
}

impl std::ops::Mul<Number> for UnitValue {
    type Output = UnitValue;
    fn mul(self, rhs: Number) -> Self::Output {
        UnitValue {
            unit: self.unit,
            value: self.value * rhs.0,
        }
    }
}

impl std::ops::Mul<UnitValue> for Number {
    type Output = UnitValue;
    fn mul(self, rhs: UnitValue) -> Self::Output {
        UnitValue {
            unit: rhs.unit,
            value: self.0 * rhs.value,
        }
    }
}

impl std::ops::Div<Number> for UnitValue {
    type Output = UnitValue;
    fn div(self, rhs: Number) -> Self::Output {
        UnitValue {
            unit: self.unit,
            value: self.value / rhs.0,
        }
    }
}

impl UnitValue {
    pub fn new(value: f64, unit: String) -> Self {
        Self { value, unit }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn unit(&self) -> &str {
        &self.unit
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
