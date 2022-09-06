use std::collections::HashMap;

use half::f16;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_float, types::value::Proc,
    unary, Type, Value,
};
use num::ToPrimitive;
use num::bigint::ToBigInt;
use num::bigint::ToBigUint;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};

macro_rules! typecast_f16_to_int {
    ($vo:ident $f:ident, $x:ident) => {
        Value::$vo(get_param!($x, 0, F16).to_f64().$f()?)
    };
}

pub const fn f16_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "f16");
    unary!(h, "f16", "_un_add", |x: &Vec<Value>| Some(x[0].to_owned()));
    unary!(h, "f16", "_un_sub", |x: &Vec<Value>| Some(Value::F16(
        get_param!(x, 0, F16).neg()
    )));
    unary!(h, "f16", "_not", |x: &Vec<Value>| Some(Value::Bool(
        get_param!(x, 0, F16) == f16::ZERO || get_param!(x, 0, F16) == f16::NEG_ZERO
    )));
    arith_opr_num!(h, float default "f16" F16);
    comp_opr_num!(h, default "f16" F16);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_float!("f16" => type),
                "str" => typecast_float!(F16 => str, x),
                "bool" => Value::Bool(
                    get_param!(x, 0, F16) != f16::ZERO && get_param!(x, 0, F16) != f16::NEG_ZERO
                ),
                "i8" => typecast_f16_to_int!(I8 to_i8, x),
                "i16" => typecast_f16_to_int!(I16 to_i16, x),
                "i32" => typecast_f16_to_int!(I32 to_i32, x),
                "i64" => typecast_f16_to_int!(I64 to_i64, x),
                "i128" => typecast_f16_to_int!(I128 to_i128, x),
                "isize" => typecast_f16_to_int!(Isize to_isize, x),
                "ibig" => typecast_f16_to_int!(Ibig to_bigint, x),
                "u8" => typecast_f16_to_int!(U8 to_u8, x),
                "u16" => typecast_f16_to_int!(U16 to_u16, x),
                "u32" => typecast_f16_to_int!(U32 to_u32, x),
                "u64" => typecast_f16_to_int!(U64 to_u64, x),
                "u128" => typecast_f16_to_int!(U128 to_u128, x),
                "usize" => typecast_f16_to_int!(Usize to_usize, x),
                "ubig" => typecast_f16_to_int!(Ubig to_biguint, x),
                "f16" => x[0].to_owned(),
                "f32" => Value::F32(get_param!(x, 0, F16).to_f32()),
                "f64" => Value::F64(get_param!(x, 0, F16).to_f64()),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "f16", "_typecast", ["type"], "_any", typecast);

    h
}
    