use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use enum_as_inner::EnumAsInner;
use half::f16;
use itertools::Itertools;
use num::{BigInt, BigUint};

use crate::{
    ast::{Argument, Ast, Block, Literal},
    primitives::*,
    types::r#type::Type,
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
            Self::Builtin { f, .. } =>
            {
                #[allow(clippy::fn_to_numeric_cast_any)]
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
    PreType(Type<Ast>),
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
        if let Self::Return(v) = self {
            return Debug::fmt(&v, f);
        }
        write!(
            f,
            "{}",
            match self {
                Self::I8(v) => format!("{v}@i8"),
                Self::I16(v) => format!("{v}@i16"),
                Self::I32(v) => format!("{v}@i32"),
                Self::I64(v) => format!("{v}@i64"),
                Self::I128(v) => format!("{v}@i128"),
                Self::Isize(v) => format!("{v}@isize"),
                Self::Ibig(v) => format!("{v}@ibig"),
                Self::U8(v) => format!("{v}@u8"),
                Self::U16(v) => format!("{v}@u16"),
                Self::U32(v) => format!("{v}@u32"),
                Self::U64(v) => format!("{v}@u64"),
                Self::U128(v) => format!("{v}@u128"),
                Self::Usize(v) => format!("{v}@usize"),
                Self::Ubig(v) => format!("{v}@ubig"),
                Self::F16(v) => format!("{v}@f16"),
                Self::F32(v) => format!("{v}@f32"),
                Self::F64(v) => format!("{v}@f64"),
                Self::Str(v) => format!("\"{v}\""),
                Self::Type(v) => format!("{v:?}"),
                Self::PreType(v) => format!("{v:?}"),
                Self::Bool(_) | Self::ClassInstance { .. } | Self::Proc { .. } | Self::Unit =>
                    self.to_string(),
                Self::Return(_) => unreachable!(),
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
                Self::Builtin { signature, .. } => {
                    signature
                        .iter()
                        .map(|s| {
                            let (args, ret): (Vec<Type<Value>>, Type<Value>) = s();
                            format!(
                                "fn|{}|: {}",
                                args.iter().map(ToString::to_string).join(","),
                                ret
                            )
                        })
                        .join(" / ")
                }
                Self::Defined {
                    is_fn,
                    args,
                    return_type,
                    ..
                } => format!(
                    "{}|{}|: {}",
                    if *is_fn { "fn" } else { "proc" },
                    args.iter().map(ToString::to_string).join(","),
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
                Self::I8(v) => v.to_string(),
                Self::I16(v) => v.to_string(),
                Self::I32(v) => v.to_string(),
                Self::I64(v) => v.to_string(),
                Self::I128(v) => v.to_string(),
                Self::Isize(v) => v.to_string(),
                Self::Ibig(v) => v.to_string(),
                Self::U8(v) => v.to_string(),
                Self::U16(v) => v.to_string(),
                Self::U32(v) => v.to_string(),
                Self::U64(v) => v.to_string(),
                Self::U128(v) => v.to_string(),
                Self::Usize(v) => v.to_string(),
                Self::Ubig(v) => v.to_string(),
                Self::F16(v) => v.to_string(),
                Self::F32(v) => v.to_string(),
                Self::F64(v) => v.to_string(),
                Self::Str(v) => v.to_owned(),
                Self::Bool(v) => v.to_string(),
                Self::Type(v) | Self::ClassInstance { ty: v, .. } => format!("<{v}>"),
                Self::PreType(v) => format!("<{v}>"),
                Self::Unit => "()".to_owned(),
                Self::Return(v) => v.to_string(),
                Self::Proc(v) => v.to_string(),
            }
        )
    }
}

impl Value {
    #[must_use]
    pub const fn is_num(&self) -> bool {
        matches!(
            self,
            Self::I8(_)
                | Self::I16(_)
                | Self::I32(_)
                | Self::I64(_)
                | Self::I128(_)
                | Self::Isize(_)
                | Self::Ibig(_)
                | Self::U8(_)
                | Self::U16(_)
                | Self::U32(_)
                | Self::U64(_)
                | Self::U128(_)
                | Self::Usize(_)
                | Self::Ubig(_)
                | Self::F16(_)
                | Self::F32(_)
                | Self::F64(_)
                | Self::Bool(_)
        )
    }
    pub fn get_type_obj(&self) -> Type<Self> {
        match self {
            Self::I8(..) => I8_T.get_instance(),
            Self::I16(..) => I16_T.get_instance(),
            Self::I32(..) => I32_T.get_instance(),
            Self::I64(..) => I64_T.get_instance(),
            Self::I128(..) => I128_T.get_instance(),
            Self::Isize(..) => ISIZE_T.get_instance(),
            Self::Ibig(..) => IBIG_T.get_instance(),
            Self::U8(..) => U8_T.get_instance(),
            Self::U16(..) => U16_T.get_instance(),
            Self::U32(..) => U32_T.get_instance(),
            Self::U64(..) => U64_T.get_instance(),
            Self::U128(..) => U128_T.get_instance(),
            Self::Usize(..) => USIZE_T.get_instance(),
            Self::Ubig(..) => UBIG_T.get_instance(),
            Self::F16(..) => F16_T.get_instance(),
            Self::F32(..) => F32_T.get_instance(),
            Self::F64(..) => F64_T.get_instance(),
            Self::Str(..) => STR_T.get_instance(),
            Self::Bool(..) => BOOL_T.get_instance(),
            Self::Type(..) | Self::PreType(..) => TYPE_T.get_instance(),
            Self::Proc(_) => PROC_T.get_instance(),
            Self::ClassInstance { ty, .. } => ty.to_owned(),
            Self::Unit => UNIT_T.get_instance(),
            Self::Return(v) => v.get_type_obj(),
        }
    }
    #[must_use]
    pub fn get_type(&self) -> Self {
        Self::Type(self.get_type_obj())
    }
    #[must_use]
    pub fn as_ast(&self) -> Ast {
        Ast::Literal(Literal {
            span: None,
            content: self.to_owned(),
        })
    }
}
