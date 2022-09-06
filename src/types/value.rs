use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use anyhow::Result;
use enum_as_inner::EnumAsInner;
use half::f16;
use num::{BigInt, BigUint};

use crate::{
    types::{element::Argument, token::OprType, typeobj::Type},
    Element, ZyxtError,
};
use crate::types::typeobj::i8_t::I8_T;
use crate::types::typeobj::i16_t::I16_T;
use crate::types::typeobj::i32_t::I32_T;
use crate::types::typeobj::i64_t::I64_T;
use crate::types::typeobj::i128_t::I128_T;
use crate::types::typeobj::ibig_t::IBIG_T;
use crate::types::typeobj::isize_t::ISIZE_T;
use crate::types::typeobj::u8_t::U8_T;
use crate::types::typeobj::u16_t::U16_T;
use crate::types::typeobj::u32_t::U32_T;
use crate::types::typeobj::u64_t::U64_T;
use crate::types::typeobj::u128_t::U128_T;
use crate::types::typeobj::ubig_t::UBIG_T;
use crate::types::typeobj::usize_t::USIZE_T;
use crate::types::typeobj::str_t::STR_T;
use crate::types::typeobj::bool_t::BOOL_T;
use crate::types::typeobj::type_t::TYPE_T;
use crate::types::typeobj::unit_t::UNIT_T;
use crate::types::typeobj::f16_t::F16_T;
use crate::types::typeobj::f32_t::F32_T;
use crate::types::typeobj::f64_t::F64_T;

#[derive(Clone)]
pub enum Proc {
    Builtin {
        f: fn(&Vec<Value>) -> Option<Value>,
        signature: Vec<(Vec<Type>, Type)>,
    },
    Defined {
        is_fn: bool,
        args: Vec<Argument>,
        return_type: Type,
        content: Vec<Element>,
    },
}

#[derive(Clone, EnumAsInner)]
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
    Type(Type),
    Proc(Proc),
    ClassInstance {
        type_: Type,
        attrs: HashMap<String, Value>,
    },
    Unit,
    Return(Box<Value>),
}

impl Debug for Proc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
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
                Value::Bool(_)
                | Value::Type(_)
                | Value::ClassInstance { .. }
                | Value::Proc { .. }
                | Value::Unit => self.to_string(),
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
                Proc::Builtin { f, signature } => "<{builtin}>".to_string(), // TODO
                Proc::Defined {
                    is_fn,
                    args,
                    return_type,
                    ..
                } => format!(
                    "{}|{}|: {}",
                    if *is_fn { "fn" } else { "proc" },
                    args.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
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
                Value::Proc(v) => v.to_string(),
                Value::Unit => "()".to_string(),
                Value::Return(v) => v.to_string(),
            }
        )
    }
}

