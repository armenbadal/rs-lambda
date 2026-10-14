use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::Expression;

#[derive(Clone)]
enum Value {
    Real(f64),
    Boolean(bool),
    List(Vec<Value>),
    Function(Rc<Closure>),
}

impl Value {
    fn is_boolean(&self, expected: bool) -> bool {
        match self {
            Value::Boolean(true) => true,
            _ => false
        }
    }
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
    fn get(&self, name: &str) -> Option<Value> {
        match self.get(name) {
            Some(v) => Some(v.clone()),
            None => self.parent.as_ref()?.get(name)
        }
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
            Ok(env.get(name).unwrap())
        }
        Expression::Lambda { parameters, body } => {
            let c = Closure {
                parameters: parameters.clone(),
                body: (**body).clone(),
                env,
            };
            Ok(Value::Function(Rc::new(c)))
        }
        Expression::Apply { callee, arguments } => {
            todo!()
        }
        Expression::If { condition, decision, alternative } => {
            let cv = evaluate(condition, env.clone())?;
            if cv.is_boolean(true) {
                Ok(evaluate(decision, env.clone())?)
            } else {
                Ok(evaluate(alternative, env.clone())?)
            }
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
