use std::{collections::HashMap, fmt, io::{self, Write}};

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
            '0'..='9'|'a'..='z'|'A'..='Z' => Token::Atom(c),
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
        '=' => (0.2, 0.1),
        '+' |'-' => (1.0, 1.1),
        '*'|'/'=> (2.0, 2.1),
        '^' => (3.1, 3.0),
        // Just add new operators here
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

    fn is_asign(&self) -> Option<(char, &Expression)> {
        match self {
            Expression::Atom(_) => return None,
            Expression::Operation(c, operands) => {
                if *c == '=' { // Check if the operation is an assignment
                    let var_name = match operands.first().unwrap() {
                        Expression::Atom(c) => {
                            if *c >= 'a' && *c <= 'z' || *c >= 'A' && *c <= 'Z' {
                                *c
                            } else {
                                panic!("Not a variable name: {}", c);
                            }
                        }
                        _ => unreachable!() 
                        
                    };
                    return Some((var_name, operands.last().unwrap()));
                }
                return None;
        }

    }
}

    fn eval(&self, variables: &HashMap<char, f32>) -> f32 {
        match self {
            Expression::Atom(c) => {
                match c {
                    '0'..='9' => c.to_digit(10).unwrap() as f32,
                    'a'..='z' | 'A'..='Z' => {
                        *variables.get(c).expect(&format!("Undefined variable {}", c))
                    }
                _ => unreachable!()
                }
            }
            Expression::Operation(operator, operands) => {
                let lhs = operands.first().unwrap().eval(variables);
                let rhs = operands.last().unwrap().eval(variables);

                match operator {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    '^' => lhs.powf(rhs),
                    op => panic!("Bad operator: {}", op),
                }
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Atom(i) => write!(f, "{}", i),
            Expression::Operation(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

// Parser
fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression{
    // Left Hand Side: must be an atom
    let mut lhs = match lexer.next() {
        Token::Atom(it) => Expression::Atom(it),
        Token::Op('(') => {
            let lhs = parse_expression(lexer, 0.0); // fold deeper expressions
            assert_eq!(lexer.next(), Token::Op(')')); // consume and expect closing parenthesis
            lhs
            },
        t => panic!("bad token: {:?}", t)
    };

    // To keep parsing until we reach an operator with lower binding power 
    loop {
        // Check if next token is operater
        let op = match lexer.peek() {
            Token::Op(')') => break, // escape loop on closing parenthesis
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
    let mut variables: HashMap<char,f32> = HashMap::new(); // Store

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let input = {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            buf
        };

        if input.trim() == "exit" {
            println!("Terminated");
            break;
        }
    
        let expr = Expression::from_str(&input);
        // If assignment expression
        if let Some((var_name, lhs)) = expr.is_asign(){
            let value = lhs.eval(&variables);
            variables.insert(var_name, value);
            continue;
        }
        let value = expr.eval(&variables);
        println!("{}", value);
    }
}
        
        
