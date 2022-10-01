use smol_str::SmolStr;

use crate::{
    types::element::{ElementData, ElementVariant},
    Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Comment {
    content: SmolStr,
}

impl ElementData for Comment {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Comment(self.to_owned())
    }
}
