use std::collections::HashMap;

use half::f16;
use lazy_static::lazy_static;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, typecast_to_type,
    types::{
        typeobj::{bool_t::BOOL_T, str_t::STR_T, type_t::TYPE_T},
        value::{Proc, Value},
    },
    unary, Type,
};

const fn i8_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, I8_T);
    unary!(h, signed default I8_T I8);
    arith_opr_num!(h, default I8_T I8);
    comp_opr_num!(h, default I8_T I8);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(I8_T),
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
    binary!(h, I8_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref I8_T: Type = Type::Definition {
        name: Some("i8".into()),
        generics: vec![],
        implementations: i8_t(),
        inst_fields: HashMap::new(),
    };
}
