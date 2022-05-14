use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
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

    pub fn put<T>(&mut self, key: &str, value: T) -> JsonValue {
        match *self {
            JsonValue::Object(btree) => btree.insert(key, value),
            _ => todo!(),
        }
    }
}
