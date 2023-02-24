use std::fmt::{Display, Formatter};

use crate::{
    ast::{Ast, AstData, Ident, Reconstruct},
    errors::ZResult,
    types::position::{GetSpan, Span},
};

#[derive(Clone, PartialEq, Debug)]
pub struct Argument {
    pub name: Ident,
    pub ty: Box<Ast>,
    pub default: Option<Ast>,
}

impl GetSpan for Argument {
    fn span(&self) -> Option<Span> {
        self.name.merge_span(&self.ty).merge_span(&self.default)
    }
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        /*write!(
            f,
            "{}{}{}",
            self.name.name,
            if self.ty.span().raw != "_any" {
                // TODO
                format!(": {}", self.ty.span.raw)
            } else {
                "".to_string()
            },
            if let Some(r) = &self.default {
                format!(": {}", r.span.raw.trim())
            } else {
                "".to_string()
            }
        )*/
        write!(f, "")
    }
}

impl Argument {
    pub fn desugar(&mut self) -> ZResult<()> {
        self.default = self.default.as_ref().map(AstData::desugared).transpose()?;
        Ok(())
    }
}

impl Reconstruct for Argument {
    fn reconstruct(&self) -> String {
        format!(
            "{}: {}: {}",
            self.name.reconstruct(),
            self.ty.reconstruct(),
            self.default
                .as_ref()
                .map_or(String::new(), |a| a.reconstruct())
        )
    }
}
