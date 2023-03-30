use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use smol_str::SmolStr;

use crate::{
    ast::{Ast, AstData},
    errors::{ToZResult, ZError, ZResult},
    primitives::{I32_T, PRIMS, PRIMS_VAL, TYPE_T},
    types::{
        position::GetSpan,
        r#type::{Type, TypeCheckType},
        value::Value,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub enum TypeCheckFrameType {
    NormalReturnable(Option<Arc<Type>>),
    Normal,
    Constants,
    Function(Option<Arc<Type>>),
}

#[derive(Debug, Clone)]
pub struct TypeCheckSymTable(pub VecDeque<TypeCheckFrame>);

#[derive(Debug, Clone)]
pub struct TypeCheckFrame {
    pub ty: TypeCheckFrameType,
    pub table: HashMap<SmolStr, TypeCheckType>,
    pub defer: Vec<Ast>,
}

impl Default for TypeCheckSymTable {
    fn default() -> Self {
        let mut table = Self(VecDeque::new());
        table.add_frame(TypeCheckFrameType::Constants);
        for (k, v) in &*PRIMS {
            table
                .declare_val(k, TypeCheckType::Const(Arc::clone(v)))
                .unwrap_or_else(|_| unreachable!());
        }
        table.add_frame(TypeCheckFrameType::NormalReturnable(Some(Arc::clone(
            &I32_T,
        ))));
        table
    }
}

impl TypeCheckSymTable {
    pub fn front_mut(&mut self) -> ZResult<&mut TypeCheckFrame> {
        self.0.front_mut().z()
    }

    #[tracing::instrument(skip(self))]
    pub fn add_frame(&mut self, ty: TypeCheckFrameType) -> &mut TypeCheckFrame {
        self.0.push_front(TypeCheckFrame {
            table: HashMap::new(),
            defer: vec![],
            ty,
        });
        self.front_mut().unwrap_or_else(|_| unreachable!())
    }

    #[tracing::instrument(skip(self))]
    pub fn set_block_return(&mut self, ty: Arc<Type>, span: impl GetSpan) -> ZResult<()> {
        for frame in &mut self.0 {
            if let TypeCheckFrameType::Function(ret_ty)
            | TypeCheckFrameType::NormalReturnable(ret_ty) = &mut frame.ty
            {
                if let Some(ret_ty) = ret_ty {
                    if !Arc::ptr_eq(ret_ty, &ty) {
                        return Err(ZError::t003(ret_ty, &ty).with_span(span));
                    }
                } else {
                    *ret_ty = Some(ty);
                }
                return Ok(());
            }
        }
        Err(ZError::t017().with_span(span))
    }

    #[tracing::instrument(skip(self))]
    pub fn get_block_return(&self) -> Arc<Type> {
        for frame in &self.0 {
            if let TypeCheckFrameType::Function(ret_ty)
            | TypeCheckFrameType::NormalReturnable(ret_ty) = &frame.ty
            {
                if let Some(ret_ty) = ret_ty {
                    return Arc::clone(ret_ty);
                }
            }
        }
        unreachable!()
    }

    #[tracing::instrument(skip(self))]
    pub fn declare_val(&mut self, name: &str, value: TypeCheckType) -> ZResult<()> {
        self.front_mut()?.table.insert(name.into(), value);
        Ok(())
    }
    pub fn pop_frame(&mut self) -> ZResult<()> {
        let mut temp_self = self.to_owned();
        for defer in &mut self.front_mut()?.defer {
            defer.type_check(&mut temp_self)?;
        }
        *self = temp_self;
        self.0.pop_front();
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn set_val(&mut self, name: &str, value: TypeCheckType, span: impl GetSpan) -> ZResult<()> {
        if *value == *TYPE_T {
            return Err(ZError::t001().with_span(span));
        }
        let mut only_consts = false;
        for frame in &mut self.0 {
            if (only_consts && frame.ty == TypeCheckFrameType::Constants)
                || frame.table.contains_key(name)
            {
                if frame.ty == TypeCheckFrameType::Constants {
                    return Err(ZError::t001().with_span(span));
                }
                if frame.table.get(name) == Some(&value) {
                    return Err(ZError::t011(frame.table.get(name).unwrap(), &value));
                }
                frame.table.insert(name.into(), value);
                return Ok(());
            }
            if let TypeCheckFrameType::Function(_) = frame.ty {
                only_consts = true;
            }
        }
        Err(ZError::t002(name).with_span(span))
    }

    #[tracing::instrument(skip(self))]
    pub fn get_val(&mut self, name: &str, span: impl GetSpan) -> ZResult<TypeCheckType> {
        let mut only_consts = false;
        for frame in &self.0 {
            if (only_consts && frame.ty == TypeCheckFrameType::Constants)
                || frame.table.contains_key(name)
            {
                return Ok(frame
                    .table
                    .get(name)
                    .unwrap_or_else(|| unreachable!())
                    .to_owned());
            }
            if let TypeCheckFrameType::Function(_) = frame.ty {
                only_consts = true;
            }
        }
        Err(ZError::t002(name).with_span(span))
    }
    #[tracing::instrument(skip(self))]
    pub fn get_type(&mut self, name: &str, span: impl GetSpan) -> ZResult<Arc<Type>> {
        Ok(Arc::clone(self.get_val(name, span)?.as_const()?))
    }

    #[tracing::instrument(skip(self))]
    pub fn delete_val(&mut self, name: &str, span: impl GetSpan) -> ZResult<TypeCheckType> {
        let Some(first_frame) = self.0.front_mut() else {
            return Err(ZError::t002(name).with_span(span))
        };
        if let Some(v) = first_frame.table.remove(name) {
            Ok(v)
        } else {
            Err(ZError::t002(name).with_span(span))
        }
    }
    #[tracing::instrument(skip(self))]
    pub fn add_defer(&mut self, content: Ast) -> ZResult<()> {
        self.front_mut()?.defer.push(content);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterpretFrameType {
    Normal,
    Constants,
    Function,
}

#[derive(Debug, Clone)]
pub struct InterpretSymTable(pub VecDeque<InterpretFrame>);

#[derive(Debug, Clone)]
pub struct InterpretFrame {
    pub ty: InterpretFrameType,
    pub table: HashMap<SmolStr, Value>,
    pub defer: Vec<Ast>,
}

impl Default for InterpretSymTable {
    fn default() -> Self {
        let mut table = Self(VecDeque::new());
        table.add_frame(InterpretFrameType::Constants);
        for (k, v) in &*PRIMS_VAL {
            table.declare_val(k, Value::Type(Arc::clone(v)));
        }
        table.add_frame(InterpretFrameType::Normal);
        table
    }
}

impl InterpretSymTable {
    pub fn front_mut(&mut self) -> ZResult<&mut InterpretFrame> {
        self.0.front_mut().z()
    }

    #[tracing::instrument(skip(self))]
    pub fn add_frame(&mut self, ty: InterpretFrameType) -> &mut InterpretFrame {
        self.0.push_front(InterpretFrame {
            table: HashMap::new(),
            defer: vec![],
            ty,
        });
        self.front_mut().unwrap_or_else(|_| unreachable!())
    }

    #[tracing::instrument(skip(self))]
    pub fn declare_val(&mut self, name: &str, value: Value) {
        let frame = if let Some(frame) = self.0.front_mut() {
            frame
        } else {
            self.add_frame(InterpretFrameType::Normal)
        };
        frame.table.insert(name.into(), value);
    }
    pub fn pop_frame(&mut self) -> ZResult<()> {
        let mut temp_self = self.to_owned();
        for defer in &mut self.front_mut()?.defer {
            defer.interpret_expr(&mut temp_self)?;
        }
        *self = temp_self;
        self.0.pop_front();
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn set_val(&mut self, name: &str, value: Value, span: impl GetSpan) -> ZResult<()> {
        if value.ty() == *TYPE_T {
            return Err(ZError::t001().with_span(span));
        }
        let mut only_consts = false;
        for frame in &mut self.0 {
            if (only_consts && frame.ty == InterpretFrameType::Constants)
                || frame.table.contains_key(name)
            {
                if frame.ty == InterpretFrameType::Constants {
                    return Err(ZError::t001().with_span(span));
                }
                frame.table.insert(name.into(), value);
                return Ok(());
            }
            if frame.ty == InterpretFrameType::Function {
                only_consts = true;
            }
        }
        Err(ZError::t002(name).with_span(span))
    }

    #[tracing::instrument(skip(self))]
    pub fn get_val(&mut self, name: &str, span: impl GetSpan) -> ZResult<Value> {
        let mut only_consts = false;
        for frame in &self.0 {
            if (only_consts && frame.ty == InterpretFrameType::Constants)
                || frame.table.contains_key(name)
            {
                return Ok(frame
                    .table
                    .get(name)
                    .unwrap_or_else(|| unreachable!())
                    .to_owned());
            }
            if frame.ty == InterpretFrameType::Function {
                only_consts = true;
            }
        }
        Err(ZError::t002(name).with_span(span))
    }
    #[tracing::instrument(skip(self))]
    pub fn delete_val(&mut self, name: &str, span: impl GetSpan) -> ZResult<Value> {
        let Some(first_frame) = self.0.front_mut() else {
            return Err(ZError::t002(name).with_span(span))
        };
        if let Some(v) = first_frame.table.remove(name) {
            Ok(v)
        } else {
            Err(ZError::t002(name).with_span(span))
        }
    }
    #[tracing::instrument(skip(self))]
    pub fn add_defer(&mut self, content: Ast) -> ZResult<()> {
        self.front_mut()?.defer.push(content);
        Ok(())
    }
}
