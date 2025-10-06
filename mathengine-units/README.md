# mathengine-units

A comprehensive unit conversion system supporting multiple measurement dimensions.

[![Crates.io](https://img.shields.io/crates/v/mathengine-units.svg)](https://crates.io/crates/mathengine-units)
[![Documentation](https://docs.rs/mathengine-units/badge.svg)](https://docs.rs/mathengine-units)

## Features

- **Length Units**: Meters, centimeters, millimeters, kilometers, feet, inches, yards, miles
- **Temperature Units**: Celsius, Fahrenheit, Kelvin
- **Type-Safe Conversions**: Compile-time dimension checking
- **Canonical Representations**: Consistent unit string formatting
- **Error Handling**: Comprehensive error types for invalid units and conversions

## Supported Units

### Length
- `m`, `meter`, `meters` - Meters
- `cm`, `centimeter`, `centimeters` - Centimeters
- `mm`, `millimeter`, `millimeters` - Millimeters
- `km`, `kilometer`, `kilometers` - Kilometers
- `ft`, `foot`, `feet` - Feet
- `in`, `inch`, `inches` - Inches
- `yd`, `yard`, `yards` - Yards
- `mi`, `mile`, `miles` - Miles

### Temperature
- `C`, `celsius` - Celsius
- `F`, `fahrenheit` - Fahrenheit
- `K`, `kelvin` - Kelvin

## Usage

```rust
use mathengine_units::{
    length::LengthDimension,
    temperature::TemperatureDimension,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Length conversions
    let meters = LengthDimension::from_unit("m", 10.0)?;
    let feet_unit = LengthDimension::parse_unit("feet")?;
    let feet = meters.convert_to(feet_unit);

    println!("10 meters = {} feet", feet.value()); // ~32.8 feet

    // Temperature conversions
    let celsius = TemperatureDimension::from_unit("C", 23.0)?;
    let fahrenheit_unit = TemperatureDimension::parse_unit("F")?;
    let fahrenheit = celsius.convert_to(fahrenheit_unit);

    println!("23°C = {}°F", fahrenheit.value()); // 73.4°F

    Ok(())
}
```

## Type Safety

The system prevents invalid conversions at compile time:

```rust
use mathengine_units::{length::LengthDimension, temperature::TemperatureDimension};

// This won't compile - can't convert length to temperature
// let invalid = length_meters.convert_to(temperature_unit); // ❌
```

## Error Handling

Comprehensive error handling for runtime issues:

```rust
use mathengine_units::{length::LengthDimension, UnitError};

match LengthDimension::parse_unit("invalid_unit") {
    Ok(unit) => println!("Valid unit: {:?}", unit),
    Err(UnitError::UnknownUnit(unit)) => {
        println!("Unknown unit: '{}'", unit);
    }
}
```

## Architecture

Part of the [mathengine](https://github.com/username/mathengine) workspace. This crate provides the foundation for unit-aware mathematical calculations and can be used independently in any application requiring unit conversions.

## Adding New Units

The architecture makes it easy to add new unit dimensions:

1. Create a new unit enum
2. Implement conversion factors
3. Add parsing logic
4. Implement the dimension trait

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.