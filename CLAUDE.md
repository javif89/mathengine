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

This is a mathematical expression parser and evaluator with unit conversion capabilities, implemented in Rust. The project follows a traditional compiler architecture pattern:

### Core Components

1. **Lexer** (`src/lexer/mod.rs`): Fully implemented tokenizer that converts input strings into tokens. Handles:
   - Mathematical operators (+, -, *, /)
   - Numbers (integers and floats)
   - Parentheses for grouping
   - Unit conversion keywords ("to")
   - Unit identifiers (feet, meters, celsius, fahrenheit, etc.)

2. **Parser** (`src/main.rs`): Currently partially implemented. Will build an Abstract Syntax Tree (AST) from tokens using recursive descent parsing. The AST nodes represent:
   - Binary operations (left operator right)
   - Unary operations
   - Unit conversions
   - Numeric literals

3. **Evaluator**: Not yet implemented. Will traverse the AST to compute results and perform unit conversions.

### Token Flow
Input String → Lexer (tokenization) → Parser (AST construction) → Evaluator (computation) → Result

### Key Design Decisions
- No external dependencies - pure Rust implementation
- Token-based approach allows for clear separation of concerns
- Enum-based token representation provides type safety
- Supports both mathematical expressions and unit conversions in a unified syntax

## Current State

- **Working**: Lexer successfully tokenizes all supported input formats
- **In Progress**: Parser structure exists but needs completion
- **TODO**: Evaluator implementation, unit conversion logic, error handling improvements