use crate::objects::value::utils::OprError;
use crate::objects::value::Value;

pub fn concat(x: &Value, y: Value) -> Result<Value, OprError> {
    Ok(Value::Str(x.to_string() + &y.to_string()))
}
