use std::iter::FromIterator;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

use crate::codec::{SerializeValue, DeserializeValue};

use super::{expr::{DeserializeExpr, SerializeExpr}, Expr};

/// Like `Value`, except it is used in place of lists of `Value`s in
/// templates.
///
/// For example, if you have a parameter called `SubnetIds` of type
/// `List<AWS::EC2::Subnet::Id>` then, you can use `ValueList::reference("SubnetIds")`
/// to reference it.
#[derive(Debug, PartialEq, Eq)]
pub struct ValueList<T>(ValueListInner<T>);

impl<T> ValueList<T> {
    /// Create a new value list.
    pub fn new<I>(values: I) -> ValueList<T>
        where I: IntoIterator<Item = crate::Value<T>>
    {
        ValueList(ValueListInner::Values(Vec::from_iter(values)))
    }

    /// Create a new value list backed by a reference.
    pub fn reference<S: Into<String>>(id: S) -> ValueList<T> {
        ValueList(ValueListInner::Ref(id.into()))
    }

    /// Create a new value list backed by an expression.
    pub fn expression(expr: Expr) -> ValueList<T> {
        ValueList(ValueListInner::Expr(expr))
    }

    /// If the list contains values, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_values(&self) -> Option<&[crate::Value<T>]> {
        if let ValueListInner::Values(ref values) = self.0 {
            Some(values)
        } else {
            None
        }
    }

    /// If the list is a reference, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_reference(&self) -> Option<&str> {
        if let ValueListInner::Ref(ref ref_id) = self.0 {
            Some(ref_id)
        } else {
            None
        }
    }

    /// If the value is an expression, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_expression(&self) -> Option<&Expr> {
        if let ValueListInner::Expr(ref expr) = self.0 {
            Some(expr)
        } else {
            None
        }
    }
}

impl<T> Default for ValueList<T> {
    fn default() -> ValueList<T> {
        ValueList(ValueListInner::Values(Vec::new()))
    }
}

impl<T> FromIterator<crate::Value<T>> for ValueList<T> {
    fn from_iter<I>(iter: I) -> ValueList<T>
        where I: IntoIterator<Item = crate::Value<T>>
    {
        ValueList::new(iter)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ValueListInner<T> {
    Values(Vec<crate::Value<T>>),
    Ref(String),
    Expr(Expr),
}

#[derive(Serialize, Deserialize)]
struct SerdeRef<'a> {
    #[serde(rename = "Ref", borrow)]
    id: &'a str
}

#[derive(Serialize)]
#[serde(untagged, bound = "T: SerializeValue")]
enum SerializeValueList<'a, T: 'a> {
    Values(&'a Vec<crate::Value<T>>),
    #[serde(borrow)]
    Ref(SerdeRef<'a>),
    Expr(SerializeExpr<'a>)
}

#[derive(Deserialize)]
#[serde(untagged, bound = "T: DeserializeValue")]
enum DeserializeValueList<'a, T> {
    Values(Vec<crate::Value<T>>),
    #[serde(borrow)]
    Ref(SerdeRef<'a>),
    Expr(DeserializeExpr)
}

impl<T: SerializeValue> Serialize for ValueList<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let proxy = match self.0 {
            ValueListInner::Values(ref values) => SerializeValueList::Values(values),
            ValueListInner::Ref(ref id) => SerializeValueList::Ref(SerdeRef { id }),
            ValueListInner::Expr(ref expr) => SerializeValueList::Expr(expr.into()),
        };
        Serialize::serialize(&proxy, s)
    }
}

impl<'de, T: DeserializeValue> Deserialize<'de> for ValueList<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(d).map(|proxy| {
            let inner = match proxy {
                DeserializeValueList::Values(t) =>
                    ValueListInner::Values(t),
                DeserializeValueList::Ref(SerdeRef { id }) =>
                    ValueListInner::Ref(id.to_owned()),
                DeserializeValueList::Expr(expr) =>
                    ValueListInner::Expr(expr.into()),
            };
            ValueList(inner)
        })
    }
}
