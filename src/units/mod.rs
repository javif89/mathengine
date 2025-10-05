#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    Length,
    Temperature,
    Mass,
}

pub enum Length {
    Meters,
    Centimeters,
    Milimiters,
    Feet,
    Inches,
}

pub enum Temperature {
    Kelvin,
    Farenheit,
    Celcius,
}

pub enum Mass {
    Pounds,
    Kilograms,
    Miligrams,
}

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    pub name: &'static str,
    pub dimension: Dimension,
    pub to_base: fn(f64) -> f64,
    pub from_base: fn(f64) -> f64,
}
