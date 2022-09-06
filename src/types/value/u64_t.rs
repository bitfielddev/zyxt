use std::collections::HashMap;

use half::f16;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, types::value::Proc,
    unary, Type, Value,
};

pub const fn u64_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "u64");
    unary!(h, unsigned default "u64" U64);
    arith_opr_num!(h, default "u64" U64);
    comp_opr_num!(h, default "u64" U64);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_int!("u64" => type),
                "str" => typecast_int!(U64 => str, x),
                "bool" => typecast_int!(U64 => bool, x),
                "i8" => typecast_int!(U64 => I8, x),
                "i16" => typecast_int!(U64 => I16, x),
                "i32" => typecast_int!(U64 => I32, x),
                "i64" => typecast_int!(U64 => I64, x),
                "i128" => typecast_int!(U64 => I128, x),
                "isize" => typecast_int!(U64 => Isize, x),
                "ibig" => typecast_int!(U64 => Ibig, x),
                "u8" => typecast_int!(U64 => U8, x),
                "u16" => typecast_int!(U64 => U16, x),
                "u32" => typecast_int!(U64 => U32, x),
                "u64" => x[0].to_owned(),
                "u128" => typecast_int!(U64 => U128, x),
                "usize" => typecast_int!(U64 => Usize, x),
                "ubig" => typecast_int!(U64 => Ubig, x),
                "f16" => typecast_int!(U64 => f16, x),
                "f32" => typecast_int!(U64 => f32, x),
                "f64" => typecast_int!(U64 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "u64", "_typecast", ["type"], "_any", typecast);

    h
}
