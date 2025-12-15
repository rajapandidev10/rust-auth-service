use serde_json::Value;

pub fn is_empty(value: &Value) -> bool {
    match value {
        Value::Null => true,

        Value::String(s) => s.trim().is_empty(),

        Value::Array(arr) => arr.is_empty(),

        Value::Object(map) => map.is_empty(),

        Value::Bool(_) => false,

        Value::Number(_) => false,
    }
}
