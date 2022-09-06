use std::{
    collections::HashMap,
    ops::{Neg, Rem},
};

use half::f16;
use num_traits::ToPrimitive;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, types::value::Proc,
    unary, Type, Value,
};

pub const fn ibig_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "ibig");

    unary!(h, "ibig", "_un_add", |x: &Vec<Value>| Some(x[0].to_owned()));
    unary!(h, "ibig", "_un_sub", |x: &Vec<Value>| Some(Value::Ibig(
        get_param!(x, 0, Ibig).neg()
    )));
    unary!(h, "ibig", "_not", |x: &Vec<Value>| Some(Value::Bool(
        get_param!(x, 0, Ibig) == 0.into()
    )));

    arith_opr_num!(h, big default "ibig" Ibig);
    comp_opr_num!(h, default "ibig" Ibig);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_int!("ibig" => type),
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
    binary!(h, "ibig", "_typecast", ["type"], "_any", typecast);

    h
}
