use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
    sync::Arc,
};

use enum_as_inner::EnumAsInner;
use half::f16;
use itertools::Itertools;
use num::{BigInt, BigUint};
use smol_str::SmolStr;

use crate::{
    ast::{Ast, Block, Literal},
    errors::ZResult,
    primitives::*,
    types::{
        position::GetSpan,
        r#type::{Type, TypeCheckType, ValueType},
        sym_table::{InterpretFrameType, InterpretSymTable},
    },
};

pub type BuiltinFunction = dyn Fn(&Vec<Value>) -> Option<Value> + Send + Sync;

#[derive(Clone)]
pub enum Proc {
    Builtin {
        f: Arc<BuiltinFunction>,
        id: usize,
        ty: LazyGenericProc,
    },
    Defined {
        is_fn: bool,
        content: Block,
        args: Vec<SmolStr>,
    },
}
impl PartialEq for Proc {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (Self::Builtin { id: id1, .. }, Self::Builtin { id: id2, .. }) => id1 == id2,
            (
                Self::Defined {
                    is_fn: is_fn1,
                    content: content1,
                    args: args1,
                },
                Self::Defined {
                    is_fn: is_fn2,
                    content: content2,
                    args: args2,
                },
            ) => is_fn1 == is_fn2 && content1 == content2 && args1 == args2,
            _ => false,
        }
    }
}

