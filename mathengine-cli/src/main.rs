use mathengine_evaluator::{EvalError, evaluate};
use mathengine_lexer::{LexError, Lexer};
use mathengine_parser::{ParseError, Parser};

fn main() {
    let expressions = vec![
        "2 + 3 * (100.50 - 4)",
        "10m to feet",
        "10m + 2",
        // "20lbs to kg",
        "10 feet to in",
        "2^10",
        "23C to f",
        "1m to miles",
        // Test error cases
        "",           // Empty input
        "2 + + 3",    // Invalid syntax
        "2 / 0",      // Division by zero
        "10xyz to m", // Unknown unit
    ];

    for e in expressions {
        println!("\nExpression: {}", e);

        // Handle the entire pipeline with proper error handling
        match process_expression(e) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn process_expression(input: &str) -> Result<String, String> {
    // Lexical analysis
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize().map_err(|e| format_lex_error(e))?;

    // Parsing
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().map_err(|e| format_parse_error(e))?;

    // Evaluation
    let result = evaluate(&expr).map_err(|e| format_eval_error(e))?;

    Ok(result.to_string())
}

fn format_lex_error(err: LexError) -> String {
    format!("Lexer error: {}", err)
}

fn format_parse_error(err: ParseError) -> String {
    format!("Parser error: {}", err)
}

fn format_eval_error(err: EvalError) -> String {
    format!("Evaluation error: {}", err)
}
