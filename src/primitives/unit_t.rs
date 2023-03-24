use std::collections::HashMap;

use once_cell::sync::Lazy;
use smol_str::SmolStr;
use tracing::trace;

use crate::{
    primitives::*,
    types::value::{Proc, Value},
    Ast, Type,
};
fn comp_opr_unit<'a>(h: &mut HashMap<&'a str, Value>, n: &'a str, res: bool) {
    binary(
        h,
        n,
        Arc::new(move |_| Some(res.into())),
        &UNIT_T,
        &UNIT_T,
        &BOOL_T,
    );
}

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
#[allow(unused_variables)]
fn unit_t() -> BuiltinType {
    let mut h = HashMap::new();
    trace!("Initialising unit");
    h.insert("_default", Value::Unit);
    concat(&mut h, &UNIT_T);
    comp_opr_unit(&mut h, "_eq", true);
    comp_opr_unit(&mut h, "_ne", false);
    comp_opr_unit(&mut h, "_gt", false);
    comp_opr_unit(&mut h, "_ge", true);
    comp_opr_unit(&mut h, "_lt", false);
    comp_opr_unit(&mut h, "_le", true);

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&UNIT_T_VAL)),
            p if p == *STR_T_VAL => Value::Str("()".into()),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, &UNIT_T);

    BuiltinType {
        name: Some(Ident::new("unit")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: HashMap::default(),
        type_args: vec![],
    }
}

pub static UNIT_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(unit_t().into()));
pub static UNIT_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(unit_t().into()));

use std::sync::Arc;

use crate::{
    ast::Ident,
    primitives::utils::{binary, concat, get_param, type_cast},
    types::r#type::{BuiltinType, ValueType},
};
