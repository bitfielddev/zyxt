use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    sync::Arc,
};

use half::f16;
use num::{
    bigint::{ToBigInt, ToBigUint},
    ToPrimitive,
};
use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    ast::Ident,
    primitives::{
        utils::{
            arith_opr_float_default, comp_opr_default, concat, get_param, type_cast,
            unary_float_default,
        },
        *,
    },
    typecast_float,
    types::{
        r#type::{BuiltinType, ValueType},
        value::{Proc, Value},
    },
    Type,
};

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn f64_t() -> BuiltinType {
    let mut h = HashMap::new();
    h.insert("_default", Value::F64(0.0));
    concat(&mut h, Arc::clone(&F64_T));
    unary_float_default::<f64>(&mut h, Arc::clone(&F64_T));
    arith_opr_float_default::<f64>(&mut h, Arc::clone(&F64_T));
    comp_opr_default::<f64>(&mut h, Arc::clone(&F64_T));

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&F64_T_VAL)),
            p if p == *STR_T_VAL => typecast_float!(f64 => str, x),
            p if p == *BOOL_T_VAL => typecast_float!(f64 => bool, x),
            p if p == *I8_T_VAL => typecast_float!(f64 => I8 to_i8, x),
            p if p == *I16_T_VAL => typecast_float!(f64 => I16 to_i16, x),
            p if p == *I32_T_VAL => typecast_float!(f64 => I32 to_i32, x),
            p if p == *I64_T_VAL => typecast_float!(f64 => I64 to_i64, x),
            p if p == *I128_T_VAL => typecast_float!(f64 => I128 to_i128, x),
            p if p == *ISIZE_T_VAL => typecast_float!(f64 => Isize to_isize, x),
            p if p == *IBIG_T_VAL => typecast_float!(f64 => Ibig to_bigint, x),
            p if p == *U8_T_VAL => typecast_float!(f64 => U8 to_u8, x),
            p if p == *U16_T_VAL => typecast_float!(f64 => U16 to_u16, x),
            p if p == *U32_T_VAL => typecast_float!(f64 => U32 to_u32, x),
            p if p == *U64_T_VAL => typecast_float!(f64 => U64 to_u64, x),
            p if p == *U128_T_VAL => typecast_float!(f64 => U128 to_u128, x),
            p if p == *USIZE_T_VAL => typecast_float!(f64 => Usize to_usize, x),
            p if p == *UBIG_T_VAL => typecast_float!(f64 => Ubig to_biguint, x),
            p if p == *F16_T_VAL => typecast_float!(f64 => f16, x),
            p if p == *F32_T_VAL => typecast_float!(f64 => F32 to_f32, x),
            p if p == *F64_T_VAL => x[0].to_owned(),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, Arc::clone(&F64_T));

    BuiltinType {
        name: Some(Ident::new("f64")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: Default::default(),
        type_args: vec![],
    }
}

pub static F64_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(f64_t().into()));
pub static F64_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(f64_t().into()));
