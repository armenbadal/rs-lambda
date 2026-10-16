use crate::evaluator::Value;

pub fn numeric_add(args: &[Value]) -> Result<Value,String> {
    let [arg0, arg1] = args else {
        return Err("պարամետրերի սխալ քանակ".to_string())
    };

    let Value::Real(a) = arg0 else {
        return Err("առաջին արգումենտը թիվ չէ".to_string())
    };

    let Value::Real(b) = arg1 else {
        return Err("երկրորդ արգումենտը թիվ չէ".to_string())
    };

    Ok(Value::Real(a + b))
}

pub fn list_create(args: &[Value]) -> Result<Value,String> {
    Ok(Value::List(Vec::from(args)))
}

pub fn list_is_empty(args: &[Value]) -> Result<Value,String> {
    Ok(Value::Boolean(false))
}
