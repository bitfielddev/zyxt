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

pub const fn f32_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "f32");
    unary!(h, float "f32" F32);
    arith_opr_num!(h, float default "f32" F32);
    comp_opr_num!(h, default "f32" F32);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_float!("f32" => type),
                "str" => typecast_float!(F32 => str, x),
                "bool" => typecast_float!(F32 => bool, x),
                "i8" => typecast_float!(F32 => I8 to_i8, x),
                "i16" => typecast_float!(F32 => I16 to_i16, x),
                "i32" => typecast_float!(F32 => I32 to_i32, x),
                "i64" => typecast_float!(F32 => I64 to_i64, x),
                "i128" => typecast_float!(F32 => I128 to_i128, x),
                "isize" => typecast_float!(F32 => Isize to_isize, x),
                "ibig" => typecast_float!(F32 => Ibig to_bigint, x),
                "u8" => typecast_float!(F32 => U8 to_u8, x),
                "u16" => typecast_float!(F32 => U16 to_u16, x),
                "u32" => typecast_float!(F32 => U32 to_u32, x),
                "u64" => typecast_float!(F32 => U64 to_u64, x),
                "u128" => typecast_float!(F32 => U128 to_u128, x),
                "usize" => typecast_float!(F32 => Usize to_usize, x),
                "ubig" => typecast_float!(F32 => Ubig to_biguint, x),
                "f16" => typecast_float!(F32 => f16, x),
                "f32" => x[0].to_owned(),
                "f64" => typecast_float!(F32 => F64 to_f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "f32", "_typecast", ["type"], "_any", typecast);

    h
}
    