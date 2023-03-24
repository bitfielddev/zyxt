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
fn i128_t() -> BuiltinType {
    let mut h = HashMap::new();
    trace!("Initialising i128");
    h.insert("_default", Value::I128(0));
    concat(&mut h, &I128_T);
    unary_signed_default::<i128>(&mut h, &I128_T);
    arith_opr_default::<i128>(&mut h, &I128_T);
    comp_opr_default::<i128>(&mut h, &I128_T);

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&I128_T_VAL)),
            p if p == *STR_T_VAL => typecast_int!(i128 => str, x),
            p if p == *BOOL_T_VAL => typecast_int!(i128 => bool, x),
            p if p == *I8_T_VAL => typecast_int!(i128 => I8, x),
            p if p == *I16_T_VAL => typecast_int!(i128 => I16, x),
            p if p == *I32_T_VAL => typecast_int!(i128 => I32, x),
            p if p == *I64_T_VAL => typecast_int!(i128 => I64, x),
            p if p == *I128_T_VAL => x[0].to_owned(),
            p if p == *ISIZE_T_VAL => typecast_int!(i128 => Isize, x),
            p if p == *IBIG_T_VAL => typecast_int!(i128 => Ibig, x),
            p if p == *U8_T_VAL => typecast_int!(i128 => U8, x),
            p if p == *U16_T_VAL => typecast_int!(i128 => U16, x),
            p if p == *U32_T_VAL => typecast_int!(i128 => U32, x),
            p if p == *U64_T_VAL => typecast_int!(i128 => U64, x),
            p if p == *U128_T_VAL => typecast_int!(i128 => U128, x),
            p if p == *USIZE_T_VAL => typecast_int!(i128 => Usize, x),
            p if p == *UBIG_T_VAL => typecast_int!(i128 => Ubig, x),
            p if p == *F16_T_VAL => typecast_int!(i128 => f16, x),
            p if p == *F32_T_VAL => typecast_int!(i128 => f32, x),
            p if p == *F64_T_VAL => typecast_int!(i128 => f64, x),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, &I128_T);

    BuiltinType {
        name: Some(Ident::new("i128")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: HashMap::default(),
        type_args: vec![],
    }
}

pub static I128_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(i128_t().into()));
pub static I128_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(i128_t().into()));

use std::sync::Arc;

use crate::{
    ast::Ident,
    primitives::utils::{
        arith_opr_default, comp_opr_default, concat, get_param, type_cast, unary_signed_default,
    },
    types::r#type::{BuiltinType, ValueType},
};
