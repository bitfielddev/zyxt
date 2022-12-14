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

fn i128_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::I128(0));
    concat_vals!(h, I128_T);
    unary!(h, signed default I128_T I128);
    arith_opr_num!(h, default I128_T I128);
    comp_opr_num!(h, default I128_T I128);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(I128_T),
            p if p == STR_T.as_type() => typecast_int!(I128 => str, x),
            p if p == BOOL_T.as_type() => typecast_int!(I128 => bool, x),
            p if p == I8_T.as_type() => typecast_int!(I128 => I8, x),
            p if p == I16_T.as_type() => typecast_int!(I128 => I16, x),
            p if p == I32_T.as_type() => typecast_int!(I128 => I32, x),
            p if p == I64_T.as_type() => typecast_int!(I128 => I64, x),
            p if p == I128_T.as_type() => x[0].to_owned(),
            p if p == ISIZE_T.as_type() => typecast_int!(I128 => Isize, x),
            p if p == IBIG_T.as_type() => typecast_int!(I128 => Ibig, x),
            p if p == U8_T.as_type() => typecast_int!(I128 => U8, x),
            p if p == U16_T.as_type() => typecast_int!(I128 => U16, x),
            p if p == U32_T.as_type() => typecast_int!(I128 => U32, x),
            p if p == U64_T.as_type() => typecast_int!(I128 => U64, x),
            p if p == U128_T.as_type() => typecast_int!(I128 => U128, x),
            p if p == USIZE_T.as_type() => typecast_int!(I128 => Usize, x),
            p if p == UBIG_T.as_type() => typecast_int!(I128 => Ubig, x),
            p if p == F16_T.as_type() => typecast_int!(I128 => f16, x),
            p if p == F32_T.as_type() => typecast_int!(I128 => f32, x),
            p if p == F64_T.as_type() => typecast_int!(I128 => f64, x),
            _ => return None,
        })
    };
    binary!(
        h,
        I128_T.as_type(),
        "_typecast",
        [TYPE_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

pub static I128_T: Lazy<TypeDefinition<Value>> = Lazy::new(|| TypeDefinition {
    name: Some("{builtin i128}".into()),
    inst_name: Some("i128".into()),
    generics: vec![],
    implementations: i128_t(),
    inst_fields: HashMap::new(),
});
