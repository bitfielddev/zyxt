use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use smol_str::SmolStr;

use crate::{
    ast::Ast,
    errors::{ZError, ZResult},
    primitives::{I32_T, PRIMS, PRIMS_VAL, TYPE_T},
    types::{
        position::GetSpan,
        r#type::{Type, TypeCheckType},
        value::Value,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub enum TypeCheckFrameType {
    Normal(Option<Arc<Type>>),
    Constants,
    Function(Option<Arc<Type>>),
}

#[derive(Debug)]
pub struct TypeCheckSymTable(pub VecDeque<TypeCheckFrame>);

#[derive(Debug)]
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
            table.declare_val(k, TypeCheckType::Const(Arc::clone(v)));
        }
        table.add_frame(TypeCheckFrameType::Normal(Some(Arc::clone(&I32_T))));
        table
    }
}

impl TypeCheckSymTable {
    #[tracing::instrument(skip(self))]
    pub fn add_frame(&mut self, ty: TypeCheckFrameType) -> &mut TypeCheckFrame {
        self.0.push_front(TypeCheckFrame {
            table: HashMap::new(),
            defer: vec![],
            ty,
        });
        self.0.front_mut().unwrap()
    }

    #[tracing::instrument(skip(self))]
    pub fn set_block_return(&mut self, ty: Arc<Type>, span: impl GetSpan) -> ZResult<()> {
        for frame in &mut self.0 {
            if let TypeCheckFrameType::Function(ret_ty) | TypeCheckFrameType::Normal(ret_ty) =
                &mut frame.ty
            {
                if let Some(ret_ty) = ret_ty {
                    if *ret_ty != ty {
                        return Err(ZError::t003(ret_ty, &ty).with_span(span));
                    }
                } else {
                    *ret_ty = Some(ty);
                }
                return Ok(());
            }
        }
        todo!("error")
    }

    #[tracing::instrument(skip(self))]
    pub fn get_block_return(&self) -> Arc<Type> {
        for frame in &self.0 {
            if let TypeCheckFrameType::Function(ret_ty) | TypeCheckFrameType::Normal(ret_ty) =
                &frame.ty
            {
                if let Some(ret_ty) = ret_ty {
                    return Arc::clone(ret_ty);
                }
            }
        }
        todo!("error")
    }

    #[tracing::instrument(skip(self))]
    pub fn declare_val(&mut self, name: &str, value: TypeCheckType) {
        let frame = if let Some(frame) = self.0.front_mut() {
            frame
        } else {
            self.add_frame(TypeCheckFrameType::Normal(None))
        };
        frame.table.insert(name.into(), value);
    }
    pub fn pop_frame(&mut self) {
        // TODO settle defers
        self.0.pop_front();
    }

