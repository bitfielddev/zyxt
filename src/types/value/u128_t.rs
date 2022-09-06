use std::collections::HashMap;

use half::f16;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, types::value::Proc,
    unary, Type, Value,
};

pub const fn u128_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "u128");
    unary!(h, unsigned default "u128" U128);
    arith_opr_num!(h, default "u128" U128);
    comp_opr_num!(h, default "u128" U128);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_int!("u128" => type),
                "str" => typecast_int!(U128 => str, x),
                "bool" => typecast_int!(U128 => bool, x),
                "i8" => typecast_int!(U128 => I8, x),
                "i16" => typecast_int!(U128 => I16, x),
                "i32" => typecast_int!(U128 => I32, x),
                "i64" => typecast_int!(U128 => I64, x),
                "i128" => typecast_int!(U128 => I128, x),
                "isize" => typecast_int!(U128 => Isize, x),
                "ibig" => typecast_int!(U128 => Ibig, x),
                "u8" => typecast_int!(U128 => U8, x),
                "u16" => typecast_int!(U128 => U16, x),
                "u32" => typecast_int!(U128 => U32, x),
                "u64" => typecast_int!(U128 => U64, x),
                "u128" => x[0].to_owned(),
                "usize" => typecast_int!(U128 => Usize, x),
                "ubig" => typecast_int!(U128 => Ubig, x),
                "f16" => typecast_int!(U128 => f16, x),
                "f32" => typecast_int!(U128 => f32, x),
                "f64" => typecast_int!(U128 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "u128", "_typecast", ["type"], "_any", typecast);

    h
}
