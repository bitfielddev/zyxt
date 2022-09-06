use std::collections::HashMap;

use half::f16;

use crate::{arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, unary, Type, typecast_to_type};
use crate::types::value::{Proc, Value};
use lazy_static::lazy_static;
use crate::types::typeobj::type_t::TYPE_T;
use crate::types::typeobj::str_t::STR_T;
use crate::types::typeobj::bool_t::BOOL_T;

const fn isize_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, ISIZE_T);
    unary!(h, signed default ISIZE_T Isize);
    arith_opr_num!(h, default ISIZE_T Isize);
    comp_opr_num!(h, default ISIZE_T Isize);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(ISIZE_T),
                "str" => typecast_int!(Isize => str, x),
                "bool" => typecast_int!(Isize => bool, x),
                "i8" => typecast_int!(Isize => I8, x),
                "i16" => typecast_int!(Isize => I16, x),
                "i32" => typecast_int!(Isize => I32, x),
                "i164" => typecast_int!(Isize => I64, x),
                "i128" => typecast_int!(Isize => I128, x),
                "isize" => x[0].to_owned(),
                "ibig" => typecast_int!(Isize => Ibig, x),
                "u8" => typecast_int!(Isize => U8, x),
                "u16" => typecast_int!(Isize => U16, x),
                "u32" => typecast_int!(Isize => U32, x),
                "u64" => typecast_int!(Isize => U64, x),
                "u128" => typecast_int!(Isize => U128, x),
                "usize" => typecast_int!(Isize => Usize, x),
                "ubig" => typecast_int!(Isize => Ubig, x),
                "f16" => typecast_int!(Isize => f16, x),
                "f32" => typecast_int!(Isize => f32, x),
                "f64" => typecast_int!(Isize => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, ISIZE_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref ISIZE_T: Type = Type::Definition {
        name: Some("isize".into()),
        generics: vec![],
        implementations: isize_t(),
        inst_fields: HashMap::new(),
    };
}
