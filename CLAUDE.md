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

The project is organized as a Cargo workspace with three main crates:

1. **mathengine-lexer**: Tokenization and lexical analysis
2. **mathengine-units**: Unit conversion system and dimension handling
3. **mathengine-parser**: AST parsing, evaluation, and main binary

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

3. **Parser & Evaluator** (`mathengine-parser/src/main.rs`): Fully implemented parser and evaluator using Pratt parsing. Features:
   - Recursive descent parsing with operator precedence
   - Binary operations (left operator right)
   - Unary operations (negation)
   - Unit conversions
   - Numeric literals and unit values
   - Complete evaluation engine

### Token Flow
Input String → Lexer (tokenization) → Parser (AST construction) → Evaluator (computation) → Result

### Key Design Decisions
- Workspace structure for clean separation of concerns
- No external dependencies - pure Rust implementation
- Token-based approach allows for clear separation of concerns
- Enum-based token representation provides type safety
- Supports both mathematical expressions and unit conversions in a unified syntax
- Pratt parsing for correct operator precedence and associativity

## Current State

- **Working**: Complete tokenization, parsing, evaluation, and unit conversion system
- **Functional**: All mathematical operations and unit conversions working correctly
- **Architecture**: Clean workspace structure with separate crates for lexer, units, and parser