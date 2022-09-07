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
            bool_t::BOOL_T, f16_t::F16_T, f32_t::F32_T, i128_t::I128_T, i16_t::I16_T, i32_t::I32_T,
            i64_t::I64_T, i8_t::I8_T, ibig_t::IBIG_T, isize_t::ISIZE_T, str_t::STR_T,
            type_t::TYPE_T, u128_t::U128_T, u16_t::U16_T, u32_t::U32_T, u64_t::U64_T, u8_t::U8_T,
            ubig_t::UBIG_T, usize_t::USIZE_T,
        },
        value::{Proc, Value},
    },
    unary, Type,
};

fn f64_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, F64_T);
    unary!(h, float F64_T F64);
    arith_opr_num!(h, float default F64_T F64);
    comp_opr_num!(h, default F64_T F64);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == *TYPE_T => typecast_to_type!(F64_T),
            p if p == *STR_T => typecast_float!(F64 => str, x),
            p if p == *BOOL_T => typecast_float!(F64 => bool, x),
            p if p == *I8_T => typecast_float!(F64 => I8 to_i8, x),
            p if p == *I16_T => typecast_float!(F64 => I16 to_i16, x),
            p if p == *I32_T => typecast_float!(F64 => I32 to_i32, x),
            p if p == *I64_T => typecast_float!(F64 => I64 to_i64, x),
            p if p == *I128_T => typecast_float!(F64 => I128 to_i128, x),
            p if p == *ISIZE_T => typecast_float!(F64 => Isize to_isize, x),
            p if p == *IBIG_T => typecast_float!(F64 => Ibig to_bigint, x),
            p if p == *U8_T => typecast_float!(F64 => U8 to_u8, x),
            p if p == *U16_T => typecast_float!(F64 => U16 to_u16, x),
            p if p == *U32_T => typecast_float!(F64 => U32 to_u32, x),
            p if p == *U64_T => typecast_float!(F64 => U64 to_u64, x),
            p if p == *U128_T => typecast_float!(F64 => U128 to_u128, x),
            p if p == *USIZE_T => typecast_float!(F64 => Usize to_usize, x),
            p if p == *UBIG_T => typecast_float!(F64 => Ubig to_biguint, x),
            p if p == *F16_T => typecast_float!(F64 => f16, x),
            p if p == *F32_T => typecast_float!(F64 => F32 to_f32, x),
            p if p == *F64_T => x[0].to_owned(),
            _ => return None,
        })
    };
    binary!(h, F64_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k.into(), Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref F64_T: Type<Value> = Type::Definition {
        name: Some("{builtin}".into()),
        inst_name: Some("f16".into()),
        generics: vec![],
        implementations: f64_t(),
        inst_fields: HashMap::new(),
    };
}
