use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use smol_str::SmolStr;

use crate::{
    gen_instructions,
    interpreter::interpret_block,
    types::{
        errors::ZyxtError,
        interpreter_data::InterpreterData,
        position::Position,
        printer::Print,
        token::{Flag, OprType, Token},
        typeobj::{proc_t::PROC_T, type_t::TYPE_T, unit_t::UNIT_T, Type},
        value::Value,
    },
};
use crate::types::typeobj::bool_t::BOOL_T;

#[derive(Clone, PartialEq, Debug)]
pub struct Condition {
    pub condition: Element,
    pub if_true: Vec<Element>,
}
#[derive(Clone, PartialEq, Debug)]
pub struct Argument {
    pub name: SmolStr,
    pub type_: Element,
    pub default: Option<Element>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Element {
    Comment {
        position: Position,
        raw: String,
        content: SmolStr,
    },
    Call {
        position: Position,
        raw: String,
        called: Box<Element>,
        args: Vec<Element>,
        kwargs: HashMap<SmolStr, Element>,
    },
    UnaryOpr {
        position: Position,
        raw: String,
        type_: OprType,
        operand: Box<Element>,
    },
    BinaryOpr {
        position: Position,
        raw: String,
        type_: OprType,
        operand1: Box<Element>,
        operand2: Box<Element>,
    },
    Declare {
        position: Position,
        raw: String,
        variable: Box<Element>, // variable
        content: Box<Element>,
        flags: Vec<Flag>,
        type_: Box<Element>, // variable
    },
    Set {
        position: Position,
        raw: String,
        variable: Box<Element>, // variable
        content: Box<Element>,
    },
    Literal {
        position: Position,
        raw: String,
        content: Value,
    },
    Ident {
        position: Position,
        raw: String,
        name: SmolStr,
        parent: Box<Element>,
    },
    If {
        position: Position,
        raw: String,
        conditions: Vec<Condition>,
    },
    Block {
        position: Position,
        raw: String,
        content: Vec<Element>,
    },
    Delete {
        position: Position,
        raw: String,
        names: Vec<SmolStr>,
    },
    Return {
        position: Position,
        raw: String,
        value: Box<Element>,
    },
    Procedure {
        position: Position,
        raw: String,
        is_fn: bool,
        args: Vec<Argument>,
        return_type: Box<Element>,
        content: Vec<Element>,
    },
    Preprocess {
        position: Position,
        raw: String,
        content: Vec<Element>,
    },
    Defer {
        position: Position,
        raw: String,
        content: Vec<Element>,
    },
    Class {
        position: Position,
        raw: String,
        is_struct: bool,
        implementations: HashMap<SmolStr, Element>,
        inst_fields: HashMap<SmolStr, (Element, Option<Box<Element>>)>,
        content: Vec<Element>,
        args: Option<Vec<Argument>>,
    },
    NullElement,
    Token(Token),
}
impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.name,
            if self.type_.get_name() != "_any" {
                format!(": {}", self.type_.get_name())
            } else {
                "".to_string()
            },
            if let Some(r) = &self.default {
                format!(": {}", r.get_raw().trim())
            } else {
                "".to_string()
            }
        )
    }
}
pub trait VecElementRaw {
    fn get_raw(&self) -> String;
}
impl VecElementRaw for Vec<Element> {
    fn get_raw(&self) -> String {
        self.iter()
            .map(|e| e.get_raw())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Element {
    pub fn get_pos(&self) -> &Position {
        match self {
            Element::NullElement => panic!("null element"),
            Element::Token(Token { position, .. })
            | Element::Ident { position, .. }
            | Element::Literal { position, .. }
            | Element::Comment { position, .. }
            | Element::Call { position, .. }
            | Element::UnaryOpr { position, .. }
            | Element::BinaryOpr { position, .. }
            | Element::Declare { position, .. }
            | Element::Set { position, .. }
            | Element::If { position, .. }
            | Element::Block { position, .. }
            | Element::Delete { position, .. }
            | Element::Return { position, .. }
            | Element::Procedure { position, .. }
            | Element::Preprocess { position, .. }
            | Element::Defer { position, .. }
            | Element::Class { position, .. } => position,
        }
    }
    pub fn get_raw(&self) -> String {
        match self {
            Element::NullElement => "".to_string(),
            Element::Token(t) => t.get_raw(),
            Element::Ident { raw, .. }
            | Element::Literal { raw, .. }
            | Element::Comment { raw, .. }
            | Element::Call { raw, .. }
            | Element::UnaryOpr { raw, .. }
            | Element::BinaryOpr { raw, .. }
            | Element::Declare { raw, .. }
            | Element::Set { raw, .. }
            | Element::If { raw, .. }
            | Element::Block { raw, .. }
            | Element::Delete { raw, .. }
            | Element::Return { raw, .. }
            | Element::Procedure { raw, .. }
            | Element::Preprocess { raw, .. }
            | Element::Defer { raw, .. }
            | Element::Class { raw, .. } => raw.to_owned(),
        }
    }
    pub fn get_raw_mut(&mut self) -> Option<&mut String> {
        match self {
            Element::NullElement | Element::Token(_) => None,
            Element::Ident { raw, .. }
            | Element::Literal { raw, .. }
            | Element::Comment { raw, .. }
            | Element::Call { raw, .. }
            | Element::UnaryOpr { raw, .. }
            | Element::BinaryOpr { raw, .. }
            | Element::Declare { raw, .. }
            | Element::Set { raw, .. }
            | Element::If { raw, .. }
            | Element::Block { raw, .. }
            | Element::Delete { raw, .. }
            | Element::Return { raw, .. }
            | Element::Procedure { raw, .. }
            | Element::Preprocess { raw, .. }
            | Element::Defer { raw, .. }
            | Element::Class { raw, .. } => Some(raw),
        }
    }
    pub fn get_name(&self) -> SmolStr {
        if let Element::Ident { name: type1, .. } = self {
            type1.to_owned()
        } else {
            panic!("not variable")
        }
    }
    pub fn block_type<O: Print>(
        content: &mut [Element],
        typelist: &mut InterpreterData<Type<Element>, O>,
        add_set: bool,
    ) -> Result<(Type<Element>, Option<Type<Element>>), ZyxtError> {
        let mut last = UNIT_T.as_type_element();
        let mut return_type = None;
        if add_set {
            typelist.add_frame(None);
        }
        for ele in content.iter_mut() {
            last = ele.process(typelist)?;
            if let Type::Return(value) = last.to_owned() {
                if return_type.to_owned().is_none() {
                    return_type = Some(*value);
                } else if last != return_type.to_owned().unwrap() {
                    return Err(ZyxtError::error_4_t(last, return_type.unwrap())
                        .with_pos_and_raw(ele.get_pos(), &ele.get_raw()));
                }
            }
        }
        if let Some(return_type) = return_type.to_owned() {
            if last != return_type {
                let last_ele = content.last().unwrap();
                return Err(ZyxtError::error_4_t(last, return_type)
                    .with_pos_and_raw(last_ele.get_pos(), &last_ele.get_raw()));
            }
        }
        if add_set {
            typelist.pop_frame();
        }
        Ok((last, if add_set { None } else { return_type }))
    }
    pub fn is_pattern(&self) -> bool {
        matches!(self, Element::Ident { .. })
    }
    pub fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        match self {
            Element::Literal { content, .. } => Ok(content.get_type_obj().as_type_element()),
            Element::Ident {
                name,
                position,
                raw,
                ..
            } => typelist.get_val(name, position, raw),
            Element::Block { content, .. } => Ok(Element::block_type(content, typelist, true)?.0),
            Element::Call { called, args, .. } => {
                let called_type = called.process(typelist)?;
                if let Element::Procedure {args: args_objs, ..} = **called {
                    for (i, arg) in args.iter_mut().enumerate() {
                        if arg.process(typelist)? != arg_objs.get(i).unwrap() {
                            todo!("errors")
                        }
                    }
                } else {
                    *called = called_type // TODO
                }
            },
            Element::Declare {
                position,
                variable,
                content,
                flags,
                type_: pre_type,
                raw,
            } => {
                if !variable.is_pattern() {
                    return Err(
                        ZyxtError::error_2_2(*variable.to_owned()).with_element(&**variable)
                    );
                }
                let content_type = content.process(typelist)?;
                let type_ = pre_type.process(typelist)?;
                if type_ == UNIT_T.as_type_element() {
                    typelist.declare_val(&variable.get_name(), &content_type);
                } else {
                    typelist.declare_val(&variable.get_name(), &type_);
                    if content_type != type_ {
                        let new_content = Element::BinaryOpr {
                            position: position.to_owned(),
                            raw: raw.to_owned(),
                            type_: OprType::TypeCast,
                            operand1: content.to_owned(),
                            operand2: Box::new(type_.as_literal()),
                        };
                        *self = Element::Declare {
                            type_: pre_type.to_owned(),
                            content: Box::new(new_content),
                            variable: variable.to_owned(),
                            position: position.to_owned(),
                            raw: raw.to_owned(),
                            flags: flags.to_owned(),
                        };
                    }
                };
                Ok(content_type)
            }
            Element::If { conditions, .. } => {
                Ok(Element::block_type(&mut conditions[0].if_true, typelist, true)?.0)
            } // TODO consider all returns
            Element::BinaryOpr {
                type_,
                operand1,
                operand2,
                position,
                raw,
                ..
            } => {
                let type1 = operand1.process(typelist)?;
                let type2 = operand2.process(typelist)?;
                Ok(match type_ {
                    OprType::TypeCast => {
                        if type2 == TYPE_T.as_type_element() {
                            type2.get_instance().unwrap() // TODO handle error
                        } else {
                            todo!("Error here")
                        }
                    }
                    OprType::And | OprType::Or => BOOL_T.get_instance().unwrap().as_type_element(),
                    _ => {
                        *self = Element::Call {
                            position: position.to_owned(),
                            raw: raw.to_owned(),
                            called: Box::new(Element::Ident {
                                position: Default::default(),
                                raw: "".to_string(),
                                name: match type_ {
                                    OprType::Plus => "_add",
                                    OprType::Minus => "_sub",
                                    OprType::AstMult => "_mul",
                                    OprType::Div => "_div",
                                    OprType::Modulo => "_rem",
                                    OprType::Eq => "_eq",
                                    OprType::Noteq => "_ne",
                                    OprType::Lt => "_lt",
                                    OprType::Lteq => "_le",
                                    OprType::Gt => "_gt",
                                    OprType::Gteq => "_ge",
                                    OprType::Concat => "_concat",
                                    _ => unimplemented!()
                                }.into(),
                                parent: Box::new(type1.as_literal())
                            }),
                            args: vec![*operand1.to_owned(), *operand2.to_owned()],
                            kwargs: Default::default()
                        };
                        self.process(typelist)?
                    },
                })
            }
            Element::UnaryOpr {
                type_,
                operand,
                position,
                raw,
                ..
            } => {
                let operand_type = operand.process(typelist)?;
                *self = Element::Call {
                    position: position.to_owned(),
                    raw: raw.to_owned(),
                    called: Box::new(Element::Ident {
                        position: Default::default(),
                        raw: "".to_string(),
                        name: match type_ {
                            OprType::Not => "_not",
                            OprType::PlusSign => "_un_plus",
                            OprType::MinusSign => "_un_minus",
                            _ => panic!()
                        }.into(),
                        parent: Box::new(operand_type.as_literal())
                    }),
                    args: vec![*operand.to_owned()],
                    kwargs: Default::default()
                };
                self.process(typelist)
            }
            Element::Procedure {
                is_fn,
                return_type: pre_return_type,
                content,
                args,
                position,
                raw
            } => {
                let mut a = InterpreterData::default_type(typelist.out);
                let typelist = if *is_fn {
                    &mut a
                } else {
                    typelist
                };
                typelist.add_frame(None);
                let return_type = pre_return_type.process(typelist)?;
                for arg in args {
                    typelist.declare_val(&arg.name, &arg.type_.process(typelist)?);
                }
                let (res, block_return_type) = Element::block_type(content, typelist, false)?;
                /*if return_type == &UNIT_T || block_return_type.is_none() {
                    *return_type = res;
                } else*/
                if let Some(block_return_type) = block_return_type {
                    if return_type != block_return_type {
                        return Err(ZyxtError::error_4_t(
                            return_type.to_owned(),
                            block_return_type,
                        )
                        .with_pos_and_raw(position, raw));
                    }
                }
                typelist.pop_frame();
                Ok(Type::Instance {
                    name: Some("proc".into()),
                    //name: Some(if *is_fn { "fn" } else { "proc" }.into()),
                    type_args: vec![UNIT_T.as_type_element(), return_type],
                    implementation: Box::new(PROC_T.as_type_element()),
                })
            } // TODO angle bracket thingy when it is implemented
            Element::Preprocess { content, .. } => {
                let mut pre_typelist = InterpreterData::default_type(typelist.out);
                let pre_instructions = gen_instructions(content.to_owned(), &mut pre_typelist)?;
                let mut i_data = InterpreterData::default_variable(typelist.out);
                let pre_value = interpret_block(&pre_instructions, &mut i_data, true, false)?;
                *self = pre_value.as_element();
                self.process(typelist)
            }
            Element::Defer { content, .. } =>
            // TODO check block return against call stack
            {
                Ok(Element::block_type(content, typelist, false)?.0)
            }
            Element::Set {
                position,
                variable,
                content,
                raw,
                ..
            } => {
                if !variable.is_pattern() {
                    return Err(
                        ZyxtError::error_2_2(*variable.to_owned()).with_element(&**variable)
                    );
                }
                let content_type = content.process(typelist)?;
                let var_type = typelist.get_val(&variable.get_name(), position, raw)?;
                if content_type != var_type {
                    Err(
                        ZyxtError::error_4_3(variable.get_name(), var_type, content_type)
                            .with_pos_and_raw(position, raw),
                    )
                } else {
                    Ok(var_type)
                }
            }
            Element::Class {
                content,
                implementations,
                inst_fields,
                args,
                is_struct,
                position,
                raw,
                ..
            } => {
                typelist.add_frame(None);
                for expr in content.iter_mut() {
                    expr.process(typelist)?;
                    if let Element::Declare {
                        variable,
                        content,
                        flags,
                        type_,
                        ..
                    } = expr
                    {
                        if flags.contains(&Flag::Inst) && args != &None {
                            todo!("raise error here")
                        }
                        if flags.contains(&Flag::Inst) {
                            inst_fields.insert(
                                variable.get_name(),
                                (*type_.to_owned(), Some(content.to_owned())),
                            );
                        }
                    }
                }
                if args.is_some() && implementations.contains_key("_init") {
                    todo!("raise error here")
                }
                for item in implementations.values_mut() {
                    item.process(typelist)?;
                }
                let new_inst_fields = inst_fields.iter_mut()
                    .map(|(ident, (ty, default))| {
                        let ty = ty.process(typelist)?;
                        if let Some(default) = default {
                            if ty != default.process(typelist)? {
                                todo!("raise error")
                            }
                        }
                        Ok((ident.to_owned(), (Box::new(ty), default.map(|a| *a))))
                    }).collect::<Result<HashMap<_, _>, _>>()?;
                typelist.pop_frame();
                Ok(Type::Definition {
                    inst_name: None,
                    name: Some(if *is_struct { "struct" } else { "class" }.into()),
                    generics: vec![],
                    implementations: implementations.to_owned(),
                    inst_fields: new_inst_fields
                })
            }
            Element::NullElement
            | Element::Delete { .. }
            | Element::Comment { .. }
            | Element::Return { .. } => Ok(UNIT_T.as_type_element()),
            Element::Token(Token {
                position, value, ..
            }) => Err(ZyxtError::error_2_1_0(value.to_owned())
                .with_pos_and_raw(position, &value.to_string())),
        }
    }
}
