use std::collections::HashMap;

use half::f16;
use lazy_static::lazy_static;
use smol_str::SmolStr;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, typecast_to_type,
    types::{
        typeobj::{
            bool_t::BOOL_T, f16_t::F16_T, f32_t::F32_T, f64_t::F64_T, i16_t::I16_T, i32_t::I32_T,
            i64_t::I64_T, i8_t::I8_T, ibig_t::IBIG_T, isize_t::ISIZE_T, str_t::STR_T,
            type_t::TYPE_T, u128_t::U128_T, u16_t::U16_T, u32_t::U32_T, u64_t::U64_T, u8_t::U8_T,
            ubig_t::UBIG_T, usize_t::USIZE_T,
        },
        value::{Proc, Value},
    },
    unary, Type,
};

fn i128_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, I128_T);
    unary!(h, signed default I128_T I128);
    arith_opr_num!(h, default I128_T I128);
    comp_opr_num!(h, default I128_T I128);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == *TYPE_T => typecast_to_type!(I128_T),
            p if p == *STR_T => typecast_int!(I128 => str, x),
            p if p == *BOOL_T => typecast_int!(I128 => bool, x),
            p if p == *I8_T => typecast_int!(I128 => I8, x),
            p if p == *I16_T => typecast_int!(I128 => I16, x),
            p if p == *I32_T => typecast_int!(I128 => I32, x),
            p if p == *I64_T => typecast_int!(I128 => I64, x),
            p if p == *I128_T => x[0].to_owned(),
            p if p == *ISIZE_T => typecast_int!(I128 => Isize, x),
            p if p == *IBIG_T => typecast_int!(I128 => Ibig, x),
            p if p == *U8_T => typecast_int!(I128 => U8, x),
            p if p == *U16_T => typecast_int!(I128 => U16, x),
            p if p == *U32_T => typecast_int!(I128 => U32, x),
            p if p == *U64_T => typecast_int!(I128 => U64, x),
            p if p == *U128_T => typecast_int!(I128 => U128, x),
            p if p == *USIZE_T => typecast_int!(I128 => Usize, x),
            p if p == *UBIG_T => typecast_int!(I128 => Ubig, x),
            p if p == *F16_T => typecast_int!(I128 => f16, x),
            p if p == *F32_T => typecast_int!(I128 => f32, x),
            p if p == *F64_T => typecast_int!(I128 => f64, x),
            _ => return None,
        })
    };
    binary!(h, I128_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k.into(), Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref I128_T: Type<Value> = Type::Definition {
        name: Some("{builtin}".into()),
        inst_name: Some("i128".into()),
        generics: vec![],
        implementations: i128_t(),
        inst_fields: HashMap::new(),
    };
}
