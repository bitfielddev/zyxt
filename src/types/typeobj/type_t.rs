use std::collections::HashMap;

use lazy_static::lazy_static;
use smol_str::SmolStr;

use crate::{binary, concat_vals, get_param, typecast_to_type, types::{
    typeobj::{bool_t::BOOL_T, str_t::STR_T},
    value::{Proc, Value},
}, Type, Element};

fn type_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::Type(Type::Any));
    concat_vals!(h, TYPE_T);
    binary!(h, TYPE_T, "_eq", [TYPE_T], BOOL_T, |x: &Vec<Value>| {
        Some(Value::Bool(
            get_param!(x, 0, Type) == get_param!(x, 1, Type),
        ))
    });
    binary!(h, TYPE_T, "_ne", [TYPE_T], BOOL_T, |x: &Vec<Value>| {
        Some(Value::Bool(
            get_param!(x, 0, Type) != get_param!(x, 1, Type),
        ))
    });

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == *TYPE_T => typecast_to_type!(TYPE_T),
            p if p == *STR_T => Value::Str(get_param!(x, 0, Type).to_string()),
            _ => return None,
        })
    };
    binary!(h, TYPE_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

lazy_static! {
    pub static ref TYPE_T: Type<Value> = Type::Definition {
        name: Some("{builtin}".into()),
        inst_name: Some("type".into()),
        generics: vec![],
        implementations: type_t(),
        inst_fields: HashMap::new(),
    };
    pub static ref TYPE_T_ELE: Type<Element> = TYPE_T.as_type_element();
}