impl Value {
    pub fn call(&self, args: Vec<Value>) -> Result<Value, OprError> {
        if args.len() == 1 {
            macro_rules! mult {
                () => {
                    self.bin_opr(&OprType::AstMult, args.get(0).unwrap().to_owned())
                };
            }
            match self {
                Value::I8(_) => mult!(),
                Value::I16(_) => mult!(),
                Value::I32(_) => mult!(),
                Value::I64(_) => mult!(),
                Value::I128(_) => mult!(),
                Value::Isize(_) => mult!(),
                Value::Ibig(_) => mult!(),
                Value::U8(_) => mult!(),
                Value::U16(_) => mult!(),
                Value::U32(_) => mult!(),
                Value::U64(_) => mult!(),
                Value::U128(_) => mult!(),
                Value::Usize(_) => mult!(),
                Value::Ubig(_) => mult!(),
                Value::F32(_) => mult!(),
                Value::F64(_) => mult!(),
                Value::Proc { .. } => panic!(),
                Value::Return(v) => v.call(args),
                Value::Type(_v) => todo!(),
                Value::ClassInstance { type_: _, .. } => todo!(),
                _ => Err(OprError::NoImplForOpr),
            }
        } else {
            Err(OprError::NoImplForOpr)
        }
    }
    pub fn un_opr(&self, type_: &OprType) -> Result<Value, OprError> {
        if let Value::Return(v) = self {
            return v.un_opr(type_);
        }
        match type_ {
            OprType::MinusSign => old::unary::un_minus(self),
            OprType::PlusSign => old::unary::un_plus(self),
            OprType::Not => old::unary::un_not(self),
            _ => Err(OprError::NoImplForOpr),
        }
    }
    pub fn bin_opr(&self, type_: &OprType, other: Value) -> Result<Value, OprError> {
        if let Value::Return(v) = self {
            return v.bin_opr(type_, other);
        }
        match type_ {
            OprType::Plus => old::add::add(self, other),
            OprType::Minus => old::sub::sub(self, other),
            OprType::AstMult | OprType::DotMult | OprType::CrossMult => old::mul::mul(self, other),
            OprType::Div | OprType::FractDiv => old::div::div(self, other),
            OprType::Modulo => old::modulo::modulo(self, other),

            OprType::Eq => old::eq::eq(self, other),
            OprType::Noteq => old::eq::noteq(self, other),
            OprType::Lt => old::lt::lt(self, other),
            OprType::Lteq => old::lt::lteq(self, other),
            OprType::Gt => old::gt::gt(self, other),
            OprType::Gteq => old::gt::gteq(self, other),
            OprType::Iseq => old::eq::iseq(self, other),
            OprType::Isnteq => old::eq::isnteq(self, other),

            OprType::And => unreachable!(),
            OprType::Or => unreachable!(),
            OprType::Xor => old::logic::xor(self, &other),

            OprType::Concat => old::concat::concat(self, other),
            OprType::TypeCast => old::typecast::typecast(self, other),
            _ => Err(OprError::NoImplForOpr),
        }
    }
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
    pub fn default(type_: Type) -> Result<Self, ZyxtError> {
        match type_.to_owned() {
            Type::Instance { name, .. } => Ok(match &*name {
                "i8" => Value::I8(0),
                "i16" => Value::I16(0),
                "i32" => Value::I32(0),
                "i64" => Value::I64(0),
                "i128" => Value::I128(0),
                "isize" => Value::Isize(0),
                "ibig" => Value::Ibig(0i32.into()),
                "u8" => Value::U8(0),
                "u16" => Value::U16(0),
                "u32" => Value::U32(0),
                "u64" => Value::U64(0),
                "u128" => Value::U128(0),
                "usize" => Value::Usize(0),
                "ubig" => Value::Ubig(0u32.into()),
                "f32" => Value::F32(0.0),
                "f64" => Value::F64(0.0),
                "str" => Value::Str("".to_string()),
                "bool" => Value::Bool(false),
                "_unit" | "_any" => Value::Unit, // TODO move _any somewhere else
                "type" => Value::Type(UNIT_T.to_owned()),
                _ => panic!("{:#?}", type_),
            }),
            _ => panic!(),
        }
    }
    pub fn from_type_content(type_: Type, content: String) -> Value {
        match type_ {
            Type::Instance { name, .. } => match &*name {
                "i8" => Value::I8(content.parse::<i8>().unwrap()),
                "i16" => Value::I16(content.parse::<i16>().unwrap()),
                "i32" => Value::I32(content.parse::<i32>().unwrap()),
                "i64" => Value::I64(content.parse::<i64>().unwrap()),
                "i128" => Value::I128(content.parse::<i128>().unwrap()),
                "isize" => Value::Isize(content.parse::<isize>().unwrap()),
                "ibig" => Value::Ibig(content.parse::<BigInt>().unwrap()),
                "u8" => Value::U8(content.parse::<u8>().unwrap()),
                "u16" => Value::U16(content.parse::<u16>().unwrap()),
                "u32" => Value::U32(content.parse::<u32>().unwrap()),
                "u64" => Value::U64(content.parse::<u64>().unwrap()),
                "u128" => Value::U128(content.parse::<u128>().unwrap()),
                "usize" => Value::Usize(content.parse::<usize>().unwrap()),
                "ubig" => Value::Ubig(content.parse::<BigUint>().unwrap()),
                "f16" => Value::F16(content.parse::<f16>().unwrap()),
                "f32" => Value::F32(content.parse::<f32>().unwrap()),
                "f64" => Value::F64(content.parse::<f64>().unwrap()),
                "str" => Value::Str(content),
                "bool" => Value::Bool(&*content == "true"),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
    pub fn get_type_obj(&self) -> &Type {
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
            Value::Type(..) => &TYPE_T,
            Value::Proc(_) =>
            /*Type::Instance {
                name: if *is_fn { "fn" } else { "proc" }.into(),
                type_args: vec![Type::null(), return_type.to_owned()],
                inst_attrs: Default::default(),
                implementation: None,
            }, // TODO angle bracket thingy when it is implemented*/
            {
                todo!()
            }
            Value::ClassInstance { type_, .. } => type_,
            Value::Unit => &UNIT_T,
            Value::Return(v) => v.get_type_obj(),
        }
    }
    pub fn get_type(&self) -> Value {
        Value::Type(self.get_type_obj())
    }
    pub fn as_element(&self) -> Element {
        Element::Literal {
            position: Default::default(),
            raw: self.to_string(),
            content: self.to_owned(),
        }
    }
}
