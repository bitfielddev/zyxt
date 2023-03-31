use std::collections::HashMap;

use half::f16;
use num_traits::ToPrimitive;
use once_cell::sync::Lazy;
use tracing::trace;

use crate::{primitives::*, typecast_int, types::value::Value, Type};
#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn ubig_t() -> BuiltinType {
    let mut h = HashMap::new();
    trace!("Initialising ubig");
    h.insert("_default", Value::Ubig(0u8.into()));
    concat(&mut h, &UBIG_T);
    unary_unsigned_default::<BigUint>(&mut h, &UBIG_T);
    arith_opr::<BigUint>(&mut h, "_rem", &std::ops::Rem::rem, &IBIG_T);
    arith_opr_big_default::<BigUint>(&mut h, &UBIG_T);
    comp_opr_default::<BigUint>(&mut h, &UBIG_T);

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&UBIG_T_VAL)),
            p if p == *STR_T_VAL => typecast_int!(BigUint => str, x),
            p if p == *BOOL_T_VAL => Value::Bool(get_param::<BigUint>(x, 0)? == 0u8.into()),
            p if p == *I8_T_VAL => typecast_int!(BigUint => I8, x),
            p if p == *I16_T_VAL => typecast_int!(BigUint => I16, x),
            p if p == *I32_T_VAL => typecast_int!(BigUint => I32, x),
            p if p == *I64_T_VAL => typecast_int!(BigUint => I64, x),
            p if p == *I128_T_VAL => typecast_int!(BigUint => I128, x),
            p if p == *ISIZE_T_VAL => typecast_int!(BigUint => Ubig, x),
            p if p == *IBIG_T_VAL => typecast_int!(BigUint => Ibig, x),
            p if p == *U8_T_VAL => typecast_int!(BigUint => U8, x),
            p if p == *U16_T_VAL => typecast_int!(BigUint => U16, x),
            p if p == *U32_T_VAL => typecast_int!(BigUint => U32, x),
            p if p == *U64_T_VAL => typecast_int!(BigUint => U64, x),
            p if p == *U128_T_VAL => typecast_int!(BigUint => U128, x),
            p if p == *USIZE_T_VAL => typecast_int!(BigUint => Usize, x),
            p if p == *UBIG_T_VAL => x[0].to_owned(),
            p if p == *F16_T_VAL => typecast_int!(big BigUint => f16, x),
            p if p == *F32_T_VAL => typecast_int!(big BigUint => f32, x),
            p if p == *F64_T_VAL => typecast_int!(big BigUint => f64, x),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, &UBIG_T);

    BuiltinType {
        name: Some(Ident::new("ubig")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: HashMap::default(),
        type_args: vec![],
    }
}

pub static UBIG_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(ubig_t().into()));
pub static UBIG_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(ubig_t().into()));

use std::sync::Arc;

use num::BigUint;

use crate::{
    ast::Ident,
    primitives::utils::{
        arith_opr, arith_opr_big_default, comp_opr_default, concat, get_param, type_cast,
        unary_unsigned_default,
    },
    types::r#type::{BuiltinType, ValueType},
};
