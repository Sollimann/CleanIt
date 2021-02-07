use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Str(String),
    Bool(bool),
    Int16(i16),
    Uint16(u16),
    Int8(i8),
    Uint8(u8),
    HashMap(HashMap<String, u8>),
}

pub fn inspect(value: &Value) -> String {
    match value {
        Value::Str(v) => {
            format!("{}", v)
        }
        Value::Bool(v) => {
            format!("{}", v)
        }
        Value::Int16(v) => {
            format!("{}", v)
        }
        Value::Uint16(v) => {
            format!("{}", v)
        }
        Value::Int8(v) => {
            format!("{}", v)
        }
        Value::Uint8(v) => {
            format!("{}", v)
        }
        Value::HashMap(v) => {
            format!("{:?}", v)
        }
    }
}
