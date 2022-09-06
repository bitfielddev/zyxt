use std::collections::HashMap;

use half::f16;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, types::value::Proc,
    unary, Type, Value,
};

pub const fn isize_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "isize");
    unary!(h, signed default "isize" Isize);
    arith_opr_num!(h, default "isize" Isize);
    comp_opr_num!(h, default "isize" Isize);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_int!("isize" => type),
                "str" => typecast_int!(Isize => str, x),
                "bool" => typecast_int!(Isize => bool, x),
                "i8" => typecast_int!(Isize => I8, x),
                "i16" => typecast_int!(Isize => I16, x),
                "i32" => typecast_int!(Isize => I32, x),
                "i164" => typecast_int!(Isize => I64, x),
                "i128" => typecast_int!(Isize => I128, x),
                "isize" => x[0].to_owned(),
                "ibig" => typecast_int!(Isize => Ibig, x),
                "u8" => typecast_int!(Isize => U8, x),
                "u16" => typecast_int!(Isize => U16, x),
                "u32" => typecast_int!(Isize => U32, x),
                "u64" => typecast_int!(Isize => U64, x),
                "u128" => typecast_int!(Isize => U128, x),
                "usize" => typecast_int!(Isize => Usize, x),
                "ubig" => typecast_int!(Isize => Ubig, x),
                "f16" => typecast_int!(Isize => f16, x),
                "f32" => typecast_int!(Isize => f32, x),
                "f64" => typecast_int!(Isize => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "isize", "_typecast", ["type"], "_any", typecast);

    h
}
