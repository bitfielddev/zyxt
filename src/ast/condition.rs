use tracing::debug;

use crate::{
    ast::{Ast, AstData, Block},
    errors::{ToZResult, ZResult},
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
        debug!(span = ?self.span(), "Desugaring condition");
        self.condition
            .as_mut()
            .map(|e| {
                *e = e.desugared()?;
                Ok(())
            })
            .transpose()?;
        self.if_true = self.if_true.desugared()?.as_block().z()?.to_owned();
        Ok(())
    }
}
