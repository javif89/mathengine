use mathengine::{Error, Value, evaluate_expression};

fn main() {
    let expressions = vec![
        // Test unit arithmetic
        // "1m + 1m + 100cm", // Should be 3m
        // "10ft + 2m",       // Should convert to base (meters)
        // "5m - 200cm",      // Should be 3m
        // "100in - 1ft",     // Should convert to base (meters)
        "(1m + 2m) to cm",
        "(1m + 2m) to yards",
        "1 meter to centimeters",
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
