use std::collections::HashMap;

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
    Element, Type,
};

macro_rules! comp_opr_unit {
    ($h:ident, $fn_name:literal, $res:literal) => {
        binary!(
            $h,
            UNIT_T.as_type(),
            $fn_name,
            [UNIT_T.as_type()],
            BOOL_T.as_type(),
            |x: &Vec<Value>| { Some(Value::Bool($res)) }
        );
    };
}

#[allow(unused_variables)]
fn unit_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::Unit);
    concat_vals!(h, UNIT_T);
    comp_opr_unit!(h, "_eq", true);
    comp_opr_unit!(h, "_ne", false);
    comp_opr_unit!(h, "_gt", false);
    comp_opr_unit!(h, "_ge", true);
    comp_opr_unit!(h, "_lt", false);
    comp_opr_unit!(h, "_le", true);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(UNIT_T),
            p if p == STR_T.as_type() => Value::Str("()".into()),
            _ => return None,
        })
    };
    binary!(
        h,
        UNIT_T.as_type(),
        "_typecast",
        [TYPE_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

pub static UNIT_T: Lazy<TypeDefinition<Value>> = Lazy::new(|| TypeDefinition {
    name: Some("{builtin unit}".into()),
    inst_name: Some("_unit".into()),
    generics: vec![],
    implementations: unit_t(),
    inst_fields: HashMap::new(),
});
pub static UNIT_T_ELE: Lazy<TypeDefinition<Element>> = Lazy::new(|| UNIT_T.as_type_element());
