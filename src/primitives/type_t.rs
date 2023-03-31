use std::collections::HashMap;

use once_cell::sync::Lazy;
use tracing::trace;

use crate::{primitives::*, types::value::Value, Type};
#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn type_t() -> BuiltinType {
    let mut h = HashMap::new();
    trace!("Initialising type");
    h.insert("_default", Value::Type(Arc::clone(&ANY_T_VAL)));
    concat(&mut h, &TYPE_T);
    binary(
        &mut h,
        "_eq",
        Arc::new(|x: &Vec<Value>| {
            Some(Value::Bool(
                get_param::<Arc<ValueType>>(x, 0)? == get_param::<Arc<ValueType>>(x, 1)?,
            ))
        }),
        &TYPE_T,
        &TYPE_T,
        &BOOL_T,
    );
    binary(
        &mut h,
        "_ne",
        Arc::new(|x: &Vec<Value>| {
            Some(Value::Bool(
                get_param::<Arc<ValueType>>(x, 0)? != get_param::<Arc<ValueType>>(x, 1)?,
            ))
        }),
        &TYPE_T,
        &TYPE_T,
        &BOOL_T,
    );

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&TYPE_T_VAL)),
            p if p == *STR_T_VAL => Value::Str(get_param::<Arc<ValueType>>(x, 0)?.to_string()),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, &TYPE_T);

    BuiltinType {
        name: Some(Ident::new("type")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: HashMap::default(),
        type_args: vec![],
    }
}

pub static TYPE_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(type_t().into()));
pub static TYPE_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(type_t().into()));

use std::sync::Arc;

use crate::{
    ast::Ident,
    primitives::utils::{binary, concat, get_param, type_cast},
    types::r#type::{BuiltinType, ValueType},
};
