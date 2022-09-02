use crate::{
    types::value::{typecast::typecast, utils::OprError, Value},
    Type,
};

macro_rules! typecast_eq {
    ($t:ident, $s:literal, $x:ident, $y:ident) => {
        $y.is_num()
            && $x
                == typecast(&$y, Value::Type(Type::from_name($s)))?
                    .$t()
                    .unwrap()
    };
}

pub fn eq(x: &Value, y: Value) -> Result<Value, OprError> {
    Ok(Value::Bool(match x {
        Value::I8(x) => typecast_eq!(as_i8, "i8", x, y),
        Value::I16(x) => typecast_eq!(as_i16, "i16", x, y),
        Value::I32(x) => typecast_eq!(as_i32, "i32", x, y),
        Value::I64(x) => typecast_eq!(as_i64, "i64", x, y),
        Value::I128(x) => typecast_eq!(as_i128, "i128", x, y),
        Value::Isize(x) => typecast_eq!(as_isize, "isize", x, y),
        Value::Ibig(x) => typecast_eq!(as_ibig, "ibig", x, y),
        Value::U8(x) => typecast_eq!(as_u8, "u8", x, y),
        Value::U16(x) => typecast_eq!(as_u16, "u16", x, y),
        Value::U32(x) => typecast_eq!(as_u32, "u32", x, y),
        Value::U64(x) => typecast_eq!(as_u64, "u64", x, y),
        Value::U128(x) => typecast_eq!(as_u128, "u128", x, y),
        Value::Usize(x) => typecast_eq!(as_usize, "usize", x, y),
        Value::Ubig(x) => typecast_eq!(as_ubig, "ubig", x, y),
        Value::F16(x) => typecast_eq!(as_f16, "f16", x, y),
        Value::F32(x) => typecast_eq!(as_f32, "f32", x, y),
        Value::F64(x) => typecast_eq!(as_f64, "f64", x, y),
        Value::Bool(x) => typecast_eq!(as_bool, "bool", x, y),
        _ => *iseq(x, y)?.as_bool().unwrap(),
    }))
}

pub fn noteq(x: &Value, y: Value) -> Result<Value, OprError> {
    Ok(Value::Bool(!eq(x, y)?.as_bool().unwrap()))
}

pub fn iseq(x: &Value, y: Value) -> Result<Value, OprError> {
    Ok(Value::Bool(x.eq(&y)))
}

pub fn isnteq(x: &Value, y: Value) -> Result<Value, OprError> {
    Ok(Value::Bool(!iseq(x, y)?.as_bool().unwrap()))
}
