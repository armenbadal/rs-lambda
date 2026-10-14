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

impl<'a> Scanner<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { source: text.chars().peekable() }
    }

    pub fn scan(&mut self) -> Result<Option<Token>,String> {
        Ok(Some(self.next_token()?))
    }

    fn next_token(&mut self) -> Result<Token,String> {
        while self.source.next_if(|c| c.is_whitespace()).is_some() {}

        match self.source.peek() {
            None => Ok(Token::Eos),
            Some(&c) => match c {
                'a'..='z' | 'A'..='Z' =>
                    self.scan_identifier_or_keyword(),
                '0'..='9' =>
                    self.scan_number(),
                '[' => {
                    self.source.next();
                    self.scan_quoted_identifier()
                }
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
                    Err(format!("Unexpected character {c}"))
                }
            }
        }
    }

    fn scan_quoted_identifier(&mut self) -> Result<Token,String> {
        let lexeme: String = self.source
            .by_ref()
            .take_while(|c| *c != ']')
            .collect();
        Ok(Token::Ident(format!("[{lexeme}]")))
    }

    fn scan_identifier_or_keyword(&mut self) -> Result<Token,String> {
        let mut lexeme = String::new();
        while let Some(c) = self.source.next_if(|c| c.is_alphanumeric()) {
            lexeme.push(c);
        }

        Ok(match lexeme.as_str() {
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
            _ => Token::Ident(lexeme),
        })
    }

    fn scan_number(&mut self) -> Result<Token,String> {
        let number: String = self.source
            .by_ref()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();

        number.parse()
            .map(Token::Real)
            .map_err(|_| format!("Invalid number {number}"))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_identifier() {
        let text = "identifier";
        let mut scanner = Scanner::new(&text);
        assert_eq!(scanner.scan().unwrap(), Some(Token::Ident("identifier".to_string())));
    }

    #[test]
    fn test_scan_quoted_identifier() {
        let text = "[++] and [dashed-name]";
        let mut scanner = Scanner::new(&text);
        let tok = scanner.scan().unwrap();
        assert_eq!(tok, Some(Token::Ident("[++]".to_string())))
    }

    #[test]
    fn test_scan_number() {
        let text = "42.5";
        let mut scanner = Scanner::new(&text);
        assert_eq!(scanner.scan().unwrap(), Some(Token::Real(42.5)))
    }

    #[test]
    fn test_scan_left_and_right_paren() {
        let text = "()";
        let mut scanner = Scanner::new(&text);
        assert_eq!(scanner.scan().unwrap(), Some(Token::LeftPar));
        assert_eq!(scanner.scan().unwrap(), Some(Token::RightPar));
    }
}
