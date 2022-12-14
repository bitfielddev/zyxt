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

fn i8_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::I8(0));
    concat_vals!(h, I8_T);
    unary!(h, signed default I8_T I8);
    arith_opr_num!(h, default I8_T I8);
    comp_opr_num!(h, default I8_T I8);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(I8_T),
            p if p == STR_T.as_type() => typecast_int!(I8 => str, x),
            p if p == BOOL_T.as_type() => typecast_int!(I8 => bool, x),
            p if p == I8_T.as_type() => x[0].to_owned(),
            p if p == I16_T.as_type() => typecast_int!(I8 => I16, x),
            p if p == I32_T.as_type() => typecast_int!(I8 => I32, x),
            p if p == I64_T.as_type() => typecast_int!(I8 => I64, x),
            p if p == I128_T.as_type() => typecast_int!(I8 => I128, x),
            p if p == ISIZE_T.as_type() => typecast_int!(I8 => Isize, x),
            p if p == IBIG_T.as_type() => typecast_int!(I8 => Ibig, x),
            p if p == U8_T.as_type() => typecast_int!(I8 => U8, x),
            p if p == U16_T.as_type() => typecast_int!(I8 => U16, x),
            p if p == U32_T.as_type() => typecast_int!(I8 => U32, x),
            p if p == U64_T.as_type() => typecast_int!(I8 => U64, x),
            p if p == U128_T.as_type() => typecast_int!(I8 => U128, x),
            p if p == USIZE_T.as_type() => typecast_int!(I8 => Usize, x),
            p if p == UBIG_T.as_type() => typecast_int!(I8 => Ubig, x),
            p if p == F16_T.as_type() => typecast_int!(I8 => f16, x),
            p if p == F32_T.as_type() => typecast_int!(I8 => f32, x),
            p if p == F64_T.as_type() => typecast_int!(I8 => f64, x),
            _ => return None,
        })
    };
    binary!(
        h,
        I8_T.as_type(),
        "_typecast",
        [TYPE_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

pub static I8_T: Lazy<TypeDefinition<Value>> = Lazy::new(|| TypeDefinition {
    name: Some("{builtin i8}".into()),
    inst_name: Some("i8".into()),
    generics: vec![],
    implementations: i8_t(),
    inst_fields: HashMap::new(),
});
