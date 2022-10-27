use crate::{
    types::{
        element::{block::Block, Element, ElementData, ElementVariant},
        position::PosRaw,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Condition {
    pub condition: Option<Element>,
    pub if_true: Element<Block>,
}
impl Condition {
    pub fn desugar(&mut self, pos_raw: &PosRaw, out: &mut impl Print) -> Result<(), ZyxtError> {
        self.condition.map(|e| e.desugared(out)).transpose()?;
        self.if_true.data = self
            .if_true
            .data
            .desugared(pos_raw, out)?
            .as_block()
            .unwrap();
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct If {
    pub(crate) conditions: Vec<Condition>,
}

impl ElementData for If {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::If(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        false
    }
    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        Ok(self.conditions[0]
            .if_true
            .data
            .block_type(typelist, true)?
            .0)
        // TODO consider all returns
    }

    fn desugared(
        &self,
        pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariant, ZyxtError> {
        Ok(Self {
            conditions: self
                .conditions
                .iter()
                .map(|a| {
                    let mut a = a.to_owned();
                    a.desugar(pos_raw, out)?;
                    Ok(a)
                })
                .collect()?,
        }
        .as_variant())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        for cond in &self.conditions {
            if cond.condition == Element::NullElement {
                return cond.if_true.data.interpret_block(i_data, false, true);
            } else if let Some(Value::Bool(true)) = cond
                .condition
                .map(|cond| cond.interpret_expr(i_data))
                .transpose()?
            {
                return cond.if_true.data.interpret_block(i_data, false, true);
            }
        }
        Ok(Value::Unit)
    }
}
