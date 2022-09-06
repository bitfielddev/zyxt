use std::collections::HashMap;
use crate::{concat_vals, get_param, binary, Type, Value, typecast_to_type};
use crate::types::value::Proc;

pub const fn type_t() -> HashMap<&'static str, Proc> {
    let mut h = HashMap::new();
    concat_vals!(h, "str");

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!("type"),
                "str" => Value::Str(get_param!(x, 0, Type).to_string()),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, "type", "_typecast", ["type"], "_any", typecast);

    h
}
