use mathengine::{Error, Value, evaluate_expression};

fn main() {
    let expressions = vec![
        // "2 + 3 * (100.50 - 4)",
        // "10m to feet",
        // "10m + 2",
        // // "20lbs to kg",
        // "10 feet to in",
        // "2^10",
        // "23C to f",
        // "1m to miles",
        "1m + 1m + 100cm",
        // Test error cases
        // "",           // Empty input
        // "2 + + 3",    // Invalid syntax
        // "2 / 0",      // Division by zero
        // "10xyz to m", // Unknown unit
    ];

    for e in expressions {
        println!("\nExpression: {}", e);

        match evaluate_expression(e) {
            Ok(value) => print_result(value),
            Err(err) => print_error(err),
        }
    }
}

fn print_result(value: Value) {
    match value {
        Value::Number(n) => println!("Result: {}", n),
        Value::UnitValue(uv) => println!("UNIT RESULT: {}", uv),
    }
}

fn print_error(err: Error) {
    // The Display implementation already formats errors nicely
    eprintln!("Error: {}", err);

    // If you want to get the source error for more detail:
    if let Some(source) = std::error::Error::source(&err) {
        eprintln!("  Caused by: {}", source);
    }
}
