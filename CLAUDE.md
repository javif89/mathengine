# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Build and Run
- **Build project**: `cargo build`
- **Run project**: `cargo run`
- **Build release version**: `cargo build --release`
- **Check compilation without building**: `cargo check`

### Code Quality
- **Format code**: `cargo fmt`
- **Run linter**: `cargo clippy`
- **Fix linter warnings**: `cargo clippy --fix`

### Testing
- **Run tests**: `cargo test` (Note: No tests currently exist)
- **Run tests with output**: `cargo test -- --nocapture`
- **Run single test**: `cargo test test_name`

## Architecture

This is a mathematical expression parser and evaluator with unit conversion capabilities, implemented in Rust as a Cargo workspace with separate crates for clean dependency management.

### Workspace Structure

The project is organized as a Cargo workspace with five main crates following classic compiler architecture:

1. **mathengine-lexer**: Tokenization and lexical analysis
2. **mathengine-units**: Unit conversion system and dimension handling
3. **mathengine-parser**: AST definition and parsing logic
4. **mathengine-evaluator**: Expression evaluation engine
5. **mathengine-cli**: Command-line interface and main binary

### Core Components

1. **Lexer** (`mathengine-lexer/src/lib.rs`): Fully implemented tokenizer that converts input strings into tokens. Handles:
   - Mathematical operators (+, -, *, /)
   - Numbers (integers and floats)
   - Parentheses for grouping
   - Unit conversion keywords ("to")
   - Unit identifiers (feet, meters, celsius, fahrenheit, etc.)

2. **Units System** (`mathengine-units/`): Complete unit conversion system with:
   - Length conversions (meters, feet, inches, miles, etc.)
   - Temperature conversions (Celsius, Fahrenheit, Kelvin)
   - Error handling for unknown units
   - Canonical unit representations

3. **Parser** (`mathengine-parser/src/`): AST definition and parsing logic using Pratt parsing. Features:
   - Expression AST types in `ast.rs`
   - Recursive descent parser with operator precedence
   - Handles binary operations, unary operations, and unit values
   - Clean separation between parsing and evaluation

4. **Evaluator** (`mathengine-evaluator/src/lib.rs`): Expression evaluation engine. Features:
   - Traverses AST to compute results
   - Handles arithmetic operations and unit conversions
   - Type-safe evaluation with proper error handling
   - Supports mixed unit/number arithmetic

5. **CLI** (`mathengine-cli/src/main.rs`): Command-line interface that orchestrates the pipeline:
   - Lexer → Parser → Evaluator → Result
   - Clean separation of concerns
   - Easy to extend with different interfaces

### Processing Pipeline
Input String → Lexer (tokenization) → Parser (AST construction) → Evaluator (computation) → Result

### Key Design Decisions
- **Classic Compiler Architecture**: Clear separation of lexing, parsing, and evaluation phases
- **Workspace Structure**: Each phase isolated in its own crate with minimal dependencies
- **AST as Contract**: Expression AST serves as stable interface between parser and evaluator
- **No external dependencies**: Pure Rust implementation
- **Type Safety**: Enum-based token and AST representation
- **Pratt Parsing**: Correct operator precedence and associativity
- **Extensible Design**: Easy to add new frontends, backends, or optimization passes

### Dependency Graph
```
CLI → Evaluator → Parser → Lexer
              ↘         ↗
                 Units
```

## Current State

- **Complete Pipeline**: Full lexer → parser → evaluator → CLI implementation
- **Functional**: All mathematical operations and unit conversions working correctly
- **Clean Architecture**: Five-crate workspace with proper separation of concerns
- **Ready for Extension**: Architecture supports adding new features, frontends, or optimization passes