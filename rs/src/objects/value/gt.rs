use crate::objects::value::eq::eq;
use crate::objects::value::typecast::typecast;
use crate::objects::value::utils::OprError;
use crate::objects::value::Value;
use crate::Type;

macro_rules! typecast_gt {
    ($t:ident, $s:literal, $x:ident, $y:ident) => {
        Ok(Value::Bool(
            $x > &match $y {
                Value::I8(_)
                | Value::I16(_)
                | Value::I32(_)
                | Value::I64(_)
                | Value::I128(_)
                | Value::Isize(_)
                | Value::Ibig(_)
                | Value::U8(_)
                | Value::U16(_)
                | Value::U32(_)
                | Value::U64(_)
                | Value::U128(_)
                | Value::Usize(_)
                | Value::Ubig(_)
                | Value::F16(_)
                | Value::F32(_)
                | Value::F64(_) => typecast(&$y, Value::Type(Type::from_str($s)))?
                    .$t()
                    .unwrap()
                    .to_owned(),
                _ => return Err(OprError::NoImplForOpr),
            },
        ))
    };
}

pub fn gt(x: &Value, y: Value) -> Result<Value, OprError> {
    match x {
        Value::I8(x) => typecast_gt!(as_i8, "i8", x, y),
        Value::I16(x) => typecast_gt!(as_i16, "i16", x, y),
        Value::I32(x) => typecast_gt!(as_i32, "i32", x, y),
        Value::I64(x) => typecast_gt!(as_i64, "i64", x, y),
        Value::I128(x) => typecast_gt!(as_i128, "i128", x, y),
        Value::Isize(x) => typecast_gt!(as_isize, "isize", x, y),
        Value::Ibig(x) => typecast_gt!(as_ibig, "ibig", x, y),
        Value::U8(x) => typecast_gt!(as_u8, "u8", x, y),
        Value::U16(x) => typecast_gt!(as_u16, "u16", x, y),
        Value::U32(x) => typecast_gt!(as_u32, "u32", x, y),
        Value::U64(x) => typecast_gt!(as_u64, "u64", x, y),
        Value::U128(x) => typecast_gt!(as_u128, "u128", x, y),
        Value::Usize(x) => typecast_gt!(as_usize, "usize", x, y),
        Value::Ubig(x) => typecast_gt!(as_ubig, "ubig", x, y),
        Value::F16(x) => typecast_gt!(as_f16, "f16", x, y),
        Value::F32(x) => typecast_gt!(as_f32, "f32", x, y),
        Value::F64(x) => typecast_gt!(as_f64, "f64", x, y),
        Value::Bool(x) => typecast_gt!(as_bool, "bool", x, y),
        Value::Str(x) => {
            if let Value::Str(y) = y {
                Ok(Value::Bool(x < &y))
            } else {
                Err(OprError::NoImplForOpr)
            }
        }
        _ => Err(OprError::NoImplForOpr),
    }
}

pub fn gteq(x: &Value, y: Value) -> Result<Value, OprError> {
    Ok(Value::Bool(
        *gt(x, y.to_owned())?.as_bool().unwrap() || *eq(x, y)?.as_bool().unwrap(),
    ))
}
