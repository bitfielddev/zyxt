use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    sync::Arc,
};

use itertools::Either;
use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    ast::Ident,
    primitives::{ANY_T, ANY_T_VAL},
    types::value::Value,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Any,
    Type {
        name: Option<Ident>,
        namespace: HashMap<SmolStr, Arc<Type>>,
        fields: HashMap<SmolStr, Arc<Type>>,
        type_args: Vec<(SmolStr, Arc<Type>)>,
    },
    Generic {
        type_args: Vec<(SmolStr, Either<Value, Arc<Type>>)>,
        base: Arc<Type>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    Any,
    Type {
        name: Option<Ident>,
        namespace: HashMap<SmolStr, Value>,
        fields: HashMap<SmolStr, Arc<Type>>,
        type_args: Vec<(SmolStr, Value)>,
    },
}

impl From<ValueType> for Type {
    fn from(value: ValueType) -> Self {
        todo!()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltinType {
    pub name: Option<Ident>,
    pub namespace: HashMap<SmolStr, Value>,
    pub fields: HashMap<SmolStr, Arc<Type>>,
    pub type_args: Vec<(SmolStr, Arc<Type>)>,
}
impl From<BuiltinType> for Type {
    fn from(value: BuiltinType) -> Self {
        Self::Type {
            name: value.name,
            namespace: value
                .namespace
                .into_iter()
                .map(|(k, v)| (k, v.ty()))
                .collect(),
            fields: value.fields,
            type_args: vec![],
        }
    }
}
impl From<BuiltinType> for ValueType {
    fn from(value: BuiltinType) -> Self {
        Self::Type {
            name: value.name,
            namespace: value.namespace,
            fields: value.fields,
            type_args: value
                .type_args
                .into_iter()
                .map(|(k, _)| (k, Value::Type(Arc::clone(&ANY_T_VAL))))
                .collect(),
        }
    }
}
