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

const fn u16_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, U16_T);
    unary!(h, signed default U16_T U16);
    arith_opr_num!(h, default U16_T U16);
    comp_opr_num!(h, default U16_T U16);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(U16_T),
                "str" => typecast_int!(U16 => str, x),
                "bool" => typecast_int!(U16 => bool, x),
                "i8" => typecast_int!(U16 => I8, x),
                "i16" => typecast_int!(U16 => I16, x),
                "i32" => typecast_int!(U16 => I32, x),
                "i64" => typecast_int!(U16 => I64, x),
                "i128" => typecast_int!(U16 => I128, x),
                "isize" => typecast_int!(U16 => Isize, x),
                "ibig" => typecast_int!(U16 => Ibig, x),
                "u8" => typecast_int!(U16 => U8, x),
                "u16" => x[0].to_owned(),
                "u32" => typecast_int!(U16 => U32, x),
                "u64" => typecast_int!(U16 => U64, x),
                "u128" => typecast_int!(U16 => U128, x),
                "usize" => typecast_int!(U16 => Usize, x),
                "ubig" => typecast_int!(U16 => Ubig, x),
                "f16" => typecast_int!(U16 => f16, x),
                "f32" => typecast_int!(U16 => f32, x),
                "f64" => typecast_int!(U16 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, U16_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref U16_T: Type = Type::Definition {
        name: Some("u16".into()),
        generics: vec![],
        implementations: u16_t(),
        inst_fields: HashMap::new(),
    };
}
