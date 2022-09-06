use std::collections::HashMap;

use half::f16;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, types::value::Proc,
    unary, Type, Value,
};

pub const fn i64_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "i64");
    unary!(h, signed default "i64" I64);
    arith_opr_num!(h, default "i64" I64);
    comp_opr_num!(h, default "i64" I64);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_int!("i64" => type),
                "str" => typecast_int!(I64 => str, x),
                "bool" => typecast_int!(I64 => bool, x),
                "i8" => typecast_int!(I64 => I8, x),
                "i16" => typecast_int!(I64 => I16, x),
                "i32" => typecast_int!(I64 => I32, x),
                "i64" => x[0].to_owned(),
                "i128" => typecast_int!(I64 => I128, x),
                "isize" => typecast_int!(I64 => Isize, x),
                "ibig" => typecast_int!(I64 => Ibig, x),
                "u8" => typecast_int!(I64 => U8, x),
                "u16" => typecast_int!(I64 => U16, x),
                "u32" => typecast_int!(I64 => U32, x),
                "u64" => typecast_int!(I64 => U64, x),
                "u128" => typecast_int!(I64 => U128, x),
                "usize" => typecast_int!(I64 => Usize, x),
                "ubig" => typecast_int!(I64 => Ubig, x),
                "f16" => typecast_int!(I64 => f16, x),
                "f32" => typecast_int!(I64 => f32, x),
                "f64" => typecast_int!(I64 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "i64", "_typecast", ["type"], "_any", typecast);

    h
}
