use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::codec::{DeserializeValue, SerializeValue};
use std::collections::HashMap;

use super::{Value, ValueList};

#[derive(Debug, PartialEq, Eq)]
/// AWS CloudFormation provides several built-in functions that help you manage your stacks.
/// Use intrinsic functions in your templates to assign values to properties that are not available until runtime.
pub enum Expr {
    /// Append a set of values into a single value, separated by the specified delimiter.
    /// If a delimiter is the empty string, the set of values are concatenated with no delimiter.
    Join {
        /// The value you want to occur between fragments.
        /// - The delimiter will occur between fragments only.
        /// - It will not terminate the final value.
        delimiter: String,

        /// The list of values you want combined.
        values: Vec<Value<String>>,
    },

    /// Encodes the specified text to base64.
    Base64 {
        /// The text to be base64 encoded.
        text: Box<Value<String>>,
    },

    /// Returns a CIDR address block.
    Cidr {
        /// The base CIDR block to use for creating the CIDR range.
        cidr_block: String,

        /// The number of CIDR blocks to return.
        count: u32,

        /// The size of each CIDR block.
        size: u32,
    },

    /// Returns the value of a key from a mapping.
    FindInMap {
        /// The name of the mapping.
        map_name: String,

        /// The top-level key of the mapping.
        top_level_key: String,

        /// The second-level key within the mapping.
        second_level_key: String,
    },

    /// Returns the value of an attribute from a resource.
    GetAtt {
        /// The name of the resource.
        resource: String,

        /// The attribute of the resource to get.
        attribute: String,
    },

    /// Returns Availability Zones for a region.
    GetAZs {
        /// The region name. If an empty string, the current region is used.
        region: String,
    },

    /// Returns the value of a previously exported output.
    ImportValue {
        /// The name of the exported output to retrieve.
        export_name: String,
    },

    /// Returns the number of elements in a list.
    Length {
        /// The list whose length is to be determined.
        value: Vec<Value<String>>,
    },

    /// Returns the value at the specified index from a list.
    Select {
        /// The index of the value in the list.
        index: u32,

        /// The list from which to select the value.
        values: Box<ValueList<String>>,
    },

    /// Splits a string into a list of substrings.
    Split {
        /// The delimiter used to split the string.
        delimiter: String,

        /// The string to split.
        value: Box<Value<String>>,
    },

    /// Performs string substitution.
    Sub {
        /// The string template containing placeholders to replace.
        template: String,

        /// The values to substitute into the template.
        values: Option<HashMap<String, String>>,
    },
}

#[derive(Serialize)]
#[serde(untagged)]
enum SerializeFnSub<'a> {
    WithMap((&'a str, &'a HashMap<String, String>)),
    WithoutMap(&'a str),
}

