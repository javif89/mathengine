use mathengine_lexer::Operation;

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
