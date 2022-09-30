use std::collections::HashMap;

use half::f16;
use lazy_static::lazy_static;
use smol_str::SmolStr;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, typecast_to_type,
    types::{
        typeobj::{
            bool_t::BOOL_T, f16_t::F16_T, f32_t::F32_T, f64_t::F64_T, i128_t::I128_T, i16_t::I16_T,
            i64_t::I64_T, i8_t::I8_T, ibig_t::IBIG_T, isize_t::ISIZE_T, str_t::STR_T,
            type_t::TYPE_T, u128_t::U128_T, u16_t::U16_T, u32_t::U32_T, u64_t::U64_T, u8_t::U8_T,
            ubig_t::UBIG_T, usize_t::USIZE_T, TypeDefinition,
        },
        value::{Proc, Value},
    },
    unary, Type,
};

fn i32_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::I32(0));
    concat_vals!(h, I32_T);
    unary!(h, signed default I32_T I32);
    arith_opr_num!(h, default I32_T I32);
    comp_opr_num!(h, default I32_T I32);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.to_type() => typecast_to_type!(I32_T),
            p if p == STR_T.to_type() => typecast_int!(I32 => str, x),
            p if p == BOOL_T.to_type() => typecast_int!(I32 => bool, x),
            p if p == I8_T.to_type() => typecast_int!(I32 => I8, x),
            p if p == I16_T.to_type() => typecast_int!(I32 => I16, x),
            p if p == I32_T.to_type() => x[0].to_owned(),
            p if p == I64_T.to_type() => typecast_int!(I32 => I64, x),
            p if p == I128_T.to_type() => typecast_int!(I32 => I128, x),
            p if p == ISIZE_T.to_type() => typecast_int!(I32 => Isize, x),
            p if p == IBIG_T.to_type() => typecast_int!(I32 => Ibig, x),
            p if p == U8_T.to_type() => typecast_int!(I32 => U8, x),
            p if p == U16_T.to_type() => typecast_int!(I32 => U16, x),
            p if p == U32_T.to_type() => typecast_int!(I32 => U32, x),
            p if p == U64_T.to_type() => typecast_int!(I32 => U64, x),
            p if p == U128_T.to_type() => typecast_int!(I32 => U128, x),
            p if p == USIZE_T.to_type() => typecast_int!(I32 => Usize, x),
            p if p == UBIG_T.to_type() => typecast_int!(I32 => Ubig, x),
            p if p == F16_T.to_type() => typecast_int!(I32 => f16, x),
            p if p == F32_T.to_type() => typecast_int!(I32 => f32, x),
            p if p == F64_T.to_type() => typecast_int!(I32 => f64, x),
            _ => return None,
        })
    };
    binary!(
        h,
        I32_T.to_type(),
        "_typecast",
        [TYPE_T.to_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

lazy_static! {
    pub static ref I32_T: TypeDefinition<Value> = TypeDefinition {
        name: Some("{builtin i32}".into()),
        inst_name: Some("i32".into()),
        generics: vec![],
        implementations: i32_t(),
        inst_fields: HashMap::new(),
    };
}
