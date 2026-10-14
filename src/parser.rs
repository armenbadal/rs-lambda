use crate::ast::Expression;
use crate::scanner::{Token, Scanner};

pub struct Parser<'a>
{
    scanner: Scanner<'a>,
    lookahead: Option<Token>
}



impl<'a> Parser<'a>
{
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self { scanner, lookahead: None }
    }

    pub fn parse(&mut self) -> Result<Expression,String> {
        self.advance()?;
        self.parse_expression()
    }

    fn advance(&mut self) -> Result<(),String> {
        self.lookahead = self.scanner.scan()?;
        Ok(())
    }

    fn advance_if(&mut self, expected: Token) -> Result<(),String> {
        match self.peek() {
            Some(t) if *t == expected =>self.advance(),
            Some(_) | None => Err("unexpected token".to_string()),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.lookahead.as_ref()
    }

    fn parse_expression(&mut self) -> Result<Expression,String> {
        match self.peek() {
            Some(Token::Ident(name)) => {
                let expr = Expression::Ident(name.clone());
                self.advance()?;
                Ok(expr)
            }
            Some(Token::Real(value)) => {
                let expr = Expression::Real(*value);
                self.advance()?;
                Ok(expr)
            }
            Some(Token::Boolean(value)) => {
                let expr = Expression::Boolean(*value);
                self.advance()?;
                Ok(expr)
            }
            Some(Token::Lambda) => self.parse_lambda(),
            Some(Token::Apply) => self.parse_apply(),
            Some(Token::If) => self.parse_if(),
            Some(Token::Let) => self.parse_let(),
            Some(Token::LeftPar) => self.parse_grouping(),
            None => Err("Unexpected end of stream".to_string()),
            _ => Err("Unexpected start of expression".to_string()),
        }
    }
    fn parse_lambda(&mut self) -> Result<Expression,String> {
        self.advance_if(Token::Lambda)?;

        let mut parameters= Vec::new();
        while let Some(Token::Ident(name)) = self.peek() {
            parameters.push(name.clone());
            self.advance()?
        }

        self.advance_if(Token::Colon)?;

        let body =self.parse_expression()?;
        Ok(Expression::Lambda { parameters, body: Box::new(body) })
    }

    fn parse_apply(&mut self) -> Result<Expression,String> {
        self.advance_if(Token::Apply)?;

        let callee = self.parse_expression()?;

        let mut arguments = Vec::new();
        while self.starts_expression() {
            let item = self.parse_expression()?;
            arguments.push(item)
        }

        Ok(Expression::Apply{ callee: Box::new(callee), arguments })
    }

    fn parse_if(&mut self) -> Result<Expression,String> {
        self.advance_if(Token::If)?;
        let condition = Box::new(self.parse_expression()?);

        self.advance_if(Token::Then)?;
        let decision = Box::new(self.parse_expression()?);

        self.advance_if(Token::Else)?;
        let alternative = Box::new(self.parse_expression()?);

        Ok(Expression::If{ condition, decision, alternative })
    }

    fn parse_let(&mut self) -> Result<Expression,String> {
        self.advance_if(Token::Let)?;

        let mut bindings = Vec::new();
        loop {
            let name = match self.peek() {
                Some(Token::Ident(n)) => {
                    let name = n.clone();
                    self.advance()?;
                    name
                }
                Some(_) | None => return Err("expecting identifier here".to_string())
            };

            self.advance_if(Token::Is)?;

            let expr = self.parse_expression()?;
            bindings.push((name, expr));

            if matches!(self.peek(), Some(Token::And)) {
                self.advance()?;
            } else {
                break;
            }
        }

        self.advance_if(Token::In)?;
        let body = Box::new(self.parse_expression()?);

        Ok(Expression::Let{ bindings, body })
    }

    fn parse_grouping(&mut self) -> Result<Expression,String> {
        self.advance_if(Token::LeftPar)?;
        let expr = self.parse_expression()?;
        self.advance_if(Token::RightPar)?;
        Ok(expr)
    }

    fn starts_expression(&mut self) -> bool {
        matches!(
            self.peek(),
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
    use crate::{parser, scanner};
    use super::*;

    #[test]
    fn test_parse_big_expression() {
        let example0 = "lambda n m : apply sqrt apply [+] (apply mul n n) (apply mul m m)";
        let mut scanner = scanner::Scanner::new(example0);
        let mut parser = parser::Parser::new(scanner);
        match parser.parse() {
            Ok(e) => {
                println!("parsed");
                println!("{e:?}")
            }
            Err(_) => { println!("failed") }
        }
        // Lambda {
        //      parameters: ["n", "m"],
        //      body: Apply {
        //              callee: Ident("sqrt"),
        //              arguments: [
        //                  Apply {
        //                      callee: Ident("[+]"),
        //                      arguments: [
        //                          Apply {
        //                              callee: Ident("mul"),
        //                              arguments: [Ident("n"), Ident("n")]
        //                          },
        //                          Apply {
        //                              callee: Ident("mul"),
        //                              arguments: [Ident("m"), Ident("m")]
        //                          }
        //                      ]
        //                  }
        //              ]
        //          }
        //      }
    }
}
