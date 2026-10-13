#[derive(Debug,Clone,PartialEq)]
pub enum Expression {
    Real(f64),
    Boolean(bool),
    Ident(String),
    Lambda { parameters: Vec<String>, body: Box<Expression> },
    Apply { callee: Box<Expression>, arguments: Vec<Expression> },
    If { condition: Box<Expression>, decision: Box<Expression>, alternative: Box<Expression> },
    Let { bindings: Vec<(String, Expression)>, body: Box<Expression> },
}
