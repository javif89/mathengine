use std::fmt::Display;

/// Represents a numeric value in mathematical expressions.
#[derive(Debug, Clone)]
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