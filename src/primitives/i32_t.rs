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
fn i32_t() -> BuiltinType {
    let mut h = HashMap::new();
    trace!("Initialising i32");
    h.insert("_default", Value::I32(0));
    concat(&mut h, &I32_T);
    unary_signed_default::<i8>(&mut h, &I8_T);
    arith_opr_default::<i8>(&mut h, &I8_T);
    comp_opr_default::<i8>(&mut h, &I8_T);

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&I32_T_VAL)),
            p if p == *STR_T_VAL => typecast_int!(i32 => str, x),
            p if p == *BOOL_T_VAL => typecast_int!(i32 => bool, x),
            p if p == *I8_T_VAL => typecast_int!(i32 => I8, x),
            p if p == *I16_T_VAL => typecast_int!(i32 => I16, x),
            p if p == *I32_T_VAL => x[0].to_owned(),
            p if p == *I64_T_VAL => typecast_int!(i32 => I64, x),
            p if p == *I128_T_VAL => typecast_int!(i32 => I128, x),
            p if p == *ISIZE_T_VAL => typecast_int!(i32 => Isize, x),
            p if p == *IBIG_T_VAL => typecast_int!(i32 => Ibig, x),
            p if p == *U8_T_VAL => typecast_int!(i32 => U8, x),
            p if p == *U16_T_VAL => typecast_int!(i32 => U16, x),
            p if p == *U32_T_VAL => typecast_int!(i32 => U32, x),
            p if p == *U64_T_VAL => typecast_int!(i32 => U64, x),
            p if p == *U128_T_VAL => typecast_int!(i32 => U128, x),
            p if p == *USIZE_T_VAL => typecast_int!(i32 => Usize, x),
            p if p == *UBIG_T_VAL => typecast_int!(i32 => Ubig, x),
            p if p == *F16_T_VAL => typecast_int!(i32 => f16, x),
            p if p == *F32_T_VAL => typecast_int!(i32 => f32, x),
            p if p == *F64_T_VAL => typecast_int!(i32 => f64, x),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, &I32_T);

    BuiltinType {
        name: Some(Ident::new("i32")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: HashMap::default(),
        type_args: vec![],
    }
}

pub static I32_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(i32_t().into()));
pub static I32_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(i32_t().into()));

use std::sync::Arc;

use crate::{
    ast::Ident,
    primitives::utils::{
        arith_opr_default, binary, comp_opr_default, concat, get_param, type_cast,
        unary_signed_default,
    },
    types::r#type::{BuiltinType, ValueType},
};
