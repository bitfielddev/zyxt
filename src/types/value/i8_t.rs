use std::collections::HashMap;

use half::f16;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, types::value::Proc,
    unary, Type, Value,
};

pub const fn i8_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "i8");
    unary!(h, signed default "i8" I8);
    arith_opr_num!(h, default "i8" I8);
    comp_opr_num!(h, default "i8" I8);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_int!("i8" => type),
                "str" => typecast_int!(I8 => str, x),
                "bool" => typecast_int!(I8 => bool, x),
                "i8" => x[0].to_owned(),
                "i16" => typecast_int!(I8 => I16, x),
                "i32" => typecast_int!(I8 => I32, x),
                "i64" => typecast_int!(I8 => I64, x),
                "i128" => typecast_int!(I8 => I128, x),
                "isize" => typecast_int!(I8 => Isize, x),
                "ibig" => typecast_int!(I8 => Ibig, x),
                "u8" => typecast_int!(I8 => U8, x),
                "u16" => typecast_int!(I8 => U16, x),
                "u32" => typecast_int!(I8 => U32, x),
                "u64" => typecast_int!(I8 => U64, x),
                "u128" => typecast_int!(I8 => U128, x),
                "usize" => typecast_int!(I8 => Usize, x),
                "ubig" => typecast_int!(I8 => Ubig, x),
                "f16" => typecast_int!(I8 => f16, x),
                "f32" => typecast_int!(I8 => f32, x),
                "f64" => typecast_int!(I8 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "i8", "_typecast", ["type"], "_any", typecast);

    h
}
