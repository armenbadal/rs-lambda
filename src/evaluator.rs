use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::Expression;

enum Value {
    Real(f64),
    Boolean(bool),
    List(Vec<Value>),
    Function(Rc<Closure>),
}

struct Closure {
    parameters: Vec<String>,
    body: Expression,
    env: Rc<Environment>,
}

struct Environment {
    values: HashMap<String,Value>,
    parent: Option<Rc<Environment>>,
}

impl Environment {
    fn get(&self, name: &String) -> Option<&Value> {
        todo!()
    }
}

fn evaluate(expr: &Expression, env: Rc<Environment>) -> Result<Value, String> {
    match expr {
        Expression::Real(value) => {
            Ok(Value::Real(*value))
        }
        Expression::Boolean(value) => {
            Ok(Value::Boolean(*value))
        }
        Expression::Ident(name) => {
            todo!()
        }
        Expression::Lambda { parameters, body } => {
            let c = Closure {
                parameters: parameters.clone(),
                body: (**body).clone(),
                env: env,
            };
            Ok(Value::Function(Rc::new(c)))
        }
        Expression::Apply { callee, arguments } => {
            todo!()
        }
        Expression::If { condition, decision, alternative } => {
            todo!()
        }
        Expression::Let { bindings, body } => {
            todo!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}
