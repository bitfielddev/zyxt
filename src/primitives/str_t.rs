use std::collections::HashMap;

use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    primitives::*,
    types::value::{Proc, Value},
    Type,
};

macro_rules! typecast_str_to_num {
    ($v:ident, $x:ident) => {
        Value::$v(get_param::<String>($x, 0)?.parse().ok()?)
    };
}

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn str_t() -> BuiltinType {
    let mut h = HashMap::new();
    h.insert("_default", Value::Str(String::new()));
    concat(&mut h, Arc::clone(&STR_T));

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&STR_T_VAL)),
            p if p == *STR_T_VAL => x[0].to_owned(),
            p if p == *BOOL_T_VAL => Value::Bool(get_param::<String>(x, 0)?.is_empty()),
            p if p == *I8_T_VAL => typecast_str_to_num!(I8, x),
            p if p == *I16_T_VAL => typecast_str_to_num!(I16, x),
            p if p == *I32_T_VAL => typecast_str_to_num!(I32, x),
            p if p == *I64_T_VAL => typecast_str_to_num!(I64, x),
            p if p == *I128_T_VAL => typecast_str_to_num!(I128, x),
            p if p == *ISIZE_T_VAL => typecast_str_to_num!(Isize, x),
            p if p == *IBIG_T_VAL => typecast_str_to_num!(Ibig, x),
            p if p == *U8_T_VAL => typecast_str_to_num!(U8, x),
            p if p == *U16_T_VAL => typecast_str_to_num!(U16, x),
            p if p == *U32_T_VAL => typecast_str_to_num!(U32, x),
            p if p == *U64_T_VAL => typecast_str_to_num!(U64, x),
            p if p == *U128_T_VAL => typecast_str_to_num!(U128, x),
            p if p == *USIZE_T_VAL => typecast_str_to_num!(Usize, x),
            p if p == *UBIG_T_VAL => typecast_str_to_num!(Ubig, x),
            p if p == *F16_T_VAL => typecast_str_to_num!(F16, x),
            p if p == *F32_T_VAL => typecast_str_to_num!(F32, x),
            p if p == *F64_T_VAL => typecast_str_to_num!(F64, x),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, Arc::clone(&STR_T));
    binary(
        &mut h,
        "_mul",
        Arc::new(|x: &Vec<Value>| {
            Some(Value::Str(get_param::<String>(x, 0)?.repeat(get_param::<
                usize,
            >(
                x, 1
            )?)))
        }),
        Arc::clone(&STR_T),
        Arc::clone(&USIZE_T),
        Arc::clone(&STR_T),
    );

    BuiltinType {
        name: Some(Ident::new("str")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: Default::default(),
        type_args: vec![],
    }
}

pub static STR_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(str_t().into()));
pub static STR_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(str_t().into()));

use std::sync::Arc;

use crate::{
    ast::Ident,
    primitives::utils::{binary, concat, get_param, type_cast},
    types::r#type::{BuiltinType, ValueType},
};
