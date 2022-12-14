use crate::{
    ast::{block::Block, Ast, AstData},
    types::position::{GetSpan, Span},
    InterpreterData, Type, Value, ZResult,
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
        self.if_true = self.if_true.desugared()?.as_block().unwrap().to_owned();
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

impl AstData for If {
    fn as_variant(&self) -> Ast {
        Ast::If(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        false
    }
    fn process(&mut self, typelist: &mut InterpreterData<Type<Ast>>) -> ZResult<Type<Ast>> {
        Ok(self.conditions[0].if_true.block_type(typelist, true)?.0)
        // TODO consider all returns
    }

    fn desugared(&self) -> ZResult<Ast> {
        Ok(Self {
            conditions: self
                .conditions
                .iter()
                .map(|a| {
                    let mut a = a.to_owned();
                    a.desugar()?;
                    Ok(a)
                })
                .collect::<Result<_, _>>()?,
        }
        .as_variant())
    }

    fn interpret_expr(&self, i_data: &mut InterpreterData<Value>) -> ZResult<Value> {
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
