use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use crate::{Element, Varstack, ZyxtError};
use crate::objects::element::Argument;
use crate::objects::token::OprType;
use crate::objects::typeobj::TypeObj;

#[derive(Clone, PartialEq)]
pub enum Variable {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    F32(f32),
    F64(f64),
    Str(String),
    Bool(bool),
    Type(TypeObj),
    Proc{
        is_fn: bool,
        args: Vec<Argument>,
        return_type: TypeObj,
        content: Vec<Element>
    },
    Null,
    Return(Box<Variable>)
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Variable::I8(v) => v.to_string(),
            Variable::I16(v) => v.to_string(),
            Variable::I32(v) => v.to_string(),
            Variable::I64(v) => v.to_string(),
            Variable::I128(v) => v.to_string(),
            Variable::Isize(v) => v.to_string(),
            Variable::U8(v) => v.to_string(),
            Variable::U16(v) => v.to_string(),
            Variable::U32(v) => v.to_string(),
            Variable::U64(v) => v.to_string(),
            Variable::U128(v) => v.to_string(),
            Variable::Usize(v) => v.to_string(),
            Variable::F32(v) => v.to_string(),
            Variable::F64(v) => v.to_string(),
            Variable::Str(v) => v.clone(),
            Variable::Bool(v) => v.to_string(),
            Variable::Type(v) => "<".to_owned()+&*v.to_string()+">",
            Variable::Proc{is_fn, args, return_type, ..} =>
                format!("{}|{}|: {}",
                    if *is_fn {"fn"} else {"proc"},
                    args.iter().map(|a| format!("{}{}{}",
                        a.name, if a.type_ != TypeObj::any()
                        {": ".to_owned()+&*a.type_.to_string()} else {"".to_string()},
                        if let Some(_) = &a.default {": TODO"} else {""}
                    )).collect::<Vec<String>>().join(","),
                        return_type.to_string()),
            Variable::Null => "null".to_string(),
            Variable::Return(v) => v.to_string()
        })
    }
}