impl Proc {
    pub fn call(&self, vals: Vec<Value>, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        match self {
            Self::Builtin { f, .. } => {
                let Some(res) = (*f)(&vals) else {
                    todo!()
                };
                Ok(res)
            }
            Self::Defined {
                content,
                is_fn,
                args,
            } => {
                val_symt.add_frame(if *is_fn {
                    InterpretFrameType::Function
                } else {
                    InterpretFrameType::Normal
                });
                for (name, val) in args.iter().zip_eq(vals) {
                    val_symt.declare_val(name, val)
                }
                let res = content.interpret_block(val_symt, true, false);
                val_symt.pop_frame();
                res
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
    Type(Arc<ValueType>),
    Proc(Proc),
    ClassInstance {
        ty: Arc<ValueType>,
        attrs: HashMap<String, Value>,
    },
    Unit,
    Return(Box<Value>),
}

pub trait ValueInner: TryFrom<Value> + Into<Value> + 'static {}

macro_rules! from_to {
    ($variant:ident, $ty:ty) => {
        impl From<$ty> for Value {
            fn from(value: $ty) -> Self {
                Value::$variant(value)
            }
        }
        impl TryFrom<Value> for $ty {
            type Error = ();

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                if let Value::$variant(v) = value {
                    Ok(v)
                } else {
                    Err(()) // TODO
                }
            }
        }
        impl ValueInner for $ty {}
    };
}

from_to!(I8, i8);
from_to!(I16, i16);
from_to!(I32, i32);
from_to!(I64, i64);
from_to!(I128, i128);
from_to!(Isize, isize);
from_to!(Ibig, BigInt);
from_to!(U8, u8);
from_to!(U16, u16);
from_to!(U32, u32);
from_to!(U64, u64);
from_to!(U128, u128);
from_to!(Usize, usize);
from_to!(Ubig, BigUint);
from_to!(F16, f16);
from_to!(F32, f32);
from_to!(F64, f64);
from_to!(Str, String);
from_to!(Bool, bool);
from_to!(Type, Arc<ValueType>);
from_to!(Proc, Proc);

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::Unit
    }
}
impl TryFrom<Value> for () {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value == Value::Unit {
            Ok(())
        } else {
            Err(()) // TODO
        }
    }
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
                Self::Bool(_) | Self::ClassInstance { .. } | Self::Proc { .. } | Self::Unit =>
                    self.to_string(),
                Self::Return(_) => unreachable!(),
            }
        )
    }
}
impl Display for Proc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Builtin { id, ty, .. } => {
                write!(f, "builtin@{id}@{ty}")
            }
            Self::Defined { is_fn, content, .. } => {
                write!(f, "{}", if *is_fn { "fn" } else { "proc" })?;
                if let Some(span) = content.span() {
                    write!(f, "@{}", span.start_pos)?;
                }
                Ok(())
            }
        }
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
    pub fn ty(&self) -> Arc<Type> {
        match self {
            Self::I8(..) => Arc::clone(&I8_T),
            Self::I16(..) => Arc::clone(&I16_T),
            Self::I32(..) => Arc::clone(&I32_T),
            Self::I64(..) => Arc::clone(&I64_T),
            Self::I128(..) => Arc::clone(&I128_T),
            Self::Isize(..) => Arc::clone(&ISIZE_T),
            Self::Ibig(..) => Arc::clone(&IBIG_T),
            Self::U8(..) => Arc::clone(&U8_T),
            Self::U16(..) => Arc::clone(&U16_T),
            Self::U32(..) => Arc::clone(&U32_T),
            Self::U64(..) => Arc::clone(&U64_T),
            Self::U128(..) => Arc::clone(&U128_T),
            Self::Usize(..) => Arc::clone(&USIZE_T),
            Self::Ubig(..) => Arc::clone(&UBIG_T),
            Self::F16(..) => Arc::clone(&F16_T),
            Self::F32(..) => Arc::clone(&F32_T),
            Self::F64(..) => Arc::clone(&F64_T),
            Self::Str(..) => Arc::clone(&STR_T),
            Self::Bool(..) => Arc::clone(&BOOL_T),
            Self::Type(..) => Arc::clone(&TYPE_T),
            Self::Proc(proc) => Arc::clone(match proc {
                Proc::Builtin { ty, .. } => ty,
                Proc::Defined { .. } => &PROC_T,
            }),
            Self::ClassInstance { .. } => todo!(),
            Self::Unit => Arc::clone(&UNIT_T),
            Self::Return(v) => v.ty(),
        }
    }
    pub fn type_check_ty(&self) -> TypeCheckType {
        match self {
            Self::Type(ty) => TypeCheckType::Const(ty.to_type()),
            v => v.ty().into(),
        }
    }
    pub fn value_ty(&self) -> Arc<ValueType> {
        match self {
            Self::I8(..) => Arc::clone(&I8_T_VAL),
            Self::I16(..) => Arc::clone(&I16_T_VAL),
            Self::I32(..) => Arc::clone(&I32_T_VAL),
            Self::I64(..) => Arc::clone(&I64_T_VAL),
            Self::I128(..) => Arc::clone(&I128_T_VAL),
            Self::Isize(..) => Arc::clone(&ISIZE_T_VAL),
            Self::Ibig(..) => Arc::clone(&IBIG_T_VAL),
            Self::U8(..) => Arc::clone(&U8_T_VAL),
            Self::U16(..) => Arc::clone(&U16_T_VAL),
            Self::U32(..) => Arc::clone(&U32_T_VAL),
            Self::U64(..) => Arc::clone(&U64_T_VAL),
            Self::U128(..) => Arc::clone(&U128_T_VAL),
            Self::Usize(..) => Arc::clone(&USIZE_T_VAL),
            Self::Ubig(..) => Arc::clone(&UBIG_T_VAL),
            Self::F16(..) => Arc::clone(&F16_T_VAL),
            Self::F32(..) => Arc::clone(&F32_T_VAL),
            Self::F64(..) => Arc::clone(&F64_T_VAL),
            Self::Str(..) => Arc::clone(&STR_T_VAL),
            Self::Bool(..) => Arc::clone(&BOOL_T_VAL),
            Self::Type(..) => Arc::clone(&TYPE_T_VAL),
            Self::Proc(_) => Arc::clone(&PROC_T_VAL),
            Self::ClassInstance { ty, .. } => Arc::clone(ty),
            Self::Unit => Arc::clone(&UNIT_T_VAL),
            Self::Return(v) => v.value_ty(),
        }
    }
    #[must_use]
    pub fn as_ast(&self) -> Ast {
        Ast::Literal(Literal {
            span: None,
            content: self.to_owned(),
        })
    }
}
