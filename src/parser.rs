use std::iter::Peekable;
use crate::ast::Expression;
use crate::scanner::Token;

pub struct Parser<S>
where
    S: Iterator<Item=Token>
{
    scanner: Peekable<S>,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEof,
    UnexpectedExpression,
    UnexpectedToken(Token),
    ExpectedIdent(Token),
}

impl<S> Parser<S> 
where
    S: Iterator<Item=Token> 
{
    pub fn new(scanner: S) -> Self {
        Self { scanner: scanner.peekable() }
    }

    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match self.scanner.peek() {
            Some(Token::Ident(_)) => self.parse_ident(),
            Some(Token::Real(_)) => self.parse_real(),
            Some(Token::Boolean(_)) => self.parse_boolean(),
            Some(Token::Lambda) => self.parse_lambda(),
            Some(Token::Apply) => self.parse_apply(),
            Some(Token::If) => self.parse_if(),
            Some(Token::Let) => self.parse_let(),
            Some(Token::LeftPar) => self.parse_grouping(),
            None => Err(ParseError::UnexpectedEof),
            _ => Err(ParseError::UnexpectedExpression),
        }
    }

    fn parse_ident(&mut self) -> Result<Expression, ParseError> {
        if let Some(Token::Ident(name)) = self.scanner.next() {
            Ok(Expression::Ident(name))
        } else {
            Err(ParseError::UnexpectedEof) // review
        }
    }

    fn parse_real(&mut self) -> Result<Expression, ParseError> {
        if let Some(Token::Real(value)) = self.scanner.next() {
            Ok(Expression::Real(value))
        } else {
            Err(ParseError::UnexpectedEof) // review
        }
    }

    fn parse_boolean(&mut self) -> Result<Expression, ParseError> {
        if let Some(Token::Boolean(value)) = self.scanner.next() {
            Ok(Expression::Boolean(value))
        } else {
            Err(ParseError::UnexpectedEof) // review
        }
    }

    fn parse_lambda(&mut self) -> Result<Expression, ParseError> {
        self.expect(Token::Lambda)?;
        let parameters = self.parse_ident_list();
        self.expect(Token::Colon)?;
        let body =self.parse_expression()?;
        Ok(Expression::Lambda { parameters, body: Box::new(body) })
    }

    fn parse_apply(&mut self) -> Result<Expression, ParseError> {
        self.expect(Token::Apply)?;

        let functor = self.parse_expression()?;
        let mut arguments = Vec::new();
        while self.starts_expression() {
            let item = self.parse_expression()?;
            arguments.push(item)
        }
        Ok(Expression::Apply{ callee: Box::new(functor), arguments })
    }

    fn parse_if(&mut self) -> Result<Expression, ParseError> {
        self.expect(Token::If)?;
        let condition = Box::new(self.parse_expression()?);
        self.expect(Token::Then)?;
        let decision = Box::new(self.parse_expression()?);
        self.expect(Token::Else)?;
        let alternative = Box::new(self.parse_expression()?);
        Ok(Expression::If{ condition, decision, alternative })
    }

    fn parse_let(&mut self) -> Result<Expression, ParseError> {
        self.expect(Token::Let)?;
        let mut bindings = Vec::new();
        bindings.push(self.parse_binding()?);
        while matches!(self.scanner.peek(), Some(Token::And)) {
            self.expect(Token::And)?;
            bindings.push(self.parse_binding()?);
        }
        self.expect(Token::In)?;
        let body = Box::new(self.parse_expression()?);
        Ok(Expression::Let{ bindings, body })
    }

    fn parse_binding(&mut self) -> Result<(String, Expression), ParseError> {
        let name = match self.scanner.next() {
            Some(Token::Ident(name)) => name,
            Some(token) => return Err(ParseError::ExpectedIdent(token)),
            None => return Err(ParseError::UnexpectedEof),
        };
        self.expect(Token::Is)?;
        let expr = self.parse_expression()?;
        Ok((name, expr))
    }

    fn parse_grouping(&mut self) -> Result<Expression, ParseError> {
        self.expect(Token::LeftPar)?;
        let expr = self.parse_expression()?;
        self.expect(Token::RightPar)?;
        Ok(expr)
    }

    fn parse_ident_list(&mut self) -> Vec<String> {
        let mut names = Vec::new();
        while matches!(self.scanner.peek(), Some(Token::Ident(_))) {
            if let Some(Token::Ident(name)) = self.scanner.next() {
                names.push(name);
            }
        }
        names
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.scanner.peek() {
            Some(t) if t == &expected => {
                self.scanner.next();
                Ok(())
            }
            Some(t) => Err(ParseError::UnexpectedToken(t.clone())),
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn starts_expression(&mut self) -> bool {
        matches!(
            self.scanner.peek(),
            Some(Token::Ident(_)
               | Token::Real(_)
               | Token::Boolean(_)
               | Token::LeftPar
               | Token::Lambda
               | Token::Apply
               | Token::If
               | Token::Let))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
