use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use enum_as_inner::EnumAsInner;
use half::f16;
use itertools::Itertools;
use num::{BigInt, BigUint};

use crate::{
    ast::{block::Block, literal::Literal, procedure::Argument, Element},
    primitives::*,
    types::typeobj::Type,
};

type BuiltinFunction = Vec<&'static (dyn Fn() -> (Vec<Type<Value>>, Type<Value>) + Sync)>;

#[derive(Clone)]
pub enum Proc {
    Builtin {
        f: fn(&Vec<Value>) -> Option<Value>,
        signature: BuiltinFunction,
    },
    Defined {
        is_fn: bool,
        args: Vec<Argument>,
        return_type: Type<Value>,
        content: Block,
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
        write!(f, "{self}")
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
                Value::I8(v) => format!("{v}@i8"),
                Value::I16(v) => format!("{v}@i16"),
                Value::I32(v) => format!("{v}@i32"),
                Value::I64(v) => format!("{v}@i64"),
                Value::I128(v) => format!("{v}@i128"),
                Value::Isize(v) => format!("{v}@isize"),
                Value::Ibig(v) => format!("{v}@ibig"),
                Value::U8(v) => format!("{v}@u8"),
                Value::U16(v) => format!("{v}@u16"),
                Value::U32(v) => format!("{v}@u32"),
                Value::U64(v) => format!("{v}@u64"),
                Value::U128(v) => format!("{v}@u128"),
                Value::Usize(v) => format!("{v}@usize"),
                Value::Ubig(v) => format!("{v}@ubig"),
                Value::F16(v) => format!("{v}@f16"),
                Value::F32(v) => format!("{v}@f32"),
                Value::F64(v) => format!("{v}@f64"),
                Value::Str(v) => format!("\"{v}\""),
                Value::Type(v) => format!("{v:?}"),
                Value::PreType(v) => format!("{v:?}"),
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
                Value::Type(v) | Value::ClassInstance { ty: v, .. } => format!("<{v}>"),
                Value::PreType(v) => format!("<{v}>"),
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
        Element::Literal(Literal {
            span: None,
            content: self.to_owned(),
        })
    }
}
