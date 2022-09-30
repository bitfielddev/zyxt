use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use half::f16;
use lazy_static::lazy_static;
use num::{
    bigint::{ToBigInt, ToBigUint},
    ToPrimitive,
};
use smol_str::SmolStr;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_float, typecast_to_type,
    types::{
        typeobj::{
            bool_t::BOOL_T, f16_t::F16_T, f64_t::F64_T, i128_t::I128_T, i16_t::I16_T, i32_t::I32_T,
            i64_t::I64_T, i8_t::I8_T, ibig_t::IBIG_T, isize_t::ISIZE_T, str_t::STR_T,
            type_t::TYPE_T, u128_t::U128_T, u16_t::U16_T, u32_t::U32_T, u64_t::U64_T, u8_t::U8_T,
            ubig_t::UBIG_T, usize_t::USIZE_T, TypeDefinition,
        },
        value::{Proc, Value},
    },
    unary, Type,
};

fn f32_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::F32(0.0));
    concat_vals!(h, F32_T);
    unary!(h, float F32_T F32);
    arith_opr_num!(h, float default F32_T F32);
    comp_opr_num!(h, default F32_T F32);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.to_type() => typecast_to_type!(F32_T),
            p if p == STR_T.to_type() => typecast_float!(F32 => str, x),
            p if p == BOOL_T.to_type() => typecast_float!(F32 => bool, x),
            p if p == I8_T.to_type() => typecast_float!(F32 => I8 to_i8, x),
            p if p == I16_T.to_type() => typecast_float!(F32 => I16 to_i16, x),
            p if p == I32_T.to_type() => typecast_float!(F32 => I32 to_i32, x),
            p if p == I64_T.to_type() => typecast_float!(F32 => I64 to_i64, x),
            p if p == I128_T.to_type() => typecast_float!(F32 => I128 to_i128, x),
            p if p == ISIZE_T.to_type() => typecast_float!(F32 => Isize to_isize, x),
            p if p == IBIG_T.to_type() => typecast_float!(F32 => Ibig to_bigint, x),
            p if p == U8_T.to_type() => typecast_float!(F32 => U8 to_u8, x),
            p if p == U16_T.to_type() => typecast_float!(F32 => U16 to_u16, x),
            p if p == U32_T.to_type() => typecast_float!(F32 => U32 to_u32, x),
            p if p == U64_T.to_type() => typecast_float!(F32 => U64 to_u64, x),
            p if p == U128_T.to_type() => typecast_float!(F32 => U128 to_u128, x),
            p if p == USIZE_T.to_type() => typecast_float!(F32 => Usize to_usize, x),
            p if p == UBIG_T.to_type() => typecast_float!(F32 => Ubig to_biguint, x),
            p if p == F16_T.to_type() => typecast_float!(F32 => f16, x),
            p if p == F32_T.to_type() => x[0].to_owned(),
            p if p == F64_T.to_type() => typecast_float!(F32 => F64 to_f64, x),
            _ => return None,
        })
    };
    binary!(
        h,
        F32_T.to_type(),
        "_typecast",
        [TYPE_T.to_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

lazy_static! {
    pub static ref F32_T: TypeDefinition<Value> = TypeDefinition {
        name: Some("{builtin f32}".into()),
        inst_name: Some("f32".into()),
        generics: vec![],
        implementations: f32_t(),
        inst_fields: HashMap::new(),
    };
}
