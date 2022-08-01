use crate::objects::value::typecast::typecast;
use crate::objects::value::utils::OprError;
use crate::objects::value::Value;
use crate::Type;
use half::f16;
use num::bigint::{ToBigInt, ToBigUint};

macro_rules! typecast_div {
    ($e:ident, $t:ident, $s:literal, $x:ident, $y:ident, $zero:expr) => {{
        if $y.is_num() {
            let n = typecast(&$y, Value::Type(Type::from_str($s)))?.$t().unwrap().to_owned();
            if n == $zero {
                // TODO undef / indet handling, but for now,
                return Ok(Value::$e($x.to_owned()));
            }
            Ok(Value::$e($x / n))
        } else {Err(OprError::NoImplForOpr)}
    }};
}

pub fn div(x: &Value, y: Value) -> Result<Value, OprError> {
    match x {
        Value::I8(x) => typecast_div!(I8, as_i8, "i8", x, y, 0),
        Value::I16(x) => typecast_div!(I16, as_i16, "i16", x, y, 0),
        Value::I32(x) => typecast_div!(I32, as_i32, "i32", x, y, 0),
        Value::I64(x) => typecast_div!(I64, as_i64, "i64", x, y, 0),
        Value::I128(x) => typecast_div!(I128, as_i128, "i128", x, y, 0),
        Value::Isize(x) => typecast_div!(Isize, as_isize, "isize", x, y, 0),
        Value::Ibig(x) => typecast_div!(Ibig, as_ibig, "ibig", x, y, 0i32.to_bigint().unwrap()),
        Value::U8(x) => typecast_div!(U8, as_u8, "u8", x, y, 0),
        Value::U16(x) => typecast_div!(U16, as_u16, "u16", x, y, 0),
        Value::U32(x) => typecast_div!(U32, as_u32, "u32", x, y, 0),
        Value::U64(x) => typecast_div!(U64, as_u64, "u64", x, y, 0),
        Value::U128(x) => typecast_div!(U128, as_u128, "u128", x, y, 0),
        Value::Usize(x) => typecast_div!(Usize, as_usize, "usize", x, y, 0),
        Value::Ubig(x) => typecast_div!(Ubig, as_ubig, "ubig", x, y, 0i32.to_biguint().unwrap()),
        Value::F16(x) => typecast_div!(F16, as_f16, "f16", x, y, f16::from_f64(0.0)),
        Value::F32(x) => typecast_div!(F32, as_f32, "f32", x, y, 0.0),
        Value::F64(x) => typecast_div!(F64, as_f64, "f64", x, y, 0.0),
        _ => Err(OprError::NoImplForOpr),
    }
}
