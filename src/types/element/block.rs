use crate::{
    types::{
        element::{Element, ElementData, ElementVariants, PosRaw},
        interpreter_data::FrameType,
        typeobj::unit_t::UNIT_T,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Block {
    pub content: Vec<Element>,
}

impl ElementData for Block {
    fn as_variant(&self) -> ElementVariants {
        todo!()
    }

    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        Ok(self.data.block_type(typelist, true)?.0)
    }

    fn desugared(
        &self,
        _pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariants, ZyxtError> {
        Ok(ElementVariants::Block(Self {
            content: self.content.iter().map(|c| c.desugared(_pos_raw)).collect(),
        }))
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
impl Block {
    pub fn block_type<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
        add_set: bool,
    ) -> Result<(Type<Element>, Option<Type<Element>>), ZyxtError> {
        let mut last = UNIT_T.as_type().as_type_element();
        let mut return_type = None;
        if add_set {
            typelist.add_frame(None, FrameType::Normal);
        }
        for ele in self.content.iter_mut() {
            last = ele.process(typelist)?;
            if let Type::Return(value) = last.to_owned() {
                if return_type.to_owned().is_none() {
                    return_type = Some(*value);
                } else if last != return_type.to_owned().unwrap() {
                    return Err(ZyxtError::error_4_t(last, return_type.unwrap())
                        .with_pos_and_raw(ele.get_pos(), &ele.get_raw()));
                }
            }
        }
        if let Some(return_type) = return_type.to_owned() {
            if last != return_type {
                let last_ele = self.content.last().unwrap();
                return Err(ZyxtError::error_4_t(last, return_type)
                    .with_pos_and_raw(last_ele.get_pos(), &last_ele.get_raw()));
            }
        }
        if add_set {
            typelist.pop_frame();
        }
        Ok((last, if add_set { None } else { return_type }))
    }
}
