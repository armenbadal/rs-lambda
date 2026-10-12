use std::str::Chars;

mod parser;
mod ast;

fn main() {
    let example0 = "lambda n m : apply sqrt (apply mul n n) (apply mul m m)";
    let mut scanner = parser::Scanner::new(example0);
    let mut parser = parser::Parser::new(&mut scanner);
    let ast = parser.parse();
}
