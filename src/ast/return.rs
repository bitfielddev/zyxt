use crate::{
    ast::{Ast, AstData},
    primitives::UNIT_T,
    types::position::{GetSpan, Span},
    InterpreterData, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Return {
    pub kwd_span: Option<Span>,
    pub value: Box<Ast>,
}
impl GetSpan for Return {
    fn span(&self) -> Option<Span> {
        self.kwd_span.merge_span(&self.value)
    }
}

impl AstData for Return {
    fn as_variant(&self) -> Ast {
        Ast::Return(self.to_owned())
    }

    fn process(&mut self, _typelist: &mut InterpreterData<Type<Ast>>) -> ZResult<Type<Ast>> {
        Ok(UNIT_T.as_type().as_type_element())
    }

    fn desugared(&self) -> ZResult<Ast> {
        Ok(Self {
            kwd_span: self.kwd_span.to_owned(),
            value: self.value.desugared()?.into(),
        }
        .as_variant())
    }

    fn interpret_expr(&self, i_data: &mut InterpreterData<Value>) -> ZResult<Value> {
        Ok(Value::Return(Box::new(self.value.interpret_expr(i_data)?)))
    }
}
