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
        element::{block::Block, literal::Literal, procedure::Argument, ElementVariant},
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
        content: Element<Block>,
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
        ty: Type<Value>,
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
                Value::Type(v) | Value::ClassInstance { ty: v, .. } => format!("<{}>", v),
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
    pub fn get_type_obj(&self) -> Type<Value> {
        match self {
            Value::I8(..) => I8_T.get_instance(),
            Value::I16(..) => I16_T.get_instance(),
            Value::I32(..) => I32_T.get_instance(),
            Value::I64(..) => I64_T.get_instance(),
            Value::I128(..) => I128_T.get_instance(),
            Value::Isize(..) => ISIZE_T.get_instance(),
            Value::Ibig(..) => IBIG_T.get_instance(),
            Value::U8(..) => U8_T.get_instance(),
            Value::U16(..) => U16_T.get_instance(),
            Value::U32(..) => U32_T.get_instance(),
            Value::U64(..) => U64_T.get_instance(),
            Value::U128(..) => U128_T.get_instance(),
            Value::Usize(..) => USIZE_T.get_instance(),
            Value::Ubig(..) => UBIG_T.get_instance(),
            Value::F16(..) => F16_T.get_instance(),
            Value::F32(..) => F32_T.get_instance(),
            Value::F64(..) => F64_T.get_instance(),
            Value::Str(..) => STR_T.get_instance(),
            Value::Bool(..) => BOOL_T.get_instance(),
            Value::Type(..) | Value::PreType(..) => TYPE_T.get_instance(),
            Value::Proc(_) => PROC_T.get_instance(),
            Value::ClassInstance { ty, .. } => ty.to_owned(),
            Value::Unit => UNIT_T.get_instance(),
            Value::Return(v) => v.get_type_obj(),
        }
    }
    pub fn get_type(&self) -> Value {
        Value::Type(self.get_type_obj())
    }
    pub fn as_element(&self) -> Element {
        Element {
            pos_raw: Default::default(),
            data: Box::new(ElementVariant::Literal(Literal {
                content: self.to_owned(),
            })),
        }
    }
}
