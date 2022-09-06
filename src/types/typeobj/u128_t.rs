use std::collections::HashMap;

use half::f16;

use crate::{arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, unary, Type, typecast_to_type};
use crate::types::value::{Proc, Value};
use lazy_static::lazy_static;
use crate::types::typeobj::type_t::TYPE_T;
use crate::types::typeobj::str_t::STR_T;
use crate::types::typeobj::bool_t::BOOL_T;

const fn u128_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, U128_T);
    unary!(h, signed default U128_T U128);
    arith_opr_num!(h, default U128_T U128);
    comp_opr_num!(h, default U128_T U128);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(U128_T),
                "str" => typecast_int!(U128 => str, x),
                "bool" => typecast_int!(U128 => bool, x),
                "i8" => typecast_int!(U128 => I8, x),
                "i16" => typecast_int!(U128 => I16, x),
                "i32" => typecast_int!(U128 => I32, x),
                "i64" => typecast_int!(U128 => I64, x),
                "i128" => typecast_int!(U128 => I128, x),
                "isize" => typecast_int!(U128 => Isize, x),
                "ibig" => typecast_int!(U128 => Ibig, x),
                "u8" => typecast_int!(U128 => U8, x),
                "u16" => typecast_int!(U128 => U16, x),
                "u32" => typecast_int!(U128 => U32, x),
                "u64" => typecast_int!(U128 => U64, x),
                "u128" => x[0].to_owned(),
                "usize" => typecast_int!(U128 => Usize, x),
                "ubig" => typecast_int!(U128 => Ubig, x),
                "f16" => typecast_int!(U128 => f16, x),
                "f32" => typecast_int!(U128 => f32, x),
                "f64" => typecast_int!(U128 => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, U128_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref U128_T: Type = Type::Definition {
        name: Some("u128".into()),
        generics: vec![],
        implementations: u128_t(),
        inst_fields: HashMap::new(),
    };
}
