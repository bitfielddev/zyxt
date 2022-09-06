use std::{collections::HashMap, ops::Rem};

use half::f16;
use lazy_static::lazy_static;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, ToPrimitive};

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, typecast_to_type,
    types::{
        typeobj::{bool_t::BOOL_T, str_t::STR_T, type_t::TYPE_T},
        value::{Proc, Value},
    },
    unary, Type,
};

const fn ubig_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, UBIG_T);

    unary!(h, UBIG_T, "_un_add", |x: &Vec<Value>| Some(x[0].to_owned()));
    unary!(h, UBIG_T, "_not", |x: &Vec<Value>| Some(Value::Bool(
        get_param!(x, 0, Ubig) == 0u8.into()
    )));

    arith_opr_num!(h, big default UBIG_T Ubig);
    comp_opr_num!(h, default UBIG_T Ubig);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(UBIG_T),
                "str" => typecast_int!(Ubig => str, x),
                "bool" => typecast_int!(Ubig => into bool, x),
                "i8" => typecast_int!(Ubig => I8, x),
                "i16" => typecast_int!(Ubig => I16, x),
                "i32" => typecast_int!(Ubig => I32, x),
                "i164" => typecast_int!(Ubig => I64, x),
                "i128" => typecast_int!(Ubig => I128, x),
                "isize" => typecast_int!(Ubig => Ubig, x),
                "ibig" => typecast_int!(Ubig => Ibig, x),
                "u8" => typecast_int!(Ubig => U8, x),
                "u16" => typecast_int!(Ubig => U16, x),
                "u32" => typecast_int!(Ubig => U32, x),
                "u64" => typecast_int!(Ubig => U64, x),
                "u128" => typecast_int!(Ubig => U128, x),
                "usize" => typecast_int!(Ubig => Usize, x),
                "ubig" => x[0].to_owned(),
                "f16" => typecast_int!(big Ubig => f16, x),
                "f32" => typecast_int!(big Ubig => f32, x),
                "f64" => typecast_int!(big Ubig => f64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, UBIG_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref UBIG_T: Type<Value> = Type::Definition {
        inst_name: Some("ubig".into()),
        generics: vec![],
        implementations: ubig_t(),
        inst_fields: HashMap::new(),
    };
}
