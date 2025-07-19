# Study Pratt-parsing

## Parsing

1. Manually
    - Hand-writing the parser.
    - Use **Pratt Parsing**
2. **Context Free Grammar**
    - Using a **domain-specific language** to specify and abstract the grammar of the language.
    - Use **Backus-Naur Form**

## Token

In here, token can be...
- **'0' to '9'**, not '10'
- **'a' to 'z'**, **'A' to 'Z'**
- **'+', '-', '*', '/', ...**

```rust
enum Token{
    Atom(char),
    Op(char),
    Eof
}
```

## Lexer
Lexer takes input string and tokenize it into **sequence of tokens**.

```rust
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
```

- For **"a * b - 1 / a"**, lexer follows next steps.

```
1. "a * b - 1 / a"
2. ['a', ' ', '*', ' ', 'b', ' ', '-', ' ', '1', ' ', '/', ' ', 'a']

3. ['a', '*', 'b', '-', '1', '/', 'a']
4. [
    Token::Atom('a'),
    Token::Op('*'),
    Token::Atom('b'),
    Token::Op('-'),
    Token::Atom('1'),
    Token::Op('/'),
    Token::Atom('a')
   ]
```

## Expression
Expression can be ...
- either **single atom** like number or a **variable**
- **operation** between other expressions

```rust
// Expr -> Atom | Expr Op Expr
enum Expression {
    Atom(char),
    Operation(char,Vec<Expression>)
    // ex) Expression::Operation('-', vec![Expression::Atom('a'), Expression::Atom('b')])
}
```

- For **"a * b - 1 / a"**, it can be parsed into

```rust
Expression::Operation('-', vec![
        Expression::Operation('*', vec![
            Expression::Atom('a'),
            Expression::Atom('b')
            ]
        ),
        Expression::Operation('/', vec![
            Expression::Atom('1'),
            Expression::Atom('a')
            ]
        )
    ]
)
```

## Binding Power
In Pratt-Parsing, instead of thinking in terms of whether one operator has higher or lower precedence than another, we talk about **binding power**.

For example, '+', '-' have 1 binding power, while '*', '/' have 2 binding power.

But, sometimes an operand might have the **same operator on both sides**, like in the case of `a + b * c + 2 / 4`. In this case, we can assign **more binding power on one side than on the other**.

For example:
```
EXPR:       a    +    b    *    c    +    2    /    4 
B POWER:  0  (1.0,1.1) (2.0,2.1) (1.0,1.1) (2.0,2.1)  0

EXPR:       a    +    (b*c)    +    (2/4)
B POWER:  0  (1.0,1.1)     (1.0,1.1)      0

EXPR:       (a+(b*c))    +    (2/4)
B POWER:  0          (1.0,1.1)      0

EXPR:       ((a+(b*c))+(2/4))
B POWER:  0                   0
```


```rust
fn infix_binding_power(op:char) -> (f32, f32) {
    match op {
        '+' |'-' => (1.0, 1.1),
        '*'|'/'=> (2.0, 2.1),
        _ => panic!("bad op: {:?}", op)
    }
}
```

## Parser
Parser takes sequence of tokens produced from lexer, and parses it into tree.

```rust
impl Expression {
    fn from_str(input:&str) -> Expression{
        let mut lexer = Lexer::new(input); // sequence of tokens
        parse_expression(&mut lexer, 0.0) // parse it into Expression.
    }
}

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
            Token::Op(op) => op,
            Token::Op(')') => break, // escape loop on closing parenthesis
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
```