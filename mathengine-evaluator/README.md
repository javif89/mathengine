# mathengine-evaluator

A type-safe evaluation engine for mathematical expressions with comprehensive unit conversion support.

[![Crates.io](https://img.shields.io/crates/v/mathengine-evaluator.svg)](https://crates.io/crates/mathengine-evaluator)
[![Documentation](https://docs.rs/mathengine-evaluator/badge.svg)](https://docs.rs/mathengine-evaluator)

## Features

- **AST Evaluation**: Traverses parsed expressions to compute results
- **Unit-Aware Arithmetic**: Handles mixed unit/number operations
- **Type Safety**: Prevents invalid operations at runtime with detailed errors
- **Comprehensive Error Handling**: Division by zero, incompatible units, etc.
- **Cross-Dimension Conversions**: Length, temperature, and more

## Usage

```rust
use mathengine_lexer::Lexer;
use mathengine_parser::Parser;
use mathengine_evaluator::evaluate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "10m + 5ft to inches";

    // Parse the expression
    let tokens = Lexer::new(input).tokenize()?;
    let ast = Parser::new(tokens).parse()?;

    // Evaluate
    let result = evaluate(&ast)?;
    println!("Result: {}", result); // Result: 196.85in

    Ok(())
}
```

## Supported Operations

### Arithmetic Operations
- **Addition**: `2 + 3`, `10m + 5ft`
- **Subtraction**: `5 - 2`, `100cm - 1m`
- **Multiplication**: `3 * 4`, `10m * 2`
- **Division**: `8 / 2`, `20ft / 4`
- **Power**: `2^3`, `3^2`

### Unit Conversions
- **Length**: `10m to feet`, `5mi to km`
- **Temperature**: `23C to F`, `300K to celsius`

### Mixed Operations
- **Unit + Number**: `10m + 5` (adds 5 meters)
- **Number * Unit**: `2 * 10kg` (multiplies unit by scalar)

## Error Handling

Comprehensive error types for robust applications:

```rust
use mathengine_evaluator::{evaluate, EvalError};

match evaluate(&ast) {
    Ok(result) => println!("Success: {}", result),
    Err(EvalError::DivisionByZero) => {
        println!("Cannot divide by zero");
    }
    Err(EvalError::IncompatibleUnits { left_unit, right_unit, operation }) => {
        println!("Cannot {} {} and {}", operation, left_unit, right_unit);
    }
    Err(EvalError::UnknownUnit { unit }) => {
        println!("Unknown unit: '{}'", unit);
    }
    Err(e) => println!("Evaluation error: {}", e),
}
```

## Type System Integration

Works seamlessly with the mathengine type system:

```rust
use mathengine_parser::types::{Value, Number, UnitValue};

let result = evaluate(&ast)?;
match result {
    Value::Number(n) => println!("Pure number: {}", n),
    Value::UnitValue(uv) => println!("Value with unit: {}", uv),
}
```

## Evaluation Rules

### Precedence
Operations are evaluated according to mathematical precedence rules, as determined by the parser.

### Unit Compatibility
- **Same dimension**: `10m + 5ft` (converts to common unit)
- **Cross-dimension**: `10m + 5kg` ❌ (error)
- **Scalar operations**: `10m * 2` ✅ (multiplies by scalar)

### Conversion Logic
- Automatic conversion to canonical units for arithmetic
- Explicit conversions via `to` operator
- Cross-dimension conversion detection and errors

## Performance

- **Zero-allocation evaluation**: Reuses parsed AST structure
- **Lazy conversion**: Only converts units when necessary
- **Error short-circuiting**: Fails fast on invalid operations

## Architecture

Part of the [mathengine](https://github.com/username/mathengine) workspace. This crate provides the evaluation engine that computes final results from parsed ASTs, handling all mathematical operations and unit conversions.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.