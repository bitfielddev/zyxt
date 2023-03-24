use std::collections::HashMap;

use half::f16;
use once_cell::sync::Lazy;
use smol_str::SmolStr;
use tracing::trace;

use crate::{
    primitives::*,
    typecast_int,
    types::value::{Proc, Value},
    Type,
};

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn isize_t() -> BuiltinType {
    let mut h = HashMap::new();
    trace!("Initialising isize");
    h.insert("_default", Value::Isize(0));
    concat(&mut h, &ISIZE_T);
    unary_signed_default::<isize>(&mut h, &ISIZE_T);
    arith_opr_default::<isize>(&mut h, &ISIZE_T);
    comp_opr_default::<isize>(&mut h, &ISIZE_T);

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&ISIZE_T_VAL)),
            p if p == *STR_T_VAL => typecast_int!(isize => str, x),
            p if p == *BOOL_T_VAL => typecast_int!(isize => bool, x),
            p if p == *I8_T_VAL => typecast_int!(isize => I8, x),
            p if p == *I16_T_VAL => typecast_int!(isize => I16, x),
            p if p == *I32_T_VAL => typecast_int!(isize => I32, x),
            p if p == *I64_T_VAL => typecast_int!(isize => I64, x),
            p if p == *I128_T_VAL => typecast_int!(isize => I128, x),
            p if p == *ISIZE_T_VAL => x[0].to_owned(),
            p if p == *IBIG_T_VAL => typecast_int!(isize => Ibig, x),
            p if p == *U8_T_VAL => typecast_int!(isize => U8, x),
            p if p == *U16_T_VAL => typecast_int!(isize => U16, x),
            p if p == *U32_T_VAL => typecast_int!(isize => U32, x),
            p if p == *U64_T_VAL => typecast_int!(isize => U64, x),
            p if p == *U128_T_VAL => typecast_int!(isize => U128, x),
            p if p == *USIZE_T_VAL => typecast_int!(isize => Usize, x),
            p if p == *UBIG_T_VAL => typecast_int!(isize => Ubig, x),
            p if p == *F16_T_VAL => typecast_int!(isize => f16, x),
            p if p == *F32_T_VAL => typecast_int!(isize => f32, x),
            p if p == *F64_T_VAL => typecast_int!(isize => f64, x),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, &ISIZE_T);

    BuiltinType {
        name: Some(Ident::new("isize")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: HashMap::default(),
        type_args: vec![],
    }
}

pub static ISIZE_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(isize_t().into()));
pub static ISIZE_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(isize_t().into()));

use std::sync::Arc;

use crate::{
    ast::Ident,
    primitives::utils::{
        arith_opr_default, comp_opr_default, concat, get_param, type_cast, unary_signed_default,
    },
    types::r#type::{BuiltinType, ValueType},
};
