use crate::{
    ast::{call::Call, ident::Ident, Ast, AstData},
    primitives::BOOL_T,
    types::{
        position::{GetSpan, Span},
        token::OprType,
    },
    InterpreterData, Print, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct BinaryOpr {
    pub ty: OprType,
    pub opr_span: Option<Span>,
    pub operand1: Box<Ast>,
    pub operand2: Box<Ast>,
}
impl GetSpan for BinaryOpr {
    fn span(&self) -> Option<Span> {
        self.operand1
            .merge_span(&self.opr_span)
            .merge_span(&self.operand2)
    }
}

impl AstData for BinaryOpr {
    fn as_variant(&self) -> Ast {
        Ast::BinaryOpr(self.to_owned())
    }

    fn desugared(&self, out: &mut impl Print) -> ZResult<Ast> {
        Ok(match self.ty {
            OprType::And | OprType::Or => {
                let mut new_self = self.to_owned();
                for operand in [&mut new_self.operand1, &mut new_self.operand2] {
                    *operand = BinaryOpr {
                        ty: OprType::TypeCast,
                        opr_span: self.opr_span.to_owned(),
                        operand1: operand.desugared(out)?.into(),
                        operand2: BOOL_T
                            .as_type_element()
                            .get_instance()
                            .as_literal()
                            .as_variant()
                            .into(),
                    }
                    .desugared(out)?
                    .into();
                }
                new_self.as_variant()
            }
            _ => Call {
                called: Ident {
                    name: match self.ty {
                        OprType::Add => "_add",
                        OprType::Sub => "_sub",
                        OprType::Mul => "_mul",
                        OprType::Div => "_div",
                        OprType::Mod => "_rem",
                        OprType::Eq => "_eq",
                        OprType::Ne => "_ne",
                        OprType::Lt => "_lt",
                        OprType::Le => "_le",
                        OprType::Gt => "_gt",
                        OprType::Ge => "_ge",
                        OprType::Concat => "_concat",
                        OprType::TypeCast => "_typecast",
                        _ => unimplemented!("{:#?}", self.ty),
                    }
                    .into(),
                    name_span: None,
                    dot_span: None,
                    parent: Some(self.operand1.desugared(out)?.into()),
                }
                .as_variant()
                .into(),
                paren_spans: None,
                args: vec![self.operand2.desugared(out)?],
                kwargs: Default::default(),
            }
            .desugared(out)?,
        })
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        match self.ty {
            OprType::And => {
                if let Value::Bool(b) = self.operand1.interpret_expr(i_data)? {
                    if b {
                        if let Value::Bool(b) = self.operand2.interpret_expr(i_data)? {
                            Ok(Value::Bool(b))
                        } else {
                            panic!()
                        }
                    } else {
                        Ok(Value::Bool(false))
                    }
                } else {
                    panic!()
                }
            }
            OprType::Or => {
                if let Value::Bool(b) = self.operand1.interpret_expr(i_data)? {
                    if !b {
                        if let Value::Bool(b) = self.operand2.interpret_expr(i_data)? {
                            Ok(Value::Bool(b))
                        } else {
                            panic!()
                        }
                    } else {
                        Ok(Value::Bool(true))
                    }
                } else {
                    panic!()
                }
            }
            _opr => panic!("{_opr:?}"),
        }
    }
}
