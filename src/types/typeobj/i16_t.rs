use std::collections::HashMap;

use half::f16;

use crate::{arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, unary, Type, typecast_to_type};
use crate::types::value::{Proc, Value};
use lazy_static::lazy_static;
use crate::types::typeobj::type_t::TYPE_T;
use crate::types::typeobj::str_t::STR_T;
use crate::types::typeobj::bool_t::BOOL_T;

const fn i16_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, I16_T);
    unary!(h, signed default I16_T I16);
    arith_opr_num!(h, default I16_T I16);
    comp_opr_num!(h, default I16_T I16);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(I16_T),
                "str" => typecast_int!(I16 => str, x),
                "bool" => typecast_int!(I16 => bool, x),
                "i8" => typecast_int!(I16 => I8, x),
                "i16" => x[0].to_owned(),
                "i32" => typecast_int!(I16 => I32, x),
                "i64" => typecast_int!(I16 => I64, x),
                "i128" => typecast_int!(I16 => I128, x),
                "isize" => typecast_int!(I16 => Isize, x),
                "ibig" => typecast_int!(I16 => Ibig, x),
                "u8" => typecast_int!(I16 => U8, x),
                "u16" => typecast_int!(I16 => U16, x),
                "u32" => typecast_int!(I16 => U32, x),
                "u64" => typecast_int!(I16 => U64, x),
                "u128" => typecast_int!(I16 => U128, x),
                "usize" => typecast_int!(I16 => Usize, x),
                "ubig" => typecast_int!(I16 => Ubig, x),
                "f16" => typecast_int!(I16 => f16, x),
                "f32" => typecast_int!(I16 => f32, x),
                "f64" => typecast_int!(I16 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, I16_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref I16_T: Type = Type::Definition {
        name: Some("i16".into()),
        generics: vec![],
        implementations: i16_t(),
        inst_fields: HashMap::new(),
    };
}
