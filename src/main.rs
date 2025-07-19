

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum Token {
    Atom(char),
    Op(char),
    Eof
}

struct Lexer{
    tokens: Vec<Token>
}

impl Lexer {
    fn new(input: &str) -> Lexer{
        // Tokenize each characters
        let mut tokens = input
        .chars()
        .filter(|it| !it.is_ascii_whitespace())
        .map(|c| match c {
            '0'..='9' | 'a'..='z'|'A'..='Z' => Token::Atom(c),
            _ => Token::Op(c),
        }).collect::<Vec<_>>();
        tokens.reverse(); // To use pop

        Lexer { tokens }
    }

    fn next(&mut self) -> Token{
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&mut self)-> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

fn infix_binding_power(op:char) -> (f32, f32) {
    match op {
        '+' |'-' => (1.0, 1.1),
        '*'|'/'=> (2.0, 2.1),
        _ => panic!("bad op: {:?}", op)
    }
}

// Expr -> Atom | Expr Op Expr
enum Expression {
    Atom(char),
    Operation(char,Vec<Expression>)
    // ex) Expression::Operation('-', vec![Expression::Atom('a'), Expression::Atom('b')])
}

impl Expression {
    fn from_str(input:&str) -> Expression{
        let mut lexer = Lexer::new(input);
        parse_expression(&mut lexer, 0.0)
    }
}

fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression{
    // Left Hand Side: must be an atom
    let mut lhs = match lexer.next() {
        Token::Atom(it) => Expression::Atom(it),
        t => panic!("bad token: {:?}", t)
    };

    // To keep parsing until we reach an operator with lower binding power 
    loop {
        // Check if next token is operater
        let op = match lexer.peek() {
            Token::Op(op) => op,
            Token::Eof => break,
            t => panic!("bad token: {:?}", t) // unexpected token 
        };
        let (l_bp, r_bp) = infix_binding_power(op); // next operator's binding power
        // Break if the next binding power is less than the current binding power
        if l_bp < min_bp {
            break;
        }
        lexer.next(); // consume the operator 
        
        // Right Hand Side: parse the next expression
        // recursively call to fold deeper expressions
        // until it finds an operator with lower binding power than the current one
        let rhs = parse_expression(lexer, r_bp);
        lhs = Expression::Operation(op, vec![lhs, rhs]); // create an operation expression tree
    }
    lhs
}

fn main() {
    println!("Hello, world!");
}
        
        
