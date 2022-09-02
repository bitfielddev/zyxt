use crate::interpreter::interpret_block;
use crate::objects::errors::ZyxtError;
use crate::objects::position::Position;
use crate::objects::typeobj::Type;
use crate::objects::value::Value;
use crate::Element;
use std::collections::HashMap;
use std::fmt::Display;
use ansi_term::Color::{Red, White, Yellow};
use ansi_term::Style;

const PRIM_NAMES: [&str; 22] = [
    "str", "bool", "i8", "i16", "i32", "i64", "i128", "isize", "ibig", "u8", "u16", "u32", "u64",
    "u128", "usize", "ubig", "f16", "f32", "f64", "_null", "_any", "type",
];

pub trait Print: Clone {
    fn println(&mut self, s: impl Display);
    fn print(&mut self, s: impl Display);
    fn eprintln(&mut self, s: impl Display);
    fn eprint(&mut self, s: impl Display);

    fn debug(&mut self, msg: impl Display);
    fn info(&mut self, msg: impl Display);
    fn warn(&mut self, msg: impl Display);
    fn error(&mut self, msg: impl Display);

    fn verbosity(&self) -> u8;
}
#[derive(Clone)]
pub struct StdIoPrint(pub u8);
impl StdIoPrint {
    fn log_print(&mut self, msg: impl Display, min_verbosity: u8, prefix: &str, color: Style) {
        if self.0 >= min_verbosity {
            self.eprintln(
                msg.to_string()
                    .split('\n')
                    .map(|s| format!("{} {}", color.paint(prefix), s))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        }
    }
}
impl Print for StdIoPrint {
    fn println(&mut self, s: impl Display) {
        println!("{}", s)
    }
    fn print(&mut self, s: impl Display) {
        print!("{}", s)
    }
    fn eprintln(&mut self, s: impl Display) {
        eprintln!("{}", s)
    }
    fn eprint(&mut self, s: impl Display) {
        eprint!("{}", s)
    }

    fn debug(&mut self, msg: impl Display) {
        self.log_print(msg, 2, "[D]", White.bold().dimmed());
    }
    fn info(&mut self, msg: impl Display) {
        self.log_print(msg, 1, "[I]", White.bold());
    }
    fn warn(&mut self, msg: impl Display) {
        self.log_print(msg, 0, "[W]", Yellow.bold());
    }
    fn error(&mut self, msg: impl Display) {
        self.log_print(msg, 0, "[E]", Red.bold());
    }

    fn verbosity(&self) -> u8 {
        self.0
    }
}

pub struct FrameData<T: Clone + Display> {
    pub position: Position,
    pub raw_call: String,
    pub args: HashMap<String, T>,
}
pub struct InterpreterData<'a, T: Clone + Display, O: Print> {
    pub heap: Vec<HashMap<String, T>>,
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
                t.to_string(),
                Value::Type(Type::Instance {
                    name: t.to_string(),
                    type_args: vec![],
                    inst_attrs: Default::default(),
                    implementation: None,
                }),
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

impl<'a, O: Print> InterpreterData<'a, Type, O> {
    pub fn default_type(out: &'a mut O) -> InterpreterData<'a, Type, O> {
        let mut v = InterpreterData {
            heap: vec![HashMap::new()],
            defer: vec![vec![]],
            frame_data: vec![],
            out,
        };
        for t in PRIM_NAMES {
            v.heap[0].insert(
                t.to_string(),
                Type::Instance {
                    name: "type".to_string(),
                    type_args: vec![],
                    inst_attrs: Default::default(),
                    implementation: None,
                },
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
    pub fn declare_val(&mut self, name: &str, value: &T) {
        self.heap
            .last_mut()
            .unwrap()
            .insert(name.to_string(), value.to_owned());
    }
    pub fn set_val(
        &mut self,
        name: &String,
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
        name: &String,
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
        name: &String,
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
