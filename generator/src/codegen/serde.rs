use std::collections::BTreeMap;
use std::io;

use super::{generate_field_type, mutate_field_name, Printer};
use crate::model::*;

pub(super) fn generate_serialize(
    trait_name: &str,
    name: &str,
    props: &BTreeMap<String, PropertySpecification>,
    p: &mut Printer,
) -> io::Result<()> {
    p.block(format_args!("impl {} for {}", trait_name, name), |p| {
        p.block(format_args!("fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error>"), |p| {
            if props.len() > 0 {
                p.line(format_args!("let mut map = ::serde::Serializer::serialize_map(s, None)?;"))?;
                for (prop_name, prop_spec) in props {
                    let field_name = mutate_field_name(prop_name);
                    if prop_spec.required.unwrap_or(true) {
                        p.line(format_args!("::serde::ser::SerializeMap::serialize_entry(&mut map, \"{}\", &self.{})?;", prop_name, field_name))?;
                    } else {
                        p.block(format_args!("if let Some(ref {}) = self.{}", field_name, field_name), |p| {
                            p.line(format_args!("::serde::ser::SerializeMap::serialize_entry(&mut map, \"{}\", {})?;", prop_name, field_name))
                        })?;
                    }
                }
            } else {
                p.line(format_args!("let map = ::serde::Serializer::serialize_map(s, None)?;"))?;
            }
            p.line(format_args!("::serde::ser::SerializeMap::end(map)"))
        })
    })
}

pub(super) fn generate_deserialize(
    name: &str,
    namespace_opt: Option<&str>,
    props: &BTreeMap<String, PropertySpecification>,
    p: &mut Printer,
) -> io::Result<()> {
    p.block(
        format_args!("impl<'de> ::serde::Deserialize<'de> for {}", name),
        |p| {
            p.block(
                format_args!(
                    "fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<{}, D::Error>",
                    name
                ),
                |p| generate_deserialize_body(name, namespace_opt, props, p),
            )
        },
    )
}

pub(super) fn generate_deserialize_value(
    name: &str,
    namespace_opt: Option<&str>,
    props: &BTreeMap<String, PropertySpecification>,
    p: &mut Printer,
) -> io::Result<()> {
    p.block(format_args!("impl crate::codec::DeserializeValue for {}", name), |p| {
        p.block(format_args!("fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<{}, D::Error>", name), |p| {
            generate_deserialize_body(name, namespace_opt, props, p)
        })
    })
}

fn generate_deserialize_body(
    name: &str,
    namespace_opt: Option<&str>,
    props: &BTreeMap<String, PropertySpecification>,
    p: &mut Printer,
) -> io::Result<()> {
    p.line(format_args!("struct Visitor;"))?;
    p.newline()?;
    p.block(
        format_args!("impl<'de> ::serde::de::Visitor<'de> for Visitor"),
        |p| {
            p.line(format_args!("type Value = {};", name))?;
            p.newline()?;
            p.line(format_args!(
                "fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{"
            ))?;
            p.line(format_args!("    write!(f, \"a struct of type {}\")", name))?;
            p.line(format_args!("}}"))?;
            p.newline()?;
            if props.len() > 0 {
                generate_deserialize_visit_map_non_empty(name, namespace_opt, props, p)?;
            } else {
                generate_deserialize_visit_map_empty(name, p)?;
            }
            Ok(())
        },
    )?;
    p.newline()?;
    p.line(format_args!("d.deserialize_map(Visitor)"))
}

fn generate_deserialize_visit_map_empty(name: &str, p: &mut Printer) -> io::Result<()> {
    p.block(format_args!("fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error>"), |p| {
        p.line(format_args!("Ok({} {{}})", name))
    })
}

fn generate_deserialize_visit_map_non_empty(
    name: &str,
    namespace_opt: Option<&str>,
    props: &BTreeMap<String, PropertySpecification>,
    p: &mut Printer,
) -> io::Result<()> {
    p.block(format_args!("fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error>"), |p| {
        for (prop_name, prop_spec) in props {
            let field_name = mutate_field_name(prop_name);
            let field_type = generate_field_type(namespace_opt, &prop_spec);
            p.line(format_args!("let mut {}: Option<{}> = None;", field_name, field_type))?;
        }
        p.newline()?;
        p.block(format_args!("while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)?"), |p| {
            p.block(format_args!("match __cfn_key.as_ref()"), |p| {
                for (prop_name, _prop_spec) in props {
                    let field_name = mutate_field_name(prop_name);
                    p.line(format_args!("\"{}\" => {{", prop_name))?;
                    p.line(format_args!("    {} = ::serde::de::MapAccess::next_value(&mut map)?;", field_name))?;
                    p.line(format_args!("}}"))?;
                }
                p.line(format_args!("_ => {{}}"))
            })
        })?;
        p.newline()?;
        p.line(format_args!("Ok({} {{", name))?;
        for (prop_name, prop_spec) in props {
            let field_name = mutate_field_name(prop_name);
            if prop_spec.required.unwrap_or(true) {
                p.line(format_args!("    {}: {}.ok_or(::serde::de::Error::missing_field(\"{}\"))?,", field_name, field_name, prop_name))?;
            } else {
                p.line(format_args!("    {}: {},", field_name, field_name))?;
            }
        }
        p.line(format_args!("}})"))
    })
}
