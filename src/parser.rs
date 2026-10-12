use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug,PartialEq,Clone)]
enum Token {
    Real(f64),
    Boolean(bool),
    Ident(String),
    LeftPar,
    RightPar,
    Lambda,
    Colon,
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

struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
}

#[derive(Debug, PartialEq)]
enum ScanError {
    UnexpectedChar(char),
    InvalidNumber(String),
}

impl<'a> Scanner<'a> {
    fn new(text: &'a str) -> Self {
        Self { source: text.chars().peekable() }
    }

    fn next_token(&mut self) -> Option<Result<Token,ScanError>> {
        self.skip_whitespace();

        let next = *self.source.peek()?;
        let result = match next {
            c if c.is_alphabetic() =>
                Ok(self.scan_identifier_or_keyword()),
            c if c.is_ascii_digit() =>
                self.scan_number(),
            '(' => {
                self.source.next();
                Ok(Token::LeftPar)
            }
            ')' => {
                self.source.next();
                Ok(Token::RightPar)
            }
            ':' => {
                self.source.next();
                Ok(Token::Colon)
            }
            c => {
                self.source.next();
                Err(ScanError::UnexpectedChar(c))
            }
        };

        Some(result)
    }

    fn skip_whitespace(&mut self) {
        while self.source.next_if(|c| c.is_whitespace()).is_some() {}
    }

    fn scan_identifier_or_keyword(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(c) = self.source.next_if(|c| c.is_alphanumeric()) {
            identifier.push(c);
        }

        match identifier.as_str() {
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "lambda" => Token::Lambda,
            "apply" => Token::Apply,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "let" => Token::Let,
            "is" => Token::Is,
            "and" => Token::And,
            "in" => Token::In,
            _ => Token::Ident(identifier),
        }
    }

    fn scan_number(&mut self) -> Result<Token,ScanError> {
        let number: String = self
            .source
            .by_ref()
            .take_while(|c| c.is_ascii_digit())
            .collect();

        let mut number = number;

        if self.source.next_if(|c| *c == '.').is_some() {
            number.push('.');
            let decimal: String = self
                .source
                .by_ref()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            number.push_str(&decimal);
        }

        number
            .parse()
            .map(Token::Real)
            .map_err(|_| ScanError::InvalidNumber(number))
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token,ScanError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

struct Parser<S> 
where
    S: Iterator<Item=Token> 
{
    scanner: Peekable<S>,
}

impl<S> Parser<S> 
where
    S: Iterator<Item=Token> 
{
    fn new(scanner: S) -> Self {
        Self { scanner: scanner.peekable() }
    }

    fn parse_expression(&mut self) {
        let look = self.scanner.peek();
    }

    fn parse_lambda(&mut self) {
        self.match_token(Token::Lambda);
        while let Some(Token::Ident(_)) = self.scanner.peek() {
            self.scanner.next();
        }
        self.match_token(Token::Colon);
        self.parse_expression()
    }

    fn parse_apply(&mut self) {
        self.match_token(Token::Apply);
        self.parse_expression();
        while self.starts_expression() {
            self.parse_expression();
        }
    }

    fn parse_if(&mut self) {
        self.match_token(Token::If);
        self.parse_expression();
        self.match_token(Token::Then);
        self.parse_expression();
        self.match_token(Token::Else);
        self.parse_expression();
    }

    fn parse_let(&mut self) {
        self.match_token(Token::Let);
        self.match_token(Token::Ident);
        self.match_token(Token::Is);
        self.parse_expression();
        while self.scanner.peek().unwrap() == Token::And {
            self::match_token(Token::And);
            self.match_token(Token::Ident);
            self.match_token(Token::Is);
            self.parse_expression();
        }
        self.match_token(Token::In);
        self.parse_expression();
    }

    fn match_token(&mut self, expected: Token) -> bool {
        if let Some(token) = self.scanner.peek() {
            if *token == expected {
                self.scanner.next();
                return true;
            }
        }
        false
    }

    fn starts_expression(&mut self) -> bool {
        let current = self.scanner.peek().unwrap();
        vec![
            Token::Ident(String::new()),
            Token::Real(0.0),
            Token::Boolean(true),
            Token::Boolean(false),
            Token::LeftPar,
            Token::Lambda,
            Token::Apply,
            Token::If,
            Token::Let,
        ].contains(&current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_identifier() {
        let text = "identifier".into();
        let mut scanner = Scanner::new(&text);
        assert_eq!(scanner.next(), Some(Token::Ident("identifier".into())));
    }

    #[test]
    fn test_scan_number() {
        let text = "42.5".into();
        let mut scanner = Scanner::new(&text);
        assert_eq!(scanner.next(), Some(Token::Real(42.5)));
    }

    #[test]
    fn test_scan_left_paren() {
        let text = "()".into();
        let mut scanner = Scanner::new(&text);
        assert_eq!(scanner.next(), Some(Token::LeftPar));
        assert_eq!(scanner.next(), Some(Token::RightPar));
    }
}

