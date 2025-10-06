use mathengine_units::{
    length::LengthUnit,
    temperature::TemperatureUnit,
    UnitType, UnitConversion, Dimension
};

/// Represents the dimension type of a unit
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DimensionType {
    Length,
    Temperature,
    Unknown,
}

/// Unified enum for any unit type in the system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Length(mathengine_units::length::LengthUnit),
    Temperature(mathengine_units::temperature::TemperatureUnit),
}

impl Unit {
    /// Get the canonical string for this unit
    pub fn canonical_string(&self) -> &'static str {
        match self {
            Unit::Length(u) => u.canonical_string(),
            Unit::Temperature(u) => u.canonical_string(),
        }
    }

    /// Get the dimension type for this unit
    pub fn dimension_type(&self) -> DimensionType {
        match self {
            Unit::Length(_) => DimensionType::Length,
            Unit::Temperature(_) => DimensionType::Temperature,
        }
    }
}

impl DimensionType {
    /// Determine the dimension type from a unit string
    pub fn from_unit(unit: &str) -> Self {
        if LengthUnit::parse(unit).is_ok() {
            DimensionType::Length
        } else if TemperatureUnit::parse(unit).is_ok() {
            DimensionType::Temperature
        } else {
            DimensionType::Unknown
        }
    }

    /// Parse a unit string into a Unit
    pub fn parse_unit_str(&self, unit_str: &str) -> Result<Unit, mathengine_units::UnitError> {
        match self {
            DimensionType::Length => {
                LengthUnit::parse(unit_str)
                    .map(Unit::Length)
            }
            DimensionType::Temperature => {
                TemperatureUnit::parse(unit_str)
                    .map(Unit::Temperature)
            }
            DimensionType::Unknown => Err(mathengine_units::UnitError::UnknownUnit(unit_str.to_string())),
        }
    }

    /// Get the canonical string for a unit (with dimension validation)
    pub fn canonical_string(&self, unit: &Unit) -> Option<&'static str> {
        if unit.dimension_type() == *self {
            Some(unit.canonical_string())
        } else {
            None
        }
    }

    /// Convert a value to the base unit for this dimension (with validation)
    pub fn to_base_value(&self, unit: &Unit, value: f64) -> Option<f64> {
        match (self, unit) {
            (DimensionType::Length, Unit::Length(u)) => {
                Some(<Dimension<LengthUnit> as UnitConversion<LengthUnit>>::to_base_value(*u, value))
            }
            (DimensionType::Temperature, Unit::Temperature(u)) => {
                Some(<Dimension<TemperatureUnit> as UnitConversion<TemperatureUnit>>::to_base_value(*u, value))
            }
            _ => None,
        }
    }

    /// Convert a value between units within this dimension (with validation)
    pub fn convert_value(&self, from_unit: &Unit, to_unit: &Unit, value: f64) -> Option<f64> {
        match (self, from_unit, to_unit) {
            (DimensionType::Length, Unit::Length(from), Unit::Length(to)) => {
                Some(Dimension::<LengthUnit>::convert_value(*from, *to, value))
            }
            (DimensionType::Temperature, Unit::Temperature(from), Unit::Temperature(to)) => {
                Some(Dimension::<TemperatureUnit>::convert_value(*from, *to, value))
            }
            _ => None, // Cross-dimension conversion rejected
        }
    }

    /// Get the base unit string for this dimension
    pub fn base_unit_string(&self) -> &'static str {
        match self {
            DimensionType::Length => <Dimension<LengthUnit> as UnitConversion<LengthUnit>>::base_unit().canonical_string(),
            DimensionType::Temperature => <Dimension<TemperatureUnit> as UnitConversion<TemperatureUnit>>::base_unit().canonical_string(),
            DimensionType::Unknown => "unknown",
        }
    }
}