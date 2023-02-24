use crate::{
    ast::{argument::Argument, Ast, AstData, Block, Reconstruct},
    primitives::{PROC_T, UNIT_T},
    types::{
        position::{GetSpan, Span},
        sym_table::FrameType,
        typeobj::TypeInstance,
        value::Proc,
    },
    SymTable, Type, Value, ZError, ZResult,
};

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

    fn process(&mut self, ty_symt: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        ty_symt.add_frame(
            None,
            if self.is_fn {
                FrameType::Function
            } else {
                FrameType::Normal
            },
        );
        let return_type = if let Some(ty) = &mut self.return_type {
            ty.process(ty_symt)?
        } else {
            UNIT_T.as_type().as_type_element()
        };
        for arg in &mut self.args {
            let value = arg.ty.process(ty_symt)?;
            ty_symt.declare_val(&arg.name.name, &value);
        }
        let (res, block_return_type) = self.content.block_type(ty_symt, false)?;
        if return_type == UNIT_T.get_instance().as_type_element() || block_return_type.is_none() {
            self.return_type = Some(res.as_literal().into());
        } else if let Some(block_return_type) = block_return_type {
            if return_type != block_return_type {
                return Err(ZError::t009(&return_type, &block_return_type).with_span(&*self));
                // TODO span
            }
        }
        ty_symt.pop_frame();
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
        new_self.content = self
            .content
            .desugared()?
            .as_block()
            .unwrap_or_else(|| unreachable!())
            .to_owned();
        Ok(new_self.as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut SymTable<Value>) -> ZResult<Value> {
        Ok(Value::Proc(Proc::Defined {
            is_fn: self.is_fn,
            args: self.args.to_owned(),
            return_type: if let Value::Type(value) = self
                .return_type
                .as_ref()
                .unwrap_or_else(|| unreachable!())
                .interpret_expr(val_symt)?
            {
                value
            } else {
                panic!("{self:#?}")
            },
            content: self.content.to_owned(),
        }))
    }
}
impl Reconstruct for Procedure {
    fn reconstruct(&self) -> String {
        format!("todo")
    }
}
