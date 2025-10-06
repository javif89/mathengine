# mathengine-lexer

A high-performance lexical analyzer for mathematical expressions with unit support.

[![Crates.io](https://img.shields.io/crates/v/mathengine-lexer.svg)](https://crates.io/crates/mathengine-lexer)
[![Documentation](https://docs.rs/mathengine-lexer/badge.svg)](https://docs.rs/mathengine-lexer)

## Features

- **Mathematical Operators**: `+`, `-`, `*`, `/`, `^` (power)
- **Numbers**: Integers and floating-point literals
- **Unit Values**: Numbers with attached units (e.g., `10m`, `23.5C`)
- **Parentheses**: Grouping support with `(` and `)`
- **Unit Conversion**: `to` keyword for conversions
- **Comprehensive Error Handling**: Detailed error messages with position information

## Usage

```rust
use mathengine_lexer::{Lexer, Token, Operation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lexer = Lexer::new("10m + 2 * (3.5 - 1)");
    let tokens = lexer.tokenize()?;

    for token in tokens {
        println!("{:?}", token);
    }
    // Output:
    // UnitValue { value: 10.0, unit: "m" }
    // Operation(Add)
    // Number(2.0)
    // Operation(Multiply)
    // Lparen
    // Number(3.5)
    // Operation(Subtract)
    // Number(1.0)
    // Rparen

    Ok(())
}
```

## Token Types

- `Token::Number(f64)` - Numeric literals
- `Token::UnitValue { value: f64, unit: String }` - Numbers with units
- `Token::Unit(String)` - Standalone units
- `Token::Operation(Operation)` - Mathematical operators
- `Token::Lparen` / `Token::Rparen` - Parentheses

## Error Handling

The lexer provides detailed error information:

```rust
use mathengine_lexer::{Lexer, LexError};

let lexer = Lexer::new("10 + @invalid");
match lexer.tokenize() {
    Ok(tokens) => println!("Tokens: {:?}", tokens),
    Err(LexError::UnexpectedCharacter { char, position }) => {
        println!("Unexpected character '{}' at position {}", char, position);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Architecture

Part of the [mathengine](https://github.com/username/mathengine) workspace - a modular mathematical expression evaluator. This crate focuses purely on lexical analysis and can be used independently or as part of the complete pipeline.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.