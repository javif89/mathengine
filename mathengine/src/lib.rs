use mathengine_evaluator::{evaluate, EvalError};
use mathengine_lexer::{LexError, Lexer};
use mathengine_parser::{ParseError, Parser};

/// Error type for expression evaluation
#[derive(Debug)]
pub enum MathEngineError {
    /// Error during lexical analysis
    Lexer(LexError),
    /// Error during parsing
    Parser(ParseError),
    /// Error during evaluation
    Evaluator(EvalError),
}

impl std::fmt::Display for MathEngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathEngineError::Lexer(e) => write!(f, "Lexer error: {}", e),
            MathEngineError::Parser(e) => write!(f, "Parser error: {}", e),
            MathEngineError::Evaluator(e) => write!(f, "Evaluation error: {}", e),
        }
    }
}

impl std::error::Error for MathEngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MathEngineError::Lexer(e) => Some(e),
            MathEngineError::Parser(e) => Some(e),
            MathEngineError::Evaluator(e) => Some(e),
        }
    }
}

impl From<LexError> for MathEngineError {
    fn from(err: LexError) -> Self {
        MathEngineError::Lexer(err)
    }
}

impl From<ParseError> for MathEngineError {
    fn from(err: ParseError) -> Self {
        MathEngineError::Parser(err)
    }
}

impl From<EvalError> for MathEngineError {
    fn from(err: EvalError) -> Self {
        MathEngineError::Evaluator(err)
    }
}

/// Evaluate a mathematical expression and return the computed value.
///
/// This function supports:
/// - Basic arithmetic: `+`, `-`, `*`, `/`, `^` (power)
/// - Parentheses for grouping: `(1 + 2) * 3`
/// - Unit arithmetic: `1m + 50cm`, `2ft - 6in`
/// - Unit conversions: `100cm to meters`, `32F to celsius`
/// - Mixed expressions: `(1m + 2m) to feet`
///
/// # Examples
///
/// Basic arithmetic:
/// ```
/// use mathengine::evaluate_expression;
///
/// let result = evaluate_expression("2 + 3 * 4").unwrap();
/// assert_eq!(result.to_string(), "14");
/// ```
///
/// Unit arithmetic and conversion:
/// ```
/// use mathengine::evaluate_expression;
///
/// // Unit arithmetic returns result in base units (meters for length)
/// let result = evaluate_expression("1m + 50cm").unwrap();
/// assert_eq!(result.to_string(), "1.5m");
///
/// // Unit conversion
/// let result = evaluate_expression("100cm to meters").unwrap();
/// assert_eq!(result.to_string(), "1m");
///
/// // Complex expressions with conversion
/// let result = evaluate_expression("(1m + 2m) to feet").unwrap();
/// // Returns approximately 9.84ft
/// ```
///
/// # Supported Units
///
/// **Length**: m, cm, mm, km, ft, in, yd, mi
/// **Temperature**: C, F, K
///
/// # Errors
///
/// Returns a `MathEngineError` if:
/// - The input cannot be tokenized (lexer error)
/// - The tokens cannot be parsed into a valid expression (parser error)
/// - The expression cannot be evaluated (evaluation error)
pub fn evaluate_expression<S: AsRef<str>>(expression: S) -> Result<crate::Value, MathEngineError> {
    // Lexical analysis
    let lexer = Lexer::new(expression.as_ref());
    let tokens = lexer.tokenize()?;

    // Parsing
    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;

    // Evaluation
    let result = evaluate(&expr)?;

    Ok(result)
}

// Re-export commonly used types for convenience
pub use mathengine_parser::types::{Value, Number, UnitValue};
pub use MathEngineError as Error;