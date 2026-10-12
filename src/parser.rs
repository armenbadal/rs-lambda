use std::iter::Peekable;
use std::str::Chars;

enum Token {
    Real(f64),
    Boolean(bool),
    Ident(String),
    LeftPar,
    RightPar,
    LeftBr,
    RightBr,
    Colon,
    Lambda,
    Apply,
    If,
    Then,
    Else,
    Let,
    Is,
    And,
    In,
    Eos
}

struct Lexeme {
    token: Token,
    value: String
}

struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    fn new(text: &'a String) -> Self {
        Self { source: text.chars().peekable() }
    }

    fn next_lexeme(&mut self) -> Option<Lexeme> {
        self.skip_whitespace();
        if let Some(c) = self.source.peek() {
            if c.is_alphabetic() {
                let s = self.scan_identifier_or_keyword();
            }
        }
        None
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.source.peek() {
            if !c.is_whitespace() { break }
            self.source.next();
        }
    }

    fn scan_identifier_or_keyword(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.source.peek() {
            if !c.is_alphanumeric() { break }
            result.push(self.source.next().unwrap());
        }
        result
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_lexeme()
    }
}

struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
}
