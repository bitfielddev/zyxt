use crate::objects::value::typecast::typecast;
use crate::objects::value::utils::OprError;
use crate::objects::value::Value;
use crate::Type;

macro_rules! typecast_sub {
    ($e:ident, $t:ident, $s:literal, $x:ident, $y:ident) => {
        Ok(Value::$e($x - typecast($y, Value::Type(Type::from_str($s)))?.$t().unwrap()))
    };
}

fn sub(x: Value, y: Value) -> Result<Value, OprError> {
    match x {
        Value::I8(x) => typecast_sub!(I8, as_i8, "i8", x, y),
        Value::I16(x) => typecast_sub!(I16, as_i16, "i16", x, y),
        Value::I32(x) => typecast_sub!(I32, as_i32, "i32", x, y),
        Value::I64(x) => typecast_sub!(I64, as_i64, "i64", x, y),
        Value::I128(x) => typecast_sub!(I128, as_i128, "i128", x, y),
        Value::Isize(x) => typecast_sub!(Isize, as_isize, "isize", x, y),
        Value::Ibig(x) => typecast_sub!(Ibig, as_ibig, "ibig", x, y),
        Value::U8(x) => typecast_sub!(U8, as_u8, "u8", x, y),
        Value::U16(x) => typecast_sub!(U16, as_u16, "u16", x, y),
        Value::U32(x) => typecast_sub!(U32, as_u32, "u32", x, y),
        Value::U64(x) => typecast_sub!(U64, as_u64, "u64", x, y),
        Value::U128(x) => typecast_sub!(U128, as_u128, "u128", x, y),
        Value::Usize(x) => typecast_sub!(Usize, as_usize, "usize", x, y),
        Value::Ubig(x) => typecast_sub!(Ubig, as_ubig, "ubig", x, y),
        Value::F16(x) => typecast_sub!(F16, as_f16, "f16", x, y),
        Value::F32(x) => typecast_sub!(F32, as_f32, "f32", x, y),
        Value::F64(x) => typecast_sub!(F64, as_f64, "f64", x, y),
        _ => Err(OprError::NoImplForOpr)
    }
}