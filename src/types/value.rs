use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use enum_as_inner::EnumAsInner;
use half::f16;
use itertools::Itertools;
use num::{BigInt, BigUint};

use crate::{
    types::{
        element::Argument,
        typeobj::{
            bool_t::BOOL_T, f16_t::F16_T, f32_t::F32_T, f64_t::F64_T, i128_t::I128_T, i16_t::I16_T,
            i32_t::I32_T, i64_t::I64_T, i8_t::I8_T, ibig_t::IBIG_T, isize_t::ISIZE_T,
            proc_t::PROC_T, str_t::STR_T, type_t::TYPE_T, u128_t::U128_T, u16_t::U16_T,
            u32_t::U32_T, u64_t::U64_T, u8_t::U8_T, ubig_t::UBIG_T, unit_t::UNIT_T,
            usize_t::USIZE_T, Type,
        },
    },
    Element,
};

#[derive(Clone)]
pub enum Proc {
    Builtin {
        f: fn(&Vec<Value>) -> Option<Value>,
        signature: Vec<&'static (dyn Fn() -> (Vec<Type<Value>>, Type<Value>) + Sync)>,
    },
    Defined {
        is_fn: bool,
        args: Vec<Argument>,
        return_type: Type<Value>,
        content: Vec<Element>,
    },
}
impl PartialEq for Proc {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            Self::Builtin { f, .. } => {
                if let Self::Builtin { f: f2, .. } = other {
                    *f as usize == *f2 as usize
                } else {
                    false
                }
            }
            Self::Defined {
                is_fn,
                args,
                return_type,
                content,
            } => {
                if let Self::Defined {
                    is_fn: is_fn2,
                    args: args2,
                    return_type: return_type2,
                    content: content2,
                } = other
                {
                    is_fn == is_fn2
                        && args == args2
                        && return_type == return_type2
                        && content == content2
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, EnumAsInner)]
pub enum Value {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    Ibig(BigInt),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Ubig(BigUint),
    F16(f16),
    F32(f32),
    F64(f64),
    Str(String),
    Bool(bool),
    Type(Type<Value>),
    PreType(Type<Element>),
    Proc(Proc),
    ClassInstance {
        type_: Type<Value>,
        attrs: HashMap<String, Value>,
    },
    Unit,
    Return(Box<Value>),
}

impl Debug for Proc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Value::Return(v) = self {
            return Debug::fmt(&v, f);
        }
        write!(
            f,
            "{}",
            match self {
                Value::I8(v) => format!("{}@i8", v),
                Value::I16(v) => format!("{}@i16", v),
                Value::I32(v) => format!("{}@i32", v),
                Value::I64(v) => format!("{}@i64", v),
                Value::I128(v) => format!("{}@i128", v),
                Value::Isize(v) => format!("{}@isize", v),
                Value::Ibig(v) => format!("{}@ibig", v),
                Value::U8(v) => format!("{}@u8", v),
                Value::U16(v) => format!("{}@u16", v),
                Value::U32(v) => format!("{}@u32", v),
                Value::U64(v) => format!("{}@u64", v),
                Value::U128(v) => format!("{}@u128", v),
                Value::Usize(v) => format!("{}@usize", v),
                Value::Ubig(v) => format!("{}@ubig", v),
                Value::F16(v) => format!("{}@f16", v),
                Value::F32(v) => format!("{}@f32", v),
                Value::F64(v) => format!("{}@f64", v),
                Value::Str(v) => format!("\"{}\"", v),
                Value::Type(v) => format!("{:?}", v),
                Value::PreType(v) => format!("{:?}", v),
                Value::Bool(_) | Value::ClassInstance { .. } | Value::Proc { .. } | Value::Unit =>
                    self.to_string(),
                Value::Return(_) => unreachable!(),
            }
        )
    }
}
impl Display for Proc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Proc::Builtin { signature, .. } => {
                    signature
                        .iter()
                        .map(|s| {
                            let (args, ret): (Vec<Type<Value>>, Type<Value>) = s();
                            format!(
                                "fn|{}|: {}",
                                args.iter().map(|a| a.to_string()).join(","),
                                ret
                            )
                        })
                        .join(" / ")
                }
                Proc::Defined {
                    is_fn,
                    args,
                    return_type,
                    ..
                } => format!(
                    "{}|{}|: {}",
                    if *is_fn { "fn" } else { "proc" },
                    args.iter().map(|a| a.to_string()).join(","),
                    return_type
                ),
            }
        )
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::I8(v) => v.to_string(),
                Value::I16(v) => v.to_string(),
                Value::I32(v) => v.to_string(),
                Value::I64(v) => v.to_string(),
                Value::I128(v) => v.to_string(),
                Value::Isize(v) => v.to_string(),
                Value::Ibig(v) => v.to_string(),
                Value::U8(v) => v.to_string(),
                Value::U16(v) => v.to_string(),
                Value::U32(v) => v.to_string(),
                Value::U64(v) => v.to_string(),
                Value::U128(v) => v.to_string(),
                Value::Usize(v) => v.to_string(),
                Value::Ubig(v) => v.to_string(),
                Value::F16(v) => v.to_string(),
                Value::F32(v) => v.to_string(),
                Value::F64(v) => v.to_string(),
                Value::Str(v) => v.to_owned(),
                Value::Bool(v) => v.to_string(),
                Value::Type(v) | Value::ClassInstance { type_: v, .. } => format!("<{}>", v),
                Value::PreType(v) => format!("<{}>", v),
                Value::Unit => "()".to_string(),
                Value::Return(v) => v.to_string(),
                Value::Proc(v) => v.to_string(),
            }
        )
    }
}

