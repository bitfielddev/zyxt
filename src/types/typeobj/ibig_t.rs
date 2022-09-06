use std::{
    collections::HashMap,
    ops::{Neg, Rem},
};

use half::f16;
use num_traits::ToPrimitive;

use crate::{arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, unary, Type, typecast_to_type};
use crate::types::value::{Proc, Value};
use lazy_static::lazy_static;
use crate::types::typeobj::type_t::TYPE_T;
use crate::types::typeobj::str_t::STR_T;
use crate::types::typeobj::bool_t::BOOL_T;

const fn ibig_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, IBIG_T);

    unary!(h, IBIG_T, "_un_add", |x: &Vec<Value>| Some(x[0].to_owned()));
    unary!(h, IBIG_T, "_un_sub", |x: &Vec<Value>| Some(Value::Ibig(
        get_param!(x, 0, Ibig).neg()
    )));
    unary!(h, IBIG_T, "_not", |x: &Vec<Value>| Some(Value::Bool(
        get_param!(x, 0, Ibig) == 0.into()
    )));

    arith_opr_num!(h, big default IBIG_T Ibig);
    comp_opr_num!(h, default IBIG_T Ibig);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(IBIG_T),
                "str" => typecast_int!(Ibig => str, x),
                "bool" => typecast_int!(Ibig => into bool, x),
                "i8" => typecast_int!(Ibig => I8, x),
                "i16" => typecast_int!(Ibig => I16, x),
                "i32" => typecast_int!(Ibig => I32, x),
                "i164" => typecast_int!(Ibig => I64, x),
                "i128" => typecast_int!(Ibig => I128, x),
                "isize" => typecast_int!(Ibig => Ibig, x),
                "ibig" => x[0].to_owned(),
                "u8" => typecast_int!(Ibig => U8, x),
                "u16" => typecast_int!(Ibig => U16, x),
                "u32" => typecast_int!(Ibig => U32, x),
                "u64" => typecast_int!(Ibig => U64, x),
                "u128" => typecast_int!(Ibig => U128, x),
                "usize" => typecast_int!(Ibig => Usize, x),
                "ubig" => typecast_int!(Ibig => Ubig, x),
                "f16" => typecast_int!(big Ibig => f16, x),
                "f32" => typecast_int!(big Ibig => f32, x),
                "f64" => typecast_int!(big Ibig => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, IBIG_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref IBIG_T: Type = Type::Definition {
        name: Some("ibig".into()),
        generics: vec![],
        implementations: ibig_t(),
        inst_fields: HashMap::new(),
    };
}
