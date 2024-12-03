use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::codec::{DeserializeValue, SerializeValue};

use super::Expr;

/// Value that a property can assume.
///
/// This can either be a literal value, or an invocation of an instrinsic function.
#[derive(Debug, Eq, PartialEq)]
pub struct Value<T>(ValueInner<T>);

impl<T> Value<T> {
    /// Create a new value.
    pub fn new(value: T) -> Value<T> {
        Value(ValueInner::Value(Box::new(value)))
    }

    /// Create a new value backed by a reference.
    pub fn reference<S: Into<String>>(id: S) -> Value<T> {
        Value(ValueInner::Ref(id.into()))
    }

    /// Create a new value backed by an expression.
    pub fn expression(expr: Expr) -> Value<T> {
        Value(ValueInner::Expr(expr))
    }

    /// If the value contains a literal value, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_value(&self) -> Option<&T> {
        if let ValueInner::Value(ref value) = self.0 {
            Some(value)
        } else {
            None
        }
    }

    /// If the value is a reference, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_reference(&self) -> Option<&str> {
        if let ValueInner::Ref(ref ref_id) = self.0 {
            Some(ref_id)
        } else {
            None
        }
    }

    /// If the value is an expression, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_expression(&self) -> Option<&Expr> {
        if let ValueInner::Expr(ref expr) = self.0 {
            Some(expr)
        } else {
            None
        }
    }
}

impl<T: Default> Default for Value<T> {
    fn default() -> Value<T> {
        Value::new(T::default())
    }
}

