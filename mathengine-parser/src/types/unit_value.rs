use std::fmt::Display;
use crate::types::{ConversionError, DimensionType, Number};

/// Represents a value with an associated unit (e.g., "5 meters", "32 fahrenheit").
///
/// UnitValues automatically track their dimension type (Length, Temperature, etc.)
/// and support arithmetic operations with automatic unit conversion to base units.
#[derive(Debug, Clone)]
pub struct UnitValue {
    value: f64,
    unit: String,
    dimension: DimensionType,
}

impl UnitValue {
    /// Create a new UnitValue with the given value and unit string.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathengine_parser::types::UnitValue;
    ///
    /// let length = UnitValue::new(5.0, "meters".to_string());
    /// let temp = UnitValue::new(32.0, "F".to_string());
    /// ```
    pub fn new(value: f64, unit: String) -> Self {
        let dimension = DimensionType::from_unit(&unit);
        Self {
            value,
            unit,
            dimension,
        }
    }

    /// Get the numeric value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get the unit string.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Get the dimension type of this unit value.
    pub fn dimension(&self) -> DimensionType {
        self.dimension
    }

    /// Get the canonical unit name for display purposes.
    ///
    /// Converts unit names to their standard abbreviated forms:
    /// - "meters" → "m"
    /// - "inches" → "in"
    /// - "celsius" → "C"
    ///
    /// # Examples
    ///
    /// ```
    /// use mathengine_parser::types::UnitValue;
    ///
    /// let length = UnitValue::new(5.0, "meters".to_string());
    /// assert_eq!(length.canonical_unit_name(), "m");
    /// ```
    pub fn canonical_unit_name(&self) -> String {
        self.dimension.parse_unit_str(&self.unit)
            .ok()
            .and_then(|unit| self.dimension.canonical_string(&unit).map(|s| s.to_string()))
            .unwrap_or_else(|| self.unit.clone())
    }

    /// Convert this unit value to base units for its dimension
    fn to_base_value(&self) -> f64 {
        self.dimension.parse_unit_str(&self.unit)
            .ok()
            .and_then(|unit| self.dimension.to_base_value(&unit, self.value))
            .unwrap_or(self.value)
    }

    /// Get the base unit string for this dimension
    fn base_unit(&self) -> String {
        self.dimension.base_unit_string().to_string()
    }

    /// Convert this unit value to another unit
    ///
    /// # Examples
    ///
    /// ```
    /// use mathengine_parser::types::UnitValue;
    ///
    /// let length = UnitValue::new(100.0, "cm".to_string());
    /// let in_meters = length.convert_to("m").unwrap();
    /// assert_eq!(in_meters.value(), 1.0);
    /// assert_eq!(in_meters.unit(), "m");
    /// ```
    pub fn convert_to(&self, target_unit: &str) -> Result<UnitValue, ConversionError> {
        // Check if target is same dimension
        let target_dimension = DimensionType::from_unit(target_unit);
        if target_dimension != self.dimension || target_dimension == DimensionType::Unknown {
            return Err(ConversionError::CrossDimension);
        }

        // Parse both units
        let from_unit = self.dimension.parse_unit_str(&self.unit)
            .map_err(|_| ConversionError::UnknownUnit(self.unit.clone()))?;
        let to_unit = self.dimension.parse_unit_str(target_unit)
            .map_err(|_| ConversionError::UnknownUnit(target_unit.to_string()))?;

        // Convert the value
        let new_value = self.dimension.convert_value(&from_unit, &to_unit, self.value)
            .ok_or(ConversionError::Failed)?;

        Ok(UnitValue::new(new_value, target_unit.to_string()))
    }

    /// Check if this unit value can be converted to another unit
    ///
    /// # Examples
    ///
    /// ```
    /// use mathengine_parser::types::UnitValue;
    ///
    /// let length = UnitValue::new(5.0, "m".to_string());
    /// assert!(length.can_convert_to("cm"));
    /// assert!(!length.can_convert_to("C"));
    /// ```
    pub fn can_convert_to(&self, target_unit: &str) -> bool {
        let target_dimension = DimensionType::from_unit(target_unit);
        target_dimension == self.dimension && target_dimension != DimensionType::Unknown
    }

