use std::{collections::HashMap, ops::Neg};

use half::f16;
use num_traits::{ToPrimitive, Zero};
use once_cell::sync::Lazy;
use tracing::trace;

use crate::{primitives::*, typecast_int, types::value::Value, Type};

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn ibig_t() -> BuiltinType {
    let mut h = HashMap::new();
    trace!("Initialising ibig");
    h.insert("_default", Value::Ibig(0.into()));
    concat(&mut h, &IBIG_T);
    unary(
        &mut h,
        "_un_add",
        Arc::new(|x: &Vec<Value>| Some(x[0].to_owned())),
        &IBIG_T,
        &IBIG_T,
    );
    unary(
        &mut h,
        "_un_sub",
        Arc::new(|x: &Vec<Value>| Some(get_param::<BigInt>(x, 0)?.neg().into())),
        &IBIG_T,
        &IBIG_T,
    );
    unary(
        &mut h,
        "_not",
        Arc::new(|x: &Vec<Value>| Some(get_param::<BigInt>(x, 0)?.is_zero().into())),
        &IBIG_T,
        &BOOL_T,
    );
    arith_opr_big_default::<BigInt>(&mut h, &IBIG_T);
    arith_opr::<BigInt>(&mut h, "_rem", &std::ops::Rem::rem, &IBIG_T);
    comp_opr_default::<BigInt>(&mut h, &IBIG_T);

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&IBIG_T_VAL)),
            p if p == *STR_T_VAL => typecast_int!(BigInt => str, x),
            p if p == *BOOL_T_VAL => Value::Bool(get_param::<BigInt>(x, 0)? == 0.into()),
            p if p == *I8_T_VAL => typecast_int!(BigInt => I8, x),
            p if p == *I16_T_VAL => typecast_int!(BigInt => I16, x),
            p if p == *I32_T_VAL => typecast_int!(BigInt => I32, x),
            p if p == *I64_T_VAL => typecast_int!(BigInt => I64, x),
            p if p == *I128_T_VAL => typecast_int!(BigInt => I128, x),
            p if p == *ISIZE_T_VAL => typecast_int!(BigInt => Ibig, x),
            p if p == *IBIG_T_VAL => x[0].to_owned(),
            p if p == *U8_T_VAL => typecast_int!(BigInt => U8, x),
            p if p == *U16_T_VAL => typecast_int!(BigInt => U16, x),
            p if p == *U32_T_VAL => typecast_int!(BigInt => U32, x),
            p if p == *U64_T_VAL => typecast_int!(BigInt => U64, x),
            p if p == *U128_T_VAL => typecast_int!(BigInt => U128, x),
            p if p == *USIZE_T_VAL => typecast_int!(BigInt => Usize, x),
            p if p == *UBIG_T_VAL => typecast_int!(BigInt => Ubig, x),
            p if p == *F16_T_VAL => typecast_int!(big BigInt => f16, x),
            p if p == *F32_T_VAL => typecast_int!(big BigInt => f32, x),
            p if p == *F64_T_VAL => typecast_int!(big BigInt => f64, x),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, &IBIG_T);

    BuiltinType {
        name: Some(Ident::new("ibig")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: HashMap::default(),
        type_args: vec![],
    }
}

pub static IBIG_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(ibig_t().into()));
pub static IBIG_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(ibig_t().into()));

use std::sync::Arc;

use num::BigInt;

use crate::{
    ast::Ident,
    primitives::utils::{
        arith_opr, arith_opr_big_default, comp_opr_default, concat, get_param, type_cast, unary,
    },
    types::r#type::{BuiltinType, ValueType},
};
