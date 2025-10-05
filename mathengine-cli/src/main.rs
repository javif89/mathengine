use mathengine_lexer::Lexer;
use mathengine_parser::Parser;
use mathengine_evaluator::evaluate;

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
    ];

    for e in expressions {
        println!("\nExpression: {}", e);
        let l = Lexer::new(e);
        let tokens = l.tokenize();

        // println!("Tokens: {:?}", tokens);

        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(expr) => {
                // println!("AST: {:#?}", expr);
                println!("Result: {}", evaluate(&expr));
            }
            Err(err) => println!("Parse error: {}", err),
        }
    }
}