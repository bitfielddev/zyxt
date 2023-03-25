use std::sync::Arc;

use smol_str::SmolStr;

use crate::{
    ast::{Ast, AstData, Reconstruct},
    primitives::TYPE_T,
    types::{
        position::{GetSpan, Span},
        token::AccessType,
    },
    InterpretSymTable, Type, TypecheckSymTable, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Member {
    pub ty: AccessType,
    pub name: SmolStr,
    pub parent: Box<Ast>,
    pub name_span: Option<Span>,
    pub dot_span: Option<Span>,
}
impl GetSpan for Member {
    fn span(&self) -> Option<Span> {
        self.parent
            .merge_span(&self.dot_span)
            .merge_span(&self.name_span)
    }
}

impl AstData for Member {
    fn as_variant(&self) -> Ast {
        Ast::Member(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        true
    }

    fn typecheck(&mut self, ty_symt: &mut TypecheckSymTable) -> ZResult<Arc<Type>> {
        let parent_type = self.parent.typecheck(ty_symt)?;
        let res = match self.ty {
            AccessType::Method => unreachable!(),
            AccessType::Namespace => {
                if parent_type != *TYPE_T {
                    todo!()
                };

                parent_type
                    .namespace()
                    .get(&self.name)
                    .ok_or_else(|| todo!())
                    .map(|a| Arc::clone(a))?
            }
            AccessType::Field => parent_type
                .fields()
                .get(&self.name)
                .ok_or_else(|| todo!())
                .map(Arc::clone)?,
        };
        Ok(res)
    }

    fn desugared(&self) -> ZResult<Ast> {
        let mut new_self = self.to_owned();
        new_self.parent.desugar()?;
        Ok(new_self.as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        val_symt.get_val(&self.name, &self.name_span)
    }
}

impl Reconstruct for Member {
    fn reconstruct(&self) -> String {
        format!(
            "{} {} {}",
            self.parent.reconstruct(),
            match self.ty {
                AccessType::Field => ".",
                AccessType::Method => ":.",
                AccessType::Namespace => "::",
            },
            self.name
        )
    }
}
