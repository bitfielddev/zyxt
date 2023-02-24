use crate::{
    ast::{Ast, AstData, Reconstruct},
    types::position::{GetSpan, Span},
    SymTable, Type, Value, ZResult,
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
        let mut pre_instructions = self.content.desugared()?;
        let mut pre_ty_symt = SymTable::<Type<Ast>>::default();
        pre_instructions.process(&mut pre_ty_symt)?;
        let mut val_symt = SymTable::<Value>::default();
        let pre_value = pre_instructions.interpret_expr(&mut val_symt)?;
        Ok(pre_value.as_element())
    }
}

impl Reconstruct for Preprocess {
    fn reconstruct(&self) -> String {
        format!("pre {}", self.content.reconstruct())
    }
}
