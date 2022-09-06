use std::collections::HashMap;

use half::f16;

use crate::{arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, unary, Type, typecast_to_type};
use crate::types::value::{Proc, Value};
use lazy_static::lazy_static;
use crate::types::typeobj::type_t::TYPE_T;
use crate::types::typeobj::str_t::STR_T;
use crate::types::typeobj::bool_t::BOOL_T;

const fn u8_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, U8_T);
    unary!(h, signed default U8_T U8);
    arith_opr_num!(h, default U8_T U8);
    comp_opr_num!(h, default U8_T U8);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(U8_T),
                "str" => typecast_int!(U8 => str, x),
                "bool" => typecast_int!(U8 => bool, x),
                "i8" => typecast_int!(U8 => I8, x),
                "i16" => typecast_int!(U8 => I16, x),
                "i32" => typecast_int!(U8 => I32, x),
                "i64" => typecast_int!(U8 => I64, x),
                "i128" => typecast_int!(U8 => I128, x),
                "isize" => typecast_int!(U8 => Isize, x),
                "ibig" => typecast_int!(U8 => Ibig, x),
                "u8" => x[0].to_owned(),
                "u16" => typecast_int!(U8 => U16, x),
                "u32" => typecast_int!(U8 => U32, x),
                "u64" => typecast_int!(U8 => U64, x),
                "u128" => typecast_int!(U8 => U128, x),
                "usize" => typecast_int!(U8 => Usize, x),
                "ubig" => typecast_int!(U8 => Ubig, x),
                "f16" => typecast_int!(U8 => f16, x),
                "f32" => typecast_int!(U8 => f32, x),
                "f64" => typecast_int!(U8 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, U8_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref U8_T: Type = Type::Definition {
        name: Some("u8".into()),
        generics: vec![],
        implementations: u8_t(),
        inst_fields: HashMap::new(),
    };
}
