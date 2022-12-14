use crate::{
    ast::{Ast, AstData},
    primitives::UNIT_T,
    types::{
        interpreter_data::FrameType,
        position::{GetSpan, Span},
    },
    SymTable, Type, Value, ZError, ZResult,
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

    fn process(&mut self, typelist: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        Ok(self.block_type(typelist, true)?.0)
    }

    fn desugared(&self) -> ZResult<Ast> {
        Ok(Ast::Block(Self {
            brace_spans: self.brace_spans.to_owned(),
            content: self
                .content
                .iter()
                .map(AstData::desugared)
                .collect::<Result<_, _>>()?,
        }))
    }

    fn interpret_expr(&self, i_data: &mut SymTable<Value>) -> ZResult<Value> {
        self.interpret_block(i_data, true, true)
    }
}
impl Block {
    pub fn block_type(
        &mut self,
        typelist: &mut SymTable<Type<Ast>>,
        add_set: bool,
    ) -> ZResult<(Type<Ast>, Option<Type<Ast>>)> {
        let mut last = UNIT_T.as_type().as_type_element();
        let mut return_type = None;
        if add_set {
            typelist.add_frame(None, FrameType::Normal);
        }
        for ele in &mut self.content {
            last = ele.process(typelist)?;
            if let Type::Return(value) = last.to_owned() {
                if let Some(return_type) = &return_type {
                    if last != *return_type {
                        return Err(
                            ZError::error_4_t(last, return_type.to_owned()), // TODO
                        );
                    }
                } else {
                    return_type = Some(*value);
                }
            }
        }
        if let Some(return_type) = return_type.to_owned() {
            if last != return_type {
                let _last_ele = self.content.last().unwrap_or_else(|| unreachable!());
                return Err(ZError::error_4_t(last, return_type)); // TODO
            }
        }
        if add_set {
            typelist.pop_frame();
        }
        Ok((last, if add_set { None } else { return_type }))
    }
    pub fn interpret_block(
        &self,
        i_data: &mut SymTable<Value>,
        returnable: bool,
        add_frame: bool,
    ) -> ZResult<Value> {
        let mut last = Value::Unit;

        macro_rules! pop {
            () => {
                if add_frame {
                    let res = i_data.pop_frame()?;
                    if let Some(res) = res {
                        return Ok(res);
                    }
                }
            };
        }

        if add_frame {
            i_data.add_frame(None, FrameType::Normal);
        }
        for ele in &self.content {
            if let Ast::Return(r#return) = ele {
                if returnable {
                    last = r#return.value.interpret_expr(i_data)?;
                } else {
                    last = ele.interpret_expr(i_data)?;
                }
                pop!();
                return Ok(last);
            }
            last = ele.interpret_expr(i_data)?;
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
