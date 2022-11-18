use crate::{
    types::{
        element::{Element, ElementData},
        interpreter_data::FrameType,
        position::{GetSpan, Span},
        typeobj::unit_t::UNIT_T,
    },
    InterpreterData, Print, Type, Value, ZError, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    pub brace_spans: Option<(Span, Span)>,
    pub content: Vec<Element>,
}
impl GetSpan for Block {
    fn span(&self) -> Option<Span> {
        let start_brace = self.brace_spans.as_ref().map(|a| &a.0);
        let end_brace = self.brace_spans.as_ref().map(|a| &a.1);
        start_brace.merge_span(&self.content).merge_span(end_brace)
    }
}

impl ElementData for Block {
    fn as_variant(&self) -> Element {
        Element::Block(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        Ok(self.block_type(typelist, true)?.0)
    }

    fn desugared(&self, out: &mut impl Print) -> ZResult<Element> {
        Ok(Element::Block(Self {
            brace_spans: self.brace_spans.to_owned(),
            content: self
                .content
                .iter()
                .map(|c| c.desugared(out))
                .collect::<Result<_, _>>()?,
        }))
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        self.interpret_block(i_data, true, true)
    }
}
impl Block {
    pub fn block_type<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
        add_set: bool,
    ) -> ZResult<(Type<Element>, Option<Type<Element>>)> {
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
                    return Err(
                        ZError::error_4_t(last, return_type.unwrap()), // TODO
                    );
                }
            }
        }
        if let Some(return_type) = return_type.to_owned() {
            if last != return_type {
                let _last_ele = self.content.last().unwrap();
                return Err(ZError::error_4_t(last, return_type)); // TODO
            }
        }
        if add_set {
            typelist.pop_frame();
        }
        Ok((last, if add_set { None } else { return_type }))
    }
    pub fn interpret_block<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
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
            if let Element::Return(r#return) = ele {
                if returnable {
                    last = r#return.value.interpret_expr(i_data)?
                } else {
                    last = ele.interpret_expr(i_data)?;
                }
                pop!();
                return Ok(last);
            } else {
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
        }
        pop!();
        Ok(last)
    }
}
