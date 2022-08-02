use crate::errors::ZyxtError;
use crate::gen_instructions;
use crate::interpreter::interpret_block;
use crate::objects::interpreter_data::{InterpreterData, Print};
use crate::objects::position::Position;
use crate::objects::token::{Flag, OprType, Token};
use crate::objects::typeobj::Type;
use crate::objects::value::utils::OprError;
use crate::objects::value::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Debug)]
pub struct Condition {
    pub condition: Element,
    pub if_true: Vec<Element>,
}
#[derive(Clone, PartialEq, Debug)]
pub struct Argument {
    pub name: String,
    pub type_: Type,
    pub default: Option<Element>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Element {
    Comment {
        position: Position,
        raw: String,
        content: String,
    },
    Call {
        position: Position,
        raw: String,
        called: Box<Element>,
        args: Vec<Element>,
        kwargs: HashMap<String, Element>,
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
        type_: Type, // variable
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
        type_: Type,
        content: String,
    },
    Variable {
        position: Position,
        raw: String,
        name: String,
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
        names: Vec<String>,
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
        return_type: Type,
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
        class_attrs: HashMap<String, Element>,
        inst_attrs: HashMap<String, Element>,
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
            if self.type_ != Type::any() {
                format!(": {}", self.type_)
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
            | Element::Variable { position, .. }
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
            Element::Variable { raw, .. }
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
    pub fn get_name(&self) -> String {
        if let Element::Variable { name: type1, .. } = self {
            type1.to_owned()
        } else {
            panic!("not variable")
        }
    }
    pub fn as_type(&self) -> Type {
        if let Element::Variable { name: type1, .. } = self {
            Type::Instance {
                name: type1.to_owned(),
                type_args: vec![],
                inst_attrs: Default::default(),
                implementation: None,
            }
        } else {
            panic!("not variable")
        }
    }
    pub fn bin_op_return_type(
        type_: &OprType,
        type1: Type,
        type2: Type,
        position: &Position,
        raw: &String,
    ) -> Result<Type, ZyxtError> {
        if type_ == &OprType::TypeCast {
            return Ok(type2);
        } else if [
            OprType::Eq,
            OprType::Noteq,
            OprType::Lt,
            OprType::Lteq,
            OprType::Gt,
            OprType::Gteq,
            OprType::Iseq,
            OprType::Eq,
            OprType::And,
            OprType::Or,
            OprType::Xor,
        ]
        .contains(type_)
        {
            return Ok(Type::from_str("bool"));
        }

        match Value::default(type1.to_owned())? // TODO
            .bin_opr(type_, Value::default(type2.to_owned())?)
        {
            Ok(v) => Ok(v.get_type_obj()),
            Err(OprError::NoImplForOpr) => {
                Err(
                    ZyxtError::error_4_0_0(type_.to_string(), type1.to_string(), type2.to_string())
                        .with_pos_and_raw(position, raw),
                )
            }
            Err(OprError::TypecastError(ty)) => Ok(ty),
        }
    }
    pub fn un_op_return_type(
        type_: &OprType,
        opnd_type: Type,
        position: &Position,
        raw: &String,
    ) -> Result<Type, ZyxtError> {
        if type_ == &OprType::Not {
            return Ok(Type::from_str("bool"));
        }
        match Value::default(opnd_type.to_owned())?.un_opr(type_) {
            Ok(v) => Ok(v.get_type_obj()),
            Err(OprError::NoImplForOpr) => Err(ZyxtError::error_4_0_1(
                type_.to_string(),
                opnd_type.to_string(),
            )
            .with_pos_and_raw(position, raw)),
            Err(OprError::TypecastError(ty)) => Ok(ty),
        }
    }
    pub fn block_type<O: Print>(
        content: &mut [Element],
        typelist: &mut InterpreterData<Type, O>,
        add_set: bool,
    ) -> Result<(Type, Option<Type>), ZyxtError> {
        let mut last = Type::null();
        let mut return_type = None;
        if add_set {
            typelist.add_frame(None);
        }
        for ele in content.iter_mut() {
            last = ele.eval_type(typelist)?;
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
    pub fn call_return_type<O: Print>(
        called: &mut Element,
        args: &mut [Element],
        typelist: &mut InterpreterData<Type, O>,
    ) -> Result<Type, ZyxtError> {
        for arg in args {
            arg.eval_type(typelist)?;
        }
        if let Element::Variable {
            ref parent,
            ref name,
            ..
        } = *called
        {
            if name == &"out".to_string() && parent.get_name() == *"ter" {
                return Ok(Type::null());
            }
        }
        if let Type::Instance {
            name, type_args, ..
        } = called.eval_type(typelist)?
        {
            if name == *"proc" || name == *"fn" {
                return Ok(type_args[1].to_owned());
            }
        } // TODO type checking for args when arrays are implemented
        Ok(Type::null())
        /*if let Some(v) = Variable::default(called.eval_type(typelist)?, typelist)?.call(
            args.iter_mut().map(|e| Variable::default(e.eval_type(typelist)?, typelist))
                .collect::<Result<Vec<_>, _>>()?
        ) { // TODO same as above
            Ok(v.get_type_obj())
        } else {
            Err(ZyxtError::from_pos(called.get_pos())
                .error_3_1_0(called.to_owned(),
                             called.eval_type(typelist)?,
                             "_call".to_string()))
        }*/
    }
    pub fn is_pattern(&self) -> bool {
        matches!(self, Element::Variable { .. })
    }
    pub fn eval_type<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type, O>,
    ) -> Result<Type, ZyxtError> {
        match self {
            Element::Literal { type_, .. } => Ok(type_.to_owned()),
            Element::Variable {
                name,
                position,
                raw,
                ..
            } => typelist.get_val(name, position, raw),
            Element::Block { content, .. } => Ok(Element::block_type(content, typelist, true)?.0),
            Element::Call { called, args, .. } => Element::call_return_type(called, args, typelist),
            Element::Declare {
                position,
                variable,
                content,
                flags,
                type_,
                raw,
            } => {
                if !variable.is_pattern() {
                    return Err(
                        ZyxtError::error_2_2(*variable.to_owned()).with_element(&**variable)
                    );
                }
                let content_type = content.eval_type(typelist)?;
                if *type_ == Type::null() {
                    typelist.declare_val(&variable.get_name(), &content_type);
                    *self = Element::Declare {
                        type_: content_type.to_owned(),
                        content: content.to_owned(),
                        variable: variable.to_owned(),
                        position: position.to_owned(),
                        raw: raw.to_owned(),
                        flags: flags.to_owned(),
                    };
                } else {
                    typelist.declare_val(&variable.get_name(), type_);
                    if content_type != *type_ {
                        let new_content = Element::BinaryOpr {
                            position: position.to_owned(),
                            raw: raw.to_owned(),
                            type_: OprType::TypeCast,
                            operand1: content.to_owned(),
                            operand2: Box::new(type_.as_element()),
                        };
                        *self = Element::Declare {
                            type_: type_.to_owned(),
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
                let type1 = operand1.eval_type(typelist)?;
                let type2 = operand2.eval_type(typelist)?;
                if type_ == &OprType::TypeCast && type2 == Type::from_str("type") {
                    return Ok(Type::from_str(&*operand2.get_name()));
                }
                Element::bin_op_return_type(type_, type1, type2, position, raw)
            }
            Element::UnaryOpr {
                type_,
                operand,
                position,
                raw,
                ..
            } => {
                let opnd_type = operand.eval_type(typelist)?;
                Element::un_op_return_type(type_, opnd_type, position, raw)
            }
            Element::Procedure {
                is_fn,
                return_type,
                content,
                args,
                position,
                raw,
                ..
            } => {
                typelist.add_frame(None);
                for arg in args {
                    typelist.declare_val(&arg.name, &arg.type_);
                }
                let (res, block_return_type) = Element::block_type(content, typelist, false)?;
                if return_type == &Type::null() || block_return_type.is_none() {
                    *return_type = res;
                } else if let Some(block_return_type) = block_return_type {
                    if *return_type == block_return_type {
                        return Err(ZyxtError::error_4_t(
                            return_type.to_owned(),
                            block_return_type,
                        )
                        .with_pos_and_raw(position, raw));
                    }
                }
                Ok(Type::Instance {
                    name: if *is_fn { "fn" } else { "proc" }.to_string(),
                    type_args: vec![Type::null(), return_type.to_owned()],
                    inst_attrs: Default::default(),
                    implementation: None,
                })
            } // TODO angle bracket thingy when it is implemented
            Element::Preprocess { content, .. } => {
                let mut pre_typelist = InterpreterData::default_type(typelist.out);
                let pre_instructions = gen_instructions(content.to_owned(), &mut pre_typelist)?;
                let mut i_data = InterpreterData::default_variable(typelist.out);
                let pre_value = interpret_block(&pre_instructions, &mut i_data, true, false)?;
                *self = pre_value.as_element();
                self.eval_type(typelist)
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
                let content_type = content.eval_type(typelist)?;
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
                inst_attrs,
                args,
                is_struct,
                ..
            } => {
                typelist.add_frame(None);
                let class_attrs = HashMap::new();
                for expr in content.iter_mut() {
                    expr.eval_type(typelist)?;
                    if let Element::Declare {
                        variable,
                        content,
                        flags,
                        ..
                    } = expr
                    {
                        content.eval_type(typelist)?;
                        if flags.contains(&Flag::Inst) && args != &None {
                            todo!("raise error here")
                        }
                        if flags.contains(&Flag::Inst) {
                            inst_attrs.insert(variable.get_name(), *content.to_owned());
                        }
                    }
                }
                if args.is_some() && class_attrs.contains_key("_init") {
                    todo!("raise error here")
                }
                typelist.pop_frame();
                Ok(Type::Definition {
                    name: if *is_struct { "struct" } else { "class" }.to_string(),
                    generics: vec![],
                    class_attrs,
                    inst_attrs: inst_attrs.to_owned(),
                })
            }
            Element::NullElement
            | Element::Delete { .. }
            | Element::Comment { .. }
            | Element::Return { .. } => Ok(Type::null()),
            Element::Token(Token {
                position, value, ..
            }) => Err(ZyxtError::error_2_1_0(value.to_owned()).with_pos_and_raw(position, value)),
        }
    }
}
