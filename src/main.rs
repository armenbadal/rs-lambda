use std::str::Chars;

mod parser;
mod ast;
mod scanner;
mod evaluator;

fn main() {
    let example0 = "lambda n m : apply sqrt (apply mul n n) (apply mul m m)";
    let mut scanner = scanner::Scanner::new(example0);
    let mut parser = parser::Parser::new(&mut scanner);
    let ast = parser.parse();
}
