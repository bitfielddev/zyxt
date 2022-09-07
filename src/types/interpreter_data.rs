use std::{collections::HashMap, fmt::Display};
use lazy_static::lazy_static;
use maplit::hashmap;

use smol_str::SmolStr;

use crate::{
    interpreter::interpret_block,
    types::{errors::ZyxtError, position::Position, printer::Print, typeobj::{
        Type,
        bool_t::BOOL_T, f16_t::F16_T, f32_t::F32_T, f64_t::F64_T, i128_t::I128_T, i16_t::I16_T, i32_t::I32_T,
        i64_t::I64_T, i8_t::I8_T, ibig_t::IBIG_T, isize_t::ISIZE_T, str_t::STR_T,
        type_t::TYPE_T, u128_t::U128_T, u16_t::U16_T, u32_t::U32_T, u64_t::U64_T, u8_t::U8_T,
        ubig_t::UBIG_T, usize_t::USIZE_T, unit_t::UNIT_T
    }, value::Value},
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

pub struct FrameData<T: Clone + Display> {
    pub position: Position,
    pub raw_call: String,
    pub args: HashMap<SmolStr, T>,
}
pub struct InterpreterData<'a, T: Clone + Display, O: Print> {
    pub heap: Vec<HashMap<SmolStr, T>>,
    pub defer: Vec<Vec<Vec<Element>>>,
    pub frame_data: Vec<Option<FrameData<T>>>,
    pub out: &'a mut O,
}
impl<'a, O: Print> InterpreterData<'a, Value, O> {
    pub fn default_variable(out: &'a mut O) -> InterpreterData<'a, Value, O> {
        let mut v = InterpreterData {
            heap: vec![HashMap::new()],
            defer: vec![vec![]],
            frame_data: vec![],
            out,
        };
        for t in PRIM_NAMES {
            v.heap[0].insert(
                t.into(),
                Value::Type(PRIMS.get(t).unwrap().to_owned().to_owned()),
            );
        }
        v.add_frame(None);
        v
    }
    pub fn heap_to_string(&self) -> String {
        self.heap
            .iter()
            .map(|hmap| {
                hmap.iter()
                    .map(|(k, v)| format!("{}: {} = {}", k, v.get_type_obj(), v))
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n-------\n")
    }
    pub fn pop_frame(&mut self) -> Result<Option<Value>, ZyxtError> {
        self.heap.pop();
        self.frame_data.pop();

        for content in self.defer.last().unwrap().clone() {
            if let Value::Return(v) = interpret_block(&content, self, false, false)? {
                self.defer.pop();
                return Ok(Some(*v));
            }
        }
        self.defer.pop();
        Ok(None)
    }
}

impl<'a, O: Print> InterpreterData<'a, Type<Element>, O> {
    pub fn default_type(out: &'a mut O) -> InterpreterData<'a, Type<Element>, O> {
        let mut v = InterpreterData {
            heap: vec![HashMap::new()],
            defer: vec![vec![]],
            frame_data: vec![],
            out,
        };
        for t in PRIM_NAMES {
            v.heap[0].insert(
                t.into(),
                PRIMS.get(t).unwrap().as_type_element(),
            );
        }
        v.add_frame(None);
        v
    }
    pub fn pop_frame(&mut self) {
        self.heap.pop();
        self.frame_data.pop();
        self.defer.pop();
    }
}

impl<T: Clone + Display, O: Print> InterpreterData<'_, T, O> {
    pub fn add_frame(&mut self, frame_data: Option<FrameData<T>>) {
        self.heap.push(HashMap::new());
        self.defer.push(vec![]);
        self.frame_data.push(frame_data);
    }
    pub fn declare_val(&mut self, name: &SmolStr, value: &T) {
        self.heap
            .last_mut()
            .unwrap()
            .insert(name.to_owned(), value.to_owned());
    }
    pub fn set_val(
        &mut self,
        name: &SmolStr,
        value: &T,
        position: &Position,
        raw: &String,
    ) -> Result<(), ZyxtError> {
        for set in self.heap.iter_mut().rev() {
            if set.contains_key(name) {
                set.insert(name.to_owned(), value.to_owned());
                return Ok(());
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
        for set in self.heap.iter().rev() {
            if set.contains_key(name) {
                return Ok(set.get(name).unwrap().to_owned());
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
        if let Some(v) = self.heap.last_mut().unwrap().remove(name) {
            Ok(v)
        } else {
            Err(ZyxtError::error_3_0(name.to_owned()).with_pos_and_raw(position, raw))
        }
    }
    pub fn add_defer(&mut self, content: Vec<Element>) {
        self.defer.last_mut().unwrap().push(content);
    }
}