impl Value {
    pub fn is_num(&self) -> bool {
        matches!(
            self,
            Value::I8(_)
                | Value::I16(_)
                | Value::I32(_)
                | Value::I64(_)
                | Value::I128(_)
                | Value::Isize(_)
                | Value::Ibig(_)
                | Value::U8(_)
                | Value::U16(_)
                | Value::U32(_)
                | Value::U64(_)
                | Value::U128(_)
                | Value::Usize(_)
                | Value::Ubig(_)
                | Value::F16(_)
                | Value::F32(_)
                | Value::F64(_)
                | Value::Bool(_)
        )
    }
    pub fn from_type_content(type_: Type<Value>, content: String) -> Value {
        match type_ {
            p if p == *I8_T => Value::I8(content.parse::<i8>().unwrap()),
            p if p == *I16_T => Value::I16(content.parse::<i16>().unwrap()),
            p if p == *I32_T => Value::I32(content.parse::<i32>().unwrap()),
            p if p == *I64_T => Value::I64(content.parse::<i64>().unwrap()),
            p if p == *I128_T => Value::I128(content.parse::<i128>().unwrap()),
            p if p == *ISIZE_T => Value::Isize(content.parse::<isize>().unwrap()),
            p if p == *IBIG_T => Value::Ibig(content.parse::<BigInt>().unwrap()),
            p if p == *U8_T => Value::U8(content.parse::<u8>().unwrap()),
            p if p == *U16_T => Value::U16(content.parse::<u16>().unwrap()),
            p if p == *U32_T => Value::U32(content.parse::<u32>().unwrap()),
            p if p == *U64_T => Value::U64(content.parse::<u64>().unwrap()),
            p if p == *U128_T => Value::U128(content.parse::<u128>().unwrap()),
            p if p == *USIZE_T => Value::Usize(content.parse::<usize>().unwrap()),
            p if p == *UBIG_T => Value::Ubig(content.parse::<BigUint>().unwrap()),
            p if p == *F16_T => Value::F16(content.parse::<f16>().unwrap()),
            p if p == *F32_T => Value::F32(content.parse::<f32>().unwrap()),
            p if p == *F64_T => Value::F64(content.parse::<f64>().unwrap()),
            p if p == *STR_T => Value::Str(content),
            p if p == *BOOL_T => Value::Bool(&*content == "true"),
            _ => panic!(),
        }
    }
    pub fn get_type_obj(&self) -> &Type<Value> {
        match self {
            Value::I8(..) => &I8_T,
            Value::I16(..) => &I16_T,
            Value::I32(..) => &I32_T,
            Value::I64(..) => &I64_T,
            Value::I128(..) => &I128_T,
            Value::Isize(..) => &ISIZE_T,
            Value::Ibig(..) => &IBIG_T,
            Value::U8(..) => &U8_T,
            Value::U16(..) => &U16_T,
            Value::U32(..) => &U32_T,
            Value::U64(..) => &U64_T,
            Value::U128(..) => &U128_T,
            Value::Usize(..) => &USIZE_T,
            Value::Ubig(..) => &UBIG_T,
            Value::F16(..) => &F16_T,
            Value::F32(..) => &F32_T,
            Value::F64(..) => &F64_T,
            Value::Str(..) => &STR_T,
            Value::Bool(..) => &BOOL_T,
            Value::Type(..) | Value::PreType(..) => &TYPE_T,
            Value::Proc(_) => &PROC_T,
            Value::ClassInstance { type_, .. } => type_,
            Value::Unit => &UNIT_T,
            Value::Return(v) => v.get_type_obj(),
        }
    }
    pub fn get_type(&self) -> Value {
        Value::Type(self.get_type_obj().to_owned())
    }
    pub fn as_element(&self) -> Element {
        Element::Literal {
            position: Default::default(),
            raw: self.to_string(),
            content: self.to_owned(),
        }
    }
}