    #[tracing::instrument(skip(self))]
    pub fn set_val(&mut self, name: &str, value: TypeCheckType, span: impl GetSpan) -> ZResult<()> {
        if *value == *TYPE_T {
            todo!("Cannot reset type")
        }
        let mut only_consts = false;
        for frame in &mut self.0 {
            if (only_consts && frame.ty == TypeCheckFrameType::Constants)
                || frame.table.contains_key(name)
            {
                if frame.ty == TypeCheckFrameType::Constants {
                    return Err(ZError::t001().with_span(span));
                }
                // TODO sth abt all type definitions being constant
                // TODO check types
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
    pub fn add_defer(&mut self, content: Ast) {
        self.0.front_mut().unwrap().defer.push(content);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterpretFrameType {
    Normal,
    Constants,
    Function,
}

#[derive(Debug)]
pub struct InterpretSymTable(pub VecDeque<InterpretFrame>);

#[derive(Debug)]
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
    #[tracing::instrument(skip(self))]
    pub fn add_frame(&mut self, ty: InterpretFrameType) -> &mut InterpretFrame {
        self.0.push_front(InterpretFrame {
            table: HashMap::new(),
            defer: vec![],
            ty,
        });
        self.0.front_mut().unwrap()
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
    pub fn pop_frame(&mut self) {
        // TODO settle defers
        self.0.pop_front();
    }

    #[tracing::instrument(skip(self))]
    pub fn set_val(&mut self, name: &str, value: Value, span: impl GetSpan) -> ZResult<()> {
        let mut only_consts = false;
        for frame in &mut self.0 {
            if (only_consts && frame.ty == InterpretFrameType::Constants)
                || frame.table.contains_key(name)
            {
                if frame.ty == InterpretFrameType::Constants {
                    return Err(ZError::t001().with_span(span));
                }
                // TODO sth abt all type definitions being constant
                // TODO check types
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
    pub fn add_defer(&mut self, content: Ast) {
        self.0.front_mut().unwrap().defer.push(content);
    }
}

/*#[derive(Debug)]
pub struct FrameData<T: Clone + Display> {
    pub pos: Position,
    pub raw_call: String,
    pub args: HashMap<SmolStr, T>,
}

#[derive(Debug)]
pub struct Frame<T: Clone + Display + Debug> {
    pub heap: HashMap<SmolStr, T>,
    pub defer: Vec<Ast>,
    pub frame_data: Option<FrameData<T>>,
    pub typedefs: HashMap<SmolStr, Type>,
    pub ty: FrameType,
}
#[derive(Debug)]
pub struct SymTable<T: Clone + Display + Debug> {
    pub frames: VecDeque<Frame<T>>,
}
impl Default for InterpretSymTable {
    fn default() -> Self {
        let mut v = Self {
            frames: VecDeque::new(),
        };
        let const_frame = v.add_frame(None, FrameType::Constants);
        for t in PRIM_NAMES {
            const_frame.heap.insert(
                t.into(),
                Value::Type(PRIMS.get(t).unwrap().to_owned().to_owned()),
            );
        }
        v.add_frame(None, FrameType::Normal);
        v
    }
}
impl InterpretSymTable {
    #[must_use]
    pub fn heap_to_string(&self) -> String {
        self.frames
            .iter()
            .map(|frame| {
                frame
                    .heap
                    .iter()
                    .map(|(k, v)| format!("{k}: {} = {v}", v.get_type_obj()))
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n-------\n")
    }
    pub fn pop_frame(&mut self) -> ZResult<Option<Value>> {
        let Some(first_frame) = self.frames.front_mut() else {
            return Ok(None)
        };
        for content in first_frame.defer.clone() {
            if let Value::Return(v) = content.interpret_expr(self)? {
                self.frames.pop_front();
                return Ok(Some(*v));
            }
        }
        self.frames.pop_front();
        Ok(None)
    }
    pub fn declare_val(&mut self, name: &SmolStr, value: &Value) {
        if let Some(frame) = self.frames.front_mut() {
            frame
        } else {
            self.add_frame(None, FrameType::Normal)
        }
        .heap
        .insert(name.to_owned(), value.to_owned());
    }
}

impl Default for TypecheckSymTable {
    fn default() -> Self {
        let mut v = Self {
            frames: VecDeque::new(),
        };
        let const_frame = v.add_frame(None, FrameType::Constants);
        for t in PRIM_NAMES {
            const_frame.heap.insert(
                t.into(),
                PRIMS
                    .get(t)
                    .unwrap()
                    .implementation()
                    ,
            );
            const_frame
                .typedefs
                .insert(t.into(), PRIMS.get(t).unwrap());
        }
        v.add_frame(None, FrameType::Normal);
        v
    }
}

impl TypecheckSymTable {
    #[tracing::instrument(skip(self))]
    pub fn declare_val(&mut self, name: &SmolStr, value: &Type) {
        let frame = if let Some(frame) = self.frames.front_mut() {
            frame
        } else {
            self.add_frame(None, FrameType::Normal)
        };
        frame.heap.insert(name.to_owned(), value.to_owned());
        if let Type::Definition(def) = value {
            frame.typedefs.insert(name.to_owned(), def.get_instance());
        }
    }
    pub fn pop_frame(&mut self) {
        self.frames.pop_front();
    }
}

impl<T: Clone + Display + Debug> SymTable<T> {
    #[tracing::instrument(skip(self))]
    pub fn add_frame(&mut self, frame_data: Option<FrameData<T>>, ty: FrameType) -> &mut Frame<T> {
        self.frames.push_front(Frame {
            heap: HashMap::new(),
            defer: vec![],
            frame_data,
            typedefs: HashMap::new(),
            ty,
        });
        self.frames.front_mut().unwrap()
    }

    #[tracing::instrument(skip(self))]
    pub fn set_val(&mut self, name: &SmolStr, value: &T, span: impl GetSpan) -> ZResult<()> {
        let mut only_consts = false;
        for frame in &mut self.frames {
            if (only_consts && frame.ty == FrameType::Constants) || frame.heap.contains_key(name) {
                if frame.ty == FrameType::Constants {
                    return Err(ZError::t001().with_span(span));
                }
                // TODO sth abt all type definitions being constant
                frame.heap.insert(name.to_owned(), value.to_owned());
                return Ok(());
            }
            if frame.ty == FrameType::Function {
                only_consts = true;
            }
        }
        Err(ZError::t002(name).with_span(span))
    }
    #[tracing::instrument(skip(self))]
    pub fn get_val(&mut self, name: &SmolStr, span: impl GetSpan) -> ZResult<T> {
        let mut only_consts = false;
        for frame in &self.frames {
            if (only_consts && frame.ty == FrameType::Constants) || frame.heap.contains_key(name) {
                return Ok(frame
                    .heap
                    .get(name)
                    .unwrap_or_else(|| unreachable!())
                    .to_owned());
            }
            if frame.ty == FrameType::Function {
                only_consts = true;
            }
        }
        Err(ZError::t002(name).with_span(span))
    }
    #[tracing::instrument(skip(self))]
    pub fn delete_val(&mut self, name: &SmolStr, span: impl GetSpan) -> ZResult<T> {
        let Some(first_frame) = self.frames.front_mut() else {
            return Err(ZError::t002(name).with_span(span))
        };
        if let Some(v) = first_frame.heap.remove(name) {
            Ok(v)
        } else {
            Err(ZError::t002(name).with_span(span))
        }
    }
    #[tracing::instrument(skip(self))]
    pub fn add_defer(&mut self, content: Ast) {
        self.frames.front_mut().unwrap().defer.push(content);
    }
}
*/
