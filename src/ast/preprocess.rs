use tracing::debug;

use crate::{
    ast::{Ast, AstData, Reconstruct},
    types::position::{GetSpan, Span},
    InterpretSymTable, TypeCheckSymTable, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Preprocess {
    pub kwd_span: Span,
    pub content: Box<Ast>,
}
impl GetSpan for Preprocess {
    fn span(&self) -> Option<Span> {
        self.kwd_span.merge_span(&self.content)
    }
}

impl AstData for Preprocess {
    fn as_variant(&self) -> Ast {
        Ast::Preprocess(self.to_owned())
    }

    fn desugared(&self) -> ZResult<Ast> {
        debug!(span = ?self.span(), "Desugaring preprocess statement");
        let mut pre_instructions = self.content.desugared()?;
        let mut pre_ty_symt = TypeCheckSymTable::default();
        pre_instructions.type_check(&mut pre_ty_symt)?;
        let mut val_symt = InterpretSymTable::default();
        let pre_value = pre_instructions.interpret_expr(&mut val_symt)?;
        Ok(pre_value.as_ast())
    }
}

impl Reconstruct for Preprocess {
    fn reconstruct(&self) -> String {
        format!("pre {}", self.content.reconstruct())
    }
}
