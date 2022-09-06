use std::collections::HashMap;

use half::f16;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, types::value::Proc,
    unary, Type, Value,
};

pub const fn usize_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "usize");
    unary!(h, unsigned default "usize" Usize);
    arith_opr_num!(h, default "usize" Usize);
    comp_opr_num!(h, default "usize" Usize);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_int!("usize" => type),
                "str" => typecast_int!(Usize => str, x),
                "bool" => typecast_int!(Usize => bool, x),
                "i8" => typecast_int!(Usize => I8, x),
                "i16" => typecast_int!(Usize => I16, x),
                "i32" => typecast_int!(Usize => I32, x),
                "i64" => typecast_int!(Usize => I64, x),
                "i128" => typecast_int!(Usize => I128, x),
                "isize" => typecast_int!(Usize => Isize, x),
                "ibig" => typecast_int!(Usize => Ibig, x),
                "u8" => typecast_int!(Usize => U8, x),
                "u16" => typecast_int!(Usize => U16, x),
                "u32" => typecast_int!(Usize => U32, x),
                "u64" => typecast_int!(Usize => U64, x),
                "u128" => typecast_int!(Usize => U128, x),
                "usize" => x[0].to_owned(),
                "ubig" => typecast_int!(Usize => Ubig, x),
                "f16" => typecast_int!(Usize => f16, x),
                "f32" => typecast_int!(Usize => f32, x),
                "f64" => typecast_int!(Usize => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "usize", "_typecast", ["type"], "_any", typecast);

    h
}