impl Variable {
    pub fn call(&self, args: Vec<Variable>) -> Option<Variable> {
        if args.len() == 1 {
        macro_rules! mult {
            () => {self.bin_opr(&OprType::AstMult, args.get(0)?.clone())}
        }
            match self {
                Variable::I8(_) => mult!(),
                Variable::I16(_) => mult!(),
                Variable::I32(_) => mult!(),
                Variable::I64(_) => mult!(),
                Variable::I128(_) => mult!(),
                Variable::Isize(_) => mult!(),
                Variable::U8(_) => mult!(),
                Variable::U16(_) => mult!(),
                Variable::U32(_) => mult!(),
                Variable::U64(_) => mult!(),
                Variable::U128(_) => mult!(),
                Variable::Usize(_) => mult!(),
                Variable::F32(_) => mult!(),
                Variable::F64(_) => mult!(),
                Variable::Proc{..} => panic!(),
                Variable::Return(v) => v.call(args),
                _ => None
            }
        } else {None}
    }
    pub fn un_opr(&self, type_: &OprType) -> Option<Variable> {
        if let Variable::Return(v) = self {return v.un_opr(type_)}
        macro_rules! case {
            ($opr: expr => $($var_type: ident),*) => {
                match *self {
                    $(Variable::$var_type(v) => Some(Variable::$var_type($opr(v))),)*
                    _ => None
                }
            };
            ($($var_type: ident),*) => {
                match *self {
                    $(Variable::$var_type(v) => Some(Variable::$var_type(v)),)*
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
    pub fn bin_opr(&self, type_: &OprType, other: Variable) -> Option<Variable> {
        if let Variable::Return(v) = self {return v.bin_opr(type_, other)}
        macro_rules! case {
            ($opr: expr => $((
                $var_type1: ident => $(($var_type2: ident => $return_type: ident, $rs_type: ty)),*
            )),*) => {
                match *self {
                    $(Variable::$var_type1(v1) => match other {
                        $(Variable::$var_type2(v2) => Some(Variable::$return_type($opr(v1 as $rs_type, v2 as $rs_type) as $rs_type)),)*
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
                    {Some(Variable::$e(r2))} else {None}
            }
        }
        macro_rules! typecast_int {
            ($($from_num: ident),* => $enum_type: ident, $rs_type: ty) => {
                match self.clone() {
                    Variable::$enum_type(..) => Some(self.clone()),
                    $(Variable::$from_num(v) => Some(Variable::$enum_type(v as $rs_type)),)*
                    Variable::Str(v) => if let Ok(r) = v.parse::<$rs_type>()
                        {Some(Variable::$enum_type(r))} else {None},
                    Variable::Bool(v) => Some(Variable::$enum_type(if v {1} else {0} as $rs_type)),
                    Variable::Null => Some(Variable::$enum_type(0 as $rs_type)),
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
                if let Variable::Str(v1) = self.clone() {
                    if let Variable::I32(v2) = other {
                        Some(Variable::Str(v1.repeat(v2.try_into().ok()?)))
                    } else {case_arith!(Mul::mul)}
                } else if let Variable::I32(v1) = self.clone() {
                    if let Variable::Str(v2) = other {
                        Some(Variable::Str(v2.repeat(v1.try_into().ok()?)))
                    } else {case_arith!(Mul::mul)}
                } else {case_arith!(Mul::mul)}
            }, // TODO implement for all number types
            OprType::Div |
            OprType::FractDiv => {
                if other == Variable::I32(0) {
                    todo!("implement undefined type thing")
                }
                case_arith!(Div::div)
            },
            OprType::Modulo => case_arith!(Rem::rem),
            OprType::Concat => match self.clone() {
                Variable::I8(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => I8, i8),
                    Variable::I16(v2) => concatenate!(v1, v2 => I16, i16),
                    Variable::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::U8(v2) => concatenate!(v1, v2 => I8, i8),
                    Variable::U16(v2) => concatenate!(v1, v2 => I16, i16),
                    Variable::U32(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::U64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::I16(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => I16, i16),
                    Variable::I16(v2) => concatenate!(v1, v2 => I16, i16),
                    Variable::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::U8(v2) => concatenate!(v1, v2 => I16, i16),
                    Variable::U16(v2) => concatenate!(v1, v2 => I16, i16),
                    Variable::U32(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::U64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::I32(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::I16(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::U8(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::U16(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::U32(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::U64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::I64(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I16(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I32(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::U8(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::U16(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::U32(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::U64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::I128(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::I16(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::I32(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::I64(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::U8(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::U16(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::U32(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::U64(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::Isize(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::I16(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::I32(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::U8(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::U16(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::U32(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::U64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::U128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::U8(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => I8, i8),
                    Variable::I16(v2) => concatenate!(v1, v2 => I16, i16),
                    Variable::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::U8(v2) => concatenate!(v1, v2 => U8, u8),
                    Variable::U16(v2) => concatenate!(v1, v2 => U16, u16),
                    Variable::U32(v2) => concatenate!(v1, v2 => U32, u32),
                    Variable::U64(v2) => concatenate!(v1, v2 => U64, u64),
                    Variable::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => Usize, usize),
                    Variable::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::U16(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => I16, i16),
                    Variable::I16(v2) => concatenate!(v1, v2 => I16, i16),
                    Variable::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::U8(v2) => concatenate!(v1, v2 => U16, u16),
                    Variable::U16(v2) => concatenate!(v1, v2 => U16, u16),
                    Variable::U32(v2) => concatenate!(v1, v2 => U32, u32),
                    Variable::U64(v2) => concatenate!(v1, v2 => U64, u64),
                    Variable::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => Usize, usize),
                    Variable::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::U32(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::I16(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::I32(v2) => concatenate!(v1, v2 => I32, i32),
                    Variable::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::U8(v2) => concatenate!(v1, v2 => U32, u32),
                    Variable::U16(v2) => concatenate!(v1, v2 => U32, u32),
                    Variable::U32(v2) => concatenate!(v1, v2 => U32, u32),
                    Variable::U64(v2) => concatenate!(v1, v2 => U64, u64),
                    Variable::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => Usize, usize),
                    Variable::F32(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::U64(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I16(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I32(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::U8(v2) => concatenate!(v1, v2 => U64, u64),
                    Variable::U16(v2) => concatenate!(v1, v2 => U64, u64),
                    Variable::U32(v2) => concatenate!(v1, v2 => U64, u64),
                    Variable::U64(v2) => concatenate!(v1, v2 => U64, u64),
                    Variable::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => U64, u64),
                    Variable::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::U128(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::I16(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::I32(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::I64(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::U8(v2) => concatenate!(v1, v2 => U128, u128),
                    Variable::U16(v2) => concatenate!(v1, v2 => U128, u128),
                    Variable::U32(v2) => concatenate!(v1, v2 => U128, u128),
                    Variable::U64(v2) => concatenate!(v1, v2 => U128, u128),
                    Variable::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => U128, u128),
                    Variable::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::Usize(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::I16(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::I32(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::I64(v2) => concatenate!(v1, v2 => I64, i64),
                    Variable::I128(v2) => concatenate!(v1, v2 => I128, i128),
                    Variable::Isize(v2) => concatenate!(v1, v2 => Isize, isize),
                    Variable::U8(v2) => concatenate!(v1, v2 => Usize, usize),
                    Variable::U16(v2) => concatenate!(v1, v2 => Usize, usize),
                    Variable::U32(v2) => concatenate!(v1, v2 => Usize, usize),
                    Variable::U64(v2) => concatenate!(v1, v2 => U64, u64),
                    Variable::U128(v2) => concatenate!(v1, v2 => U128, u128),
                    Variable::Usize(v2) => concatenate!(v1, v2 => Usize, usize),
                    Variable::F32(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::F64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::F32(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::I16(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::I32(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::I64(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::I128(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::Isize(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::U8(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::U16(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::U32(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::U64(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::U128(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::Usize(v2) => concatenate!(v1, v2 => F32, f32),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::F64(v1) => match other {
                    Variable::I8(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::I16(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::I32(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::I64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::I128(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Isize(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::U8(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::U16(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::U32(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::U64(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::U128(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Usize(v2) => concatenate!(v1, v2 => F64, f64),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::Str(v1) => match other {
                    Variable::I8(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::I16(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::I32(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::I64(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::I128(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::Isize(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::U8(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::U16(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::U32(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::U64(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::U128(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::Usize(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::F32(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::F64(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    Variable::Bool(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                Variable::Bool(v1) => match other {
                    Variable::Str(v2) => Some(Variable::Str(concatenate!(v1, v2))),
                    _ => None
                },
                _ => None
            },
            OprType::TypeCast => match other {
                Variable::Type(t) => match &*t.to_string() {
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
                    "str" => Some(Variable::Str(self.to_string())),
                    "bool" => match self.clone() {
                        Variable::I32(v) => Some(Variable::Bool(v != 0)),
                        Variable::F64(v) => Some(Variable::Bool(v != 0.0)),
                        Variable::Str(v) => Some(Variable::Bool(v.len() != 0)),
                        Variable::Bool(..) => Some(self.clone()),
                        Variable::Type(..) => Some(Variable::Bool(true)),
                        Variable::Null => Some(Variable::Bool(false)),
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
    pub fn default(type_: TypeObj, typelist: &mut Varstack<TypeObj>) -> Result<Self, ZyxtError> {
        match type_.clone() {
            TypeObj::Prim{name, ..} => Ok(match &*name {
                "i8" => Variable::I8(0),
                "i16" => Variable::I16(0),
                "i32" => Variable::I32(0),
                "i64" => Variable::I64(0),
                "i128" => Variable::I128(0),
                "isize" => Variable::Isize(0),
                "u8" => Variable::U8(0),
                "u16" => Variable::U16(0),
                "u32" => Variable::U32(0),
                "u64" => Variable::U64(0),
                "u128" => Variable::U128(0),
                "usize" => Variable::Usize(0),
                "f32" => Variable::F32(0.0),
                "f64" => Variable::F64(0.0),
                "str" => Variable::Str("".to_string()),
                "bool" => Variable::Bool(false),
                "#null" => Variable::Null,
                "type" => Variable::Type(TypeObj::null()),
                _ => panic!("{:#?}", type_)
            }),
            TypeObj::Compound(mut ele) => {
                Variable::default(ele.get_type(typelist)?, typelist)
            }
        }
    }
    pub fn from_type_content(type_: TypeObj, content: String) -> Variable {
        match type_ {
            TypeObj::Prim{name, ..} => match &*name {
                "i8" => Variable::I8(content.parse::<i8>().unwrap()),
                "i16" => Variable::I16(content.parse::<i16>().unwrap()),
                "i32" => Variable::I32(content.parse::<i32>().unwrap()),
                "i64" => Variable::I64(content.parse::<i64>().unwrap()),
                "i128" => Variable::I128(content.parse::<i128>().unwrap()),
                "isize" => Variable::Isize(content.parse::<isize>().unwrap()),
                "u8" => Variable::U8(content.parse::<u8>().unwrap()),
                "u16" => Variable::U16(content.parse::<u16>().unwrap()),
                "u32" => Variable::U32(content.parse::<u32>().unwrap()),
                "u64" => Variable::U64(content.parse::<u64>().unwrap()),
                "u128" => Variable::U128(content.parse::<u128>().unwrap()),
                "usize" => Variable::Usize(content.parse::<usize>().unwrap()),
                "f32" => Variable::F32(content.parse::<f32>().unwrap()),
                "f64" => Variable::F64(content.parse::<f64>().unwrap()),
                "str" => Variable::Str(content),
                "bool" => Variable::Bool(&*content == "true"),
                _ => panic!()
            }
            _ => panic!()
        }
    }
    pub fn get_type_obj(&self) -> TypeObj {
        if let Variable::Return(v) = self {
            return v.get_type_obj();
        }
        match self {
            Variable::I8(..) => TypeObj::from_str("i8"),
            Variable::I16(..) => TypeObj::from_str("i16"),
            Variable::I32(..) => TypeObj::from_str("i32"),
            Variable::I64(..) => TypeObj::from_str("i64"),
            Variable::I128(..) => TypeObj::from_str("i128"),
            Variable::Isize(..) => TypeObj::from_str("isize"),
            Variable::U8(..) => TypeObj::from_str("u8"),
            Variable::U16(..) => TypeObj::from_str("u16"),
            Variable::U32(..) => TypeObj::from_str("u32"),
            Variable::U64(..) => TypeObj::from_str("u64"),
            Variable::U128(..) => TypeObj::from_str("u128"),
            Variable::Usize(..) => TypeObj::from_str("usize"),
            Variable::F32(..) => TypeObj::from_str("f32"),
            Variable::F64(..) => TypeObj::from_str("f64"),
            Variable::Str(..) => TypeObj::from_str("str"),
            Variable::Bool(..) => TypeObj::from_str("bool"),
            Variable::Type(..) => TypeObj::from_str("type"),
            Variable::Proc {is_fn, return_type, ..} =>
                TypeObj::Prim{
                    name: if *is_fn {"fn"} else {"proc"}.to_string(),
                    type_args: vec![TypeObj::null(), return_type.clone()]
                }, // TODO angle bracket thingy when it is implemented
            Variable::Null => TypeObj::null(),
            _ => panic!()
        }
    }
    pub fn get_type(&self) -> Variable {
        Variable::Type(self.get_type_obj())
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
            Variable::I8(v) => to_literal!(v),
            Variable::I16(v) => to_literal!(v),
            Variable::I32(v) => to_literal!(v),
            Variable::I64(v) => to_literal!(v),
            Variable::I128(v) => to_literal!(v),
            Variable::Isize(v) => to_literal!(v),
            Variable::U8(v) => to_literal!(v),
            Variable::U16(v) => to_literal!(v),
            Variable::U32(v) => to_literal!(v),
            Variable::U64(v) => to_literal!(v),
            Variable::U128(v) => to_literal!(v),
            Variable::Usize(v) => to_literal!(v),
            Variable::F32(v) => to_literal!(v),
            Variable::F64(v) => to_literal!(v),
            Variable::Str(v) => to_literal!(v),
            Variable::Bool(v) => to_literal!(v),
            Variable::Type(v) => to_literal!(v),
            Variable::Proc {is_fn, args, return_type, content} => Element::Procedure {
                position: Default::default(),
                raw: "".to_string(),
                is_fn: *is_fn,
                args: args.clone(),
                return_type: return_type.clone(),
                content: content.clone()
            },
            Variable::Null => Element::NullElement,
            Variable::Return(v) => Element::Return {
                position: Default::default(),
                raw: "".to_string(),
                value: Box::new(v.as_element())
            }
        }
    }
}
