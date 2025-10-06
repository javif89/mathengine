# mathengine

A complete mathematical expression evaluator with comprehensive unit conversion support.

[![Crates.io](https://img.shields.io/crates/v/mathengine.svg)](https://crates.io/crates/mathengine)
[![Documentation](https://docs.rs/mathengine/badge.svg)](https://docs.rs/mathengine)

## Features

- **Simple API**: One function to evaluate any mathematical expression
- **Arithmetic Operations**: `+`, `-`, `*`, `/`, `^` with proper precedence
- **Unit Conversions**: Length, temperature, and more dimensions
- **Type Safety**: Returns proper `Value` types, not strings
- **Comprehensive Errors**: Detailed error types for debugging

## Quick Start

```rust
use mathengine::{evaluate_expression, Value};

fn main() -> Result<(), mathengine::Error> {
    // Simple arithmetic
    let result = evaluate_expression("2 + 3 * 4")?;
    println!("Result: {}", result); // Result: 14

    // Unit conversions
    let result = evaluate_expression("10m to feet")?;
    println!("Result: {}", result); // Result: 32.808ft

    // Complex expressions
    let result = evaluate_expression("(100F - 32) * 5/9")?;
    println!("Result: {}", result); // Result: 37.778

    Ok(())
}
```

## Working with Values

The library returns a `Value` enum that can be pattern matched:

```rust
use mathengine::{evaluate_expression, Value, Number, UnitValue};

match evaluate_expression("10m + 5")? {
    Value::Number(n) => {
        println!("Plain number: {}", n);
    }
    Value::UnitValue(uv) => {
        println!("Value with unit: {}", uv);
        // Access the raw value if needed
        // let raw = uv.value();
    }
}
```

## Error Handling

The library provides detailed error information:

```rust
use mathengine::{evaluate_expression, Error};

match evaluate_expression("2 / 0") {
    Ok(value) => println!("Result: {}", value),
    Err(Error::Lexer(e)) => eprintln!("Tokenization failed: {}", e),
    Err(Error::Parser(e)) => eprintln!("Parsing failed: {}", e),
    Err(Error::Evaluator(e)) => eprintln!("Evaluation failed: {}", e),
}
```

## Supported Operations

### Arithmetic
- Addition: `2 + 3`
- Subtraction: `5 - 2`
- Multiplication: `3 * 4`
- Division: `8 / 2`
- Power: `2^3`
- Parentheses: `2 * (3 + 4)`

### Units
- Length: `m`, `cm`, `mm`, `km`, `ft`, `in`, `yd`, `mi`
- Temperature: `C`, `F`, `K`
- Conversions: `10m to feet`, `23C to F`

### Mixed Operations
- `10m + 5` (adds 5 meters)
- `10ft * 2` (multiplies by scalar)
- `100cm - 1m` (automatic conversion)

## Advanced Usage

For more control, you can use the individual components:

```rust
use mathengine_lexer::Lexer;
use mathengine_parser::Parser;
use mathengine_evaluator::evaluate;

// Manual pipeline
let tokens = Lexer::new("2 + 3").tokenize()?;
let ast = Parser::new(tokens).parse()?;
let result = evaluate(&ast)?;
```

## Crate Structure

This is the main crate that ties together:
- `mathengine-lexer`: Tokenization
- `mathengine-parser`: AST generation
- `mathengine-evaluator`: Expression evaluation
- `mathengine-units`: Unit conversion system

You can also use these crates individually for more fine-grained control.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.