use smol_str::SmolStr;

use crate::types::element::{ElementData, ElementVariant};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Comment {
    content: SmolStr,
}

impl ElementData for Comment {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Comment(self.to_owned())
    }
}
