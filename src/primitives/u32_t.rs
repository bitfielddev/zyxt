use std::collections::HashMap;

use half::f16;
use once_cell::sync::Lazy;
use tracing::trace;

use crate::{primitives::*, typecast_int, types::value::Value, Type};
#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn u32_t() -> BuiltinType {
    let mut h = HashMap::new();
    trace!("Initialising u32");
    h.insert("_default", Value::U32(0));
    concat(&mut h, &U32_T);
    unary_unsigned_default::<u32>(&mut h, &U32_T);
    arith_opr_default::<u32>(&mut h, &U32_T);
    comp_opr_default::<u32>(&mut h, &U32_T);

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&U32_T_VAL)),
            p if p == *STR_T_VAL => typecast_int!(u32 => str, x),
            p if p == *BOOL_T_VAL => typecast_int!(u32 => bool, x),
            p if p == *I8_T_VAL => typecast_int!(u32 => I8, x),
            p if p == *I16_T_VAL => typecast_int!(u32 => I16, x),
            p if p == *I32_T_VAL => typecast_int!(u32 => I32, x),
            p if p == *I64_T_VAL => typecast_int!(u32 => I64, x),
            p if p == *I128_T_VAL => typecast_int!(u32 => I128, x),
            p if p == *ISIZE_T_VAL => typecast_int!(u32 => Isize, x),
            p if p == *IBIG_T_VAL => typecast_int!(u32 => Ibig, x),
            p if p == *U8_T_VAL => typecast_int!(u32 => U8, x),
            p if p == *U16_T_VAL => typecast_int!(u32 => U16, x),
            p if p == *U32_T_VAL => x[0].to_owned(),
            p if p == *U64_T_VAL => typecast_int!(u32 => U64, x),
            p if p == *U128_T_VAL => typecast_int!(u32 => U128, x),
            p if p == *USIZE_T_VAL => typecast_int!(u32 => Usize, x),
            p if p == *UBIG_T_VAL => typecast_int!(u32 => Ubig, x),
            p if p == *F16_T_VAL => typecast_int!(u32 => f16, x),
            p if p == *F32_T_VAL => typecast_int!(u32 => f32, x),
            p if p == *F64_T_VAL => typecast_int!(u32 => f64, x),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, &U32_T);

    BuiltinType {
        name: Some(Ident::new("u32")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: HashMap::default(),
        type_args: vec![],
    }
}

pub static U32_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(u32_t().into()));
pub static U32_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(u32_t().into()));

use std::sync::Arc;

use crate::{
    ast::Ident,
    primitives::utils::{
        arith_opr_default, comp_opr_default, concat, get_param, type_cast, unary_unsigned_default,
    },
    types::r#type::{BuiltinType, ValueType},
};
