pub mod bool_t;
pub mod f16_t;
pub mod f32_t;
pub mod f64_t;
pub mod i128_t;
pub mod i16_t;
pub mod i32_t;
pub mod i64_t;
pub mod i8_t;
pub mod ibig_t;
pub mod isize_t;
pub mod macros;
pub mod proc_t;
pub mod str_t;
pub mod type_t;
pub mod u128_t;
pub mod u16_t;
pub mod u32_t;
pub mod u64_t;
pub mod u8_t;
pub mod ubig_t;
pub mod unit_t;
pub mod usize_t;

use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use itertools::Itertools;
use smol_str::SmolStr;

use crate::{
    interpreter::interpret_expr,
    types::{
        element::Argument,
        typeobj::{
            type_t::{TYPE_T, TYPE_T_ELE},
            unit_t::{UNIT_T, UNIT_T_ELE},
        },
    },
    Element, InterpreterData, Print, Value, ZyxtError,
};

#[derive(Clone, PartialEq)]
pub enum Type<T: Clone + PartialEq + Debug> {
    Instance {
        // str, bool, cpx<int> etc. Is of type Typedef
        name: Option<SmolStr>,
        type_args: Vec<Type<T>>,
        implementation: Box<Type<T>>,
    },
    Definition {
        // class, struct, (anything that implements a Type). Is of type <type> (Typedef)
        inst_name: Option<SmolStr>, // TODO inheritance
        name: Option<SmolStr>,
        generics: Vec<Argument>,
        implementations: HashMap<SmolStr, T>,
        inst_fields: HashMap<SmolStr, (Box<Type<T>>, Option<T>)>,
    },
    Any,
    Return(Box<Type<T>>),
}
impl<T: Clone + PartialEq + Debug> Debug for Type<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Instance { implementation, .. } => {
                write!(f, "{} (implementation: {:?})", self, implementation)
            }
            Type::Definition {
                implementations,
                inst_fields,
                ..
            } => {
                write!(
                    f,
                    "{} for {} (implementations: {{{}}}; fields: {{{}}})",
                    self,
                    self.get_instance()
                        .map(|a| a.to_string())
                        .unwrap_or_else(|| "Unknown".into()),
                    implementations.iter().map(|(k, _)| k).join(", "),
                    inst_fields.iter().map(|(k, _)| k).join(", ")
                )
            }
            Type::Any => write!(f, "_any"),
            Type::Return(t) => <Self as Debug>::fmt(t, f),
        }
    }
}

impl<T: Clone + PartialEq + Debug> Display for Type<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Instance {
                    name, type_args, ..
                } =>
                    if !type_args.is_empty() {
                        format!(
                            "{}<{}>",
                            name.as_ref().unwrap_or(&"{unknown}".into()),
                            type_args
                                .iter()
                                .map(|arg| format!("{}", arg))
                                .collect::<Vec<String>>()
                                .join(", ")
                        )
                    } else {
                        name.as_ref().unwrap_or(&"{unknown}".into()).to_string()
                    },
                Type::Definition { name, .. } =>
                    name.to_owned().unwrap_or_else(|| "{unknown}".into()).into(),
                Type::Any => "_any".into(),
                Type::Return(ty) => format!("{}", ty),
            }
        )
    }
}

impl<T: Clone + PartialEq + Debug> Type<T> {
    pub fn get_instance(&self) -> Option<Type<T>> {
        match &self {
            Type::Instance { .. } => None,
            Type::Return(t) => t.get_instance(),
            Type::Any => None,
            Type::Definition { inst_name, .. } => Some(Type::Instance {
                name: inst_name.to_owned(),
                type_args: vec![],
                implementation: Box::new(self.to_owned()),
            }),
        }
    }
}

impl Type<Element> {
    pub fn as_literal(&self) -> Element {
        Element::Literal {
            position: Default::default(),
            raw: "".into(),
            content: Value::PreType(self.to_owned()),
        }
    }
    pub fn implementation(&self) -> &Type<Element> {
        match &self {
            Type::Instance { implementation, .. } => implementation,
            Type::Definition { .. } => &TYPE_T_ELE,
            Type::Any => &UNIT_T_ELE,
            Type::Return(ty) => ty.implementation(),
        }
    }
    pub fn as_type_value(
        &self,
        i_data: &mut InterpreterData<Value, impl Print>,
    ) -> Result<Type<Value>, ZyxtError> {
        Ok(match &self {
            Type::Instance {
                name,
                type_args,
                implementation,
            } => Type::Instance {
                name: name.to_owned(),
                type_args: type_args
                    .iter()
                    .map(|a| a.as_type_value(i_data))
                    .collect::<Result<Vec<_>, _>>()?,
                implementation: Box::new(implementation.as_type_value(i_data)?),
            },
            Type::Definition {
                inst_name,
                name,
                generics,
                implementations,
                inst_fields,
            } => Type::Definition {
                inst_name: inst_name.to_owned(),
                name: name.to_owned(),
                generics: generics.to_owned(),
                implementations: implementations
                    .iter()
                    .map(|(k, v)| Ok((k.to_owned(), interpret_expr(v, i_data)?)))
                    .collect::<Result<HashMap<_, _>, _>>()?,
                inst_fields: inst_fields
                    .iter()
                    .map(|(k, (v1, v2))| {
                        Ok((
                            k.to_owned(),
                            (
                                Box::new(v1.as_type_value(i_data)?),
                                v2.to_owned()
                                    .map(|v2| interpret_expr(&v2, i_data))
                                    .transpose()?,
                            ),
                        ))
                    })
                    .collect::<Result<HashMap<_, _>, _>>()?,
            },
            Type::Any => Type::Any,
            Type::Return(t) => Type::Return(Box::new(t.as_type_value(i_data)?)),
        })
    }
}

impl Type<Value> {
    pub fn implementation(&self) -> &Type<Value> {
        match &self {
            Type::Instance { implementation, .. } => implementation,
            Type::Definition { .. } => &*TYPE_T,
            Type::Any => &*UNIT_T,
            Type::Return(ty) => ty.implementation(),
        }
    }
    pub fn as_type_element(&self) -> Type<Element> {
        match &self {
            Type::Instance {
                name,
                type_args,
                implementation,
            } => Type::Instance {
                name: name.to_owned(),
                type_args: type_args.iter().map(|a| a.as_type_element()).collect(),
                implementation: Box::new(implementation.as_type_element()),
            },
            Type::Definition {
                inst_name,
                name,
                generics,
                implementations,
                inst_fields,
            } => Type::Definition {
                inst_name: inst_name.to_owned(),
                name: name.to_owned(),
                generics: generics.to_owned(),
                implementations: implementations
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.to_owned(),
                            Element::Literal {
                                position: Default::default(),
                                raw: "".into(),
                                content: v.to_owned(),
                            },
                        )
                    })
                    .collect(),
                inst_fields: inst_fields
                    .iter()
                    .map(|(k, (v1, v2))| {
                        (
                            k.to_owned(),
                            (
                                Box::new(v1.as_type_element()),
                                v2.to_owned().map(|v2| Element::Literal {
                                    position: Default::default(),
                                    raw: "".into(),
                                    content: v2,
                                }),
                            ),
                        )
                    })
                    .collect(),
            },
            Type::Any => Type::Any,
            Type::Return(t) => Type::Return(Box::new(t.as_type_element())),
        }
    }
}
