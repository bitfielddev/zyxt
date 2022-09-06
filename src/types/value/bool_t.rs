use std::collections::HashMap;
use crate::{concat_vals, get_param, binary, Type, Value, typecast_to_type};
use crate::types::value::Proc;

macro_rules! typecast_bool_to_num {
    ($v:ident $v2:ty, $x:ident) => {
       Value::$v(get_param!($x, 0, Bool) as $v2)
    };
    ($v:ident, $x:ident) => {
       Value::$v(if get_param!($x, 0, Bool) {1u8} else {0u8}.into())
    }
}

pub const fn bool_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "bool");

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!("bool"),
                "str" => Value::Str(get_param!(x, 0, Bool).to_string()),
                "bool" => x[0].to_owned(),
                "i8" => typecast_bool_to_num!(I8 i8, x),
                "i16" => typecast_bool_to_num!(I16 i16, x),
                "i32" => typecast_bool_to_num!(I32 i32, x),
                "i64" => typecast_bool_to_num!(I64 i64, x),
                "i128" => typecast_bool_to_num!(I128 i128, x),
                "isize" => typecast_bool_to_num!(Isize isize, x),
                "ibig" => typecast_bool_to_num!(Ibig, x),
                "u8" => typecast_bool_to_num!(U8 u8, x),
                "u16" => typecast_bool_to_num!(U16 u16, x),
                "u32" => typecast_bool_to_num!(U32 u32, x),
                "u64" => typecast_bool_to_num!(U64 u64, x),
                "u128" => typecast_bool_to_num!(U128 u128, x),
                "usize" => typecast_bool_to_num!(Usize usize, x),
                "ubig" => typecast_bool_to_num!(Ubig, x),
                "f16" => typecast_bool_to_num!(F16, x),
                "f32" => typecast_bool_to_num!(F32, x),
                "f64" => typecast_bool_to_num!(F64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "i32", "_typecast", ["type"], "_any", typecast);

    h
}
