use crate::{
    ast::{Ast, AstData},
    types::position::{GetSpan, Span},
    InterpreterData, Print, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Defer {
    pub kwd_span: Span,
    pub content: Box<Ast>,
}
impl GetSpan for Defer {
    fn span(&self) -> Option<Span> {
        self.kwd_span.merge_span(&self.content)
    }
}

impl AstData for Defer {
    fn as_variant(&self) -> Ast {
        Ast::Defer(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Ast>, O>,
    ) -> ZResult<Type<Ast>> {
        self.content.process(typelist)
    }

    fn desugared(&self, out: &mut impl Print) -> ZResult<Ast> {
        Ok(Defer {
            kwd_span: self.kwd_span.to_owned(),
            content: self.content.desugared(out)?.as_variant().into(),
        }
        .as_variant())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        i_data.add_defer(*self.content.to_owned());
        Ok(Value::Unit)
    }
}
