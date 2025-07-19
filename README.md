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
Lexer takes input string and tokenize it into token list.

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

impl Expression {
    fn from_str(input:&str) -> Expression{
        let mut lexer = Lexer::new(input);
        parse_expression(&mut lexer)
    }
}
```

## Parser
Parser takes token list produced from lexer, and parses it into tree.
