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
        utils::{arith_opr_float_default, comp_opr_default, concat, get_param, type_cast, unary},
        *,
    },
    typecast_float,
    types::{
        r#type::{BuiltinType, ValueType},
        value::{Proc, Value},
    },
    Type,
};

macro_rules! typecast_f16_to_int {
    ($vo:ident $f:ident, $x:ident) => {
        Value::$vo(get_param::<f16>($x, 0)?.to_f64().$f()?)
    };
}

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn f16_t() -> BuiltinType {
    let mut h = HashMap::new();
    h.insert("_default", Value::F16(f16::ZERO));
    concat(&mut h, Arc::clone(&F16_T));
    unary(
        &mut h,
        "_un_add",
        Arc::new(|x: &Vec<Value>| Some(x[0].to_owned())),
        Arc::clone(&F16_T),
        Arc::clone(&F16_T),
    );
    unary(
        &mut h,
        "_un_sub",
        Arc::new(|x: &Vec<Value>| Some(get_param::<f16>(x, 0)?.neg().into())),
        Arc::clone(&F16_T),
        Arc::clone(&F16_T),
    );
    unary(
        &mut h,
        "_un_sub",
        Arc::new(|x: &Vec<Value>| {
            Some(
                (get_param::<f16>(x, 0)?.eq(&f16::ZERO)
                    || get_param::<f16>(x, 0)?.eq(&f16::NEG_ZERO))
                .into(),
            )
        }),
        Arc::clone(&F16_T),
        Arc::clone(&BOOL_T),
    );
    arith_opr_float_default::<f16>(&mut h, Arc::clone(&F16_T));
    comp_opr_default::<f16>(&mut h, Arc::clone(&F16_T));

    let typecast = Arc::new(|x: &Vec<Value>| {
        Some(match get_param::<Arc<ValueType>>(x, 1)? {
            p if p == *TYPE_T_VAL => Value::Type(Arc::clone(&F16_T_VAL)),
            p if p == *STR_T_VAL => typecast_float!(f16 => str, x),
            p if p == *BOOL_T_VAL => Value::Bool(
                get_param::<f16>(x, 0)? != f16::ZERO && get_param::<f16>(x, 0)? != f16::NEG_ZERO,
            ),
            p if p == *I8_T_VAL => typecast_f16_to_int!(I8 to_i8, x),
            p if p == *I16_T_VAL => typecast_f16_to_int!(I16 to_i16, x),
            p if p == *I32_T_VAL => typecast_f16_to_int!(I32 to_i32, x),
            p if p == *I64_T_VAL => typecast_f16_to_int!(I64 to_i64, x),
            p if p == *I128_T_VAL => typecast_f16_to_int!(I128 to_i128, x),
            p if p == *ISIZE_T_VAL => typecast_f16_to_int!(Isize to_isize, x),
            p if p == *IBIG_T_VAL => typecast_f16_to_int!(Ibig to_bigint, x),
            p if p == *U8_T_VAL => typecast_f16_to_int!(U8 to_u8, x),
            p if p == *U16_T_VAL => typecast_f16_to_int!(U16 to_u16, x),
            p if p == *U32_T_VAL => typecast_f16_to_int!(U32 to_u32, x),
            p if p == *U64_T_VAL => typecast_f16_to_int!(U64 to_u64, x),
            p if p == *U128_T_VAL => typecast_f16_to_int!(U128 to_u128, x),
            p if p == *USIZE_T_VAL => typecast_f16_to_int!(Usize to_usize, x),
            p if p == *UBIG_T_VAL => typecast_f16_to_int!(Ubig to_biguint, x),
            p if p == *F16_T_VAL => x[0].to_owned(),
            p if p == *F32_T_VAL => Value::F32(get_param::<f16>(x, 0)?.to_f32()),
            p if p == *F64_T_VAL => Value::F64(get_param::<f16>(x, 0)?.to_f64()),
            _ => return None,
        })
    });
    type_cast(&mut h, typecast, Arc::clone(&F16_T));

    BuiltinType {
        name: Some(Ident::new("f16")),
        namespace: h.drain().map(|(k, v)| (k.into(), v)).collect(),
        fields: Default::default(),
        type_args: vec![],
    }
}

pub static F16_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(f16_t().into()));
pub static F16_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(f16_t().into()));
