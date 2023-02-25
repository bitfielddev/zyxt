use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use half::f16;
use num::{
    bigint::{ToBigInt, ToBigUint},
    ToPrimitive,
};
use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param,
    primitives::*,
    typecast_float, typecast_to_type,
    types::{
        typeobj::TypeDefinition,
        value::{Proc, Value},
    },
    unary, Type,
};

macro_rules! typecast_f16_to_int {
    ($vo:ident $f:ident, $x:ident) => {
        Value::$vo(get_param!($x, 0, F16).to_f64().$f()?)
    };
}

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn f16_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::F16(f16::from_f64_const(0.0)));
    concat_vals!(h, F16_T);
    unary!(h, F16_T.as_type(), "_un_add", |x: &Vec<Value>| Some(
        x[0].to_owned()
    ));
    unary!(h, F16_T.as_type(), "_un_sub", |x: &Vec<Value>| Some(
        Value::F16(get_param!(x, 0, F16).neg())
    ));
    unary!(h, F16_T.as_type(), "_not", |x: &Vec<Value>| Some(
        Value::Bool(get_param!(x, 0, F16) == f16::ZERO || get_param!(x, 0, F16) == f16::NEG_ZERO)
    ));
    arith_opr_num!(h, float default F16_T F16);
    comp_opr_num!(h, default F16_T F16);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(F16_T),
            p if p == STR_T.as_type() => typecast_float!(F16 => str, x),
            p if p == BOOL_T.as_type() => Value::Bool(
                get_param!(x, 0, F16) != f16::ZERO && get_param!(x, 0, F16) != f16::NEG_ZERO,
            ),
            p if p == I8_T.as_type() => typecast_f16_to_int!(I8 to_i8, x),
            p if p == I16_T.as_type() => typecast_f16_to_int!(I16 to_i16, x),
            p if p == I32_T.as_type() => typecast_f16_to_int!(I32 to_i32, x),
            p if p == I64_T.as_type() => typecast_f16_to_int!(I64 to_i64, x),
            p if p == I128_T.as_type() => typecast_f16_to_int!(I128 to_i128, x),
            p if p == ISIZE_T.as_type() => typecast_f16_to_int!(Isize to_isize, x),
            p if p == IBIG_T.as_type() => typecast_f16_to_int!(Ibig to_bigint, x),
            p if p == U8_T.as_type() => typecast_f16_to_int!(U8 to_u8, x),
            p if p == U16_T.as_type() => typecast_f16_to_int!(U16 to_u16, x),
            p if p == U32_T.as_type() => typecast_f16_to_int!(U32 to_u32, x),
            p if p == U64_T.as_type() => typecast_f16_to_int!(U64 to_u64, x),
            p if p == U128_T.as_type() => typecast_f16_to_int!(U128 to_u128, x),
            p if p == USIZE_T.as_type() => typecast_f16_to_int!(Usize to_usize, x),
            p if p == UBIG_T.as_type() => typecast_f16_to_int!(Ubig to_biguint, x),
            p if p == F16_T.as_type() => x[0].to_owned(),
            p if p == F32_T.as_type() => Value::F32(get_param!(x, 0, F16).to_f32()),
            p if p == F64_T.as_type() => Value::F64(get_param!(x, 0, F16).to_f64()),
            _ => return None,
        })
    };
    binary!(
        h,
        F16_T.as_type(),
        "_typecast",
        [TYPE_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

pub static F16_T: Lazy<TypeDefinition<Value>> = Lazy::new(|| TypeDefinition {
    name: Some("{builtin f16}".into()),
    inst_name: Some("f16".into()),
    generics: vec![],
    implementations: f16_t(),
    inst_fields: HashMap::new(),
});
