# mathengine-parser

A robust parser for mathematical expressions with unit support, built using Pratt parsing for correct operator precedence.

[![Crates.io](https://img.shields.io/crates/v/mathengine-parser.svg)](https://crates.io/crates/mathengine-parser)
[![Documentation](https://docs.rs/mathengine-parser/badge.svg)](https://docs.rs/mathengine-parser)

## Features

- **Pratt Parsing**: Handles operator precedence and associativity correctly
- **AST Generation**: Produces a clean Abstract Syntax Tree
- **Unit Support**: Parses unit values and conversion expressions
- **Error Recovery**: Detailed parse error messages with position information
- **Type System**: Rich type definitions for numbers, units, and values

## Usage

```rust
use mathengine_lexer::Lexer;
use mathengine_parser::{Parser, Expression};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "2 + 3 * (10m to feet)";

    // Tokenize
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;

    // Parse
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    println!("AST: {:#?}", ast);
    Ok(())
}
```

## AST Structure

The parser generates a rich AST with these expression types:

```rust
pub enum Expression {
    Number(f64),                    // 42.0
    UnitValue { value: f64, unit: String }, // 10m
    Unit(String),                   // feet
    Binary {                        // 2 + 3
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {                         // -5
        op: Operation,
        operand: Box<Expression>,
    },
}
```

## Operator Precedence

The parser correctly handles mathematical precedence:

1. **Parentheses**: `(` `)`
2. **Power**: `^` (right-associative)
3. **Multiplication/Division**: `*` `/`
4. **Addition/Subtraction**: `+` `-`
5. **Unit Conversion**: `to` (highest precedence)

## Type System

Includes a comprehensive type system for mathematical values:

```rust
use mathengine_parser::types::{Number, UnitValue, Value};

// Pure numbers
let num = Number::from(42.0);

// Values with units
let distance = UnitValue::new(10.0, "meters".to_string());

// Unified value type
let result: Value = Value::Number(num);
```

## Error Handling

Comprehensive error reporting with position information:

```rust
use mathengine_parser::{Parser, ParseError};

match parser.parse() {
    Ok(ast) => println!("Success: {:?}", ast),
    Err(ParseError::UnexpectedToken { expected, found, position }) => {
        println!("Expected {} but found {:?} at position {}", expected, found, position);
    }
    Err(e) => println!("Parse error: {}", e),
}
```

## Integration

Designed to work seamlessly with other mathengine crates:

```rust
use mathengine_lexer::Lexer;
use mathengine_parser::Parser;
use mathengine_evaluator::evaluate;

fn pipeline(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let tokens = Lexer::new(input).tokenize()?;
    let ast = Parser::new(tokens).parse()?;
    let result = evaluate(&ast)?;
    Ok(result.to_string())
}
```

## Architecture

Part of the [mathengine](https://github.com/username/mathengine) workspace. This crate bridges the gap between lexical analysis and evaluation, producing a clean AST that can be traversed by evaluators, optimizers, or other analysis tools.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.