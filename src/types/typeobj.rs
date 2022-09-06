pub mod i128_t;
pub mod i16_t;
pub mod i32_t;
pub mod i64_t;
pub mod i8_t;
pub mod ibig_t;
pub mod isize_t;
pub mod macros;
pub mod u128_t;
pub mod u16_t;
pub mod u32_t;
pub mod u64_t;
pub mod u8_t;
pub mod ubig_t;
pub mod usize_t;
pub mod str_t;
pub mod bool_t;
pub mod type_t;
pub mod f32_t;
pub mod f64_t;
pub mod f16_t;
pub mod unit_t;

use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use smol_str::SmolStr;

use crate::{types::element::Argument, Element};
use crate::types::typeobj::type_t::TYPE_T;
use crate::types::typeobj::str_t::STR_T;
use crate::types::typeobj::bool_t::BOOL_T;
use crate::types::typeobj::unit_t::UNIT_T;
use crate::types::value::Value;

#[derive(Clone, Debug)]
pub enum Type {
    Instance {
        // str, bool, cpx<int> etc. Is of type Typedef
        name: Option<SmolStr>,
        type_args: Vec<Type>,
        fields: HashMap<SmolStr, Value>,
        implementation: Box<Type>,
    },
    Definition {
        // class, struct, (anything that implements a Type). Is of type <type> (Typedef)
        name: Option<SmolStr>, // TODO inheritance
        generics: Vec<Argument>,
        implementations: HashMap<SmolStr, Value>,
        inst_fields: HashMap<SmolStr, Option<Value>>,
    },
    Any,
    Return(Box<Type>),
}

impl Display for Type {
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
                            name.unwrap_or_else(|| "{unknown}".into()),
                            type_args
                                .iter()
                                .map(|arg| format!("{}", arg))
                                .collect::<Vec<String>>()
                                .join(", ")
                        )
                    } else {
                        name.to_string()
                    },
                Type::Definition { name, .. } => name.to_owned().into(),
                Type::Any => "_any".into(),
                Type::Return(ty) => format!("{}", ty),
            }
        )
    }
}
impl Type {
    pub fn as_element(&self) -> Element {
        match self {
            Type::Instance { name, .. } => Element::Ident {
                position: Default::default(),
                name: name.to_owned().unwrap_or_default(), // TODO type args
                raw: self.to_string(),
                parent: Box::new(Element::NullElement),
            },
            Type::Definition { .. } => todo!(),
            Type::Any => todo!(),
            Type::Return(ty) => ty.as_element(),
        }
    }
    pub fn get_field(&self, attr: SmolStr) -> Option<&Value> {
        match &self {
            Type::Instance {
                fields, ..
            } => fields.get(&attr),
            Type::Definition {
                implementations, ..
            } => implementations.get(&attr),
            Type::Any => None,
            Type::Return(ty) => ty.get_field(attr)
        }
    }
    pub fn implementation(&self) -> &Type {
        match &self {
            Type::Instance {
                implementation, ..
            } => implementation,
            Type::Definition { .. } => &TYPE_T,
            Type::Any => &UNIT_T,
            Type::Return(ty) => ty.implementation()
        }
    }
}