impl<T> From<T> for Value<T> {
    fn from(value: T) -> Value<T> {
        Value::new(value)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ValueInner<T> {
    Value(Box<T>),
    Ref(String),
    Expr(Expr),
}

#[derive(Serialize, Deserialize)]
struct SerdeRef<'a> {
    #[serde(rename = "Ref", borrow)]
    id: &'a str,
}

#[derive(Serialize)]
#[serde(untagged)]
enum SerializeValueProxy<'a, T: 'a + SerializeValue> {
    #[serde(serialize_with = "SerializeValue::serialize_borrow")]
    Value(&'a T),
    #[serde(borrow)]
    Ref(SerdeRef<'a>),
    #[serde(serialize_with = "SerializeValue::serialize_borrow")]
    Expr(&'a Expr),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum DeserializeValueProxy<'a, T: DeserializeValue> {
    #[serde(deserialize_with = "DeserializeValue::deserialize")]
    Value(T),
    #[serde(borrow)]
    Ref(SerdeRef<'a>),
    #[serde(deserialize_with = "DeserializeValue::deserialize")]
    Expr(Expr),
}

impl<T: SerializeValue> Serialize for Value<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let proxy = match self.0 {
            ValueInner::Value(ref literal) => SerializeValueProxy::Value(literal.as_ref()),
            ValueInner::Ref(ref id) => SerializeValueProxy::Ref(SerdeRef { id }),
            ValueInner::Expr(ref expr) => SerializeValueProxy::Expr(expr),
        };
        Serialize::serialize(&proxy, s)
    }
}

impl<'de, T: DeserializeValue> Deserialize<'de> for Value<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(d).map(|proxy| {
            let inner = match proxy {
                DeserializeValueProxy::Value(t) => ValueInner::Value(Box::new(t)),
                DeserializeValueProxy::Ref(SerdeRef { id }) => ValueInner::Ref(id.to_owned()),
                DeserializeValueProxy::Expr(expr) => ValueInner::Expr(expr),
            };
            Value(inner)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::ValueList;

    use super::{Expr, Value};

    #[test]
    fn serialize_u32() {
        let value = Value::new(42u32);
        assert_eq!("\"42\"", ::serde_json::to_string(&value).unwrap());
    }

    #[test]
    fn serialize_u32_string() {
        let value = ::serde_json::from_str::<Value<u32>>("\"42\"").unwrap();
        assert_eq!(&42, value.as_value().unwrap());
    }

    #[test]
    fn serialize_u32_number() {
        let value = ::serde_json::from_str::<Value<u32>>("42").unwrap();
        assert_eq!(&42, value.as_value().unwrap());
    }

    #[test]
    fn serialize_ref() {
        let value = Value::<String>::reference("foo");
        assert_eq!(
            "{\"Ref\":\"foo\"}",
            ::serde_json::to_string(&value).unwrap()
        );
    }

    #[test]
    fn deserialize_ref() {
        let value = ::serde_json::from_str::<Value<String>>("{\"Ref\":\"foo\"}").unwrap();
        assert_eq!("foo", value.as_reference().unwrap());
    }

    struct ExpressionTest {
        expression: String,
        serialized: String,
        parsed: Value<String>,
    }

    impl ExpressionTest {
        fn new(expression: &str, serialized: &str, parsed: Value<String>) -> Self {
            Self {
                expression: expression.to_string(),
                serialized: serialized.to_string(),
                parsed,
            }
        }
    }

    fn get_test_examples() -> Vec<ExpressionTest> {
        let mut values = HashMap::new();
        values.insert("Name".to_string(), "World".to_string());
        vec![
            ExpressionTest::new(
                "Fn::Join",
                r#"{"Fn::Join":[", ",["a","b"]]}"#,
                Value::<String>::expression(Expr::Join {
                    delimiter: ", ".to_owned(),
                    values: vec![Value::new("a".to_owned()), Value::new("b".to_owned())],
                }),
            ),
            ExpressionTest::new(
                "Fn::Base64",
                r#"{"Fn::Base64":"HelloWorld"}"#,
                Value::<String>::expression(Expr::Base64 {
                    text: Box::new(Value::new("HelloWorld".to_owned())),
                }),
            ),
            ExpressionTest::new(
                "Fn::Cidr",
                r#"{"Fn::Cidr":["10.0.0.0/24",1,28]}"#,
                Value::<String>::expression(Expr::Cidr {
                    cidr_block: "10.0.0.0/24".to_owned(),
                    count: 1,
                    size: 28,
                }),
            ),
            ExpressionTest::new(
                "Fn::FindInMap",
                r#"{"Fn::FindInMap":["MyMap","Prod","InstanceType"]}"#,
                Value::<String>::expression(Expr::FindInMap {
                    map_name: "MyMap".to_owned(),
                    top_level_key: "Prod".to_owned(),
                    second_level_key: "InstanceType".to_owned(),
                }),
            ),
            ExpressionTest::new(
                "Fn::GetAtt",
                r#"{"Fn::GetAtt":["MyInstance","PublicIp"]}"#,
                Value::<String>::expression(Expr::GetAtt {
                    resource: "MyInstance".to_owned(),
                    attribute: "PublicIp".to_owned(),
                }),
            ),
            ExpressionTest::new(
                "Fn::GetAZs",
                r#"{"Fn::GetAZs":"us-east-1"}"#,
                Value::<String>::expression(Expr::GetAZs {
                    region: "us-east-1".to_owned(),
                }),
            ),
            ExpressionTest::new(
                "Fn::ImportValue",
                r#"{"Fn::ImportValue":"MyExportedOutput"}"#,
                Value::<String>::expression(Expr::ImportValue {
                    export_name: "MyExportedOutput".to_owned(),
                }),
            ),
            ExpressionTest::new(
                "Fn::Length",
                r#"{"Fn::Length":["a","b","c"]}"#,
                Value::<String>::expression(Expr::Length {
                    value: vec![
                        Value::new("a".to_owned()),
                        Value::new("b".to_owned()),
                        Value::new("c".to_owned()),
                    ],
                }),
            ),
            ExpressionTest::new(
                "Fn::Select",
                r#"{"Fn::Select":[0,["a","b","c"]]}"#,
                Value::<String>::expression(Expr::Select {
                    index: 0,
                    values: Box::new(ValueList::new(vec![
                        Value::new("a".to_owned()),
                        Value::new("b".to_owned()),
                        Value::new("c".to_owned()),
                    ])),
                }),
            ),
            ExpressionTest::new(
                "Fn::Split",
                r#"{"Fn::Split":[" ","a b c"]}"#,
                Value::<String>::expression(Expr::Split {
                    delimiter: " ".to_owned(),
                    value: Box::new(Value::new("a b c".to_owned())),
                }),
            ),
            ExpressionTest::new(
                "Fn::Sub",
                r#"{"Fn::Sub":["Hello ${Name}",{"Name":"World"}]}"#,
                Value::<String>::expression(Expr::Sub {
                    template: "Hello ${Name}".to_owned(),
                    values: Some(values),
                }),
            ),
            ExpressionTest::new(
                "Fn::Sub",
                r#"{"Fn::Sub":"Hello ${Name}"}"#,
                Value::<String>::expression(Expr::Sub {
                    template: "Hello ${Name}".to_owned(),
                    values: None,
                }),
            ),
        ]
    }

    #[test]
    fn serialize_expressions() {
        let examples = get_test_examples();
        for example in examples {
            assert_eq!(
                example.serialized,
                ::serde_json::to_string(&example.parsed).unwrap(),
                "Serializing expression: {}",
                example.expression
            );
        }
    }

    #[test]
    fn deserialize_expressions() {
        let examples = get_test_examples();
        for example in examples {
            assert_eq!(
                example.parsed,
                serde_json::from_str(&example.serialized).unwrap(),
                "Deserializing expression: {}",
                example.expression
            );
        }
    }
}