#[derive(Serialize)]
enum SerializeExpr<'a> {
    #[serde(rename = "Fn::Join")]
    Join((&'a str, &'a [Value<String>])),
    #[serde(rename = "Fn::Base64")]
    Base64(&'a Value<String>),
    #[serde(rename = "Fn::Cidr")]
    Cidr((&'a str, u32, u32)),
    #[serde(rename = "Fn::FindInMap")]
    FindInMap((&'a str, &'a str, &'a str)),
    #[serde(rename = "Fn::GetAtt")]
    GetAtt((&'a str, &'a str)),
    #[serde(rename = "Fn::GetAZs")]
    GetAZs(&'a str),
    #[serde(rename = "Fn::ImportValue")]
    ImportValue(&'a str),
    #[serde(rename = "Fn::Length")]
    Length(&'a [Value<String>]),
    #[serde(rename = "Fn::Select")]
    Select((u32, &'a ValueList<String>)),
    #[serde(rename = "Fn::Split")]
    Split((&'a str, &'a Value<String>)),
    #[serde(rename = "Fn::Sub")]
    Sub(SerializeFnSub<'a>),
}

impl SerializeValue for Expr {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let proxy = match self {
            Expr::Join {
                ref delimiter,
                ref values,
            } => SerializeExpr::Join((delimiter, values)),
            Expr::Base64 { ref text } => SerializeExpr::Base64(text),
            Expr::Cidr {
                ref cidr_block,
                ref count,
                ref size,
            } => SerializeExpr::Cidr((cidr_block, *count, *size)),
            Expr::FindInMap {
                ref map_name,
                ref top_level_key,
                ref second_level_key,
            } => SerializeExpr::FindInMap((map_name, top_level_key, second_level_key)),
            Expr::GetAtt {
                ref resource,
                ref attribute,
            } => SerializeExpr::GetAtt((resource, attribute)),
            Expr::GetAZs { ref region } => SerializeExpr::GetAZs(region),
            Expr::ImportValue { ref export_name } => SerializeExpr::ImportValue(export_name),
            Expr::Length { ref value } => SerializeExpr::Length(value),
            Expr::Select {
                ref index,
                ref values,
            } => SerializeExpr::Select((*index, values)),
            Expr::Split {
                ref delimiter,
                ref value,
            } => SerializeExpr::Split((delimiter, value)),
            Expr::Sub {
                ref template,
                ref values,
            } => {
                if let Some(values) = values {
                    SerializeExpr::Sub(SerializeFnSub::WithMap((template, values)))
                } else {
                    SerializeExpr::Sub(SerializeFnSub::WithoutMap(template))
                }
            }
        };
        proxy.serialize(s)
    }
}

#[derive(Deserialize)]
#[serde(
    untagged,
    expecting = "valid Fn::Sub string or array with string and variable map"
)]
enum DeserializeFnSub {
    WithMap((String, HashMap<String, String>)),
    WithoutMap(String),
}

#[derive(Deserialize)]
enum DeserializeExpr {
    #[serde(rename = "Fn::Join")]
    Join((String, Vec<Value<String>>)),
    #[serde(rename = "Fn::Base64")]
    Base64(Value<String>),
    #[serde(rename = "Fn::Cidr")]
    Cidr((String, u32, u32)),
    #[serde(rename = "Fn::FindInMap")]
    FindInMap((String, String, String)),
    #[serde(rename = "Fn::GetAtt")]
    GetAtt((String, String)),
    #[serde(rename = "Fn::GetAZs")]
    GetAZs(String),
    #[serde(rename = "Fn::ImportValue")]
    ImportValue(String),
    #[serde(rename = "Fn::Length")]
    Length(Vec<Value<String>>),
    #[serde(rename = "Fn::Select")]
    Select((u32, ValueList<String>)),
    #[serde(rename = "Fn::Split")]
    Split((String, Value<String>)),
    #[serde(rename = "Fn::Sub")]
    Sub(DeserializeFnSub),
}

impl DeserializeValue for Expr {
    fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Expr, D::Error> {
        match Deserialize::deserialize(d)? {
            DeserializeExpr::Join((delimiter, values)) => Ok(Expr::Join { delimiter, values }),
            DeserializeExpr::Base64(text) => Ok(Expr::Base64 {
                text: Box::new(text),
            }),
            DeserializeExpr::Cidr((cidr_block, count, size)) => Ok(Expr::Cidr {
                cidr_block,
                count,
                size,
            }),
            DeserializeExpr::FindInMap((map_name, top_level_key, second_level_key)) => {
                Ok(Expr::FindInMap {
                    map_name,
                    top_level_key,
                    second_level_key,
                })
            }
            DeserializeExpr::GetAtt((resource, attribute)) => Ok(Expr::GetAtt {
                resource,
                attribute,
            }),
            DeserializeExpr::GetAZs(region) => Ok(Expr::GetAZs { region }),
            DeserializeExpr::ImportValue(export_name) => Ok(Expr::ImportValue { export_name }),
            DeserializeExpr::Length(value) => Ok(Expr::Length { value }),
            DeserializeExpr::Select((index, values)) => Ok(Expr::Select {
                index,
                values: Box::new(values),
            }),
            DeserializeExpr::Split((delimiter, value)) => Ok(Expr::Split {
                delimiter,
                value: Box::new(value),
            }),
            DeserializeExpr::Sub(predicate) => match predicate {
                DeserializeFnSub::WithMap((template, values)) => Ok(Expr::Sub {
                    template,
                    values: Some(values),
                }),
                DeserializeFnSub::WithoutMap(template) => Ok(Expr::Sub {
                    template,
                    values: None,
                }),
            },
        }
    }
}