    /// Convert this unit value to base units for its dimension
    ///
    /// # Examples
    ///
    /// ```
    /// use mathengine_parser::types::UnitValue;
    ///
    /// let length = UnitValue::new(100.0, "cm".to_string());
    /// let in_base = length.in_base_units();
    /// assert_eq!(in_base.value(), 1.0);
    /// assert_eq!(in_base.unit(), "m");
    /// ```
    pub fn in_base_units(&self) -> UnitValue {
        let base_unit_str = self.base_unit();
        // If we're already in base units, return a copy
        if self.canonical_unit_name() == base_unit_str {
            UnitValue::new(self.value, base_unit_str)
        } else {
            // Convert to base units
            self.convert_to(&base_unit_str).unwrap_or_else(|_| {
                // Fallback: just return with base unit string
                UnitValue::new(self.to_base_value(), base_unit_str)
            })
        }
    }

    /// Check if this unit value is in the same dimension as another
    ///
    /// # Examples
    ///
    /// ```
    /// use mathengine_parser::types::UnitValue;
    ///
    /// let length1 = UnitValue::new(5.0, "m".to_string());
    /// let length2 = UnitValue::new(100.0, "cm".to_string());
    /// let temp = UnitValue::new(25.0, "C".to_string());
    ///
    /// assert!(length1.same_dimension_as(&length2));
    /// assert!(!length1.same_dimension_as(&temp));
    /// ```
    pub fn same_dimension_as(&self, other: &UnitValue) -> bool {
        self.dimension == other.dimension && self.dimension != DimensionType::Unknown
    }
}

impl Display for UnitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.canonical_unit_name())
    }
}

impl std::ops::Add for UnitValue {
    type Output = UnitValue;
    fn add(self, rhs: Self) -> Self::Output {
        // Only add if dimensions match
        if !self.same_dimension_as(&rhs) {
            // For now, just return the left side if dimensions don't match
            // In the future, this should be an error
            return self;
        }

        // Convert both to base units and add
        let left_base = self.in_base_units();
        let right_base = rhs.in_base_units();

        UnitValue::new(left_base.value + right_base.value, left_base.unit)
    }
}

impl std::ops::Add<Number> for UnitValue {
    type Output = UnitValue;
    fn add(self, rhs: Number) -> Self::Output {
        // When adding a number to a unit value, treat the number as having the same unit
        UnitValue {
            value: self.value + rhs.0,
            unit: self.unit,
            dimension: self.dimension,
        }
    }
}

impl std::ops::Add<UnitValue> for Number {
    type Output = UnitValue;
    fn add(self, rhs: UnitValue) -> Self::Output {
        UnitValue {
            value: self.0 + rhs.value,
            unit: rhs.unit,
            dimension: rhs.dimension,
        }
    }
}

impl std::ops::Sub for UnitValue {
    type Output = UnitValue;
    fn sub(self, rhs: Self) -> Self::Output {
        // Only subtract if dimensions match
        if !self.same_dimension_as(&rhs) {
            // For now, just return the left side if dimensions don't match
            // In the future, this should be an error
            return self;
        }

        // Convert both to base units and subtract
        let left_base = self.in_base_units();
        let right_base = rhs.in_base_units();

        UnitValue::new(left_base.value - right_base.value, left_base.unit)
    }
}

impl std::ops::Sub<Number> for UnitValue {
    type Output = UnitValue;
    fn sub(self, rhs: Number) -> Self::Output {
        UnitValue {
            value: self.value - rhs.0,
            unit: self.unit,
            dimension: self.dimension,
        }
    }
}

impl std::ops::Sub<UnitValue> for Number {
    type Output = UnitValue;
    fn sub(self, rhs: UnitValue) -> Self::Output {
        UnitValue {
            value: self.0 - rhs.value,
            unit: rhs.unit,
            dimension: rhs.dimension,
        }
    }
}

impl std::ops::Mul<Number> for UnitValue {
    type Output = UnitValue;
    fn mul(self, rhs: Number) -> Self::Output {
        UnitValue {
            value: self.value * rhs.0,
            unit: self.unit,
            dimension: self.dimension,
        }
    }
}

impl std::ops::Mul<UnitValue> for Number {
    type Output = UnitValue;
    fn mul(self, rhs: UnitValue) -> Self::Output {
        UnitValue {
            value: self.0 * rhs.value,
            unit: rhs.unit,
            dimension: rhs.dimension,
        }
    }
}

impl std::ops::Div<Number> for UnitValue {
    type Output = UnitValue;
    fn div(self, rhs: Number) -> Self::Output {
        UnitValue {
            value: self.value / rhs.0,
            unit: self.unit,
            dimension: self.dimension,
        }
    }
}
