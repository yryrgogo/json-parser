use std::collections::BTreeMap;

use crate::JsonResult;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    String(String),
    Int(i64),
    Float(f64),
    Boolean(bool),
    Null,
    Object(BTreeMap<String, JsonValue>),
    Array(Vec<JsonValue>),
}

const NULL: JsonValue = JsonValue::Null;
const TRUE: JsonValue = JsonValue::Boolean(true);
const FALSE: JsonValue = JsonValue::Boolean(false);

impl JsonValue {
    pub fn new_object() -> JsonValue {
        JsonValue::Object(BTreeMap::new())
    }

    pub fn new_array() -> JsonValue {
        JsonValue::Array(Vec::new())
    }

    pub fn put<T>(&mut self, key: &str, value: T) -> JsonResult<()>
    where
        T: Into<JsonValue>,
    {
        match *self {
            JsonValue::Object(ref mut btree) => {
                btree.insert(key.into(), value.into());
                Ok(())
            }
            _ => todo!(),
        }
    }
}
