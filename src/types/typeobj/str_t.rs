use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::{concat_vals, get_param, binary, Type, typecast_to_type};
use crate::types::value::{Value, Proc};
use crate::types::typeobj::type_t::TYPE_T;
use crate::types::typeobj::bool_t::BOOL_T;

macro_rules! typecast_str_to_num {
    ($v:ident, $x:ident) => {
       Value::$v(get_param!($x, 0, Str).parse().ok()?)
    }
}

const fn str_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, STR_T);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(STR_T),
                "str" => x[0].to_owned(),
                "bool" => Value::Bool(get_param!(x, 0, Str).is_empty()),
                "i8" => typecast_str_to_num!(I8, x),
                "i16" => typecast_str_to_num!(I16, x),
                "i32" => typecast_str_to_num!(I32, x),
                "i64" => typecast_str_to_num!(I64, x),
                "i128" => typecast_str_to_num!(I128, x),
                "isize" => typecast_str_to_num!(Isize, x),
                "ibig" => typecast_str_to_num!(Ibig, x),
                "u8" => typecast_str_to_num!(U8, x),
                "u16" => typecast_str_to_num!(U16, x),
                "u32" => typecast_str_to_num!(U32, x),
                "u64" => typecast_str_to_num!(U64, x),
                "u128" => typecast_str_to_num!(U128, x),
                "usize" => typecast_str_to_num!(Usize, x),
                "ubig" => typecast_str_to_num!(Ubig, x),
                "f16" => typecast_str_to_num!(F16, x),
                "f32" => typecast_str_to_num!(F32, x),
                "f64" => typecast_str_to_num!(F64, x),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, STR_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref STR_T: Type = Type::Definition {
        name: Some("str".into()),
        generics: vec![],
        implementations: str_t(),
        inst_fields: HashMap::new(),
    };
}
