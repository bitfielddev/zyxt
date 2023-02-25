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
fn isize_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::Isize(0));
    concat_vals!(h, ISIZE_T);
    unary!(h, signed default ISIZE_T Isize);
    arith_opr_num!(h, default ISIZE_T Isize);
    comp_opr_num!(h, default ISIZE_T Isize);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(ISIZE_T),
            p if p == STR_T.as_type() => typecast_int!(Isize => str, x),
            p if p == BOOL_T.as_type() => typecast_int!(Isize => bool, x),
            p if p == I8_T.as_type() => typecast_int!(Isize => I8, x),
            p if p == I16_T.as_type() => typecast_int!(Isize => I16, x),
            p if p == I32_T.as_type() => typecast_int!(Isize => I32, x),
            p if p == I64_T.as_type() => typecast_int!(Isize => I64, x),
            p if p == I128_T.as_type() => typecast_int!(Isize => I128, x),
            p if p == ISIZE_T.as_type() => x[0].to_owned(),
            p if p == IBIG_T.as_type() => typecast_int!(Isize => Ibig, x),
            p if p == U8_T.as_type() => typecast_int!(Isize => U8, x),
            p if p == U16_T.as_type() => typecast_int!(Isize => U16, x),
            p if p == U32_T.as_type() => typecast_int!(Isize => U32, x),
            p if p == U64_T.as_type() => typecast_int!(Isize => U64, x),
            p if p == U128_T.as_type() => typecast_int!(Isize => U128, x),
            p if p == USIZE_T.as_type() => typecast_int!(Isize => Usize, x),
            p if p == UBIG_T.as_type() => typecast_int!(Isize => Ubig, x),
            p if p == F16_T.as_type() => typecast_int!(Isize => f16, x),
            p if p == F32_T.as_type() => typecast_int!(Isize => f32, x),
            p if p == F64_T.as_type() => typecast_int!(Isize => f64, x),
            _ => return None,
        })
    };
    binary!(
        h,
        ISIZE_T.as_type(),
        "_typecast",
        [TYPE_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

pub static ISIZE_T: Lazy<TypeDefinition<Value>> = Lazy::new(|| TypeDefinition {
    name: Some("{builtin isize}".into()),
    inst_name: Some("isize".into()),
    generics: vec![],
    implementations: isize_t(),
    inst_fields: HashMap::new(),
});
