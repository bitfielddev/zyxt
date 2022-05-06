use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::objects::position::Position;
use crate::objects::token::{Flag, OprType};
use crate::{gen_instructions, Token};
use crate::errors::ZyxtError;
use crate::interpreter::interpret_block;
use crate::objects::deferstack::DeferStack;
use crate::objects::variable::Variable;
use crate::objects::typeobj::Type;
use crate::objects::stack::Stack;

#[derive(Clone, PartialEq, Debug)]
pub struct Condition {
    pub condition: Element,
    pub if_true: Vec<Element>
}
#[derive(Clone, PartialEq, Debug)]
pub struct Argument {
    pub name: String,
    pub type_: Type,
    pub default: Option<Element>
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
        operand: Box<Element>
    },
    BinaryOpr {
        position: Position,
        raw: String,
        type_: OprType,
        operand1: Box<Element>,
        operand2: Box<Element>
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
        content: Box<Element>
    },
    Literal {
        position: Position,
        raw: String,
        type_: Type,
        content: String
    },
    Variable {
        position: Position,
        raw: String,
        name: String,
        parent: Box<Element>
    },
    If {
        position: Position,
        raw: String,
        conditions: Vec<Condition>
    },
    Block {
        position: Position,
        raw: String,
        content: Vec<Element>
    },
    Delete {
        position: Position,
        raw: String,
        names: Vec<String>,
    },
    Return {
        position: Position,
        raw: String,
        value: Box<Element>
    },
    Procedure {
        position: Position,
        raw: String,
        is_fn: bool,
        args: Vec<Argument>,
        return_type: Type,
        content: Vec<Element>
    },
    Preprocess {
        position: Position,
        raw: String,
        content: Vec<Element>
    },
    Defer {
        position: Position,
        raw: String,
        content: Vec<Element>
    },
    Class {
        position: Position,
        raw: String,
        is_struct: bool,
        class_attrs: HashMap<String, Element>,
        inst_attrs: HashMap<String, Element>,
        content: Vec<Element>,
        args: Option<Vec<Argument>>
    },
    NullElement,
    Token(Token)
}
impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.name,
               if self.type_ != Type::any() {format!(": {}", self.type_)} else {"".to_string()},
               if let Some(r) = &self.default {format!(": {}", r.get_raw().trim())} else {"".to_string()}
        )
    }
}
impl Element {
    pub fn get_pos(&self) -> &Position {
        match self {
            Element::NullElement => panic!("null element"),
            Element::Token(Token{ position, .. }) |
            Element::Variable { position, .. } |
            Element::Literal { position, .. } |
            Element::Comment { position, .. } |
            Element::Call { position, .. } |
            Element::UnaryOpr { position, .. } |
            Element::BinaryOpr { position, .. } |
            Element::Declare { position, .. } |
            Element::Set { position, .. } |
            Element::If { position, .. } |
            Element::Block { position, .. } |
            Element::Delete { position, .. } |
            Element::Return { position, .. } |
            Element::Procedure { position, .. } |
            Element::Preprocess { position, .. } |
            Element::Defer { position, .. } |
            Element::Class { position, .. } => position
        }
    }
    pub fn get_raw(&self) -> String {
        match self {
            Element::NullElement => "".to_string(),
            Element::Token(t) => t.get_raw(),
            Element::Variable { raw, .. } |
            Element::Literal { raw, .. } |
            Element::Comment { raw, .. } |
            Element::Call { raw, .. } |
            Element::UnaryOpr { raw, .. } |
            Element::BinaryOpr { raw, .. } |
            Element::Declare { raw, .. } |
            Element::Set { raw, .. } |
            Element::If { raw, .. } |
            Element::Block { raw, .. } |
            Element::Delete { raw, .. } |
            Element::Return { raw, .. } |
            Element::Procedure { raw, .. } |
            Element::Preprocess { raw, .. } |
            Element::Defer { raw, .. } |
            Element::Class { raw, .. } => raw.clone()
        }
    }
    pub fn get_name(&self) -> String {
        if let Element::Variable {name: type1, ..} = self {type1.clone()} else {panic!("not variable")}
    }
    pub fn as_type(&self) -> Type {
        if let Element::Variable {name: type1, ..} = self {
            Type::Instance {
            name: type1.clone(),
            type_args: vec![],
            implementation: None
        }} else {panic!("not variable")}
    }
    pub fn bin_op_return_type(type_: &OprType, type1: Type,
                              type2: Type, position: &Position) -> Result<Type, ZyxtError> {
        if type_ == &OprType::TypeCast {
            return Ok(type2)
        }
        if let Some(v) = Variable::default(type1.clone())?
            .bin_opr(type_, Variable::default(type2.clone())?) {
            Ok(v.get_type_obj())
        } else {
            Err(ZyxtError::from_pos(position).error_4_0_0(type_.to_string(), type1.to_string(), type2.to_string()))
        }
    }
    pub fn un_op_return_type(type_: &OprType, opnd_type: Type,
                             position: &Position) -> Result<Type, ZyxtError> {
        if let Some(v) = Variable::default(opnd_type.clone())?.un_opr(type_) {
            Ok(v.get_type_obj())
        } else {
            Err(ZyxtError::from_pos(position).error_4_0_1(type_.to_string(), opnd_type.to_string()))
        }
    }
    pub fn block_type(content: &mut [Element], typelist: &mut Stack<Type>, add_set: bool) -> Result<Type, ZyxtError> {
        let mut last = Type::null();
        if add_set {typelist.add_set();}
        for ele in content.iter_mut() {
            last = ele.eval_type(typelist)?;
        }
        if add_set {typelist.pop_set();}
        Ok(last)
    }
    pub fn call_return_type(called: &mut Element, args: &mut [Element], typelist: &mut Stack<Type>) -> Result<Type, ZyxtError> {
        if let Element::Variable {ref parent, ref name, ..} = *called {
            if name == &"println".to_string() && parent.get_name() == *"std" {
                return Ok(Type::null())
            }
        }
        if let Element::Procedure{is_fn, args: proc_args, content, position, ..} = called {
            let mut fn_typelist: Stack<Type> = Stack::<Type>::default_type();
            for (cursor, Argument {name, default, ..}) in proc_args.iter_mut().enumerate() {
                let mut input_arg = if args.len() > cursor {Ok(args.get(cursor).unwrap().clone())}
                else {default.clone().ok_or_else(|| ZyxtError::from_pos(position).error_2_3(name.clone()))}?;
                fn_typelist.declare_val(name, &input_arg.eval_type(typelist)?);
                if args.len() > cursor {*args.get_mut(cursor).unwrap() = input_arg;}
                else {*default = Some(input_arg)}
            }
            let proc_varlist = if *is_fn {&mut fn_typelist} else {
                typelist.add_set();
                for (k, v) in fn_typelist.0[0].iter() {typelist.declare_val(k, v);}
                typelist
            };
            let res = Element::block_type(content, proc_varlist, true)?;
            proc_varlist.pop_set();
            return Ok(res)
        }
        if let Type::Instance {name, type_args, ..} = called.eval_type(typelist)? {
            if name == *"proc" || name == *"fn" {return Ok(type_args[1].clone())}
        } // TODO type checking for args when arrays are implemented
        Ok(Type::null())
        /*if let Some(v) = Variable::default(called.eval_type(typelist)?, typelist)?.call(
            args.iter_mut().map(|e| Variable::default(e.eval_type(typelist)?, typelist))
                .collect::<Result<Vec<_>, _>>()?
        ) { // TODO same as above
            Ok(v.get_type_obj())
        } else {
            Err(ZyxtError::from_pos(called.get_pos())
                .error_3_1_0(called.clone(),
                             called.eval_type(typelist)?,
                             "#call".to_string()))
        }*/
    }
    pub fn eval_type(&mut self, typelist: &mut Stack<Type>) -> Result<Type, ZyxtError> {
        match self {
            Element::Literal {type_, ..} => Ok(type_.clone()),
            Element::Variable {name, position, ..} =>
                typelist.get_val(name, position),
            Element::Block {content, ..} => Element::block_type(content, typelist, true),
            Element::Call {called, args, ..} =>
                Element::call_return_type(called, args, typelist),
            Element::Declare {position, variable, content,
                flags, type_, raw} => {
                let content_type = content.eval_type(typelist)?;
                if *type_ == Type::null() {
                    typelist.declare_val(&variable.get_name(), &content_type);
                    *self = Element::Declare {
                        type_: content_type.clone(),
                        content: content.clone(),
                        variable: variable.clone(),
                        position: position.clone(),
                        raw: raw.clone(),
                        flags: flags.clone()
                    };
                } else {
                    typelist.declare_val(&variable.get_name(), type_);
                    if content_type != *type_ {
                        let new_content = Element::BinaryOpr {
                            position: position.clone(),
                            raw: raw.clone(),
                            type_: OprType::TypeCast,
                            operand1: content.clone(),
                            operand2: Box::new(type_.as_element())
                        };
                        *self = Element::Declare {
                            type_: type_.clone(),
                            content: Box::new(new_content),
                            variable: variable.clone(),
                            position: position.clone(),
                            raw: raw.clone(),
                            flags: flags.clone()
                        };
                    }
                };
                Ok(content_type)
            },
            Element::If {conditions, ..} => Element::block_type(&mut conditions[0].if_true, typelist, true), // TODO consider all returns
            Element::BinaryOpr {type_, operand1, operand2, position, ..} => {
                let type1 = operand1.eval_type(typelist)?;
                let type2 = operand2.eval_type(typelist)?;
                Element::bin_op_return_type(type_, type1, type2, position)
            },
            Element::UnaryOpr {type_, operand, position, ..} => {
                let opnd_type = operand.eval_type(typelist)?;
                Element::un_op_return_type(type_, opnd_type, position)
            },
            Element::Procedure {is_fn, return_type, content, args, ..} => {
                typelist.add_set();
                for arg in args {
                    typelist.declare_val(&arg.name, &arg.type_);
                }
                let res =  Element::block_type(content, typelist, false)?;
                if return_type == &Type::null() {*return_type = res;}
                Ok(Type::Instance {
                    name: if *is_fn {"fn"} else {"proc"}.to_string(),
                    type_args: vec![Type::null(), return_type.clone()],
                    implementation: None
                })
            }, // TODO angle bracket thingy when it is implemented
            Element::Preprocess {content, ..} => {
                let mut pre_typelist = Stack::<Type>::default_type();
                let mut varlist = Stack::<Variable>::default_variable();
                let mut deferlist = DeferStack::new();
                let pre_instructions = gen_instructions(content.clone(), &mut pre_typelist)?;
                let pre_value = interpret_block(pre_instructions, &mut varlist,
                                                &mut deferlist,true, false)?;
                *self = pre_value.as_element();
                self.eval_type(typelist)
            },
            Element::Defer {content, ..} => {
                *content = gen_instructions(content.clone(), typelist)?;
                Ok(Type::null())
            },
            Element::Set {position, variable, content, ..} => {
                let content_type = content.eval_type(typelist)?;
                let var_type = typelist.get_val(&variable.get_name(), position)?;
                if content_type != var_type {
                    Err(ZyxtError::from_pos(position).error_4_3(variable.get_name(),
                                                            var_type, content_type))
                } else {Ok(var_type)}
            },
            Element::Class {content, inst_attrs, args, is_struct, ..} => {
                typelist.add_set();
                let class_attrs = HashMap::new();
                for expr in content.iter_mut() {
                    expr.eval_type(typelist)?;
                    if let Element::Declare {variable, content, flags, ..} = expr {
                        content.eval_type(typelist)?;
                        if flags.contains(&Flag::Inst) && args != &None {todo!("raise error here")}
                        if flags.contains(&Flag::Inst) {inst_attrs.insert(variable.get_name(), *content.clone());}
                    }
                }
                if args.is_some() && class_attrs.contains_key("#init") {
                    todo!("raise error here")
                }
                typelist.pop_set();
                Ok(Type::Definition {
                    name: if *is_struct { "struct" } else { "class" }.to_string(),
                    generics: vec![],
                    class_attrs,
                    inst_attrs: inst_attrs.clone()
                })
            }
            Element::NullElement |
            Element::Delete {..} |
            Element::Comment {..} |
            Element::Return {..} => Ok(Type::null()),
            Element::Token(Token{position, ..}) =>
                Err(ZyxtError::from_pos(position).error_2_1_0(self.get_raw()))
        }
    }
}