use std::{
    collections::{vec_deque::VecDeque, HashMap},
    fmt::{Debug, Display},
};

use maplit::hashmap;
use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    types::{
        element::ElementData,
        errors::{ZError, ZResult},
        position::{GetSpan, Position, Span},
        printer::Print,
        typeobj::{
            bool_t::BOOL_T, f16_t::F16_T, f32_t::F32_T, f64_t::F64_T, i128_t::I128_T, i16_t::I16_T,
            i32_t::I32_T, i64_t::I64_T, i8_t::I8_T, ibig_t::IBIG_T, isize_t::ISIZE_T, str_t::STR_T,
            type_t::TYPE_T, u128_t::U128_T, u16_t::U16_T, u32_t::U32_T, u64_t::U64_T, u8_t::U8_T,
            ubig_t::UBIG_T, unit_t::UNIT_T, usize_t::USIZE_T, Type,
        },
        value::Value,
    },
    Element,
};

const PRIM_NAMES: [&str; 22] = [
    "str", "bool", "i8", "i16", "i32", "i64", "i128", "isize", "ibig", "u8", "u16", "u32", "u64",
    "u128", "usize", "ubig", "f16", "f32", "f64", "_unit", "_any", "type",
];
static PRIMS: Lazy<HashMap<&'static str, Type<Value>>> = Lazy::new(|| {
    hashmap! {
        "str" => STR_T.as_type(),
        "bool" => BOOL_T.as_type(),
        "type" => TYPE_T.as_type(),
        "_unit" => UNIT_T.as_type(),
        "i8" => I8_T.as_type(),
        "i16" => I16_T.as_type(),
        "i32" => I32_T.as_type(),
        "i64" => I64_T.as_type(),
        "i128" => I128_T.as_type(),
        "ibig" => IBIG_T.as_type(),
        "isize" => ISIZE_T.as_type(),
        "u8" => U8_T.as_type(),
        "u16" => U16_T.as_type(),
        "u32" => U32_T.as_type(),
        "u64" => U64_T.as_type(),
        "u128" => U128_T.as_type(),
        "ubig" => UBIG_T.as_type(),
        "usize" => USIZE_T.as_type(),
        "f16" => F16_T.as_type(),
        "f32" => F32_T.as_type(),
        "f64" => F64_T.as_type(),
        "_any" => Type::Any
    }
});

#[derive(Debug)]
pub struct FrameData<T: Clone + Display> {
    pub pos: Position,
    pub raw_call: String,
    pub args: HashMap<SmolStr, T>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameType {
    Normal,
    Constants,
    Function,
}

#[derive(Debug)]
pub struct Frame<T: Clone + Display + Debug> {
    pub heap: HashMap<SmolStr, T>,
    pub defer: Vec<Element>,
    pub frame_data: Option<FrameData<T>>,
    pub typedefs: HashMap<SmolStr, Type<Element>>,
    pub ty: FrameType,
}
#[derive(Debug)]
pub struct InterpreterData<'a, T: Clone + Display + Debug, O: Print> {
    pub frames: VecDeque<Frame<T>>,
    pub out: &'a mut O,
}
impl<'a, O: Print> InterpreterData<'a, Value, O> {
    pub fn new(out: &'a mut O) -> InterpreterData<'a, Value, O> {
        let mut v = InterpreterData {
            frames: VecDeque::new(),
            out,
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
        for content in self.frames.front_mut().unwrap().defer.clone() {
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

impl<'a, O: Print> InterpreterData<'a, Type<Element>, O> {
    pub fn new(out: &'a mut O) -> InterpreterData<'a, Type<Element>, O> {
        let mut v = InterpreterData {
            frames: VecDeque::new(),
            out,
        };
        let const_frame = v.add_frame(None, FrameType::Constants);
        for t in PRIM_NAMES {
            const_frame.heap.insert(
                t.into(),
                PRIMS
                    .get(t)
                    .unwrap()
                    .implementation()
                    .as_type()
                    .as_type_element(),
            );
            const_frame
                .typedefs
                .insert(t.into(), PRIMS.get(t).unwrap().as_type_element());
        }
        v.add_frame(None, FrameType::Normal);
        v
    }
    pub fn declare_val(&mut self, name: &SmolStr, value: &Type<Element>) {
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

impl<T: Clone + Display + Debug, O: Print> InterpreterData<'_, T, O> {
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

    pub fn set_val(&mut self, name: &SmolStr, value: &T, span: &Span) -> ZResult<()> {
        let mut only_consts = false;
        for frame in self.frames.iter_mut() {
            if (only_consts && frame.ty == FrameType::Constants) || frame.heap.contains_key(name) {
                if frame.ty == FrameType::Constants {
                    todo!("Err trying to change const value")
                }
                // TODO sth abt all type definitions being constant
                frame.heap.insert(name.to_owned(), value.to_owned());
                return Ok(());
            }
            if frame.ty == FrameType::Function {
                only_consts = true;
            }
        }
        Err(ZError::error_3_0(name.to_owned()).with_span(span))
    }
    pub fn get_val(&mut self, name: &SmolStr, span: impl GetSpan) -> ZResult<T> {
        let mut only_consts = false;
        for frame in self.frames.iter() {
            if (only_consts && frame.ty == FrameType::Constants) || frame.heap.contains_key(name) {
                return Ok(frame.heap.get(name).unwrap().to_owned());
            }
            if frame.ty == FrameType::Function {
                only_consts = true;
            }
        }
        Err(ZError::error_3_0(name.to_owned()).with_span(span))
    }
    pub fn delete_val(&mut self, name: &SmolStr, span: impl GetSpan) -> ZResult<T> {
        if let Some(v) = self.frames.front_mut().unwrap().heap.remove(name) {
            Ok(v)
        } else {
            Err(ZError::error_3_0(name.to_owned()).with_span(span))
        }
    }
    pub fn add_defer(&mut self, content: Element) {
        self.frames.front_mut().unwrap().defer.push(content);
    }
}
