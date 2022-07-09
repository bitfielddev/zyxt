use crate::{Element, InterpreterData, Type, ZyxtError};
use crate::interpreter::interpret_expr;
use crate::objects::value::typecast::typecast;
use crate::objects::value::utils::OprError;
use crate::objects::value::Value;

pub fn and(x: &Element, y: &Element, i_data: &mut InterpreterData<Value>) -> Result<Value, ZyxtError> {
    let lhs = interpret_expr(x, i_data)?;
    let lhsb = typecast(&lhs, Value::Type(Type::from_str("bool"))).unwrap();
    if !lhsb.as_bool().unwrap() {
        return Ok(Value::Bool(false))
    }
    let rhs = interpret_expr(y, i_data)?;
    let rhsb = typecast(&rhs, Value::Type(Type::from_str("bool"))).unwrap();
    Ok(Value::Bool(*rhsb.as_bool().unwrap()))
}

pub fn or(x: &Element, y: &Element, i_data: &mut InterpreterData<Value>) -> Result<Value, ZyxtError> {
    let lhs = interpret_expr(x, i_data)?;
    let lhsb = typecast(&lhs, Value::Type(Type::from_str("bool"))).unwrap();
    if *lhsb.as_bool().unwrap() {
        return Ok(Value::Bool(true))
    }
    let rhs = interpret_expr(y, i_data)?;
    let rhsb = typecast(&rhs, Value::Type(Type::from_str("bool"))).unwrap();
    Ok(Value::Bool(*rhsb.as_bool().unwrap()))
}

pub fn xor(x: &Value, y: &Value) -> Result<Value, OprError> {
    let lhs = typecast(x, Value::Type(Type::from_str("bool")))?;
    let rhs = typecast(y, Value::Type(Type::from_str("bool")))?;
    Ok(Value::Bool(lhs.as_bool().unwrap() != rhs.as_bool().unwrap()))
}