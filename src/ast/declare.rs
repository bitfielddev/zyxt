use crate::{
    ast::{Ast, AstData, BinaryOpr, Reconstruct},
    types::{
        position::{GetSpan, Span},
        token::{Flag, OprType},
    },
    SymTable, Type, Value, ZError, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Declare {
    pub variable: Box<Ast>,
    pub content: Box<Ast>,
    pub flags: Vec<(Flag, Span)>,
    pub ty: Option<Box<Ast>>,
    pub eq_span: Option<Span>,
}
impl GetSpan for Declare {
    fn span(&self) -> Option<Span> {
        self.variable
            .merge_span(self.flags.iter().map(|a| &a.1).collect::<Vec<_>>())
            .merge_span(&self.ty)
            .merge_span(&self.content)
            .merge_span(&self.eq_span)
    }
}

impl AstData for Declare {
    fn as_variant(&self) -> Ast {
        Ast::Declare(self.to_owned())
    }

    fn typecheck(&mut self, ty_symt: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        if !self.variable.is_pattern() {
            return Err(ZError::t006().with_span(&self.variable));
        }
        let content_type = self.content.typecheck(ty_symt)?;
        let ty = self
            .ty
            .as_mut()
            .map(|ty| {
                ty.typecheck(ty_symt)?;
                if let Ast::Literal(literal) = &**ty {
                    if let Value::Type(t) = &literal.content {
                        Ok(t.as_type_element())
                    } else {
                        Err(ZError::t007().with_span(ty.to_owned()))
                    }
                } else {
                    Err(ZError::t007().with_span(ty.to_owned()))
                }
            })
            .transpose()?;
        let name = if let Ast::Ident(ident) = &*self.variable {
            &ident.name
        } else {
            return Err(ZError::t008().with_span(&self.variable));
        };
        if let Some(ty) = ty {
            ty_symt.declare_val(name, &ty);
            if content_type != ty {
                let mut new_content = BinaryOpr {
                    ty: OprType::TypeCast,
                    opr_span: None,
                    operand1: self.content.to_owned(),
                    operand2: ty.as_literal().into(),
                }
                .as_variant();
                new_content.typecheck(ty_symt)?;
                *self = Self {
                    ty: self.ty.to_owned(),
                    content: new_content.into(),
                    variable: self.variable.to_owned(),
                    flags: self.flags.to_owned(),
                    eq_span: self.eq_span.to_owned(),
                };
            }
        } else {
            ty_symt.declare_val(name, &content_type);
        }
        Ok(content_type)
    }

    fn desugared(&self) -> ZResult<Ast> {
        let mut new_self = self.to_owned();
        new_self.content.desugar()?;
        new_self.ty = self
            .ty
            .as_ref()
            .map(|a| a.desugared())
            .transpose()?
            .map(Into::into);
        Ok(new_self.as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut SymTable<Value>) -> ZResult<Value> {
        let name = if let Ast::Ident(ident) = &*self.variable {
            &ident.name
        } else {
            unreachable!()
        };
        let var = self.content.interpret_expr(val_symt);
        val_symt.declare_val(name, &var.to_owned()?);
        var
    }
}
impl Reconstruct for Declare {
    fn reconstruct(&self) -> String {
        if let Some(ty) = &self.ty {
            format!(
                "{}: {} := {}",
                self.variable.reconstruct(),
                ty.reconstruct(),
                self.content.reconstruct()
            )
        } else {
            format!(
                "{} := {}",
                self.variable.reconstruct(),
                self.content.reconstruct()
            )
        }
    }
}
