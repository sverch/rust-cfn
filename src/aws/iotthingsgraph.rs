//! Types for the `IoTThingsGraph` service.

/// The [`AWS::IoTThingsGraph::FlowTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotthingsgraph-flowtemplate.html) resource type.
#[derive(Debug, Default)]
pub struct FlowTemplate {
    properties: FlowTemplateProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `FlowTemplate` resource.
#[derive(Debug, Default)]
pub struct FlowTemplateProperties {
    /// Property [`CompatibleNamespaceVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotthingsgraph-flowtemplate.html#cfn-iotthingsgraph-flowtemplate-compatiblenamespaceversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub compatible_namespace_version: Option<crate::Value<f64>>,
    /// Property [`Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotthingsgraph-flowtemplate.html#cfn-iotthingsgraph-flowtemplate-definition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub definition: crate::Value<self::flow_template::DefinitionDocument>,
}

impl ::serde::Serialize for FlowTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref compatible_namespace_version) = self.compatible_namespace_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompatibleNamespaceVersion", compatible_namespace_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Definition", &self.definition)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FlowTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FlowTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FlowTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FlowTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut compatible_namespace_version: Option<crate::Value<f64>> = None;
                let mut definition: Option<crate::Value<self::flow_template::DefinitionDocument>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CompatibleNamespaceVersion" => {
                            compatible_namespace_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Definition" => {
                            definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FlowTemplateProperties {
                    compatible_namespace_version: compatible_namespace_version,
                    definition: definition.ok_or(::serde::de::Error::missing_field("Definition"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for FlowTemplate {
    type Properties = FlowTemplateProperties;
    const TYPE: &'static str = "AWS::IoTThingsGraph::FlowTemplate";
    fn properties(&self) -> &FlowTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FlowTemplateProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for FlowTemplate {}

impl From<FlowTemplateProperties> for FlowTemplate {
    fn from(properties: FlowTemplateProperties) -> FlowTemplate {
        FlowTemplate { properties, depends_on: None }
    }
}

pub mod flow_template {
    //! Property types for the `FlowTemplate` resource.

    /// The [`AWS::IoTThingsGraph::FlowTemplate.DefinitionDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotthingsgraph-flowtemplate-definitiondocument.html) property type.
    #[derive(Debug, Default)]
    pub struct DefinitionDocument {
        /// Property [`Language`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotthingsgraph-flowtemplate-definitiondocument.html#cfn-iotthingsgraph-flowtemplate-definitiondocument-language).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub language: crate::Value<String>,
        /// Property [`Text`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotthingsgraph-flowtemplate-definitiondocument.html#cfn-iotthingsgraph-flowtemplate-definitiondocument-text).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for DefinitionDocument {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Language", &self.language)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Text", &self.text)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DefinitionDocument {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefinitionDocument, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefinitionDocument;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefinitionDocument")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut language: Option<crate::Value<String>> = None;
                    let mut text: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Language" => {
                                language = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Text" => {
                                text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefinitionDocument {
                        language: language.ok_or(::serde::de::Error::missing_field("Language"))?,
                        text: text.ok_or(::serde::de::Error::missing_field("Text"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
