use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
    binary, concat_vals, get_param, typecast_to_type,
    types::{
        typeobj::{bool_t::BOOL_T, str_t::STR_T},
        value::{Proc, Value},
    },
    Type,
};

const fn type_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, TYPE_T);
    binary!(h, TYPE_T, "_eq", [TYPE_T], BOOL_T, |x: &Vec<Value>| {
            Some(Value::Bool(get_param!(x, 0, Type) == get_param!(x, 1, Type)))
    });
    binary!(h, TYPE_T, "_neq", [TYPE_T], BOOL_T, |x: &Vec<Value>| {
            Some(Value::Bool(get_param!(x, 0, Type) != get_param!(x, 1, Type)))
    });

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(TYPE_T),
                "str" => Value::Str(get_param!(x, 0, Type).to_string()),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, TYPE_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref TYPE_T: Type<Value> = Type::Definition {
        inst_name: Some("type".into()),
        generics: vec![],
        implementations: type_t(),
        inst_fields: HashMap::new(),
    };
}
