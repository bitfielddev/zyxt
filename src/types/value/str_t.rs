use std::collections::HashMap;
use crate::{concat_vals, get_param, binary, Type, Value, typecast_to_type};
use crate::types::value::Proc;

macro_rules! typecast_str_to_num {
    ($v:ident, $x:ident) => {
       Value::$v(get_param!($x, 0, Str).parse().ok()?)
    }
}

pub const fn str_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "str");

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!("str"),
                "str" => x[0].to_owned(),
                "bool" => Value::Bool(get_param!(x, 0, Str).is_empty()),
                "i8" => typecast_str_to_num!(I8, x),
                "i16" => typecast_str_to_num!(I16, x),
                "i32" => typecast_str_to_num!(I32, x),
                "i64" => typecast_str_to_num!(I64, x),
                "i128" => typecast_str_to_num!(I128, x),
                "isize" => typecast_str_to_num!(Isize, x),
                "ibig" => typecast_str_to_num!(Ibig, x),
                "u8" => typecast_str_to_num!(U8, x),
                "u16" => typecast_str_to_num!(U16, x),
                "u32" => typecast_str_to_num!(U32, x),
                "u64" => typecast_str_to_num!(U64, x),
                "u128" => typecast_str_to_num!(U128, x),
                "usize" => typecast_str_to_num!(Usize, x),
                "ubig" => typecast_str_to_num!(Ubig, x),
                "f16" => typecast_str_to_num!(F16, x),
                "f32" => typecast_str_to_num!(F32, x),
                "f64" => typecast_str_to_num!(F64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "i32", "_typecast", ["type"], "_any", typecast);

    h
}
