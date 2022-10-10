use std::{
    collections::HashMap,
    ops::{Neg, Rem},
};

use half::f16;
use num_traits::ToPrimitive;
use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, typecast_to_type,
    types::{
        typeobj::{
            bool_t::BOOL_T, f16_t::F16_T, f32_t::F32_T, f64_t::F64_T, i128_t::I128_T, i16_t::I16_T,
            i32_t::I32_T, i64_t::I64_T, i8_t::I8_T, isize_t::ISIZE_T, str_t::STR_T, type_t::TYPE_T,
            u128_t::U128_T, u16_t::U16_T, u32_t::U32_T, u64_t::U64_T, u8_t::U8_T, ubig_t::UBIG_T,
            usize_t::USIZE_T, TypeDefinition,
        },
        value::{Proc, Value},
    },
    unary, Type,
};

fn ibig_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::Ibig(0.into()));
    concat_vals!(h, IBIG_T);

    unary!(h, IBIG_T.as_type(), "_un_add", |x: &Vec<Value>| Some(
        x[0].to_owned()
    ));
    unary!(h, IBIG_T.as_type(), "_un_sub", |x: &Vec<Value>| Some(
        Value::Ibig(get_param!(x, 0, Ibig).neg())
    ));
    unary!(h, IBIG_T.as_type(), "_not", |x: &Vec<Value>| Some(
        Value::Bool(get_param!(x, 0, Ibig) == 0.into())
    ));

    arith_opr_num!(h, big default IBIG_T Ibig);
    comp_opr_num!(h, default IBIG_T Ibig);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(IBIG_T),
            p if p == STR_T.as_type() => typecast_int!(Ibig => str, x),
            p if p == BOOL_T.as_type() => Value::Bool(get_param!(x, 0, Ibig) == 0.into()),
            p if p == I8_T.as_type() => typecast_int!(Ibig => I8, x),
            p if p == I16_T.as_type() => typecast_int!(Ibig => I16, x),
            p if p == I32_T.as_type() => typecast_int!(Ibig => I32, x),
            p if p == I64_T.as_type() => typecast_int!(Ibig => I64, x),
            p if p == I128_T.as_type() => typecast_int!(Ibig => I128, x),
            p if p == ISIZE_T.as_type() => typecast_int!(Ibig => Ibig, x),
            p if p == IBIG_T.as_type() => x[0].to_owned(),
            p if p == U8_T.as_type() => typecast_int!(Ibig => U8, x),
            p if p == U16_T.as_type() => typecast_int!(Ibig => U16, x),
            p if p == U32_T.as_type() => typecast_int!(Ibig => U32, x),
            p if p == U64_T.as_type() => typecast_int!(Ibig => U64, x),
            p if p == U128_T.as_type() => typecast_int!(Ibig => U128, x),
            p if p == USIZE_T.as_type() => typecast_int!(Ibig => Usize, x),
            p if p == UBIG_T.as_type() => typecast_int!(Ibig => Ubig, x),
            p if p == F16_T.as_type() => typecast_int!(big Ibig => f16, x),
            p if p == F32_T.as_type() => typecast_int!(big Ibig => f32, x),
            p if p == F64_T.as_type() => typecast_int!(big Ibig => f64, x),
            _ => return None,
        })
    };
    binary!(
        h,
        IBIG_T.as_type(),
        "_typecast",
        [TYPE_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

pub static IBIG_T: Lazy<TypeDefinition<Value>> = Lazy::new(|| TypeDefinition {
    name: Some("{builtin ibig}".into()),
    inst_name: Some("ibig".into()),
    generics: vec![],
    implementations: ibig_t(),
    inst_fields: HashMap::new(),
});
