use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use num::{BigInt, BigUint};
use crate::{Element, ZyxtError};
use crate::objects::element::Argument;
use crate::objects::token::OprType;
use crate::objects::typeobj::Type;

#[derive(Clone, PartialEq)]
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
    F32(f32),
    F64(f64),
    Str(String),
    Bool(bool),
    Type(Type),
    Proc{
        is_fn: bool,
        args: Vec<Argument>,
        return_type: Type,
        content: Vec<Element>
    },
    ClassInstance{
        type_: Type,
        attrs: HashMap<String, Value>,
    },
    Null,
    Return(Box<Value>)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
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
            Value::F32(v) => v.to_string(),
            Value::F64(v) => v.to_string(),
            Value::Str(v) => v.clone(),
            Value::Bool(v) => v.to_string(),
            Value::Type(v) |
            Value::ClassInstance {type_: v, ..} => format!("<{}>", v),
            Value::Proc{is_fn, args, return_type, ..} =>
                format!("{}|{}|: {}",
                    if *is_fn {"fn"} else {"proc"},
                    args.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","),
                        return_type),
            Value::Null => "null".to_string(),
            Value::Return(v) => v.to_string()
        })
    }
}

impl Value {
    pub fn call(&self, args: Vec<Value>) -> Option<Value> {
        if args.len() == 1 {
        macro_rules! mult {
            () => {self.bin_opr(&OprType::AstMult, args.get(0)?.clone())}
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
                Value::Proc{..} => panic!(),
                Value::Return(v) => v.call(args),
                Value::Type(_v) => todo!(),
                Value::ClassInstance {type_: _, ..} => todo!(),
                _ => None
            }
        } else {None}
    }
    pub fn un_opr(&self, type_: &OprType) -> Option<Value> {
        if let Value::Return(v) = self {return v.un_opr(type_)}
        macro_rules! case {
            ($opr: expr => $($var_type: ident),*) => {
                match *self {
                    $(Value::$var_type(v) => Some(Value::$var_type($opr(v))),)*
                    _ => None
                }
            };
            ($($var_type: ident),*) => {
                match *self {
                    $(Value::$var_type(v) => Some(Value::$var_type(v)),)*
                    _ => None
                }
            }
        }
        match type_ {
            OprType::MinusSign => case!(Neg::neg => I8, I16, I32, I64, I128, Isize, F32, F64),
            OprType::PlusSign => case!(I8, I16, I32, I64, I128, Isize, F32, F64),
            _ => None
        }
    }
    pub fn bin_opr(&self, type_: &OprType, other: Value) -> Option<Value> {
        if let Value::Return(v) = self {return v.bin_opr(type_, other)}
        macro_rules! case {
            ($opr: expr => $((
                $var_type1: ident => $(($var_type2: ident => $return_type: ident, $rs_type: ty)),*
            )),*) => {
                match *self {
                    $(Value::$var_type1(v1) => match other {
                        $(Value::$var_type2(v2) => Some(Value::$return_type($opr(v1 as $rs_type, v2 as $rs_type) as $rs_type)),)*
                        _ => None
                    },)*
                    _ => None
                }
            }
        }
        macro_rules! case_arith {
            ($opr: expr) => {
                case!($opr =>
                    (I8 =>
                        (I8 => I8, i8),
                        (I16 => I16, i16),
                        (I32 => I32, i32),
                        (I64 => I64, i64),
                        (I128 => I128, i128),
                        (Isize => Isize, isize),
                        (U8 => I8, i8),
                        (U16 => I16, i16),
                        (U32 => I32, i32),
                        (U64 => I64, i64),
                        (U128 => I128, i128),
                        (Usize => Isize, isize),
                        (F32 => F32, f32),
                        (F64 => F64, f64)),
                    (I16 =>
                        (I8 => I16, i16),
                        (I16 => I16, i16),
                        (I32 => I32, i32),
                        (I64 => I64, i64),
                        (I128 => I128, i128),
                        (Isize => Isize, isize),
                        (U8 => I16, i16),
                        (U16 => I16, i16),
                        (U32 => I32, i32),
                        (U64 => I64, i64),
                        (U128 => I128, i128),
                        (Usize => Isize, isize),
                        (F32 => F32, f32),
                        (F64 => F64, f64)),
                    (I32 =>
                        (I8 => I32, i32),
                        (I16 => I32, i32),
                        (I32 => I32, i32),
                        (I64 => I64, i64),
                        (I128 => I128, i128),
                        (Isize => Isize, isize),
                        (U8 => I32, i32),
                        (U16 => I32, i32),
                        (U32 => I32, i32),
                        (U64 => I64, i64),
                        (U128 => I128, i128),
                        (Usize => Isize, isize),
                        (F32 => F32, f32),
                        (F64 => F64, f64)),
                    (I64 =>
                        (I8 => I64, i64),
                        (I16 => I64, i64),
                        (I32 => I64, i64),
                        (I64 => I64, i64),
                        (I128 => I128, i128),
                        (Isize => I64, i64),
                        (U8 => I64, i64),
                        (U16 => I64, i64),
                        (U32 => I64, i64),
                        (U64 => I64, i64),
                        (U128 => I128, i128),
                        (Usize => I64, i64),
                        (F32 => F64, f64),
                        (F64 => F64, f64)),
                    (I128 =>
                        (I8 => I128, i128),
                        (I16 => I128, i128),
                        (I32 => I128, i128),
                        (I64 => I128, i128),
                        (I128 => I128, i128),
                        (Isize => I128, i128),
                        (U8 => I128, i128),
                        (U16 => I128, i128),
                        (U32 => I128, i128),
                        (U64 => I128, i128),
                        (U128 => I128, i128),
                        (Usize => I128, i128),
                        (F32 => F64, f64),
                        (F64 => F64, f64)),
                    (Isize =>
                        (I8 => Isize, isize),
                        (I16 => Isize, isize),
                        (I32 => Isize, isize),
                        (I64 => I64, i64),
                        (I128 => I128, i128),
                        (Isize => Isize, isize),
                        (U8 => Isize, isize),
                        (U16 => Isize, isize),
                        (U32 => Isize, isize),
                        (U64 => I64, i64),
                        (U128 => I128, i128),
                        (Usize => Isize, isize),
                        (F32 => F32, f32),
                        (F64 => F64, f64)),
                    (U8 =>
                        (I8 => I8, i8),
                        (I16 => I16, i16),
                        (I32 => I32, i32),
                        (I64 => I64, i64),
                        (I128 => I128, i128),
                        (Isize => Isize, isize),
                        (U8 => U8, u8),
                        (U16 => U16, u16),
                        (U32 => U32, u32),
                        (U64 => U64, u64),
                        (U128 => U128, u128),
                        (Usize => Usize, usize),
                        (F32 => F32, f32),
                        (F64 => F64, f64)),
                    (U16 =>
                        (I8 => I16, i16),
                        (I16 => I16, i16),
                        (I32 => I32, i32),
                        (I64 => I64, i64),
                        (I128 => I128, i128),
                        (Isize => Isize, isize),
                        (U8 => U16, u16),
                        (U16 => U16, u16),
                        (U32 => U32, u32),
                        (U64 => U64, u64),
                        (U128 => U128, u128),
                        (Usize => Usize, usize),
                        (F32 => F32, f32),
                        (F64 => F64, f64)),
                    (U32 =>
                        (I8 => I32, i32),
                        (I16 => I32, i32),
                        (I32 => I32, i32),
                        (I64 => I64, i64),
                        (I128 => I128, i128),
                        (Isize => Isize, isize),
                        (U8 => U32, u32),
                        (U16 => U32, u32),
                        (U32 => U32, u32),
                        (U64 => U64, u64),
                        (U128 => U128, u128),
                        (Usize => Usize, usize),
                        (F32 => F32, f32),
                        (F64 => F64, f64)),
                    (U64 =>
                        (I8 => I64, i64),
                        (I16 => I64, i64),
                        (I32 => I64, i64),
                        (I64 => I64, i64),
                        (I128 => I128, i128),
                        (Isize => I64, i64),
                        (U8 => U64, u64),
                        (U16 => U64, u64),
                        (U32 => U64, u64),
                        (U64 => U64, u64),
                        (U128 => U128, u128),
                        (Usize => U64, u64),
                        (F32 => F64, f64),
                        (F64 => F64, f64)),
                    (U128 =>
                        (I8 => I128, i128),
                        (I16 => I128, i128),
                        (I32 => I128, i128),
                        (I64 => I128, i128),
                        (I128 => I128, i128),
                        (Isize => I128, i128),
                        (U8 => U128, u128),
                        (U16 => U128, u128),
                        (U32 => U128, u128),
                        (U64 => U128, u128),
                        (U128 => U128, u128),
                        (Usize => U128, u128),
                        (F32 => F64, f64),
                        (F64 => F64, f64)),
                    (Usize =>
                        (I8 => Isize, isize),
                        (I16 => Isize, isize),
                        (I32 => Isize, isize),
                        (I64 => I64, i64),
                        (I128 => I128, i128),
                        (Isize => Isize, isize),
                        (U8 => Usize, usize),
                        (U16 => Usize, usize),
                        (U32 => Usize, usize),
                        (U64 => U64, u64),
                        (U128 => U128, u128),
                        (Usize => Usize, usize),
                        (F32 => F32, f32),
                        (F64 => F64, f64)),
                    (F32 =>
                        (I8 => F32, f32),
                        (I16 => F32, f32),
                        (I32 => F32, f32),
                        (I64 => F64, f64),
                        (I128 => F64, f64),
                        (Isize => F32, f32),
                        (U8 => F32, f32),
                        (U16 => F32, f32),
                        (U32 => F32, f32),
                        (U64 => F64, f64),
                        (U128 => F64, f64),
                        (Usize => F32, f32),
                        (F32 => F32, f32),
                        (F64 => F64, f64)),
                    (F64 =>
                        (I8 => F64, f64),
                        (I16 => F64, f64),
                        (I32 => F64, f64),
                        (I64 => F64, f64),
                        (I128 => F64, f64),
                        (Isize => F64, f64),
                        (U8 => F64, f64),
                        (U16 => F64, f64),
                        (U32 => F64, f64),
                        (U64 => F64, f64),
                        (U128 => F64, f64),
                        (Usize => F64, f64),
                        (F32 => F64, f64),
                        (F64 => F64, f64))
                    )
            }
        }
        macro_rules! concatenate {
            ($v1: ident, $v2: ident) => {
                String::from($v1.to_string()+&*$v2.to_string())
            };
            ($v1: ident, $v2: ident => $e: ident, $t: ty) => {
                if let Ok(r2) = ($v1.to_string()+&*$v2.to_string()).parse::<$t>()
                    {Some(Value::$e(r2))} else {None}
            }
        }
        macro_rules! typecast_int {
            ($($from_num: ident),* => $enum_type: ident, $rs_type: ty) => {
                match self.clone() {
                    Value::$enum_type(..) => Some(self.clone()),
                    $(Value::$from_num(v) => Some(Value::$enum_type(v as $rs_type)),)*
                    Value::Str(v) => if let Ok(r) = v.parse::<$rs_type>()
                        {Some(Value::$enum_type(r))} else {None},
                    Value::Bool(v) => Some(Value::$enum_type(if v {1} else {0} as $rs_type)),
                    Value::Null => Some(Value::$enum_type(0 as $rs_type)),
                    _ => None
                }
            }
        }
        match type_ {
            OprType::Plus => case_arith!(Add::add),
            OprType::Minus => case_arith!(Sub::sub),
            OprType::AstMult | 
            OprType::DotMult | 
            OprType::CrossMult => {
                if let Value::Str(v1) = self.clone() {
                    if let Value::I32(v2) = other {
                        Some(Value::Str(v1.repeat(v2.try_into().ok()?)))
                    } else {case_arith!(Mul::mul)}
                } else if let Value::I32(v1) = self.clone() {
                    if let Value::Str(v2) = other {
                        Some(Value::Str(v2.repeat(v1.try_into().ok()?)))
                    } else {case_arith!(Mul::mul)}
                } else {case_arith!(Mul::mul)}
            }, // TODO implement for all number types
            OprType::Div |
            OprType::FractDiv => {
                if other == Value::I32(0) {
                    todo!("implement undefined type thing")
                }
                case_arith!(Div::div)
            },
            OprType::Modulo => case_arith!(Rem::rem),
            OprType::Concat => match self.clone() {
                Value::I8(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => I8, i8),
                    Value::I16(v2) => concatenate!(v1, v2 => I16, i16),
                    Value::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::U8(v2) => concatenate!(v1, v2 => I8, i8),
                    Value::U16(v2) => concatenate!(v1, v2 => I16, i16),
                    Value::U32(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::U64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Usize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::I16(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => I16, i16),
                    Value::I16(v2) => concatenate!(v1, v2 => I16, i16),
                    Value::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::U8(v2) => concatenate!(v1, v2 => I16, i16),
                    Value::U16(v2) => concatenate!(v1, v2 => I16, i16),
                    Value::U32(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::U64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Usize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::I32(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::I16(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::U8(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::U16(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::U32(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::U64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Usize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::I64(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I16(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I32(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::U8(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::U16(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::U32(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::U64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Usize(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::I128(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::I16(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::I32(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::I64(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::U8(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::U16(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::U32(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::U64(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Usize(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::Isize(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::I16(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::I32(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::U8(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::U16(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::U32(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::U64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Usize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::U8(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => I8, i8),
                    Value::I16(v2) => concatenate!(v1, v2 => I16, i16),
                    Value::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::U8(v2) => concatenate!(v1, v2 => U8, u8),
                    Value::U16(v2) => concatenate!(v1, v2 => U16, u16),
                    Value::U32(v2) => concatenate!(v1, v2 => U32, u32),
                    Value::U64(v2) => concatenate!(v1, v2 => U64, u64),
                    Value::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Value::Usize(v2) => concatenate!(v1, v2 => Usize, usize),
                    Value::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::U16(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => I16, i16),
                    Value::I16(v2) => concatenate!(v1, v2 => I16, i16),
                    Value::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::U8(v2) => concatenate!(v1, v2 => U16, u16),
                    Value::U16(v2) => concatenate!(v1, v2 => U16, u16),
                    Value::U32(v2) => concatenate!(v1, v2 => U32, u32),
                    Value::U64(v2) => concatenate!(v1, v2 => U64, u64),
                    Value::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Value::Usize(v2) => concatenate!(v1, v2 => Usize, usize),
                    Value::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::U32(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::I16(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Value::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::U8(v2) => concatenate!(v1, v2 => U32, u32),
                    Value::U16(v2) => concatenate!(v1, v2 => U32, u32),
                    Value::U32(v2) => concatenate!(v1, v2 => U32, u32),
                    Value::U64(v2) => concatenate!(v1, v2 => U64, u64),
                    Value::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Value::Usize(v2) => concatenate!(v1, v2 => Usize, usize),
                    Value::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::U64(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I16(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I32(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::U8(v2) => concatenate!(v1, v2 => U64, u64),
                    Value::U16(v2) => concatenate!(v1, v2 => U64, u64),
                    Value::U32(v2) => concatenate!(v1, v2 => U64, u64),
                    Value::U64(v2) => concatenate!(v1, v2 => U64, u64),
                    Value::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Value::Usize(v2) => concatenate!(v1, v2 => U64, u64),
                    Value::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::U128(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::I16(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::I32(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::I64(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::U8(v2) => concatenate!(v1, v2 => U128, u128),
                    Value::U16(v2) => concatenate!(v1, v2 => U128, u128),
                    Value::U32(v2) => concatenate!(v1, v2 => U128, u128),
                    Value::U64(v2) => concatenate!(v1, v2 => U128, u128),
                    Value::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Value::Usize(v2) => concatenate!(v1, v2 => U128, u128),
                    Value::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::Usize(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::I16(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::I32(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Value::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Value::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Value::U8(v2) => concatenate!(v1, v2 => Usize, usize),
                    Value::U16(v2) => concatenate!(v1, v2 => Usize, usize),
                    Value::U32(v2) => concatenate!(v1, v2 => Usize, usize),
                    Value::U64(v2) => concatenate!(v1, v2 => U64, u64),
                    Value::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Value::Usize(v2) => concatenate!(v1, v2 => Usize, usize),
                    Value::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::F32(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::I16(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::I32(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::I64(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::I128(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::Isize(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::U8(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::U16(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::U32(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::U64(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::U128(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::Usize(v2) => concatenate!(v1, v2 => F32, f32),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::F64(v1) => match other {
                    Value::I8(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::I16(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::I32(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::I64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::I128(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Isize(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::U8(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::U16(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::U32(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::U64(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::U128(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Usize(v2) => concatenate!(v1, v2 => F64, f64),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::Str(v1) => match other {
                    Value::I8(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::I16(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::I32(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::I64(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::I128(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::Isize(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::U8(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::U16(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::U32(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::U64(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::U128(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::Usize(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::F32(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::F64(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    Value::Bool(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Value::Bool(v1) => match other {
                    Value::Str(v2) => Some(Value::Str(concatenate!(v1, v2))),
                    _ => None
                },
                _ => None
            },
            OprType::TypeCast => match other {
                Value::Type(t) => match &*t.to_string() {
                    "i8" => typecast_int!(I16, I32, I64, I128, Isize, U8,
                        U16, U32, U64, U128, Usize, F32, F64 => I8, i8),
                    "i16" => typecast_int!(I8, I32, I64, I128, Isize, U8,
                        U16, U32, U64, U128, Usize, F32, F64 => I16, i16),
                    "i32" => typecast_int!(I8, I16, I64, I128, Isize, U8,
                        U16, U32, U64, U128, Usize, F32, F64 => I32, i32),
                    "i64" => typecast_int!(I8, I16, I32, I128, Isize, U8,
                        U16, U32, U64, U128, Usize, F32, F64 => I64, i64),
                    "i128" => typecast_int!(I8, I16, I32, I64, Isize, U8,
                        U16, U32, U64, U128, Usize, F32, F64 => I128, i128),
                    "isize" => typecast_int!(I8, I16, I32, I64, I128, U8,
                        U16, U32, U64, U128, Usize, F32, F64 => Isize, isize),
                    "u8" => typecast_int!(I8, I16, I32, I64, I128, Isize,
                        U16, U32, U64, U128, Usize, F32, F64 => U8, u8),
                    "u16" => typecast_int!(I8, I16, I32, I64, I128, Isize, U8,
                        U32, U64, U128, Usize, F32, F64 => U16, u16),
                    "u32" => typecast_int!(I8, I16, I32, I64, I128, Isize, U8,
                        U16, U64, U128, Usize, F32, F64 => U32, u32),
                    "u64" => typecast_int!(I8, I16, I32, I64, I128, Isize, U8,
                        U16, U32, U128, Usize, F32, F64 => U64, u64),
                    "u128" => typecast_int!(I8, I16, I32, I64, I128, Isize, U8,
                        U16, U32, U64, Usize, F32, F64 => U128, u128),
                    "usize" => typecast_int!(I8, I16, I32, I64, I128, Isize, U8,
                        U16, U32, U64, U128, F32, F64 => Usize, usize),
                    "f32" => typecast_int!(I8, I16, I32, I64, I128, Isize, U8,
                        U16, U32, U64, U128, Usize, F64 => F32, f32),
                    "f64" => typecast_int!(I8, I16, I32, I64, I128, Isize, U8,
                        U16, U32, U64, U128, Usize, F32 => F64, f64),
                    "str" => Some(Value::Str(self.to_string())),
                    "bool" => match self.clone() {
                        Value::I32(v) => Some(Value::Bool(v != 0)),
                        Value::F64(v) => Some(Value::Bool(v != 0.0)),
                        Value::Str(v) => Some(Value::Bool(!v.is_empty())),
                        Value::Bool(..) => Some(self.clone()),
                        Value::Type(..) => Some(Value::Bool(true)),
                        Value::Null => Some(Value::Bool(false)),
                        _ => None
                    }
                    "type" => Some(self.get_type()),
                    _ => None
                },
                _ => None
            }
            _ => None
        }
    }
    pub fn default(type_: Type) -> Result<Self, ZyxtError> {
        match type_.clone() {
            Type::Instance {name, ..} => Ok(match &*name {
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
                "#null" => Value::Null,
                "type" => Value::Type(Type::null()),
                _ => panic!("{:#?}", type_)
            }),
            _ => panic!()
        }
    }
    pub fn from_type_content(type_: Type, content: String) -> Value {
        match type_ {
            Type::Instance {name, ..} => match &*name {
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
                "f32" => Value::F32(content.parse::<f32>().unwrap()),
                "f64" => Value::F64(content.parse::<f64>().unwrap()),
                "str" => Value::Str(content),
                "bool" => Value::Bool(&*content == "true"),
                _ => panic!()
            }
            _ => panic!()
        }
    }
    pub fn get_type_obj(&self) -> Type {
        match self {
            Value::I8(..) => Type::from_str("i8"),
            Value::I16(..) => Type::from_str("i16"),
            Value::I32(..) => Type::from_str("i32"),
            Value::I64(..) => Type::from_str("i64"),
            Value::I128(..) => Type::from_str("i128"),
            Value::Isize(..) => Type::from_str("isize"),
            Value::Ibig(..) => Type::from_str("ibig"),
            Value::U8(..) => Type::from_str("u8"),
            Value::U16(..) => Type::from_str("u16"),
            Value::U32(..) => Type::from_str("u32"),
            Value::U64(..) => Type::from_str("u64"),
            Value::U128(..) => Type::from_str("u128"),
            Value::Usize(..) => Type::from_str("usize"),
            Value::Ubig(..) => Type::from_str("ubig"),
            Value::F32(..) => Type::from_str("f32"),
            Value::F64(..) => Type::from_str("f64"),
            Value::Str(..) => Type::from_str("str"),
            Value::Bool(..) => Type::from_str("bool"),
            Value::Type(..) => Type::from_str("type"),
            Value::Proc {is_fn, return_type, ..} =>
                Type::Instance {
                    name: if *is_fn {"fn"} else {"proc"}.to_string(),
                    type_args: vec![Type::null(), return_type.clone()],
                    inst_attrs: Default::default(),
                    implementation: None
                }, // TODO angle bracket thingy when it is implemented
            Value::ClassInstance{type_, ..} => type_.clone(),
            Value::Null => Type::null(),
            Value::Return(v) => v.get_type_obj()
        }
    }
    pub fn get_type(&self) -> Value {
        Value::Type(self.get_type_obj())
    }
    pub fn as_element(&self) -> Element {
        macro_rules! to_literal {
            ($v: ident) => {
                Element::Literal {
                    position: Default::default(),
                    raw: $v.to_string(),
                    type_: self.get_type_obj(),
                    content: $v.to_string()
                }
            }
        }
        match self {
            Value::I8(v) => to_literal!(v),
            Value::I16(v) => to_literal!(v),
            Value::I32(v) => to_literal!(v),
            Value::I64(v) => to_literal!(v),
            Value::I128(v) => to_literal!(v),
            Value::Isize(v) => to_literal!(v),
            Value::Ibig(v) => to_literal!(v),
            Value::U8(v) => to_literal!(v),
            Value::U16(v) => to_literal!(v),
            Value::U32(v) => to_literal!(v),
            Value::U64(v) => to_literal!(v),
            Value::U128(v) => to_literal!(v),
            Value::Usize(v) => to_literal!(v),
            Value::Ubig(v) => to_literal!(v),
            Value::F32(v) => to_literal!(v),
            Value::F64(v) => to_literal!(v),
            Value::Str(v) => to_literal!(v),
            Value::Bool(v) => to_literal!(v),
            Value::Type(v) => to_literal!(v),
            Value::Proc {is_fn, args, return_type, content} => Element::Procedure {
                position: Default::default(),
                raw: "".to_string(),
                is_fn: *is_fn,
                args: args.clone(),
                return_type: return_type.clone(),
                content: content.clone()
            },
            Value::Null => Element::NullElement,
            Value::Return(v) => Element::Return {
                position: Default::default(),
                raw: "".to_string(),
                value: Box::new(v.as_element())
            },
            Value::ClassInstance{..} => todo!()
        }
    }
}
