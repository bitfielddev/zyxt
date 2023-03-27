use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use crate::{
    ast::{Ast, AstData, Ident, Reconstruct},
    errors::ZResult,
    types::{
        position::{GetSpan, Span},
        r#type::Type,
        sym_table::TypeCheckSymTable,
    },
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
        write!(f, "{}", self.name.name)?;
        write!(
            f,
            ": {}",
            if self.name.name == "_any" {
                ""
            } else {
                &self.name.name
            }
        )?;
        if let Some(r) = &self.default {
            write!(f, ": {}", r.reconstruct())?;
        }
        Ok(())
    }
}

impl Argument {
    pub fn desugar(&mut self) -> ZResult<&mut Self> {
        self.default = self.default.as_ref().map(AstData::desugared).transpose()?;
        Ok(self)
    }
    pub fn type_check(&mut self, ty_symt: &mut TypeCheckSymTable) -> ZResult<Arc<Type>> {
        let ty1 = Arc::clone(self.ty.type_check(ty_symt)?.as_const()?);
        if let Some(default) = &mut self.default {
            let ty2 = default.type_check(ty_symt)?;
            if !Arc::ptr_eq(&ty1, &ty2) {
                todo!()
            }
        }
        Ok(ty1)
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
                .map_or(String::new(), Reconstruct::reconstruct)
        )
    }
}
