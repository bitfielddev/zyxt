use std::collections::HashMap;

use half::f16;

use crate::{arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, unary, Type, typecast_to_type};
use crate::types::value::{Proc, Value};
use lazy_static::lazy_static;
use crate::types::typeobj::type_t::TYPE_T;
use crate::types::typeobj::str_t::STR_T;
use crate::types::typeobj::bool_t::BOOL_T;

const fn i32_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, I32_T);
    unary!(h, signed default I32_T I32);
    arith_opr_num!(h, default I32_T I32);
    comp_opr_num!(h, default I32_T I32);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(I32_T),
                "str" => typecast_int!(I32 => str, x),
                "bool" => typecast_int!(I32 => bool, x),
                "i8" => typecast_int!(I32 => I8, x),
                "i16" => typecast_int!(I32 => I16, x),
                "i32" => x[0].to_owned(),
                "i64" => typecast_int!(I32 => I64, x),
                "i128" => typecast_int!(I32 => I128, x),
                "isize" => typecast_int!(I32 => Isize, x),
                "ibig" => typecast_int!(I32 => Ibig, x),
                "u8" => typecast_int!(I32 => U8, x),
                "u16" => typecast_int!(I32 => U16, x),
                "u32" => typecast_int!(I32 => U32, x),
                "u64" => typecast_int!(I32 => U64, x),
                "u128" => typecast_int!(I32 => U128, x),
                "usize" => typecast_int!(I32 => Usize, x),
                "ubig" => typecast_int!(I32 => Ubig, x),
                "f16" => typecast_int!(I32 => f16, x),
                "f32" => typecast_int!(I32 => f32, x),
                "f64" => typecast_int!(I32 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, I32_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref I32_T: Type = Type::Definition {
        name: Some("i32".into()),
        generics: vec![],
        implementations: i32_t(),
        inst_fields: HashMap::new(),
    };
}
