use crate::objects::value::typecast::typecast;
use crate::objects::value::utils::OprError;
use crate::objects::value::Value;
use crate::Type;

pub fn un_plus(x: &Value) -> Result<Value, OprError> {
    match x {
        Value::I8(_) |
        Value::I16(_) |
        Value::I32(_) |
        Value::I64(_) |
        Value::I128(_) |
        Value::Isize(_) |
        Value::Ibig(_) |
        Value::U8(_) |
        Value::U16(_) |
        Value::U32(_) |
        Value::U64(_) |
        Value::U128(_) |
        Value::Usize(_) |
        Value::Ubig(_) |
        Value::F16(_) |
        Value::F32(_) |
        Value::F64(_) => Ok(x.to_owned()),
        _ => Err(OprError::NoImplForOpr),
    }
}

pub fn un_minus(x: &Value) -> Result<Value, OprError> {
    match x.to_owned() {
        Value::I8(x) => Ok(Value::I8(-x)),
        Value::I16(x) => Ok(Value::I16(-x)),
        Value::I32(x) => Ok(Value::I32(-x)),
        Value::I64(x) => Ok(Value::I64(-x)),
        Value::I128(x) => Ok(Value::I128(-x)),
        Value::Isize(x) => Ok(Value::Isize(-x)),
        Value::Ibig(x) => Ok(Value::Ibig(-x)),
        Value::F16(x) => Ok(Value::F16(-x)),
        Value::F32(x) => Ok(Value::F32(-x)),
        Value::F64(x) => Ok(Value::F64(-x)),
        _ => Err(OprError::NoImplForOpr),
    }
}

pub fn un_not(x: &Value) -> Result<Value, OprError> {
    match x.to_owned() {
        Value::Bool(x) => Ok(Value::Bool(!x)),
        _ => un_not(&typecast(x, Value::Type(Type::from_str("bool")))?),
    }
}