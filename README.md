# MathEngine

A Rust mathematical expression evaluator with unit conversion support.

## Features

- Mathematical expressions: `2 + 3 * (100.50 - 4)`
- Unit conversions: `10m to feet`, `23C to F`
- Mixed operations: `1m to cm + 10`
- Power operations: `2^10`

## Examples

```rust
use mathengine::{lexer::Lexer, Parser, evaluate};

let expression = "23C to F";
let lexer = Lexer::new(expression);
let tokens = lexer.tokenize();
let mut parser = Parser::new(tokens);
let ast = parser.parse().unwrap();
let result = evaluate(&ast);
println!("{}", result); // 73.4F
```

## Supported Units

- **Length**: meters, centimeters, feet, inches, yards, miles
- **Temperature**: Celsius, Fahrenheit, Kelvin

## Building

```bash
cargo build
cargo run
```