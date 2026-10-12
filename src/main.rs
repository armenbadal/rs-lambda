use std::str::Chars;

mod parser;
mod ast;

struct S<'a> {
    src: Chars<'a>
}

impl<'a> S<'a> {
    fn new(s: &'a String) -> Self {
        Self {
            src: s.chars()
        }
    }
    fn next(&mut self) -> Option<char> {
        let c = self.src.next();
        c
    }
}

fn main() {
    let t = "Hello, world!".to_string();
    let mut p = S::new(&t);

    for i in 0..4 {
        if let Some(c) = p.next() {
            println!("{c}")
        }
    }
}
