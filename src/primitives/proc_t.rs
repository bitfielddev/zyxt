use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    primitives::*,
    types::value::{Proc, Value},
    Type,
};

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn proc_t() -> BuiltinType {
    let mut h = HashMap::new();
    concat(&mut h, Arc::clone(&PROC_T));

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&PROC_T_VAL)),
            p if p == *STR_T_VAL => Value::Str(get_param::<Proc>(x, 0)?.to_string()),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, Arc::clone(&PROC_T));

    BuiltinType {
        name: Some(Ident::new("proc")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: Default::default(),
        type_args: vec![
            ("A".into(), Arc::clone(&UNIT_T)),
            ("R".into(), Arc::clone(&TYPE_T)),
        ],
    }
}

pub static PROC_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(proc_t().into()));
pub static PROC_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(proc_t().into()));

pub fn generic_proc(_args: Vec<Arc<Type>>, ret: Arc<Type>) -> Arc<Type> {
    Arc::new(Type::Generic {
        type_args: vec![
            ("A".into(), Either::Left(Value::Unit)),
            ("R".into(), Either::Right(ret)),
        ], // todo when vectors are out
        base: Arc::clone(&PROC_T),
    })
}

use std::sync::Arc;

use itertools::Either;

use crate::{
    ast::Ident,
    primitives::utils::{concat, get_param, type_cast},
    types::r#type::{BuiltinType, ValueType},
};
