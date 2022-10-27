use std::collections::HashMap;

use smol_str::SmolStr;

use crate::{
    types::{
        element::{
            block::Block, declare::Declare, ident::Ident, procedure::Argument, Element,
            ElementData, ElementVariant,
        },
        interpreter_data::FrameType,
        position::PosRaw,
        token::Flag,
        typeobj::TypeDefinition,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Class {
    pub is_struct: bool,
    pub implementations: HashMap<SmolStr, Element>,
    pub inst_fields: HashMap<SmolStr, (Element<Ident>, Option<Element>)>,
    pub content: Option<Element<Block>>,
    pub args: Option<Vec<Argument>>,
}

impl ElementData for Class {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Class(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        typelist.add_frame(None, FrameType::Normal);
        for expr in self.content.unwrap().data.content.iter_mut() {
            // TODO deal w unwrap
            expr.process(typelist)?;
            if let ElementVariant::Declare(Declare {
                variable,
                content,
                flags,
                ty,
                ..
            }) = &*expr.data
            {
                if flags.contains(&Flag::Inst) && self.args.is_some() {
                    todo!("raise error here")
                }
                let name = if let ElementVariant::Ident(ident) = &*variable.data {
                    &ident.name
                } else {
                    unimplemented!() // TODO
                };
                let ty = if let Some(ele) = ty {
                    if let ElementVariant::Ident(ident) = &*ele.data {
                        Element {
                            pos_raw: ele.pos_raw.to_owned(),
                            data: Box::new(ident.to_owned()),
                        }
                    } else {
                        unimplemented!() // TODO
                    }
                } else {
                    todo!("infer type")
                };
                if flags.contains(&Flag::Inst) {
                    self.inst_fields
                        .insert(name.to_owned(), (ty.to_owned(), Some(content.to_owned())));
                }
            }
        }
        if self.args.is_some() && self.implementations.contains_key("_init") {
            todo!("raise error here")
        }
        for item in self.implementations.values_mut() {
            item.process(typelist)?;
        }
        let new_inst_fields = self
            .inst_fields
            .iter_mut()
            .map(|(ident, (ty, default))| {
                let ty = ty.process(typelist)?;
                if let Some(default) = default {
                    if ty != default.process(typelist)? {
                        todo!("raise error")
                    }
                }
                Ok((ident.to_owned(), (Box::new(ty), default.to_owned())))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        typelist.pop_frame();
        Ok(Type::Definition(TypeDefinition {
            inst_name: None,
            name: Some(if self.is_struct { "struct" } else { "class" }.into()),
            generics: vec![],
            implementations: self.implementations.to_owned(),
            inst_fields: new_inst_fields,
        }))
    }

    fn desugared(
        &self,
        pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariant, ZyxtError> {
        let mut new_self = self.to_owned();
        new_self.content = if let Some(content) = new_self.content {
            Some(Element {
                pos_raw: pos_raw.to_owned(),
                data: Box::new(content.desugared(out)?.data.as_block().unwrap().to_owned()),
            })
        } else {
            None
        };
        new_self
            .args
            .map(|args| {
                args.into_iter()
                    .map(|mut arg| {
                        arg.desugar(pos_raw, out)?;
                        Ok(arg)
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        Ok(new_self.as_variant())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        Ok(Value::Type(Type::Definition(TypeDefinition {
            name: Some(if self.is_struct { "struct" } else { "class" }.into()),
            inst_name: None,
            generics: vec![],
            implementations: self
                .implementations
                .iter()
                .map(|(k, v)| Ok((k.to_owned(), v.interpret_expr(i_data)?)))
                .collect::<Result<HashMap<_, _>, _>>()?,
            inst_fields: self
                .inst_fields
                .iter()
                .map(|(k, (v1, v2))| {
                    Ok((
                        k.to_owned(),
                        (
                            Box::new(if let Value::Type(value) = v1.interpret_expr(i_data)? {
                                value
                            } else {
                                panic!()
                            }),
                            v2.to_owned()
                                .map(|v2| v2.interpret_expr(i_data))
                                .transpose()?,
                        ),
                    ))
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
        })))
    }
}
