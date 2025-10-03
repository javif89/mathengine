use parsexpr::lexer::{self, Lexer};

fn main() {
    // let expressions = vec!["2 + 3 * (100.50 - 4)", "23F to C", "1m to feet"];
    // let expressions = vec!["23C to F", "1m to feet", "10cm to in"];
    let expressions = vec!["2 + 3 * (100.50 - 4)"];

    for e in expressions {
        println!("{}", e);
        let l = Lexer::new(e);
        let tokens = l.tokenize();

        for t in tokens {
            println!("{:#?}", t);
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Binary {
        op: lexer::Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

pub struct Parser {
    tokens: Vec<lexer::Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&lexer::Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> lexer::Token {
        let tok = self
            .tokens
            .get(self.pos)
            .cloned()
            .unwrap_or(lexer::Token::EOF);
        self.pos += 1;
        tok
    }

    fn consume(&mut self, expected: &lexer::Token) {
        let tok = self.advance();
        if &tok != expected {
            panic!("Expected {:?}, got {:?}", expected, tok);
        }
    }
}
