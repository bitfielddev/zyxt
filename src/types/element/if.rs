use smol_str::SmolStr;

use crate::{
    types::element::{block::Block, Element, ElementData, ElementVariants, PosRaw},
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Condition {
    pub condition: Option<Element>,
    pub if_true: Element<Block>,
}
impl Condition {
    pub fn desugar(&mut self, pos_raw: &PosRaw) -> Result<(), ZyxtError> {
        self.condition.map(|e| e.desugared()).transpose()?;
        self.if_true.data = self.if_true.data.desugared(pos_raw)?.as_block().unwrap();
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct If {
    conditions: Vec<Condition>,
}

impl ElementData for If {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::If(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        false
    }
    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        Ok(self.data.conditions[0]
            .if_true
            .data
            .block_type(typelist, true)?
            .0)
        // TODO consider all returns
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
