use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug,PartialEq,Clone)]
pub enum Token {
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

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
}

#[derive(Debug, PartialEq)]
enum ScanError {
    UnexpectedChar(char),
    InvalidNumber(String),
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a str) -> Self {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_identifier() {
        let text = "identifier";
        let mut scanner = Scanner::new(&text);
        //assert_eq!(scanner.next(), Some(Token::Ident("identifier".to_string())));
    }

    #[test]
    fn test_scan_number() {
        let text = "42.5";
        let mut scanner = Scanner::new(&text);
        //assert_eq!(Ok(scanner.next().unwrap()), Token::Real(42.5)));
    }

    #[test]
    fn test_scan_left_paren() {
        let text = "()";
        let mut scanner = Scanner::new(&text);
        //assert_eq!(scanner.next(), Some(Token::LeftPar));
        //assert_eq!(scanner.next(), Some(Token::RightPar));
    }
}
