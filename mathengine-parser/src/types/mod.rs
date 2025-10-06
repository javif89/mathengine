pub mod conversion_error;
pub mod dimensions;
pub mod number;
pub mod unit_value;
pub mod value;

// Re-export all types for easy access
pub use conversion_error::ConversionError;
pub use dimensions::{DimensionType, Unit};
pub use number::Number;
pub use unit_value::UnitValue;
pub use value::Value;