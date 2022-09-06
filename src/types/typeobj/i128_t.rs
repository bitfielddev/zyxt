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

const fn i128_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, I128_T);
    unary!(h, signed default I128_T I128);
    arith_opr_num!(h, default I128_T I128);
    comp_opr_num!(h, default I128_T I128);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(I128_T),
                "str" => typecast_int!(I128 => str, x),
                "bool" => typecast_int!(I128 => bool, x),
                "i8" => typecast_int!(I128 => I8, x),
                "i16" => typecast_int!(I128 => I16, x),
                "i32" => typecast_int!(I128 => I32, x),
                "i64" => typecast_int!(I128 => I64, x),
                "i128" => x[0].to_owned(),
                "isize" => typecast_int!(I128 => Isize, x),
                "ibig" => typecast_int!(I128 => Ibig, x),
                "u8" => typecast_int!(I128 => U8, x),
                "u16" => typecast_int!(I128 => U16, x),
                "u32" => typecast_int!(I128 => U32, x),
                "u64" => typecast_int!(I128 => U64, x),
                "u128" => typecast_int!(I128 => U128, x),
                "usize" => typecast_int!(I128 => Usize, x),
                "ubig" => typecast_int!(I128 => Ubig, x),
                "f16" => typecast_int!(I128 => f16, x),
                "f32" => typecast_int!(I128 => f32, x),
                "f64" => typecast_int!(I128 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, I128_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref I128_T: Type<Value> = Type::Definition {
        inst_name: Some("i128".into()),
        generics: vec![],
        implementations: i128_t(),
        inst_fields: HashMap::new(),
    };
}
