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

const fn u32_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, U32_T);
    unary!(h, signed default U32_T U32);
    arith_opr_num!(h, default U32_T U32);
    comp_opr_num!(h, default U32_T U32);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(U32_T),
                "str" => typecast_int!(U32 => str, x),
                "bool" => typecast_int!(U32 => bool, x),
                "i8" => typecast_int!(U32 => I8, x),
                "i16" => typecast_int!(U32 => I16, x),
                "i32" => typecast_int!(U32 => I32, x),
                "i64" => typecast_int!(U32 => I64, x),
                "i128" => typecast_int!(U32 => I128, x),
                "isize" => typecast_int!(U32 => Isize, x),
                "ibig" => typecast_int!(U32 => Ibig, x),
                "u8" => typecast_int!(U32 => U8, x),
                "u16" => typecast_int!(U32 => U16, x),
                "u32" => x[0].to_owned(),
                "u64" => typecast_int!(U32 => U64, x),
                "u128" => typecast_int!(U32 => U128, x),
                "usize" => typecast_int!(U32 => Usize, x),
                "ubig" => typecast_int!(U32 => Ubig, x),
                "f16" => typecast_int!(U32 => f16, x),
                "f32" => typecast_int!(U32 => f32, x),
                "f64" => typecast_int!(U32 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, U32_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref U32_T: Type<Value> = Type::Definition {
        inst_name: Some("u32".into()),
        generics: vec![],
        implementations: u32_t(),
        inst_fields: HashMap::new(),
    };
}
