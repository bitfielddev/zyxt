use crate::types::value::{utils::OprError, Value};

pub fn concat(x: &Value, y: Value) -> Result<Value, OprError> {
    Ok(Value::Str(x.to_string() + &y.to_string()))
}
