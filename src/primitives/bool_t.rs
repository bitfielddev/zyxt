use std::collections::HashMap;

use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    binary, comp_opr_num, concat_vals, get_param,
    primitives::*,
    typecast_to_type,
    types::{
        typeobj::TypeDefinition,
        value::{Proc, Value},
    },
    Type,
};

macro_rules! typecast_bool_to_num {
    ($v:ident $v2:ty, $x:ident) => {
        Value::$v(get_param!($x, 0, Bool) as $v2)
    };
    ($v:ident, $x:ident) => {
        Value::$v(if get_param!($x, 0, Bool) { 1u8 } else { 0u8 }.into())
    };
}

fn bool_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::Bool(false));
    concat_vals!(h, BOOL_T);
    comp_opr_num!(h, default BOOL_T Bool);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(BOOL_T),
            p if p == STR_T.as_type() => Value::Str(get_param!(x, 0, Bool).to_string()),
            p if p == BOOL_T.as_type() => x[0].to_owned(),
            p if p == I8_T.as_type() => Value::I8(get_param!(x, 0, Bool).into()),
            p if p == I16_T.as_type() => typecast_bool_to_num!(I16, x),
            p if p == I32_T.as_type() => typecast_bool_to_num!(I32, x),
            p if p == I64_T.as_type() => typecast_bool_to_num!(I64, x),
            p if p == I128_T.as_type() => typecast_bool_to_num!(I128, x),
            p if p == ISIZE_T.as_type() => typecast_bool_to_num!(Isize, x),
            p if p == IBIG_T.as_type() => typecast_bool_to_num!(Ibig, x),
            p if p == U8_T.as_type() => typecast_bool_to_num!(U8, x),
            p if p == U16_T.as_type() => typecast_bool_to_num!(U16, x),
            p if p == U32_T.as_type() => typecast_bool_to_num!(U32, x),
            p if p == U64_T.as_type() => typecast_bool_to_num!(U64, x),
            p if p == U128_T.as_type() => typecast_bool_to_num!(U128, x),
            p if p == USIZE_T.as_type() => typecast_bool_to_num!(Usize, x),
            p if p == UBIG_T.as_type() => typecast_bool_to_num!(Ubig, x),
            p if p == F16_T.as_type() => typecast_bool_to_num!(F16, x),
            p if p == F32_T.as_type() => typecast_bool_to_num!(F32, x),
            p if p == F64_T.as_type() => typecast_bool_to_num!(F64, x),
            _ => return None,
        })
    };
    binary!(
        h,
        BOOL_T.as_type(),
        "_typecast",
        [TYPE_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

pub static BOOL_T: Lazy<TypeDefinition<Value>> = Lazy::new(|| TypeDefinition {
    name: Some("{builtin bool}".into()),
    inst_name: Some("bool".into()),
    generics: vec![],
    implementations: bool_t(),
    inst_fields: HashMap::new(),
});
