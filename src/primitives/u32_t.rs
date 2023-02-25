use std::collections::HashMap;

use half::f16;
use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param,
    primitives::*,
    typecast_int, typecast_to_type,
    types::{
        typeobj::TypeDefinition,
        value::{Proc, Value},
    },
    unary, Type,
};

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn u32_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::U32(0));
    concat_vals!(h, U32_T);
    unary!(h, signed default U32_T U32);
    arith_opr_num!(h, default U32_T U32);
    comp_opr_num!(h, default U32_T U32);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(U32_T),
            p if p == STR_T.as_type() => typecast_int!(U32 => str, x),
            p if p == BOOL_T.as_type() => typecast_int!(U32 => bool, x),
            p if p == I8_T.as_type() => typecast_int!(U32 => I8, x),
            p if p == I16_T.as_type() => typecast_int!(U32 => I16, x),
            p if p == I32_T.as_type() => typecast_int!(U32 => I32, x),
            p if p == I64_T.as_type() => typecast_int!(U32 => I64, x),
            p if p == I128_T.as_type() => typecast_int!(U32 => I128, x),
            p if p == ISIZE_T.as_type() => typecast_int!(U32 => Isize, x),
            p if p == IBIG_T.as_type() => typecast_int!(U32 => Ibig, x),
            p if p == U8_T.as_type() => typecast_int!(U32 => U8, x),
            p if p == U16_T.as_type() => typecast_int!(U32 => U16, x),
            p if p == U32_T.as_type() => x[0].to_owned(),
            p if p == U64_T.as_type() => typecast_int!(U32 => U64, x),
            p if p == U128_T.as_type() => typecast_int!(U32 => U128, x),
            p if p == USIZE_T.as_type() => typecast_int!(U32 => Usize, x),
            p if p == UBIG_T.as_type() => typecast_int!(U32 => Ubig, x),
            p if p == F16_T.as_type() => typecast_int!(U32 => f16, x),
            p if p == F32_T.as_type() => typecast_int!(U32 => f32, x),
            p if p == F64_T.as_type() => typecast_int!(U32 => f64, x),
            _ => return None,
        })
    };
    binary!(
        h,
        U32_T.as_type(),
        "_typecast",
        [TYPE_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

pub static U32_T: Lazy<TypeDefinition<Value>> = Lazy::new(|| TypeDefinition {
    name: Some("{builtin u32}".into()),
    inst_name: Some("u32".into()),
    generics: vec![],
    implementations: u32_t(),
    inst_fields: HashMap::new(),
});
