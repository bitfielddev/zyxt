use crate::{
    ast::{block::Block, Element, ElementData},
    types::position::{GetSpan, Span},
    InterpreterData, Print, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Condition {
    pub kwd_span: Option<Span>,
    pub condition: Option<Element>,
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
    pub fn desugar(&mut self, out: &mut impl Print) -> ZResult<()> {
        self.condition
            .as_mut()
            .map(|e| e.desugared(out))
            .transpose()?;
        self.if_true = self.if_true.desugared(out)?.as_block().unwrap().to_owned();
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct If {
    pub conditions: Vec<Condition>,
}
impl GetSpan for If {
    fn span(&self) -> Option<Span> {
        self.conditions.span()
    }
}

impl ElementData for If {
    fn as_variant(&self) -> Element {
        Element::If(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        false
    }
    fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        Ok(self.conditions[0].if_true.block_type(typelist, true)?.0)
        // TODO consider all returns
    }

    fn desugared(&self, out: &mut impl Print) -> ZResult<Element> {
        Ok(Self {
            conditions: self
                .conditions
                .iter()
                .map(|a| {
                    let mut a = a.to_owned();
                    a.desugar(out)?;
                    Ok(a)
                })
                .collect::<Result<_, _>>()?,
        }
        .as_variant())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        for cond in &self.conditions {
            if cond.condition.is_none() {
                return cond.if_true.interpret_block(i_data, false, true);
            } else if let Some(Value::Bool(true)) = cond
                .condition
                .as_ref()
                .map(|cond| cond.interpret_expr(i_data))
                .transpose()?
            {
                return cond.if_true.interpret_block(i_data, false, true);
            }
        }
        Ok(Value::Unit)
    }
}
