use half::f16;
use num::bigint::{BigInt, BigUint, ToBigUint};
use num::{FromPrimitive, ToPrimitive};
use crate::objects::value::utils::OprError;
use crate::objects::value::Value;
use crate::Type;

// TODO refactor this entire file

macro_rules! typecast_str_to_num {
    ($e:ident, $t:ty, $x:ident, $st:literal) => {
        if let Ok(x) = $x.parse::<$t>() {
            Ok(Value::$e(x))
        } else {
            Err(OprError::TypecastError(Type::from_str($st)))
        }
    }
}

fn typecast_str(x: String, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x)),
        "bool" => Ok(Value::Bool(!x.is_empty())),
        //"char" => Ok(Value::Char(x.chars().next().unwrap())),
        "i8" => typecast_str_to_num!(I8, i8, x, "i8"),
        "i16" => typecast_str_to_num!(I16, i16, x, "i16"),
        "i32" => typecast_str_to_num!(I32, i32, x, "i32"),
        "i64" => typecast_str_to_num!(I64, i64, x, "i64"),
        "i128" => typecast_str_to_num!(I128, i128, x, "i128"),
        "isize" => typecast_str_to_num!(Isize, isize, x, "isize"),
        "ibig" => typecast_str_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => typecast_str_to_num!(U8, u8, x, "u8"),
        "u16" => typecast_str_to_num!(U16, u16, x, "u16"),
        "u32" => typecast_str_to_num!(U32, u32, x, "u32"),
        "u64" => typecast_str_to_num!(U64, u64, x, "u64"),
        "u128" => typecast_str_to_num!(U128, u128, x, "u128"),
        "usize" => typecast_str_to_num!(Usize, usize, x, "usize"),
        "ubig" => typecast_str_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => typecast_str_to_num!(F16, f16, x, "f16"),
        "f32" => typecast_str_to_num!(F32, f32, x, "f32"),
        "f64" => typecast_str_to_num!(F64, f64, x, "f64"),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_bool(x: bool, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => Ok(Value::I8(x as i8)),
        "i16" => Ok(Value::I16(x as i16)),
        "i32" => Ok(Value::I32(x as i32)),
        "i64" => Ok(Value::I64(x as i64)),
        "i128" => Ok(Value::I128(x as i128)),
        "isize" => Ok(Value::Isize(x as isize)),
        "ibig" => Ok(Value::Ibig(x.to_string().parse::<BigInt>().unwrap())),
        "u8" => Ok(Value::U8(x as u8)),
        "u16" => Ok(Value::U16(x as u16)),
        "u32" => Ok(Value::U32(x as u32)),
        "u64" => Ok(Value::U64(x as u64)),
        "u128" => Ok(Value::U128(x as u128)),
        "usize" => Ok(Value::Usize(x as usize)),
        "ubig" => Ok(Value::Ubig(x.to_string().parse::<BigUint>().unwrap())),
        "f16" => Ok(Value::F16(f16::from(x as u8))),
        "f32" => Ok(Value::F32(x as u8 as f32)),
        "f64" => Ok(Value::F64(x as u8 as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

macro_rules! simple_num_to_num {
    ($e:ident, $t:ty, $x:expr, $st:literal) => {
        if let Ok(x) = <$t>::try_from($x) {
            Ok(Value::$e(x))
        } else {
            Err(OprError::TypecastError(Type::from_str($st)))
        }
    }
}

fn typecast_i8(x: i8, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_i16(x: i16, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_i32(x: i32, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_i64(x: i64, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_i128(x: i128, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_isize(x: isize, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}


fn typecast_ibig(x: BigInt, y: String) -> Result<Value, OprError> { // TODO option thingies
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0.into())),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x.to_f64().unwrap()))),
        "f32" => Ok(Value::F32(x.to_f32().unwrap())),
        "f64" => Ok(Value::F64(x.to_f64().unwrap())),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_u8(x: u8, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_u16(x: u16, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_u32(x: u32, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_u64(x: u64, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_u128(x: u128, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_usize(x: usize, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_ubig(x: BigUint, y: String) -> Result<Value, OprError> { // TODO same here
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0.to_biguint().unwrap())),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => simple_num_to_num!(I8, i8, x, "i8"),
        "i16" => simple_num_to_num!(I16, i16, x, "i16"),
        "i32" => simple_num_to_num!(I32, i32, x, "i32"),
        "i64" => simple_num_to_num!(I64, i64, x, "i64"),
        "i128" => simple_num_to_num!(I128, i128, x, "i128"),
        "isize" => simple_num_to_num!(Isize, isize, x, "isize"),
        "ibig" => simple_num_to_num!(Ibig, BigInt, x, "ibig"),
        "u8" => simple_num_to_num!(U8, u8, x, "u8"),
        "u16" => simple_num_to_num!(U16, u16, x, "u16"),
        "u32" => simple_num_to_num!(U32, u32, x, "u32"),
        "u64" => simple_num_to_num!(U64, u64, x, "u64"),
        "u128" => simple_num_to_num!(U128, u128, x, "u128"),
        "usize" => simple_num_to_num!(Usize, usize, x, "usize"),
        "ubig" => simple_num_to_num!(Ubig, BigUint, x, "ubig"),
        "f16" => Ok(Value::F16(f16::from_f64(x.to_f64().unwrap()))),
        "f32" => Ok(Value::F32(x.to_f32().unwrap())),
        "f64" => Ok(Value::F64(x.to_f64().unwrap())),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_f16(x: f16, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != f16::from_f64(0.0))),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "f16" => Ok(Value::F16(x)),
        "f32" => Ok(Value::F32(x.to_f32())),
        "f64" => Ok(Value::F64(x.to_f64())),
        _ => typecast_f64(x.to_f64(), y)
    }
}

fn typecast_f32(x: f32, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0.0)),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => Ok(Value::I8(x as i8)),
        "i16" => Ok(Value::I16(x as i16)),
        "i32" => Ok(Value::I32(x as i32)),
        "i64" => Ok(Value::I64(x as i64)),
        "i128" => Ok(Value::I128(x as i128)),
        "isize" => Ok(Value::Isize(x as isize)),
        "ibig" => Ok(Value::Ibig(BigInt::from_f32(x).unwrap())),
        "u8" => Ok(Value::U8(x as u8)),
        "u16" => Ok(Value::U16(x as u16)),
        "u32" => Ok(Value::U32(x as u32)),
        "u64" => Ok(Value::U64(x as u64)),
        "u128" => Ok(Value::U128(x as u128)),
        "usize" => Ok(Value::Usize(x as usize)),
        "ubig" => Ok(Value::Ubig(BigUint::from_f32(x).unwrap())),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

fn typecast_f64(x: f64, y: String) -> Result<Value, OprError> {
    match &*y {
        "str" => Ok(Value::Str(x.to_string())),
        "bool" => Ok(Value::Bool(x != 0.into())),
        //"char" => Ok(Value::Char(x.to_string().chars().next().unwrap())),
        "i8" => Ok(Value::I8(x as i8)),
        "i16" => Ok(Value::I16(x as i16)),
        "i32" => Ok(Value::I32(x as i32)),
        "i64" => Ok(Value::I64(x as i64)),
        "i128" => Ok(Value::I128(x as i128)),
        "isize" => Ok(Value::Isize(x as isize)),
        "ibig" => Ok(Value::Ibig(BigInt::from_f64(x).unwrap())),
        "u8" => Ok(Value::U8(x as u8)),
        "u16" => Ok(Value::U16(x as u16)),
        "u32" => Ok(Value::U32(x as u32)),
        "u64" => Ok(Value::U64(x as u64)),
        "u128" => Ok(Value::U128(x as u128)),
        "usize" => Ok(Value::Usize(x as usize)),
        "ubig" => Ok(Value::Ubig(BigUint::from_f64(x).unwrap())),
        "f16" => Ok(Value::F16(f16::from_f64(x as f64))),
        "f32" => Ok(Value::F32(x as f32)),
        "f64" => Ok(Value::F64(x as f64)),
        _ => Err(OprError::NoImplForOpr)
    }
}

pub fn typecast(x: &Value, y: Value) -> Result<Value, OprError> {
    match y {
        Value::Type(y) => match y {
            Type::Instance { name, .. } => if name == "type" {
                Ok(x.get_type())
            } else { match x.to_owned() {
                Value::Str(s) => typecast_str(s, name),
                Value::Bool(b) => typecast_bool(b, name),
                Value::I8(x) => typecast_i8(x, name),
                Value::I16(x) => typecast_i16(x, name),
                Value::I32(x) => typecast_i32(x, name),
                Value::I64(x) => typecast_i64(x, name),
                Value::I128(x) => typecast_i128(x, name),
                Value::Isize(x) => typecast_isize(x, name),
                Value::Ibig(x) => typecast_ibig(x, name),
                Value::U8(x) => typecast_u8(x, name),
                Value::U16(x) => typecast_u16(x, name),
                Value::U32(x) => typecast_u32(x, name),
                Value::U64(x) => typecast_u64(x, name),
                Value::U128(x) => typecast_u128(x, name),
                Value::Usize(x) => typecast_usize(x, name),
                Value::Ubig(x) => typecast_ubig(x, name),
                Value::F16(x) => typecast_f16(x, name),
                Value::F32(x) => typecast_f32(x, name),
                Value::F64(x) => typecast_f64(x, name),
                Value::Type(_) => Ok(Value::Type(Type::from_str("type"))),
                _ => Err(OprError::NoImplForOpr)
            }},
            Type::Return(y) => typecast(x, Value::Type(*y)),
            _ => Err(OprError::NoImplForOpr)
        }
        _ => Err(OprError::NoImplForOpr)
    }
}