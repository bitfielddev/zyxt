use std::collections::HashMap;

use half::f16;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, types::value::Proc,
    unary, Type, Value,
};

pub const fn u8_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "u8");
    unary!(h, unsigned default "u8" U8);
    arith_opr_num!(h, default "u8" U8);
    comp_opr_num!(h, default "u8" U8);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_int!("u8" => type),
                "str" => typecast_int!(U8 => str, x),
                "bool" => typecast_int!(U8 => bool, x),
                "i8" => typecast_int!(U8 => I8, x),
                "i16" => typecast_int!(U8 => I16, x),
                "i32" => typecast_int!(U8 => I32, x),
                "i64" => typecast_int!(U8 => I64, x),
                "i128" => typecast_int!(U8 => I128, x),
                "isize" => typecast_int!(U8 => Isize, x),
                "ibig" => typecast_int!(U8 => Ibig, x),
                "u8" => x[0].to_owned(),
                "u16" => typecast_int!(U8 => U16, x),
                "u32" => typecast_int!(U8 => U32, x),
                "u64" => typecast_int!(U8 => U64, x),
                "u128" => typecast_int!(U8 => U128, x),
                "usize" => typecast_int!(U8 => Usize, x),
                "ubig" => typecast_int!(U8 => Ubig, x),
                "f16" => typecast_int!(U8 => f16, x),
                "f32" => typecast_int!(U8 => f32, x),
                "f64" => typecast_int!(U8 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "u8", "_typecast", ["type"], "_any", typecast);

    h
}
