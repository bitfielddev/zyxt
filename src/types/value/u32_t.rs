use std::collections::HashMap;

use half::f16;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, types::value::Proc,
    unary, Type, Value,
};

pub const fn u32_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "u32");
    unary!(h, unsigned default "u32" U32);
    arith_opr_num!(h, default "u32" U32);
    comp_opr_num!(h, default "u32" U32);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_int!("u32" => type),
                "str" => typecast_int!(U32 => str, x),
                "bool" => typecast_int!(U32 => bool, x),
                "i8" => typecast_int!(U32 => I8, x),
                "i16" => typecast_int!(U32 => I16, x),
                "i32" => typecast_int!(U32 => I32, x),
                "i64" => typecast_int!(U32 => I64, x),
                "i128" => typecast_int!(U32 => I128, x),
                "isize" => typecast_int!(U32 => Isize, x),
                "ibig" => typecast_int!(U32 => Ibig, x),
                "u8" => typecast_int!(U32 => U8, x),
                "u16" => typecast_int!(U32 => U16, x),
                "u32" => x[0].to_owned(),
                "u64" => typecast_int!(U32 => U64, x),
                "u128" => typecast_int!(U32 => U128, x),
                "usize" => typecast_int!(U32 => Usize, x),
                "ubig" => typecast_int!(U32 => Ubig, x),
                "f16" => typecast_int!(U32 => f16, x),
                "f32" => typecast_int!(U32 => f32, x),
                "f64" => typecast_int!(U32 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "u32", "_typecast", ["type"], "_any", typecast);

    h
}
