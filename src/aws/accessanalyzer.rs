//! Types for the `AccessAnalyzer` service.

/// The [`AWS::AccessAnalyzer::Analyzer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-accessanalyzer-analyzer.html) resource type.
#[derive(Debug, Default)]
pub struct Analyzer {
    properties: AnalyzerProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `Analyzer` resource.
#[derive(Debug, Default)]
pub struct AnalyzerProperties {
    /// Property [`AnalyzerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-accessanalyzer-analyzer.html#cfn-accessanalyzer-analyzer-analyzername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub analyzer_name: Option<crate::Value<String>>,
    /// Property [`ArchiveRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-accessanalyzer-analyzer.html#cfn-accessanalyzer-analyzer-archiverules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub archive_rules: Option<crate::ValueList<self::analyzer::ArchiveRule>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-accessanalyzer-analyzer.html#cfn-accessanalyzer-analyzer-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-accessanalyzer-analyzer.html#cfn-accessanalyzer-analyzer-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: crate::Value<String>,
}

impl ::serde::Serialize for AnalyzerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref analyzer_name) = self.analyzer_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnalyzerName", analyzer_name)?;
        }
        if let Some(ref archive_rules) = self.archive_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArchiveRules", archive_rules)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AnalyzerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalyzerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AnalyzerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AnalyzerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut analyzer_name: Option<crate::Value<String>> = None;
                let mut archive_rules: Option<crate::ValueList<self::analyzer::ArchiveRule>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;
                let mut r#type: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AnalyzerName" => {
                            analyzer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ArchiveRules" => {
                            archive_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AnalyzerProperties {
                    analyzer_name: analyzer_name,
                    archive_rules: archive_rules,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Analyzer {
    type Properties = AnalyzerProperties;
    const TYPE: &'static str = "AWS::AccessAnalyzer::Analyzer";
    fn properties(&self) -> &AnalyzerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AnalyzerProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for Analyzer {}

impl From<AnalyzerProperties> for Analyzer {
    fn from(properties: AnalyzerProperties) -> Analyzer {
        Analyzer { properties, depends_on: None }
    }
}

pub mod analyzer {
    //! Property types for the `Analyzer` resource.

    /// The [`AWS::AccessAnalyzer::Analyzer.ArchiveRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-archiverule.html) property type.
    #[derive(Debug, Default)]
    pub struct ArchiveRule {
        /// Property [`Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-archiverule.html#cfn-accessanalyzer-analyzer-archiverule-filter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter: crate::ValueList<Filter>,
        /// Property [`RuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-archiverule.html#cfn-accessanalyzer-analyzer-archiverule-rulename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_name: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for ArchiveRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filter", &self.filter)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleName", &self.rule_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ArchiveRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ArchiveRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ArchiveRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ArchiveRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut filter: Option<crate::ValueList<Filter>> = None;
                    let mut rule_name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Filter" => {
                                filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleName" => {
                                rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ArchiveRule {
                        filter: filter.ok_or(::serde::de::Error::missing_field("Filter"))?,
                        rule_name: rule_name.ok_or(::serde::de::Error::missing_field("RuleName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AccessAnalyzer::Analyzer.Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-filter.html) property type.
    #[derive(Debug, Default)]
    pub struct Filter {
        /// Property [`Contains`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-filter.html#cfn-accessanalyzer-analyzer-filter-contains).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contains: Option<crate::ValueList<String>>,
        /// Property [`Eq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-filter.html#cfn-accessanalyzer-analyzer-filter-eq).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub eq: Option<crate::ValueList<String>>,
        /// Property [`Exists`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-filter.html#cfn-accessanalyzer-analyzer-filter-exists).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exists: Option<crate::Value<bool>>,
        /// Property [`Neq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-filter.html#cfn-accessanalyzer-analyzer-filter-neq).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub neq: Option<crate::ValueList<String>>,
        /// Property [`Property`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-filter.html#cfn-accessanalyzer-analyzer-filter-property).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for Filter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref contains) = self.contains {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Contains", contains)?;
            }
            if let Some(ref eq) = self.eq {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Eq", eq)?;
            }
            if let Some(ref exists) = self.exists {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exists", exists)?;
            }
            if let Some(ref neq) = self.neq {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Neq", neq)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Property", &self.property)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Filter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Filter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Filter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Filter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contains: Option<crate::ValueList<String>> = None;
                    let mut eq: Option<crate::ValueList<String>> = None;
                    let mut exists: Option<crate::Value<bool>> = None;
                    let mut neq: Option<crate::ValueList<String>> = None;
                    let mut property: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Contains" => {
                                contains = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Eq" => {
                                eq = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Exists" => {
                                exists = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Neq" => {
                                neq = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Property" => {
                                property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Filter {
                        contains: contains,
                        eq: eq,
                        exists: exists,
                        neq: neq,
                        property: property.ok_or(::serde::de::Error::missing_field("Property"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
