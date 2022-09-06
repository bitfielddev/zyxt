use std::collections::HashMap;

use half::f16;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, types::value::Proc,
    unary, Type, Value,
};

pub const fn u16_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "u16");
    unary!(h, unsigned default "u16" U16);
    arith_opr_num!(h, default "u16" U16);
    comp_opr_num!(h, default "u16" U16);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_int!("u16" => type),
                "str" => typecast_int!(U16 => str, x),
                "bool" => typecast_int!(U16 => bool, x),
                "i8" => typecast_int!(U16 => I8, x),
                "i16" => typecast_int!(U16 => I16, x),
                "i32" => typecast_int!(U16 => I32, x),
                "i64" => typecast_int!(U16 => I64, x),
                "i128" => typecast_int!(U16 => I128, x),
                "isize" => typecast_int!(U16 => Isize, x),
                "ibig" => typecast_int!(U16 => Ibig, x),
                "u8" => typecast_int!(U16 => U8, x),
                "u16" => x[0].to_owned(),
                "u32" => typecast_int!(U16 => U32, x),
                "u64" => typecast_int!(U16 => U64, x),
                "u128" => typecast_int!(U16 => U128, x),
                "usize" => typecast_int!(U16 => Usize, x),
                "ubig" => typecast_int!(U16 => Ubig, x),
                "f16" => typecast_int!(U16 => f16, x),
                "f32" => typecast_int!(U16 => f32, x),
                "f64" => typecast_int!(U16 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "u16", "_typecast", ["type"], "_any", typecast);

    h
}
