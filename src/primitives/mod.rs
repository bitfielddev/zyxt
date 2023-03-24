mod bool_t;
mod f16_t;
mod f32_t;
mod f64_t;
mod i128_t;
mod i16_t;
mod i32_t;
mod i64_t;
mod i8_t;
mod ibig_t;
mod isize_t;
mod proc_t;
mod str_t;
mod type_t;
mod u128_t;
mod u16_t;
mod u32_t;
mod u64_t;
mod u8_t;
mod ubig_t;
mod unit_t;
mod usize_t;
mod utils;

use std::collections::HashMap;

pub use bool_t::{BOOL_T, BOOL_T_VAL};
pub use f16_t::{F16_T, F16_T_VAL};
pub use f32_t::{F32_T, F32_T_VAL};
pub use f64_t::{F64_T, F64_T_VAL};
pub use i128_t::{I128_T, I128_T_VAL};
pub use i16_t::{I16_T, I16_T_VAL};
pub use i32_t::{I32_T, I32_T_VAL};
pub use i64_t::{I64_T, I64_T_VAL};
pub use i8_t::{I8_T, I8_T_VAL};
pub use ibig_t::{IBIG_T, IBIG_T_VAL};
pub use isize_t::{ISIZE_T, ISIZE_T_VAL};
pub use proc_t::{generic_proc, PROC_T, PROC_T_VAL};
pub use str_t::{STR_T, STR_T_VAL};
pub use type_t::{TYPE_T, TYPE_T_VAL};
pub use u128_t::{U128_T, U128_T_VAL};
pub use u16_t::{U16_T, U16_T_VAL};
pub use u32_t::{U32_T, U32_T_VAL};
pub use u64_t::{U64_T, U64_T_VAL};
pub use u8_t::{U8_T, U8_T_VAL};
pub use ubig_t::{UBIG_T, UBIG_T_VAL};
pub use unit_t::{UNIT_T, UNIT_T_VAL};
pub use usize_t::{USIZE_T, USIZE_T_VAL};

pub static ANY_T: Lazy<Arc<Type>> = Lazy::new(|| Arc::new(Type::Any));
pub static ANY_T_VAL: Lazy<Arc<ValueType>> = Lazy::new(|| Arc::new(ValueType::Any));

use std::sync::Arc;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
    ast::Ident,
    types::r#type::{BuiltinType, Type, ValueType},
};

pub static PRIMS: Lazy<HashMap<&'static str, Arc<Type>>> = Lazy::new(|| {
    hashmap! {
        "bool" => Arc::clone(&BOOL_T),
        "f16" => Arc::clone(&F16_T),
        "f32" => Arc::clone(&F32_T),
        "f64" => Arc::clone(&F64_T),
        "i128" => Arc::clone(&I128_T),
        "i16" => Arc::clone(&I16_T),
        "i32" => Arc::clone(&I32_T),
        "i64" => Arc::clone(&I64_T),
        "i8" => Arc::clone(&I8_T),
        "ibig" => Arc::clone(&IBIG_T),
        "isize" => Arc::clone(&ISIZE_T),
        "proc" => Arc::clone(&PROC_T),
        "str" => Arc::clone(&STR_T),
        "type" => Arc::clone(&TYPE_T),
        "u128" => Arc::clone(&U128_T),
        "u16" => Arc::clone(&U16_T),
        "u32" => Arc::clone(&U32_T),
        "u64" => Arc::clone(&U64_T),
        "u8" => Arc::clone(&U8_T),
        "ubig" => Arc::clone(&UBIG_T),
        "unit" => Arc::clone(&UNIT_T),
        "usize" => Arc::clone(&USIZE_T),
    }
});

pub static PRIMS_VAL: Lazy<HashMap<&'static str, Arc<ValueType>>> = Lazy::new(|| {
    hashmap! {
        "bool" => Arc::clone(&BOOL_T_VAL),
        "f16" => Arc::clone(&F16_T_VAL),
        "f32" => Arc::clone(&F32_T_VAL),
        "f64" => Arc::clone(&F64_T_VAL),
        "i128" => Arc::clone(&I128_T_VAL),
        "i16" => Arc::clone(&I16_T_VAL),
        "i32" => Arc::clone(&I32_T_VAL),
        "i64" => Arc::clone(&I64_T_VAL),
        "i8" => Arc::clone(&I8_T_VAL),
        "ibig" => Arc::clone(&IBIG_T_VAL),
        "isize" => Arc::clone(&ISIZE_T_VAL),
        "proc" => Arc::clone(&PROC_T_VAL),
        "str" => Arc::clone(&STR_T_VAL),
        "type" => Arc::clone(&TYPE_T_VAL),
        "u128" => Arc::clone(&U128_T_VAL),
        "u16" => Arc::clone(&U16_T_VAL),
        "u32" => Arc::clone(&U32_T_VAL),
        "u64" => Arc::clone(&U64_T_VAL),
        "u8" => Arc::clone(&U8_T_VAL),
        "ubig" => Arc::clone(&UBIG_T_VAL),
        "unit" => Arc::clone(&UNIT_T_VAL),
        "usize" => Arc::clone(&USIZE_T_VAL),
    }
});
