use std::collections::HashMap;

use lazy_static::lazy_static;
use smol_str::SmolStr;
use maplit::hashmap;

use crate::{
    binary, concat_vals, get_param, typecast_to_type,
    types::{
        typeobj::{str_t::STR_T, type_t::TYPE_T},
        value::{Proc, Value},
    },
    Type,
};

fn proc_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, PROC_T);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == *TYPE_T => typecast_to_type!(PROC_T),
            p if p == *STR_T => Value::Str(get_param!(x, 0, Proc).to_string()),
            _ => return None,
        })
    };
    binary!(h, PROC_T, "_typecast", [PROC_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k.into(), Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref PROC_T: Type<Value> = Type::Definition {
        name: Some("{builtin}".into()),
        inst_name: Some("proc".into()),
        generics: vec![],
        implementations: proc_t(),
        inst_fields: hashmap! {SmolStr::from("is_fn") => (Box::new(BOOL_T), None)},
    };
}
