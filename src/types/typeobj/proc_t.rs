use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

use crate::{
    binary, concat_vals, get_param, typecast_to_type,
    types::{
        typeobj::str_t::STR_T, typeobj::bool_t::BOOL_T,
        value::{Proc, Value},
    },
    Type,
};

const fn proc_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, PROC_T);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(PROC_T),
                "str" => Value::Str(get_param!(x, 0, Proc).to_string()),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, PROC_T, "_typecast", [PROC_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref PROC_T: Type<Value> = Type::Definition {
        inst_name: Some("proc".into()),
        generics: vec![],
        implementations: proc_t(),
        inst_fields: hashmap!{"is_fn".into() => (Box::new(BOOL_T), None)},
    };
}
