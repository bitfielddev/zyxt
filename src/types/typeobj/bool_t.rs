use std::collections::HashMap;

use lazy_static::lazy_static;
use smol_str::SmolStr;

use crate::{
    binary, comp_opr_num, concat_vals, get_param, typecast_to_type,
    types::{
        typeobj::{
            f16_t::F16_T, f32_t::F32_T, f64_t::F64_T, i128_t::I128_T, i16_t::I16_T, i32_t::I32_T,
            i64_t::I64_T, i8_t::I8_T, ibig_t::IBIG_T, isize_t::ISIZE_T, str_t::STR_T,
            type_t::TYPE_T, u128_t::U128_T, u16_t::U16_T, u32_t::U32_T, u64_t::U64_T, u8_t::U8_T,
            ubig_t::UBIG_T, usize_t::USIZE_T,
        },
        value::{Proc, Value},
    },
    Type,
};

macro_rules! typecast_bool_to_num {
    ($v:ident $v2:ty, $x:ident) => {
        Value::$v(get_param!($x, 0, Bool) as $v2)
    };
    ($v:ident, $x:ident) => {
        Value::$v(if get_param!($x, 0, Bool) { 1u8 } else { 0u8 }.into())
    };
}

fn bool_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, BOOL_T);
    comp_opr_num!(h, default BOOL_T Bool);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == *TYPE_T => typecast_to_type!(BOOL_T),
            p if p == *STR_T => Value::Str(get_param!(x, 0, Bool).to_string()),
            p if p == *BOOL_T => x[0].to_owned(),
            p if p == *I8_T => typecast_bool_to_num!(I8 i8, x),
            p if p == *I16_T => typecast_bool_to_num!(I16 i16, x),
            p if p == *I32_T => typecast_bool_to_num!(I32 i32, x),
            p if p == *I64_T => typecast_bool_to_num!(I64 i64, x),
            p if p == *I128_T => typecast_bool_to_num!(I128 i128, x),
            p if p == *ISIZE_T => typecast_bool_to_num!(Isize isize, x),
            p if p == *IBIG_T => typecast_bool_to_num!(Ibig, x),
            p if p == *U8_T => typecast_bool_to_num!(U8 u8, x),
            p if p == *U16_T => typecast_bool_to_num!(U16 u16, x),
            p if p == *U32_T => typecast_bool_to_num!(U32 u32, x),
            p if p == *U64_T => typecast_bool_to_num!(U64 u64, x),
            p if p == *U128_T => typecast_bool_to_num!(U128 u128, x),
            p if p == *USIZE_T => typecast_bool_to_num!(Usize usize, x),
            p if p == *UBIG_T => typecast_bool_to_num!(Ubig, x),
            p if p == *F16_T => typecast_bool_to_num!(F16, x),
            p if p == *F32_T => typecast_bool_to_num!(F32, x),
            p if p == *F64_T => typecast_bool_to_num!(F64, x),
            _ => return None,
        })
    };
    binary!(h, BOOL_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k.into(), Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref BOOL_T: Type<Value> = Type::Definition {
        name: Some("{builtin}".into()),
        inst_name: Some("bool".into()),
        generics: vec![],
        implementations: bool_t(),
        inst_fields: HashMap::new(),
    };
}
