use std::{collections::HashMap, ops::Rem};

use half::f16;
use lazy_static::lazy_static;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, ToPrimitive};
use smol_str::SmolStr;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, typecast_to_type,
    types::{
        typeobj::{
            bool_t::BOOL_T, f16_t::F16_T, f32_t::F32_T, f64_t::F64_T, i128_t::I128_T, i16_t::I16_T,
            i32_t::I32_T, i64_t::I64_T, i8_t::I8_T, ibig_t::IBIG_T, isize_t::ISIZE_T, str_t::STR_T,
            type_t::TYPE_T, u128_t::U128_T, u16_t::U16_T, u32_t::U32_T, u64_t::U64_T, u8_t::U8_T,
            usize_t::USIZE_T, TypeDefinition,
        },
        value::{Proc, Value},
    },
    unary, Type,
};

fn ubig_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::Ubig(0u8.into()));
    concat_vals!(h, UBIG_T);

    unary!(h, UBIG_T.as_type(), "_un_add", |x: &Vec<Value>| Some(
        x[0].to_owned()
    ));
    unary!(h, UBIG_T.as_type(), "_not", |x: &Vec<Value>| Some(
        Value::Bool(get_param!(x, 0, Ubig) == 0u8.into())
    ));

    arith_opr_num!(h, big default UBIG_T Ubig);
    comp_opr_num!(h, default UBIG_T Ubig);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(UBIG_T),
            p if p == STR_T.as_type() => typecast_int!(Ubig => str, x),
            p if p == BOOL_T.as_type() => Value::Bool(get_param!(x, 0, Ubig) == 0u8.into()),
            p if p == I8_T.as_type() => typecast_int!(Ubig => I8, x),
            p if p == I16_T.as_type() => typecast_int!(Ubig => I16, x),
            p if p == I32_T.as_type() => typecast_int!(Ubig => I32, x),
            p if p == I64_T.as_type() => typecast_int!(Ubig => I64, x),
            p if p == I128_T.as_type() => typecast_int!(Ubig => I128, x),
            p if p == ISIZE_T.as_type() => typecast_int!(Ubig => Ubig, x),
            p if p == IBIG_T.as_type() => typecast_int!(Ubig => Ibig, x),
            p if p == U8_T.as_type() => typecast_int!(Ubig => U8, x),
            p if p == U16_T.as_type() => typecast_int!(Ubig => U16, x),
            p if p == U32_T.as_type() => typecast_int!(Ubig => U32, x),
            p if p == U64_T.as_type() => typecast_int!(Ubig => U64, x),
            p if p == U128_T.as_type() => typecast_int!(Ubig => U128, x),
            p if p == USIZE_T.as_type() => typecast_int!(Ubig => Usize, x),
            p if p == UBIG_T.as_type() => x[0].to_owned(),
            p if p == F16_T.as_type() => typecast_int!(big Ubig => f16, x),
            p if p == F32_T.as_type() => typecast_int!(big Ubig => f32, x),
            p if p == F64_T.as_type() => typecast_int!(big Ubig => f64, x),
            _ => return None,
        })
    };
    binary!(
        h,
        UBIG_T.as_type(),
        "_typecast",
        [TYPE_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

lazy_static! {
    pub static ref UBIG_T: TypeDefinition<Value> = TypeDefinition {
        name: Some("{builtin ubig}".into()),
        inst_name: Some("ubig".into()),
        generics: vec![],
        implementations: ubig_t(),
        inst_fields: HashMap::new(),
    };
}
