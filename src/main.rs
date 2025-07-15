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

        Lexer { tokens }
    }
}
fn main() {
    println!("Hello, world!");
}
