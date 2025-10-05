use parsexpr::lexer::{self, Lexer, Operation, Token};

fn main() {
    let expressions = vec![
        "2 + 3 * (100.50 - 4)",
        "10m to feet",
        "10m + 2",
        "20lbs to kg",
        "10 feet to in",
    ];

    for e in expressions {
        let l = Lexer::new(e);
        let tokens = l.tokenize();

        println!("---{}---", e);
        for t in tokens {
            println!("{:?}", t);
        }
        println!("-------------");
    }

    // for e in expressions {
    //     println!("\nExpression: {}", e);
    //     let l = Lexer::new(e);
    //     let tokens = l.tokenize();

    //     println!("Tokens: {:?}", tokens);

    //     let mut parser = Parser::new(tokens);
    //     match parser.parse() {
    //         Ok(expr) => {
    //             println!("AST: {:#?}", expr);
    //             println!("Result: {}", evaluate(&expr));
    //         }
    //         Err(err) => println!("Parse error: {}", err),
    //     }
    // }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Binary {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        op: Operation,
        operand: Box<Expression>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    // Entry point for parsing - parses the entire token stream and ensures all tokens are consumed
    pub fn parse(&mut self) -> Result<Expression, String> {
        let expr = self.parse_expression(0)?;
        if self.pos < self.tokens.len() {
            return Err(format!(
                "Unexpected token at position {}: {:?}",
                self.pos, self.tokens[self.pos]
            ));
        }
        Ok(expr)
    }

    // Pratt parsing algorithm - handles binary operators with correct precedence and associativity
    // min_precedence determines the minimum operator precedence this call will handle
    fn parse_expression(&mut self, min_precedence: u8) -> Result<Expression, String> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Operation(op) => {
                    let precedence = self.get_precedence(op);
                    if precedence < min_precedence {
                        break;
                    }

                    let op = match self.advance() {
                        Some(Token::Operation(o)) => o.clone(),
                        _ => unreachable!(),
                    };

                    let right_precedence = if self.is_right_associative(&op) {
                        precedence
                    } else {
                        precedence + 1
                    };

                    let right = self.parse_expression(right_precedence)?;
                    left = Expression::Binary {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Parses primary expressions: numbers, parenthesized expressions, and unary operators
    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(Expression::Number(*n)),
            Some(Token::Lparen) => {
                let expr = self.parse_expression(0)?;
                match self.advance() {
                    Some(Token::Rparen) => Ok(expr),
                    Some(other) => Err(format!("Expected ')', found {:?}", other)),
                    None => Err("Expected ')', found end of input".to_string()),
                }
            }
            Some(Token::Operation(Operation::Subtract)) => {
                let operand = self.parse_primary()?;
                Ok(Expression::Unary {
                    op: Operation::Subtract,
                    operand: Box::new(operand),
                })
            }
            Some(token) => Err(format!("Unexpected token: {:?}", token)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    // Returns the current token without consuming it
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    // Consumes and returns the current token, advancing the position
    fn advance(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    // Returns the precedence level for each operator (higher number = higher precedence)
    fn get_precedence(&self, op: &Operation) -> u8 {
        match op {
            Operation::Add | Operation::Subtract => 1,
            Operation::Multiply | Operation::Divide => 2,
            Operation::Convert => 0,
        }
    }

    // Determines if an operator is right-associative (currently all ops are left-associative)
    fn is_right_associative(&self, _op: &Operation) -> bool {
        false
    }
}

fn evaluate(expr: &Expression) -> f64 {
    match expr {
        Expression::Number(n) => *n,
        Expression::Binary { op, left, right } => {
            let left_val = evaluate(left);
            let right_val = evaluate(right);
            match op {
                Operation::Add => left_val + right_val,
                Operation::Subtract => left_val - right_val,
                Operation::Multiply => left_val * right_val,
                Operation::Divide => left_val / right_val,
                _ => panic!("Unsupported operation in evaluation"),
            }
        }
        Expression::Unary { op, operand } => {
            let val = evaluate(operand);
            match op {
                Operation::Subtract => -val,
                _ => panic!("Unsupported unary operation"),
            }
        }
    }
}
