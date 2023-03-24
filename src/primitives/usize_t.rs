use std::collections::HashMap;

use half::f16;
use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    primitives::*,
    typecast_int,
    types::value::{Proc, Value},
    Type,
};

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn usize_t() -> BuiltinType {
    let mut h = HashMap::new();
    h.insert("_default", Value::Usize(0));
    concat(&mut h, Arc::clone(&USIZE_T));
    unary_unsigned_default::<usize>(&mut h, Arc::clone(&USIZE_T));
    arith_opr_default::<usize>(&mut h, Arc::clone(&USIZE_T));
    comp_opr_default::<usize>(&mut h, Arc::clone(&USIZE_T));

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&USIZE_T_VAL)),
            p if p == *STR_T_VAL => typecast_int!(usize => str, x),
            p if p == *BOOL_T_VAL => typecast_int!(usize => bool, x),
            p if p == *I8_T_VAL => typecast_int!(usize => I8, x),
            p if p == *I16_T_VAL => typecast_int!(usize => I16, x),
            p if p == *I32_T_VAL => typecast_int!(usize => I32, x),
            p if p == *I64_T_VAL => typecast_int!(usize => I64, x),
            p if p == *I128_T_VAL => typecast_int!(usize => I128, x),
            p if p == *ISIZE_T_VAL => typecast_int!(usize => Isize, x),
            p if p == *IBIG_T_VAL => typecast_int!(usize => Ibig, x),
            p if p == *U8_T_VAL => typecast_int!(usize => U8, x),
            p if p == *U16_T_VAL => typecast_int!(usize => U16, x),
            p if p == *U32_T_VAL => typecast_int!(usize => U32, x),
            p if p == *U64_T_VAL => typecast_int!(usize => U64, x),
            p if p == *U128_T_VAL => typecast_int!(usize => U128, x),
            p if p == *USIZE_T_VAL => x[0].to_owned(),
            p if p == *UBIG_T_VAL => typecast_int!(usize => Ubig, x),
            p if p == *F16_T_VAL => typecast_int!(usize => f16, x),
            p if p == *F32_T_VAL => typecast_int!(usize => f32, x),
            p if p == *F64_T_VAL => typecast_int!(usize => f64, x),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, Arc::clone(&USIZE_T));

    BuiltinType {
        name: Some(Ident::new("usize")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: Default::default(),
        type_args: vec![],
    }
}

pub static USIZE_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(usize_t().into()));
pub static USIZE_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(usize_t().into()));

use std::sync::Arc;

use crate::{
    ast::Ident,
    primitives::utils::{
        arith_opr_default, comp_opr_default, concat, get_param, type_cast, unary_unsigned_default,
    },
    types::r#type::{BuiltinType, ValueType},
};
