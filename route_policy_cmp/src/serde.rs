use std::io::Read;

use serde::Deserialize;

/// Read from reader without a recursion limit.
pub fn from_reader<T: for<'a> Deserialize<'a>, R: Read>(reader: R) -> Result<T, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    deserializer.disable_recursion_limit();
    T::deserialize(&mut deserializer)
}

/// Parse from string without a recursion limit.
pub fn from_str<T: for<'a> Deserialize<'a>>(string: &str) -> Result<T, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(string);
    deserializer.disable_recursion_limit();
    T::deserialize(&mut deserializer)
}
