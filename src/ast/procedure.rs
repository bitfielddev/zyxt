use std::fmt::{Display, Formatter};

use crate::{
    ast::{block::Block, ident::Ident, Ast, AstData},
    primitives::{PROC_T, UNIT_T},
    types::{
        interpreter_data::FrameType,
        position::{GetSpan, Span},
        typeobj::TypeInstance,
        value::Proc,
    },
    InterpreterData, Type, Value, ZError, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Argument {
    pub name: Ident,
    pub ty: Box<Ast>,
    pub default: Option<Ast>,
}
impl GetSpan for Argument {
    fn span(&self) -> Option<Span> {
        self.name.merge_span(&self.ty).merge_span(&self.default)
    }
}
impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        /*write!(
            f,
            "{}{}{}",
            self.name.name,
            if self.ty.span().raw != "_any" {
                // TODO
                format!(": {}", self.ty.span.raw)
            } else {
                "".to_string()
            },
            if let Some(r) = &self.default {
                format!(": {}", r.span.raw.trim())
            } else {
                "".to_string()
            }
        )*/
        write!(f, "")
    }
}
impl Argument {
    pub fn desugar(&mut self) -> ZResult<()> {
        self.default = self.default.as_ref().map(|e| e.desugared()).transpose()?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Procedure {
    pub is_fn: bool,
    pub kwd_span: Option<Span>,
    pub args: Vec<Argument>,
    pub return_type: Option<Box<Ast>>,
    pub content: Block,
}
impl GetSpan for Procedure {
    fn span(&self) -> Option<Span> {
        self.kwd_span
            .merge_span(&self.args)
            .merge_span(&self.return_type)
            .merge_span(&self.content)
    }
}

impl AstData for Procedure {
    fn as_variant(&self) -> Ast {
        Ast::Procedure(self.to_owned())
    }

    fn process(&mut self, typelist: &mut InterpreterData<Type<Ast>>) -> ZResult<Type<Ast>> {
        typelist.add_frame(
            None,
            if self.is_fn {
                FrameType::Function
            } else {
                FrameType::Normal
            },
        );
        let return_type = if let Some(ty) = &mut self.return_type {
            ty.process(typelist)?
        } else {
            UNIT_T.as_type().as_type_element()
        };
        for arg in &mut self.args {
            let value = arg.ty.process(typelist)?;
            typelist.declare_val(&arg.name.name, &value);
        }
        let (res, block_return_type) = self.content.block_type(typelist, false)?;
        if return_type == UNIT_T.get_instance().as_type_element() || block_return_type.is_none() {
            self.return_type = Some(res.as_literal().into());
        } else if let Some(block_return_type) = block_return_type {
            if return_type != block_return_type {
                return Err(ZError::error_4_t(return_type, block_return_type)); // TODO span
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

    fn desugared(&self) -> ZResult<Ast> {
        let mut new_self = self.to_owned();
        new_self.args = self
            .args
            .iter()
            .map(|a| {
                let mut a = a.to_owned();
                a.desugar()?;
                Ok(a)
            })
            .collect::<Result<Vec<_>, _>>()?;
        new_self.content = self.content.desugared()?.as_block().unwrap().to_owned();
        Ok(new_self.as_variant())
    }

    fn interpret_expr(&self, i_data: &mut InterpreterData<Value>) -> ZResult<Value> {
        Ok(Value::Proc(Proc::Defined {
            is_fn: self.is_fn,
            args: self.args.to_owned(),
            return_type: if let Value::Type(value) =
                self.return_type.as_ref().unwrap().interpret_expr(i_data)?
            {
                value
            } else {
                panic!("{self:#?}")
            },
            content: self.content.to_owned(),
        }))
    }
}
