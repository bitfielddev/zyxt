use std::{collections::HashMap, ops::Rem};

use half::f16;
use num::BigUint;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, ToPrimitive};

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, types::value::Proc,
    unary, Type, Value,
};

pub const fn ubig_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "ubig");

    unary!(h, "ubig", "_un_add", |x: &Vec<Value>| Some(x[0].to_owned()));
    unary!(h, "ubig", "_not", |x: &Vec<Value>| Some(Value::Bool(
        get_param!(x, 0, Ubig) == 0u8.into()
    )));

    arith_opr_num!(h, big default "ubig" Ubig);
    comp_opr_num!(h, default "ubig" Ubig);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_int!("ubig" => type),
                "str" => typecast_int!(Ubig => str, x),
                "bool" => typecast_int!(Ubig => into bool, x),
                "i8" => typecast_int!(Ubig => I8, x),
                "i16" => typecast_int!(Ubig => I16, x),
                "i32" => typecast_int!(Ubig => I32, x),
                "i164" => typecast_int!(Ubig => I64, x),
                "i128" => typecast_int!(Ubig => I128, x),
                "isize" => typecast_int!(Ubig => Ubig, x),
                "ibig" => typecast_int!(Ubig => Ibig, x),
                "u8" => typecast_int!(Ubig => U8, x),
                "u16" => typecast_int!(Ubig => U16, x),
                "u32" => typecast_int!(Ubig => U32, x),
                "u64" => typecast_int!(Ubig => U64, x),
                "u128" => typecast_int!(Ubig => U128, x),
                "usize" => typecast_int!(Ubig => Usize, x),
                "ubig" => x[0].to_owned(),
                "f16" => typecast_int!(big Ubig => f16, x),
                "f32" => typecast_int!(big Ubig => f32, x),
                "f64" => typecast_int!(big Ubig => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "ubig", "_typecast", ["type"], "_any", typecast);

    h
}
