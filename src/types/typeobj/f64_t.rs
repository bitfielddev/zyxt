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

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_float, typecast_to_type,
    types::{
        typeobj::{bool_t::BOOL_T, str_t::STR_T, type_t::TYPE_T},
        value::{Proc, Value},
    },
    unary, Type,
};

const fn f64_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, F64_T);
    unary!(h, float F64_T F64);
    arith_opr_num!(h, float default F64_T F64);
    comp_opr_num!(h, default F64_T F64);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(F64_T),
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
    binary!(h, F64_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref F64_T: Type = Type::Definition {
        name: Some("f16".into()),
        generics: vec![],
        implementations: f64_t(),
        inst_fields: HashMap::new(),
    };
}
