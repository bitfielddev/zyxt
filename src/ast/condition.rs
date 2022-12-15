use crate::{
    ast::{Ast, AstData, Block},
    errors::ZResult,
    types::position::{GetSpan, Span},
};

#[derive(Clone, PartialEq, Debug)]
pub struct Condition {
    pub kwd_span: Option<Span>,
    pub condition: Option<Ast>,
    pub if_true: Block,
}

impl GetSpan for Condition {
    fn span(&self) -> Option<Span> {
        self.kwd_span
            .merge_span(&self.condition)
            .merge_span(&self.if_true)
    }
}

impl Condition {
    pub fn desugar(&mut self) -> ZResult<()> {
        self.condition.as_mut().map(|e| e.desugared()).transpose()?;
        self.if_true = self
            .if_true
            .desugared()?
            .as_block()
            .unwrap_or_else(|| unreachable!())
            .to_owned();
        Ok(())
    }
}
