use std::collections::HashMap;
use std::rc::Rc;
use std::str::Chars;
use crate::ast::Expression;

mod parser;
mod ast;
mod scanner;
mod evaluator;
mod builtins;

type Builtin = fn(Vec<V>) -> Result<V,String>;

#[derive(Debug)]
enum V {
    R(f64),
    B(Builtin),
    L(Vec<V>),
}

fn main() {
    let mut env: HashMap<String,Builtin> = HashMap::new();
    env.insert("f".to_string(), |args: Vec<V>|->Result<V,String> {
        Ok(V::L(args))
    });
    env.insert("sqr".to_string(), |args: Vec<V>|->Result<V,String> {
        match &args[0] {
            V::R(v) => Ok(V::R(v.sqrt())),
            _ => Err("invalid argument".to_string())
        }
    });


    let a0 = V::R(3.1415);
    let a1 = V::R(0.999);
    let args = vec![a0, a1];
    let f= env.get("f").unwrap();
    let r0 = f(args).unwrap();
    println!("{r0:?}");

    let r1 = env.get("sqr").unwrap()(vec![V::R(3.1415)]);
    println!("{r1:?}");
}

fn test() {
    // let example0 = "lambda n m : apply sqrt (apply mul n n) (apply mul m m)";
    // let mut scanner = scanner::Scanner::new(example0);
    // let mut parser = parser::Parser::new(scanner);
    //let ast = parser.parse();
}