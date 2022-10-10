use std::collections::HashMap;

use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    binary, concat_vals, get_param, typecast_to_type,
    types::{
        typeobj::{bool_t::BOOL_T, str_t::STR_T, TypeDefinition},
        value::{Proc, Value},
    },
    Element, Type,
};

fn type_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::Type(Type::Any));
    concat_vals!(h, TYPE_T);
    binary!(
        h,
        TYPE_T.as_type(),
        "_eq",
        [TYPE_T.as_type()],
        BOOL_T.as_type(),
        |x: &Vec<Value>| {
            Some(Value::Bool(
                get_param!(x, 0, Type) == get_param!(x, 1, Type),
            ))
        }
    );
    binary!(
        h,
        TYPE_T.as_type(),
        "_ne",
        [TYPE_T.as_type()],
        BOOL_T.as_type(),
        |x: &Vec<Value>| {
            Some(Value::Bool(
                get_param!(x, 0, Type) != get_param!(x, 1, Type),
            ))
        }
    );

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(TYPE_T),
            p if p == STR_T.as_type() => Value::Str(get_param!(x, 0, Type).to_string()),
            _ => return None,
        })
    };
    binary!(
        h,
        TYPE_T.as_type(),
        "_typecast",
        [TYPE_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

pub static TYPE_T: Lazy<TypeDefinition<Value>> = Lazy::new(|| TypeDefinition {
    name: Some("{builtin type}".into()),
    inst_name: Some("type".into()),
    generics: vec![],
    implementations: type_t(),
    inst_fields: HashMap::new(),
});
static TYPE_T_ELE: Lazy<TypeDefinition<Element>> = Lazy::new(|| TYPE_T.as_type_element());
