use super::Value;

/// Key-value pair that can be used to identify and organise resources.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: Value<String>
}

impl Tag {
    /// Create a tag from a key-value pair .
    pub fn new<K: Into<String>, V: Into<Value<String>>>(key: K, value: V) -> Self {
        Self {
            key: key.into(),
            value: value.into()
        }
    }
}
