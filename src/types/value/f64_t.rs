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

pub const fn f64_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "f64");
    unary!(h, float "f64" F64);
    arith_opr_num!(h, float default "f64" F64);
    comp_opr_num!(h, default "f64" F64);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_float!("f64" => type),
                "str" => typecast_float!(F64 => str, x),
                "bool" => typecast_float!(F64 => bool, x),
                "i8" => typecast_float!(F64 => I8 to_i8, x),
                "i16" => typecast_float!(F64 => I16 to_i16, x),
                "i32" => typecast_float!(F64 => I32 to_i32, x),
                "i64" => typecast_float!(F64 => I64 to_i64, x),
                "i128" => typecast_float!(F64 => I128 to_i128, x),
                "isize" => typecast_float!(F64 => Isize to_isize, x),
                "ibig" => typecast_float!(F64 => Ibig to_bigint, x),
                "u8" => typecast_float!(F64 => U8 to_u8, x),
                "u16" => typecast_float!(F64 => U16 to_u16, x),
                "u32" => typecast_float!(F64 => U32 to_u32, x),
                "u64" => typecast_float!(F64 => U64 to_u64, x),
                "u128" => typecast_float!(F64 => U128 to_u128, x),
                "usize" => typecast_float!(F64 => Usize to_usize, x),
                "ubig" => typecast_float!(F64 => Ubig to_biguint, x),
                "f16" => typecast_float!(F64 => f16, x),
                "f32" => typecast_float!(F64 => F32 to_f32, x),
                "f64" => x[0].to_owned(),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "f64", "_typecast", ["type"], "_any", typecast);

    h
}
    