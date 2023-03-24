use std::sync::Arc;

use tracing::debug;

use crate::{
    ast::{Ast, AstData, Reconstruct},
    primitives::UNIT_T,
    types::{
        position::{GetSpan, Span},
        sym_table::{InterpretFrameType, TypecheckFrameType},
    },
    InterpretSymTable, Type, TypecheckSymTable, Value, ZError, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    pub brace_spans: Option<(Span, Span)>,
    pub content: Vec<Ast>,
}
impl GetSpan for Block {
    fn span(&self) -> Option<Span> {
        let start_brace = self.brace_spans.as_ref().map(|a| &a.0);
        let end_brace = self.brace_spans.as_ref().map(|a| &a.1);
        start_brace.merge_span(&self.content).merge_span(end_brace)
    }
}

impl AstData for Block {
    fn as_variant(&self) -> Ast {
        Ast::Block(self.to_owned())
    }

    fn typecheck(&mut self, ty_symt: &mut TypecheckSymTable) -> ZResult<Arc<Type>> {
        self.block_type(ty_symt, true)
    }

    fn desugared(&self) -> ZResult<Ast> {
        debug!(span = ?self.span(), "Desugaring block");
        Ok(Ast::Block(Self {
            brace_spans: self.brace_spans.to_owned(),
            content: self
                .content
                .iter()
                .map(AstData::desugared)
                .collect::<Result<_, _>>()?,
        }))
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        self.interpret_block(val_symt, true, true)
    }
}
impl Block {
    pub fn block_type(
        &mut self,
        ty_symt: &mut TypecheckSymTable,
        add_set: bool,
    ) -> ZResult<Arc<Type>> {
        let mut last = Arc::clone(&UNIT_T);
        if add_set {
            ty_symt.add_frame(TypecheckFrameType::Normal(None));
        }
        for ele in &mut self.content {
            last = ele.typecheck(ty_symt)?;
        }
        ty_symt.set_block_return(
            Arc::clone(&last),
            self.content.last().and_then(|a| a.span()),
        )?;
        if add_set {
            ty_symt.pop_frame();
        }
        Ok(last)
    }
    pub fn interpret_block(
        &self,
        val_symt: &mut InterpretSymTable,
        returnable: bool,
        add_frame: bool,
    ) -> ZResult<Value> {
        let mut last = Value::Unit;

        macro_rules! pop {
            () => {
                if add_frame {
                    let res = val_symt.pop_frame();
                }
            };
        }

        if add_frame {
            val_symt.add_frame(InterpretFrameType::Normal);
        }
        for ele in &self.content {
            if let Ast::Return(r#return) = ele {
                if returnable {
                    last = r#return.value.interpret_expr(val_symt)?;
                } else {
                    last = ele.interpret_expr(val_symt)?;
                }
                pop!();
                return Ok(last);
            }
            last = ele.interpret_expr(val_symt)?;
            if let Value::Return(value) = last {
                pop!();
                return if returnable {
                    Ok(*value)
                } else {
                    Ok(Value::Return(value))
                };
            }
        }
        pop!();
        Ok(last)
    }
}

impl Reconstruct for Block {
    fn reconstruct(&self) -> String {
        format!("{{ {} }}", self.content.reconstruct())
    }
}
