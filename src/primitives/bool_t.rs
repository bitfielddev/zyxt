use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    ast::Ident,
    primitives::{
        utils::{comp_opr_default, concat, get_param, type_cast},
        *,
    },
    types::{
        r#type::{BuiltinType, ValueType},
        value::{Proc, Value},
    },
    Type,
};

macro_rules! typecast_bool_to_num {
    ($v:ident $v2:ty, $x:ident) => {
        Value::$v(get_param::<bool>($x, 0)? as $v2)
    };
    ($v:ident, $x:ident) => {
        Value::$v(if get_param::<bool>($x, 0)? { 1u8 } else { 0u8 }.into())
    };
}

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn bool_t() -> BuiltinType {
    let mut h = HashMap::new();
    h.insert("_default", Value::Bool(false));
    concat(&mut h, Arc::clone(&BOOL_T));
    comp_opr_default::<bool>(&mut h, Arc::clone(&BOOL_T));

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&BOOL_T_VAL)),
            p if p == *STR_T_VAL => Value::Str(get_param::<bool>(x, 0)?.to_string()),
            p if p == *BOOL_T_VAL => x[0].to_owned(),
            p if p == *I8_T_VAL => Value::I8(get_param::<bool>(x, 0)?.into()),
            p if p == *I16_T_VAL => typecast_bool_to_num!(I16, x),
            p if p == *I32_T_VAL => typecast_bool_to_num!(I32, x),
            p if p == *I64_T_VAL => typecast_bool_to_num!(I64, x),
            p if p == *I128_T_VAL => typecast_bool_to_num!(I128, x),
            p if p == *ISIZE_T_VAL => typecast_bool_to_num!(Isize, x),
            p if p == *IBIG_T_VAL => typecast_bool_to_num!(Ibig, x),
            p if p == *U8_T_VAL => typecast_bool_to_num!(U8, x),
            p if p == *U16_T_VAL => typecast_bool_to_num!(U16, x),
            p if p == *U32_T_VAL => typecast_bool_to_num!(U32, x),
            p if p == *U64_T_VAL => typecast_bool_to_num!(U64, x),
            p if p == *U128_T_VAL => typecast_bool_to_num!(U128, x),
            p if p == *USIZE_T_VAL => typecast_bool_to_num!(Usize, x),
            p if p == *UBIG_T_VAL => typecast_bool_to_num!(Ubig, x),
            p if p == *F16_T_VAL => typecast_bool_to_num!(F16, x),
            p if p == *F32_T_VAL => typecast_bool_to_num!(F32, x),
            p if p == *F64_T_VAL => typecast_bool_to_num!(F64, x),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, Arc::clone(&BOOL_T));

    BuiltinType {
        name: Some(Ident::new("bool")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: Default::default(),
        type_args: vec![],
    }
}

pub static BOOL_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(bool_t().into()));
pub static BOOL_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(bool_t().into()));
