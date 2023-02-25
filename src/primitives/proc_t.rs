use std::collections::HashMap;

use maplit::hashmap;
use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    binary, concat_vals, get_param,
    primitives::*,
    typecast_to_type,
    types::{
        typeobj::TypeDefinition,
        value::{Proc, Value},
    },
    Type,
};

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn proc_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, PROC_T);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(PROC_T),
            p if p == STR_T.as_type() => Value::Str(get_param!(x, 0, Proc).to_string()),
            _ => return None,
        })
    };
    binary!(
        h,
        PROC_T.as_type(),
        "_typecast",
        [PROC_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

pub static PROC_T: Lazy<TypeDefinition<Value>> = Lazy::new(|| TypeDefinition {
    name: Some("{builtin proc}".into()),
    inst_name: Some("proc".into()),
    generics: vec![],
    implementations: proc_t(),
    inst_fields: hashmap! {SmolStr::from("is_fn") => (Box::new(BOOL_T.as_type()), None)},
});
