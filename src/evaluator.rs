use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::ast::Expression;

pub type BuiltInFunc = fn(&[Value]) ->Result<Value,String>;

#[derive(Clone)]
pub enum Value {
    Real(f64),
    Boolean(bool),
    List(Vec<Value>),
    Function(Rc<Closure>),
    BuiltIn(BuiltInFunc),
}

impl Value {
    fn is_boolean(&self, expected: bool) -> bool {
        match self {
            Value::Boolean(true) => true,
            _ => false
        }
    }
}

pub struct Closure {
    parameters: Vec<String>,
    body: Expression,
    env: Environment,
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
            todo!()
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

fn free_variables(expr: &Expression) -> HashSet<String> {
    match expr {
        Expression::Ident(name) => HashSet::from([name.clone()]),
        Expression::If{ condition, decision, alternative } => {
            let mut fvs = free_variables(condition);
            //fvs.append(&mut free_variables(decision));
            //fvs.append(&mut free_variables(alternative));
            fvs
        }
        Expression::Lambda{ parameters, body} => {
            let mut fvs = free_variables(body);
            // return fvs.filter(e => ! expr.parameters.includes(e))
            HashSet::new()
        }
        Expression::Apply{ callee, arguments } => {
            // const fvs = freeVariables(expr.callee)
            // return fvs.concat(expr.arguments.map(freeVariables).flat())
            HashSet::new()
        }
        Expression::Let { bindings, body } => {
            // const pvs = []
            // let fvs = []
            // for ( const ve of expr.bindings) {
            //     pvs.push(ve.name)
            //     fvs = fvs.concat(freeVariables(ve.value))
            // }
            //
            // const bvs = freeVariables(expr.body).filter(e => ! pvs.includes(e))
            // return fvs.concat(bvs)
            HashSet::new()
        }
        _ => HashSet::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
