use std::fmt::{Display, Formatter};

use smol_str::SmolStr;

use crate::{
    types::{
        element::{block::Block, ident::Ident, Element, ElementData, ElementVariants, PosRaw},
        interpreter_data::FrameType,
        typeobj::{proc_t::PROC_T, unit_t::UNIT_T, TypeInstance},
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Argument {
    pub name: SmolStr,
    pub type_: Element<Ident>,
    pub default: Option<Element>,
}
impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.name,
            if self.type_.get_name() != "_any" {
                format!(": {}", self.type_.get_name())
            } else {
                "".to_string()
            },
            if let Some(r) = &self.default {
                format!(": {}", r.get_raw().trim())
            } else {
                "".to_string()
            }
        )
    }
}
impl Argument {
    pub fn desugar(&mut self, pos_raw: &PosRaw, out: &mut impl Print) -> Result<(), ZyxtError> {
        self.default = self.default.map(|e| e.desugared(out)).transpose()?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Procedure {
    pub is_fn: bool,
    pub args: Vec<Argument>,
    pub return_type: Element<Ident>,
    pub content: Element<Block>,
}

impl ElementData for Procedure {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::Procedure(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        typelist.add_frame(
            None,
            if self.is_fn {
                FrameType::Function
            } else {
                FrameType::Normal
            },
        );
        let return_type = self.return_type.process(typelist)?;
        for arg in self.args {
            let value = arg.type_.process(typelist)?;
            typelist.declare_val(&arg.name, &value);
        }
        let (res, block_return_type) = self.content.data.block_type(typelist, false)?;
        if return_type == UNIT_T.get_instance().as_type_element() || block_return_type.is_none() {
            *self.return_type = Box::new(res.as_literal());
        } else if let Some(block_return_type) = block_return_type {
            if return_type != block_return_type {
                return Err(
                    ZyxtError::error_4_t(return_type, block_return_type).with_pos_raw(pos_raw)
                );
            }
        }
        typelist.pop_frame();
        Ok(Type::Instance(TypeInstance {
            name: Some("proc".into()),
            //name: Some(if *is_fn { "fn" } else { "proc" }.into()),
            type_args: vec![UNIT_T.as_type().as_type_element(), return_type],
            implementation: PROC_T.as_type_element(),
        }))
    }

    fn desugared(
        &self,
        _pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariants, ZyxtError> {
        let mut new_self = self.to_owned();
        new_self.args = self
            .args
            .iter()
            .map(|a| {
                a.desugared(out);
                Ok(a)
            })
            .collect()?;
        new_self.content = self.content.desugared(out);
        Ok(new_self.as_variant())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
