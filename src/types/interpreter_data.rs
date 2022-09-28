use std::{
    collections::{vec_deque::VecDeque, HashMap},
    fmt::{Debug, Display},
};

use lazy_static::lazy_static;
use maplit::hashmap;
use smol_str::SmolStr;

use crate::{
    interpreter::interpret_block,
    types::{
        errors::ZyxtError,
        position::Position,
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
lazy_static! {
    static ref PRIMS: HashMap<&'static str, &'static Type<Value>> = hashmap! {
        "str" => &*STR_T,
        "bool" => &*BOOL_T,
        "type" => &*TYPE_T,
        "_unit" => &*UNIT_T,
        "i8" => &*I8_T,
        "i16" => &*I16_T,
        "i32" => &*I32_T,
        "i64" => &*I64_T,
        "i128" => &*I128_T,
        "ibig" => &*IBIG_T,
        "isize" => &*ISIZE_T,
        "u8" => &*U8_T,
        "u16" => &*U16_T,
        "u32" => &*U32_T,
        "u64" => &*U64_T,
        "u128" => &*U128_T,
        "ubig" => &*UBIG_T,
        "usize" => &*USIZE_T,
        "f16" => &*F16_T,
        "f32" => &*F32_T,
        "f64" => &*F64_T,
        "_any" => &Type::Any
    };
}

#[derive(Debug)]
pub struct FrameData<T: Clone + Display> {
    pub position: Position,
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
    pub defer: Vec<Vec<Element>>,
    pub frame_data: Option<FrameData<T>>,
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
                    .map(|(k, v)| format!("{}: {} = {}", k, v.get_type_obj(), v))
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n-------\n")
    }
    pub fn pop_frame(&mut self) -> Result<Option<Value>, ZyxtError> {
        for content in self.frames.front_mut().unwrap().defer.to_owned() {
            if let Value::Return(v) = interpret_block(&content, self, false, false)? {
                self.frames.pop_front();
                return Ok(Some(*v));
            }
        }
        self.frames.pop_front();
        Ok(None)
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
            const_frame
                .heap
                .insert(t.into(), PRIMS.get(t).unwrap().as_type_element());
        }
        v
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
            ty,
        });
        self.frames.front_mut().unwrap()
    }
    pub fn declare_val(&mut self, name: &SmolStr, value: &T) {
        if let Some(frame) = self.frames.front_mut() {
            frame
        } else {
            self.add_frame(None, FrameType::Normal)
        }
        .heap
        .insert(name.to_owned(), value.to_owned());
    }
    pub fn set_val(
        &mut self,
        name: &SmolStr,
        value: &T,
        position: &Position,
        raw: &String,
    ) -> Result<(), ZyxtError> {
        for frame in self.frames.iter_mut() {
            if frame.heap.contains_key(name) {
                if frame.ty == FrameType::Constants {
                    todo!("Err trying to change const value")
                }
                frame.heap.insert(name.to_owned(), value.to_owned());
                return Ok(());
            }
            if frame.ty == FrameType::Function {
                break;
            }
        }
        Err(ZyxtError::error_3_0(name.to_owned()).with_pos_and_raw(position, raw))
    }
    pub fn get_val(
        &mut self,
        name: &SmolStr,
        position: &Position,
        raw: &String,
    ) -> Result<T, ZyxtError> {
        for frame in self.frames.iter() {
            if frame.heap.contains_key(name) {
                return Ok(frame.heap.get(name).unwrap().to_owned());
            }
            if frame.ty == FrameType::Function {
                break;
            }
        }
        Err(ZyxtError::error_3_0(name.to_owned()).with_pos_and_raw(position, raw))
    }
    pub fn delete_val(
        &mut self,
        name: &SmolStr,
        position: &Position,
        raw: &String,
    ) -> Result<T, ZyxtError> {
        if let Some(v) = self.frames.front_mut().unwrap().heap.remove(name) {
            Ok(v)
        } else {
            Err(ZyxtError::error_3_0(name.to_owned()).with_pos_and_raw(position, raw))
        }
    }
    pub fn add_defer(&mut self, content: Vec<Element>) {
        self.frames.front_mut().unwrap().defer.push(content);
    }
}
