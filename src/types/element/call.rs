use std::collections::HashMap;

use smol_str::SmolStr;

use crate::{
    types::{
        element::{ident::Ident, literal::Literal, Element, ElementData, ElementVariants, PosRaw},
        typeobj::{unit_t::UNIT_T, TypeDefinition, TypeInstance},
        value::Proc,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Call {
    pub called: Element,
    pub args: Vec<Element>,
    pub kwargs: HashMap<SmolStr, Element>,
}

impl ElementData for Call {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::Call(self.to_owned())
    }
    fn process<O: Print>(
        &mut self,
        pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        todo!()
        /*if let ElementVariants::Ident(Ident {
            name,
            parent: Some(Element {
                data: ElementVariants::Ident(Ident {
                    name: parent_name, ..
                                             }), ..
                         })
        }) = &self.called.data {
            if &**name == "out" && &**parent_name == "ter" {
                self.args.iter_mut()
                    .map(|a| a.process(typelist))
                    .collect::<Result<Vec<_>, ZyxtError>>()?;
                return Ok(UNIT_T.get_instance().as_type_element());
            }
        }
        let called_type = self.called.process(typelist)?;
        if let ElementVariants::Procedure(procedure) = self.called.data.as_mut()
        {
            for (i, arg) in self.args.iter_mut().enumerate() {
                if arg.process(typelist)?
                    != procedure.args.get_mut(i).unwrap().type_.process(typelist)?
                {
                    todo!("errors")
                }
            }
            procedure.return_type.process(typelist)
        } else if let ElementVariants::Literal(Literal { content: Value::Proc(proc) }) = self.called.data.as_mut()
        {
            Ok(match proc {
                Proc::Builtin { signature, .. } => {
                    let (mut arg_objs, ret): (Vec<Type<Value>>, Type<Value>) =
                        signature[0]();
                    for (i, arg) in self.args.iter_mut().enumerate() {
                        let arg = arg.process(typelist)?;
                        let arg_req = arg_objs.get_mut(i).unwrap().as_type_element();
                        if arg != arg_req && arg != Type::Any && arg_req != Type::Any {
                            todo!("{:#?} != {:#?}", arg, arg_req)
                        }
                    }
                    ret.as_type_element()
                }
                Proc::Defined {
                    args: arg_objs,
                    return_type,
                    ..
                } => {
                    for (i, arg) in self.args.iter_mut().enumerate() {
                        if arg.process(typelist)?
                            != arg_objs.get_mut(i).unwrap().type_.process(typelist)?
                        {
                            todo!("errors")
                        }
                    }
                    return_type.as_type_element()
                }
            })
        } else {
            if let Type::Instance(TypeInstance {
                                      name,
                                      type_args,
                                      ..
                                  }) = &called_type
            {
                if *name == Some(SmolStr::from("proc")) {
                    if let Some(return_type) = type_args.get(1) {
                        return Ok(return_type.to_owned());
                    }
                }
            }
            self.called = if let Type::Definition(TypeDefinition {
                                                  implementations, ..
                                              }) = called_type
            {
                if let Some(call) = implementations.get("_call") {
                    call.to_owned()
                } else {
                    todo!();
                }
                // TODO handle error
            } else {
                unreachable!()
            };
            self.process(typelist)
        }*/
    }

    fn desugared(
        &self,
        _pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariants, ZyxtError> {
        todo!()
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
