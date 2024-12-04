//! Types for the `CloudFront` service.

/// The [`AWS::CloudFront::CachePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cachepolicy.html) resource type.
#[derive(Debug, Default)]
pub struct CachePolicy {
    properties: CachePolicyProperties
}

/// Properties for the `CachePolicy` resource.
#[derive(Debug, Default)]
pub struct CachePolicyProperties {
    /// Property [`CachePolicyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cachepolicy.html#cfn-cloudfront-cachepolicy-cachepolicyconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_policy_config: crate::Value<self::cache_policy::CachePolicyConfig>,
}

impl ::serde::Serialize for CachePolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachePolicyConfig", &self.cache_policy_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CachePolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CachePolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CachePolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CachePolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cache_policy_config: Option<crate::Value<self::cache_policy::CachePolicyConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CachePolicyConfig" => {
                            cache_policy_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CachePolicyProperties {
                    cache_policy_config: cache_policy_config.ok_or(::serde::de::Error::missing_field("CachePolicyConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for CachePolicy {
    type Properties = CachePolicyProperties;
    const TYPE: &'static str = "AWS::CloudFront::CachePolicy";
    fn properties(&self) -> &CachePolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CachePolicyProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for CachePolicy {}

impl From<CachePolicyProperties> for CachePolicy {
    fn from(properties: CachePolicyProperties) -> CachePolicy {
        CachePolicy { properties }
    }
}

/// The [`AWS::CloudFront::CloudFrontOriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cloudfrontoriginaccessidentity.html) resource type.
#[derive(Debug, Default)]
pub struct CloudFrontOriginAccessIdentity {
    properties: CloudFrontOriginAccessIdentityProperties
}

/// Properties for the `CloudFrontOriginAccessIdentity` resource.
#[derive(Debug, Default)]
pub struct CloudFrontOriginAccessIdentityProperties {
    /// Property [`CloudFrontOriginAccessIdentityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cloudfrontoriginaccessidentity.html#cfn-cloudfront-cloudfrontoriginaccessidentity-cloudfrontoriginaccessidentityconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cloud_front_origin_access_identity_config: crate::Value<self::cloud_front_origin_access_identity::CloudFrontOriginAccessIdentityConfig>,
}

impl ::serde::Serialize for CloudFrontOriginAccessIdentityProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudFrontOriginAccessIdentityConfig", &self.cloud_front_origin_access_identity_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CloudFrontOriginAccessIdentityProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudFrontOriginAccessIdentityProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CloudFrontOriginAccessIdentityProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CloudFrontOriginAccessIdentityProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cloud_front_origin_access_identity_config: Option<crate::Value<self::cloud_front_origin_access_identity::CloudFrontOriginAccessIdentityConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CloudFrontOriginAccessIdentityConfig" => {
                            cloud_front_origin_access_identity_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CloudFrontOriginAccessIdentityProperties {
                    cloud_front_origin_access_identity_config: cloud_front_origin_access_identity_config.ok_or(::serde::de::Error::missing_field("CloudFrontOriginAccessIdentityConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for CloudFrontOriginAccessIdentity {
    type Properties = CloudFrontOriginAccessIdentityProperties;
    const TYPE: &'static str = "AWS::CloudFront::CloudFrontOriginAccessIdentity";
    fn properties(&self) -> &CloudFrontOriginAccessIdentityProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CloudFrontOriginAccessIdentityProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for CloudFrontOriginAccessIdentity {}

impl From<CloudFrontOriginAccessIdentityProperties> for CloudFrontOriginAccessIdentity {
    fn from(properties: CloudFrontOriginAccessIdentityProperties) -> CloudFrontOriginAccessIdentity {
        CloudFrontOriginAccessIdentity { properties }
    }
}

/// The [`AWS::CloudFront::Distribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-distribution.html) resource type.
#[derive(Debug, Default)]
pub struct Distribution {
    properties: DistributionProperties
}

/// Properties for the `Distribution` resource.
#[derive(Debug, Default)]
pub struct DistributionProperties {
    /// Property [`DistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-distribution.html#cfn-cloudfront-distribution-distributionconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub distribution_config: crate::Value<self::distribution::DistributionConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-distribution.html#cfn-cloudfront-distribution-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for DistributionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DistributionConfig", &self.distribution_config)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DistributionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DistributionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DistributionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DistributionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut distribution_config: Option<crate::Value<self::distribution::DistributionConfig>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DistributionConfig" => {
                            distribution_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DistributionProperties {
                    distribution_config: distribution_config.ok_or(::serde::de::Error::missing_field("DistributionConfig"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Distribution {
    type Properties = DistributionProperties;
    const TYPE: &'static str = "AWS::CloudFront::Distribution";
    fn properties(&self) -> &DistributionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DistributionProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for Distribution {}

impl From<DistributionProperties> for Distribution {
    fn from(properties: DistributionProperties) -> Distribution {
        Distribution { properties }
    }
}

/// The [`AWS::CloudFront::Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html) resource type.
#[derive(Debug, Default)]
pub struct Function {
    properties: FunctionProperties
}

/// Properties for the `Function` resource.
#[derive(Debug, Default)]
pub struct FunctionProperties {
    /// Property [`AutoPublish`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html#cfn-cloudfront-function-autopublish).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_publish: Option<crate::Value<bool>>,
    /// Property [`FunctionCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html#cfn-cloudfront-function-functioncode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_code: Option<crate::Value<String>>,
    /// Property [`FunctionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html#cfn-cloudfront-function-functionconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_config: Option<crate::Value<self::function::FunctionConfig>>,
    /// Property [`FunctionMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html#cfn-cloudfront-function-functionmetadata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_metadata: Option<crate::Value<self::function::FunctionMetadata>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html#cfn-cloudfront-function-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: crate::Value<String>,
}

impl ::serde::Serialize for FunctionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_publish) = self.auto_publish {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoPublish", auto_publish)?;
        }
        if let Some(ref function_code) = self.function_code {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionCode", function_code)?;
        }
        if let Some(ref function_config) = self.function_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionConfig", function_config)?;
        }
        if let Some(ref function_metadata) = self.function_metadata {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionMetadata", function_metadata)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FunctionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FunctionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FunctionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_publish: Option<crate::Value<bool>> = None;
                let mut function_code: Option<crate::Value<String>> = None;
                let mut function_config: Option<crate::Value<self::function::FunctionConfig>> = None;
                let mut function_metadata: Option<crate::Value<self::function::FunctionMetadata>> = None;
                let mut name: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoPublish" => {
                            auto_publish = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionCode" => {
                            function_code = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionConfig" => {
                            function_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionMetadata" => {
                            function_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FunctionProperties {
                    auto_publish: auto_publish,
                    function_code: function_code,
                    function_config: function_config,
                    function_metadata: function_metadata,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Function {
    type Properties = FunctionProperties;
    const TYPE: &'static str = "AWS::CloudFront::Function";
    fn properties(&self) -> &FunctionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FunctionProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for Function {}

impl From<FunctionProperties> for Function {
    fn from(properties: FunctionProperties) -> Function {
        Function { properties }
    }
}

/// The [`AWS::CloudFront::KeyGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-keygroup.html) resource type.
#[derive(Debug, Default)]
pub struct KeyGroup {
    properties: KeyGroupProperties
}

/// Properties for the `KeyGroup` resource.
#[derive(Debug, Default)]
pub struct KeyGroupProperties {
    /// Property [`KeyGroupConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-keygroup.html#cfn-cloudfront-keygroup-keygroupconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub key_group_config: crate::Value<self::key_group::KeyGroupConfig>,
}

impl ::serde::Serialize for KeyGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyGroupConfig", &self.key_group_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for KeyGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = KeyGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type KeyGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut key_group_config: Option<crate::Value<self::key_group::KeyGroupConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "KeyGroupConfig" => {
                            key_group_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(KeyGroupProperties {
                    key_group_config: key_group_config.ok_or(::serde::de::Error::missing_field("KeyGroupConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for KeyGroup {
    type Properties = KeyGroupProperties;
    const TYPE: &'static str = "AWS::CloudFront::KeyGroup";
    fn properties(&self) -> &KeyGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut KeyGroupProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for KeyGroup {}

impl From<KeyGroupProperties> for KeyGroup {
    fn from(properties: KeyGroupProperties) -> KeyGroup {
        KeyGroup { properties }
    }
}

/// The [`AWS::CloudFront::OriginRequestPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-originrequestpolicy.html) resource type.
#[derive(Debug, Default)]
pub struct OriginRequestPolicy {
    properties: OriginRequestPolicyProperties
}

/// Properties for the `OriginRequestPolicy` resource.
#[derive(Debug, Default)]
pub struct OriginRequestPolicyProperties {
    /// Property [`OriginRequestPolicyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-originrequestpolicy.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub origin_request_policy_config: crate::Value<self::origin_request_policy::OriginRequestPolicyConfig>,
}

impl ::serde::Serialize for OriginRequestPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginRequestPolicyConfig", &self.origin_request_policy_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for OriginRequestPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginRequestPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = OriginRequestPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type OriginRequestPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut origin_request_policy_config: Option<crate::Value<self::origin_request_policy::OriginRequestPolicyConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "OriginRequestPolicyConfig" => {
                            origin_request_policy_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(OriginRequestPolicyProperties {
                    origin_request_policy_config: origin_request_policy_config.ok_or(::serde::de::Error::missing_field("OriginRequestPolicyConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for OriginRequestPolicy {
    type Properties = OriginRequestPolicyProperties;
    const TYPE: &'static str = "AWS::CloudFront::OriginRequestPolicy";
    fn properties(&self) -> &OriginRequestPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OriginRequestPolicyProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for OriginRequestPolicy {}

impl From<OriginRequestPolicyProperties> for OriginRequestPolicy {
    fn from(properties: OriginRequestPolicyProperties) -> OriginRequestPolicy {
        OriginRequestPolicy { properties }
    }
}

/// The [`AWS::CloudFront::PublicKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-publickey.html) resource type.
#[derive(Debug, Default)]
pub struct PublicKey {
    properties: PublicKeyProperties
}

/// Properties for the `PublicKey` resource.
#[derive(Debug, Default)]
pub struct PublicKeyProperties {
    /// Property [`PublicKeyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-publickey.html#cfn-cloudfront-publickey-publickeyconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub public_key_config: crate::Value<self::public_key::PublicKeyConfig>,
}

impl ::serde::Serialize for PublicKeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicKeyConfig", &self.public_key_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PublicKeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PublicKeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PublicKeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PublicKeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut public_key_config: Option<crate::Value<self::public_key::PublicKeyConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PublicKeyConfig" => {
                            public_key_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PublicKeyProperties {
                    public_key_config: public_key_config.ok_or(::serde::de::Error::missing_field("PublicKeyConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for PublicKey {
    type Properties = PublicKeyProperties;
    const TYPE: &'static str = "AWS::CloudFront::PublicKey";
    fn properties(&self) -> &PublicKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PublicKeyProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for PublicKey {}

impl From<PublicKeyProperties> for PublicKey {
    fn from(properties: PublicKeyProperties) -> PublicKey {
        PublicKey { properties }
    }
}

/// The [`AWS::CloudFront::RealtimeLogConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-realtimelogconfig.html) resource type.
#[derive(Debug, Default)]
pub struct RealtimeLogConfig {
    properties: RealtimeLogConfigProperties
}

/// Properties for the `RealtimeLogConfig` resource.
#[derive(Debug, Default)]
pub struct RealtimeLogConfigProperties {
    /// Property [`EndPoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-realtimelogconfig.html#cfn-cloudfront-realtimelogconfig-endpoints).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub end_points: crate::ValueList<self::realtime_log_config::EndPoint>,
    /// Property [`Fields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-realtimelogconfig.html#cfn-cloudfront-realtimelogconfig-fields).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fields: crate::ValueList<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-realtimelogconfig.html#cfn-cloudfront-realtimelogconfig-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: crate::Value<String>,
    /// Property [`SamplingRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-realtimelogconfig.html#cfn-cloudfront-realtimelogconfig-samplingrate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sampling_rate: crate::Value<f64>,
}

impl ::serde::Serialize for RealtimeLogConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndPoints", &self.end_points)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fields", &self.fields)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamplingRate", &self.sampling_rate)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RealtimeLogConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RealtimeLogConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RealtimeLogConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RealtimeLogConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut end_points: Option<crate::ValueList<self::realtime_log_config::EndPoint>> = None;
                let mut fields: Option<crate::ValueList<String>> = None;
                let mut name: Option<crate::Value<String>> = None;
                let mut sampling_rate: Option<crate::Value<f64>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EndPoints" => {
                            end_points = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Fields" => {
                            fields = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SamplingRate" => {
                            sampling_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RealtimeLogConfigProperties {
                    end_points: end_points.ok_or(::serde::de::Error::missing_field("EndPoints"))?,
                    fields: fields.ok_or(::serde::de::Error::missing_field("Fields"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    sampling_rate: sampling_rate.ok_or(::serde::de::Error::missing_field("SamplingRate"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for RealtimeLogConfig {
    type Properties = RealtimeLogConfigProperties;
    const TYPE: &'static str = "AWS::CloudFront::RealtimeLogConfig";
    fn properties(&self) -> &RealtimeLogConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RealtimeLogConfigProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for RealtimeLogConfig {}

impl From<RealtimeLogConfigProperties> for RealtimeLogConfig {
    fn from(properties: RealtimeLogConfigProperties) -> RealtimeLogConfig {
        RealtimeLogConfig { properties }
    }
}

/// The [`AWS::CloudFront::StreamingDistribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-streamingdistribution.html) resource type.
#[derive(Debug, Default)]
pub struct StreamingDistribution {
    properties: StreamingDistributionProperties
}

/// Properties for the `StreamingDistribution` resource.
#[derive(Debug, Default)]
pub struct StreamingDistributionProperties {
    /// Property [`StreamingDistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-streamingdistribution.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub streaming_distribution_config: crate::Value<self::streaming_distribution::StreamingDistributionConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-streamingdistribution.html#cfn-cloudfront-streamingdistribution-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: crate::ValueList<crate::Tag>,
}

impl ::serde::Serialize for StreamingDistributionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamingDistributionConfig", &self.streaming_distribution_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StreamingDistributionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamingDistributionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StreamingDistributionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StreamingDistributionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut streaming_distribution_config: Option<crate::Value<self::streaming_distribution::StreamingDistributionConfig>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "StreamingDistributionConfig" => {
                            streaming_distribution_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StreamingDistributionProperties {
                    streaming_distribution_config: streaming_distribution_config.ok_or(::serde::de::Error::missing_field("StreamingDistributionConfig"))?,
                    tags: tags.ok_or(::serde::de::Error::missing_field("Tags"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for StreamingDistribution {
    type Properties = StreamingDistributionProperties;
    const TYPE: &'static str = "AWS::CloudFront::StreamingDistribution";
    fn properties(&self) -> &StreamingDistributionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StreamingDistributionProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for StreamingDistribution {}

impl From<StreamingDistributionProperties> for StreamingDistribution {
    fn from(properties: StreamingDistributionProperties) -> StreamingDistribution {
        StreamingDistribution { properties }
    }
}

pub mod cache_policy {
    //! Property types for the `CachePolicy` resource.

    /// The [`AWS::CloudFront::CachePolicy.CachePolicyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CachePolicyConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<crate::Value<String>>,
        /// Property [`DefaultTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-defaultttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_ttl: crate::Value<f64>,
        /// Property [`MaxTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-maxttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_ttl: crate::Value<f64>,
        /// Property [`MinTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-minttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_ttl: crate::Value<f64>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: crate::Value<String>,
        /// Property [`ParametersInCacheKeyAndForwardedToOrigin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-parametersincachekeyandforwardedtoorigin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters_in_cache_key_and_forwarded_to_origin: crate::Value<ParametersInCacheKeyAndForwardedToOrigin>,
    }

    impl crate::codec::SerializeValue for CachePolicyConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultTTL", &self.default_ttl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxTTL", &self.max_ttl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinTTL", &self.min_ttl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParametersInCacheKeyAndForwardedToOrigin", &self.parameters_in_cache_key_and_forwarded_to_origin)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for CachePolicyConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CachePolicyConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CachePolicyConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CachePolicyConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<crate::Value<String>> = None;
                    let mut default_ttl: Option<crate::Value<f64>> = None;
                    let mut max_ttl: Option<crate::Value<f64>> = None;
                    let mut min_ttl: Option<crate::Value<f64>> = None;
                    let mut name: Option<crate::Value<String>> = None;
                    let mut parameters_in_cache_key_and_forwarded_to_origin: Option<crate::Value<ParametersInCacheKeyAndForwardedToOrigin>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultTTL" => {
                                default_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxTTL" => {
                                max_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinTTL" => {
                                min_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParametersInCacheKeyAndForwardedToOrigin" => {
                                parameters_in_cache_key_and_forwarded_to_origin = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CachePolicyConfig {
                        comment: comment,
                        default_ttl: default_ttl.ok_or(::serde::de::Error::missing_field("DefaultTTL"))?,
                        max_ttl: max_ttl.ok_or(::serde::de::Error::missing_field("MaxTTL"))?,
                        min_ttl: min_ttl.ok_or(::serde::de::Error::missing_field("MinTTL"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        parameters_in_cache_key_and_forwarded_to_origin: parameters_in_cache_key_and_forwarded_to_origin.ok_or(::serde::de::Error::missing_field("ParametersInCacheKeyAndForwardedToOrigin"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::CachePolicy.CookiesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cookiesconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CookiesConfig {
        /// Property [`CookieBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cookiesconfig.html#cfn-cloudfront-cachepolicy-cookiesconfig-cookiebehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookie_behavior: crate::Value<String>,
        /// Property [`Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cookiesconfig.html#cfn-cloudfront-cachepolicy-cookiesconfig-cookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for CookiesConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookieBehavior", &self.cookie_behavior)?;
            if let Some(ref cookies) = self.cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cookies", cookies)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for CookiesConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CookiesConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CookiesConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CookiesConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookie_behavior: Option<crate::Value<String>> = None;
                    let mut cookies: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CookieBehavior" => {
                                cookie_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cookies" => {
                                cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CookiesConfig {
                        cookie_behavior: cookie_behavior.ok_or(::serde::de::Error::missing_field("CookieBehavior"))?,
                        cookies: cookies,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::CachePolicy.HeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-headersconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HeadersConfig {
        /// Property [`HeaderBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-headersconfig.html#cfn-cloudfront-cachepolicy-headersconfig-headerbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_behavior: crate::Value<String>,
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-headersconfig.html#cfn-cloudfront-cachepolicy-headersconfig-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for HeadersConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderBehavior", &self.header_behavior)?;
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for HeadersConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HeadersConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HeadersConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HeadersConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_behavior: Option<crate::Value<String>> = None;
                    let mut headers: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderBehavior" => {
                                header_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HeadersConfig {
                        header_behavior: header_behavior.ok_or(::serde::de::Error::missing_field("HeaderBehavior"))?,
                        headers: headers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::CachePolicy.ParametersInCacheKeyAndForwardedToOrigin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html) property type.
    #[derive(Debug, Default)]
    pub struct ParametersInCacheKeyAndForwardedToOrigin {
        /// Property [`CookiesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html#cfn-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin-cookiesconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies_config: crate::Value<CookiesConfig>,
        /// Property [`EnableAcceptEncodingBrotli`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html#cfn-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin-enableacceptencodingbrotli).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_accept_encoding_brotli: Option<crate::Value<bool>>,
        /// Property [`EnableAcceptEncodingGzip`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html#cfn-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin-enableacceptencodinggzip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_accept_encoding_gzip: crate::Value<bool>,
        /// Property [`HeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html#cfn-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin-headersconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers_config: crate::Value<HeadersConfig>,
        /// Property [`QueryStringsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html#cfn-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin-querystringsconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_strings_config: crate::Value<QueryStringsConfig>,
    }

    impl crate::codec::SerializeValue for ParametersInCacheKeyAndForwardedToOrigin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookiesConfig", &self.cookies_config)?;
            if let Some(ref enable_accept_encoding_brotli) = self.enable_accept_encoding_brotli {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableAcceptEncodingBrotli", enable_accept_encoding_brotli)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableAcceptEncodingGzip", &self.enable_accept_encoding_gzip)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeadersConfig", &self.headers_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringsConfig", &self.query_strings_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ParametersInCacheKeyAndForwardedToOrigin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParametersInCacheKeyAndForwardedToOrigin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParametersInCacheKeyAndForwardedToOrigin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParametersInCacheKeyAndForwardedToOrigin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookies_config: Option<crate::Value<CookiesConfig>> = None;
                    let mut enable_accept_encoding_brotli: Option<crate::Value<bool>> = None;
                    let mut enable_accept_encoding_gzip: Option<crate::Value<bool>> = None;
                    let mut headers_config: Option<crate::Value<HeadersConfig>> = None;
                    let mut query_strings_config: Option<crate::Value<QueryStringsConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CookiesConfig" => {
                                cookies_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableAcceptEncodingBrotli" => {
                                enable_accept_encoding_brotli = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableAcceptEncodingGzip" => {
                                enable_accept_encoding_gzip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeadersConfig" => {
                                headers_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStringsConfig" => {
                                query_strings_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ParametersInCacheKeyAndForwardedToOrigin {
                        cookies_config: cookies_config.ok_or(::serde::de::Error::missing_field("CookiesConfig"))?,
                        enable_accept_encoding_brotli: enable_accept_encoding_brotli,
                        enable_accept_encoding_gzip: enable_accept_encoding_gzip.ok_or(::serde::de::Error::missing_field("EnableAcceptEncodingGzip"))?,
                        headers_config: headers_config.ok_or(::serde::de::Error::missing_field("HeadersConfig"))?,
                        query_strings_config: query_strings_config.ok_or(::serde::de::Error::missing_field("QueryStringsConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::CachePolicy.QueryStringsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-querystringsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryStringsConfig {
        /// Property [`QueryStringBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-querystringsconfig.html#cfn-cloudfront-cachepolicy-querystringsconfig-querystringbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string_behavior: crate::Value<String>,
        /// Property [`QueryStrings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-querystringsconfig.html#cfn-cloudfront-cachepolicy-querystringsconfig-querystrings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_strings: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for QueryStringsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringBehavior", &self.query_string_behavior)?;
            if let Some(ref query_strings) = self.query_strings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStrings", query_strings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for QueryStringsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryStringsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryStringsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryStringsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut query_string_behavior: Option<crate::Value<String>> = None;
                    let mut query_strings: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "QueryStringBehavior" => {
                                query_string_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStrings" => {
                                query_strings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryStringsConfig {
                        query_string_behavior: query_string_behavior.ok_or(::serde::de::Error::missing_field("QueryStringBehavior"))?,
                        query_strings: query_strings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod cloud_front_origin_access_identity {
    //! Property types for the `CloudFrontOriginAccessIdentity` resource.

    /// The [`AWS::CloudFront::CloudFrontOriginAccessIdentity.CloudFrontOriginAccessIdentityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cloudfrontoriginaccessidentity-cloudfrontoriginaccessidentityconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudFrontOriginAccessIdentityConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cloudfrontoriginaccessidentity-cloudfrontoriginaccessidentityconfig.html#cfn-cloudfront-cloudfrontoriginaccessidentity-cloudfrontoriginaccessidentityconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for CloudFrontOriginAccessIdentityConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", &self.comment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for CloudFrontOriginAccessIdentityConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudFrontOriginAccessIdentityConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudFrontOriginAccessIdentityConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudFrontOriginAccessIdentityConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudFrontOriginAccessIdentityConfig {
                        comment: comment.ok_or(::serde::de::Error::missing_field("Comment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod distribution {
    //! Property types for the `Distribution` resource.

    /// The [`AWS::CloudFront::Distribution.CacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html) property type.
    #[derive(Debug, Default)]
    pub struct CacheBehavior {
        /// Property [`AllowedMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-allowedmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_methods: Option<crate::ValueList<String>>,
        /// Property [`CachePolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-cachepolicyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_policy_id: Option<crate::Value<String>>,
        /// Property [`CachedMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-cachedmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cached_methods: Option<crate::ValueList<String>>,
        /// Property [`Compress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-compress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compress: Option<crate::Value<bool>>,
        /// Property [`DefaultTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-defaultttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_ttl: Option<crate::Value<f64>>,
        /// Property [`FieldLevelEncryptionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-fieldlevelencryptionid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_level_encryption_id: Option<crate::Value<String>>,
        /// Property [`ForwardedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-forwardedvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_values: Option<crate::Value<ForwardedValues>>,
        /// Property [`FunctionAssociations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-functionassociations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_associations: Option<crate::ValueList<FunctionAssociation>>,
        /// Property [`LambdaFunctionAssociations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-lambdafunctionassociations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_function_associations: Option<crate::ValueList<LambdaFunctionAssociation>>,
        /// Property [`MaxTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-maxttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_ttl: Option<crate::Value<f64>>,
        /// Property [`MinTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-minttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_ttl: Option<crate::Value<f64>>,
        /// Property [`OriginRequestPolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-originrequestpolicyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_request_policy_id: Option<crate::Value<String>>,
        /// Property [`PathPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-pathpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path_pattern: crate::Value<String>,
        /// Property [`RealtimeLogConfigArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-realtimelogconfigarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub realtime_log_config_arn: Option<crate::Value<String>>,
        /// Property [`SmoothStreaming`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-smoothstreaming).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub smooth_streaming: Option<crate::Value<bool>>,
        /// Property [`TargetOriginId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-targetoriginid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_origin_id: crate::Value<String>,
        /// Property [`TrustedKeyGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-trustedkeygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trusted_key_groups: Option<crate::ValueList<String>>,
        /// Property [`TrustedSigners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-trustedsigners).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trusted_signers: Option<crate::ValueList<String>>,
        /// Property [`ViewerProtocolPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-viewerprotocolpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub viewer_protocol_policy: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for CacheBehavior {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_methods) = self.allowed_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedMethods", allowed_methods)?;
            }
            if let Some(ref cache_policy_id) = self.cache_policy_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachePolicyId", cache_policy_id)?;
            }
            if let Some(ref cached_methods) = self.cached_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachedMethods", cached_methods)?;
            }
            if let Some(ref compress) = self.compress {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compress", compress)?;
            }
            if let Some(ref default_ttl) = self.default_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultTTL", default_ttl)?;
            }
            if let Some(ref field_level_encryption_id) = self.field_level_encryption_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldLevelEncryptionId", field_level_encryption_id)?;
            }
            if let Some(ref forwarded_values) = self.forwarded_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedValues", forwarded_values)?;
            }
            if let Some(ref function_associations) = self.function_associations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionAssociations", function_associations)?;
            }
            if let Some(ref lambda_function_associations) = self.lambda_function_associations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionAssociations", lambda_function_associations)?;
            }
            if let Some(ref max_ttl) = self.max_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxTTL", max_ttl)?;
            }
            if let Some(ref min_ttl) = self.min_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinTTL", min_ttl)?;
            }
            if let Some(ref origin_request_policy_id) = self.origin_request_policy_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginRequestPolicyId", origin_request_policy_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathPattern", &self.path_pattern)?;
            if let Some(ref realtime_log_config_arn) = self.realtime_log_config_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RealtimeLogConfigArn", realtime_log_config_arn)?;
            }
            if let Some(ref smooth_streaming) = self.smooth_streaming {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmoothStreaming", smooth_streaming)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetOriginId", &self.target_origin_id)?;
            if let Some(ref trusted_key_groups) = self.trusted_key_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedKeyGroups", trusted_key_groups)?;
            }
            if let Some(ref trusted_signers) = self.trusted_signers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedSigners", trusted_signers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewerProtocolPolicy", &self.viewer_protocol_policy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for CacheBehavior {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CacheBehavior, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CacheBehavior;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CacheBehavior")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_methods: Option<crate::ValueList<String>> = None;
                    let mut cache_policy_id: Option<crate::Value<String>> = None;
                    let mut cached_methods: Option<crate::ValueList<String>> = None;
                    let mut compress: Option<crate::Value<bool>> = None;
                    let mut default_ttl: Option<crate::Value<f64>> = None;
                    let mut field_level_encryption_id: Option<crate::Value<String>> = None;
                    let mut forwarded_values: Option<crate::Value<ForwardedValues>> = None;
                    let mut function_associations: Option<crate::ValueList<FunctionAssociation>> = None;
                    let mut lambda_function_associations: Option<crate::ValueList<LambdaFunctionAssociation>> = None;
                    let mut max_ttl: Option<crate::Value<f64>> = None;
                    let mut min_ttl: Option<crate::Value<f64>> = None;
                    let mut origin_request_policy_id: Option<crate::Value<String>> = None;
                    let mut path_pattern: Option<crate::Value<String>> = None;
                    let mut realtime_log_config_arn: Option<crate::Value<String>> = None;
                    let mut smooth_streaming: Option<crate::Value<bool>> = None;
                    let mut target_origin_id: Option<crate::Value<String>> = None;
                    let mut trusted_key_groups: Option<crate::ValueList<String>> = None;
                    let mut trusted_signers: Option<crate::ValueList<String>> = None;
                    let mut viewer_protocol_policy: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedMethods" => {
                                allowed_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachePolicyId" => {
                                cache_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachedMethods" => {
                                cached_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Compress" => {
                                compress = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultTTL" => {
                                default_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldLevelEncryptionId" => {
                                field_level_encryption_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedValues" => {
                                forwarded_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FunctionAssociations" => {
                                function_associations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaFunctionAssociations" => {
                                lambda_function_associations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxTTL" => {
                                max_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinTTL" => {
                                min_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginRequestPolicyId" => {
                                origin_request_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PathPattern" => {
                                path_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RealtimeLogConfigArn" => {
                                realtime_log_config_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SmoothStreaming" => {
                                smooth_streaming = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetOriginId" => {
                                target_origin_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrustedKeyGroups" => {
                                trusted_key_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrustedSigners" => {
                                trusted_signers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ViewerProtocolPolicy" => {
                                viewer_protocol_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CacheBehavior {
                        allowed_methods: allowed_methods,
                        cache_policy_id: cache_policy_id,
                        cached_methods: cached_methods,
                        compress: compress,
                        default_ttl: default_ttl,
                        field_level_encryption_id: field_level_encryption_id,
                        forwarded_values: forwarded_values,
                        function_associations: function_associations,
                        lambda_function_associations: lambda_function_associations,
                        max_ttl: max_ttl,
                        min_ttl: min_ttl,
                        origin_request_policy_id: origin_request_policy_id,
                        path_pattern: path_pattern.ok_or(::serde::de::Error::missing_field("PathPattern"))?,
                        realtime_log_config_arn: realtime_log_config_arn,
                        smooth_streaming: smooth_streaming,
                        target_origin_id: target_origin_id.ok_or(::serde::de::Error::missing_field("TargetOriginId"))?,
                        trusted_key_groups: trusted_key_groups,
                        trusted_signers: trusted_signers,
                        viewer_protocol_policy: viewer_protocol_policy.ok_or(::serde::de::Error::missing_field("ViewerProtocolPolicy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cookies.html) property type.
    #[derive(Debug, Default)]
    pub struct Cookies {
        /// Property [`Forward`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cookies.html#cfn-cloudfront-distribution-cookies-forward).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forward: crate::Value<String>,
        /// Property [`WhitelistedNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cookies.html#cfn-cloudfront-distribution-cookies-whitelistednames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub whitelisted_names: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for Cookies {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Forward", &self.forward)?;
            if let Some(ref whitelisted_names) = self.whitelisted_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WhitelistedNames", whitelisted_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Cookies {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Cookies, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Cookies;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Cookies")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut forward: Option<crate::Value<String>> = None;
                    let mut whitelisted_names: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Forward" => {
                                forward = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WhitelistedNames" => {
                                whitelisted_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Cookies {
                        forward: forward.ok_or(::serde::de::Error::missing_field("Forward"))?,
                        whitelisted_names: whitelisted_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.CustomErrorResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomErrorResponse {
        /// Property [`ErrorCachingMinTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html#cfn-cloudfront-distribution-customerrorresponse-errorcachingminttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_caching_min_ttl: Option<crate::Value<f64>>,
        /// Property [`ErrorCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html#cfn-cloudfront-distribution-customerrorresponse-errorcode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_code: crate::Value<u32>,
        /// Property [`ResponseCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html#cfn-cloudfront-distribution-customerrorresponse-responsecode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_code: Option<crate::Value<u32>>,
        /// Property [`ResponsePagePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html#cfn-cloudfront-distribution-customerrorresponse-responsepagepath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_page_path: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for CustomErrorResponse {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error_caching_min_ttl) = self.error_caching_min_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorCachingMinTTL", error_caching_min_ttl)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorCode", &self.error_code)?;
            if let Some(ref response_code) = self.response_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseCode", response_code)?;
            }
            if let Some(ref response_page_path) = self.response_page_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponsePagePath", response_page_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for CustomErrorResponse {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomErrorResponse, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomErrorResponse;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomErrorResponse")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error_caching_min_ttl: Option<crate::Value<f64>> = None;
                    let mut error_code: Option<crate::Value<u32>> = None;
                    let mut response_code: Option<crate::Value<u32>> = None;
                    let mut response_page_path: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ErrorCachingMinTTL" => {
                                error_caching_min_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ErrorCode" => {
                                error_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseCode" => {
                                response_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponsePagePath" => {
                                response_page_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomErrorResponse {
                        error_caching_min_ttl: error_caching_min_ttl,
                        error_code: error_code.ok_or(::serde::de::Error::missing_field("ErrorCode"))?,
                        response_code: response_code,
                        response_page_path: response_page_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.CustomOriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomOriginConfig {
        /// Property [`HTTPPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-httpport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_port: Option<crate::Value<u32>>,
        /// Property [`HTTPSPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-httpsport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub https_port: Option<crate::Value<u32>>,
        /// Property [`OriginKeepaliveTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-originkeepalivetimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_keepalive_timeout: Option<crate::Value<u32>>,
        /// Property [`OriginProtocolPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-originprotocolpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_protocol_policy: crate::Value<String>,
        /// Property [`OriginReadTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-originreadtimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_read_timeout: Option<crate::Value<u32>>,
        /// Property [`OriginSSLProtocols`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-originsslprotocols).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_ssl_protocols: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for CustomOriginConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref http_port) = self.http_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPPort", http_port)?;
            }
            if let Some(ref https_port) = self.https_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPSPort", https_port)?;
            }
            if let Some(ref origin_keepalive_timeout) = self.origin_keepalive_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginKeepaliveTimeout", origin_keepalive_timeout)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginProtocolPolicy", &self.origin_protocol_policy)?;
            if let Some(ref origin_read_timeout) = self.origin_read_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginReadTimeout", origin_read_timeout)?;
            }
            if let Some(ref origin_ssl_protocols) = self.origin_ssl_protocols {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginSSLProtocols", origin_ssl_protocols)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for CustomOriginConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomOriginConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomOriginConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomOriginConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut http_port: Option<crate::Value<u32>> = None;
                    let mut https_port: Option<crate::Value<u32>> = None;
                    let mut origin_keepalive_timeout: Option<crate::Value<u32>> = None;
                    let mut origin_protocol_policy: Option<crate::Value<String>> = None;
                    let mut origin_read_timeout: Option<crate::Value<u32>> = None;
                    let mut origin_ssl_protocols: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HTTPPort" => {
                                http_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTPSPort" => {
                                https_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginKeepaliveTimeout" => {
                                origin_keepalive_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginProtocolPolicy" => {
                                origin_protocol_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginReadTimeout" => {
                                origin_read_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginSSLProtocols" => {
                                origin_ssl_protocols = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomOriginConfig {
                        http_port: http_port,
                        https_port: https_port,
                        origin_keepalive_timeout: origin_keepalive_timeout,
                        origin_protocol_policy: origin_protocol_policy.ok_or(::serde::de::Error::missing_field("OriginProtocolPolicy"))?,
                        origin_read_timeout: origin_read_timeout,
                        origin_ssl_protocols: origin_ssl_protocols,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.DefaultCacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html) property type.
    #[derive(Debug, Default)]
    pub struct DefaultCacheBehavior {
        /// Property [`AllowedMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-allowedmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_methods: Option<crate::ValueList<String>>,
        /// Property [`CachePolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-cachepolicyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_policy_id: Option<crate::Value<String>>,
        /// Property [`CachedMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-cachedmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cached_methods: Option<crate::ValueList<String>>,
        /// Property [`Compress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-compress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compress: Option<crate::Value<bool>>,
        /// Property [`DefaultTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-defaultttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_ttl: Option<crate::Value<f64>>,
        /// Property [`FieldLevelEncryptionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-fieldlevelencryptionid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_level_encryption_id: Option<crate::Value<String>>,
        /// Property [`ForwardedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-forwardedvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_values: Option<crate::Value<ForwardedValues>>,
        /// Property [`FunctionAssociations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-functionassociations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_associations: Option<crate::ValueList<FunctionAssociation>>,
        /// Property [`LambdaFunctionAssociations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-lambdafunctionassociations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_function_associations: Option<crate::ValueList<LambdaFunctionAssociation>>,
        /// Property [`MaxTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-maxttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_ttl: Option<crate::Value<f64>>,
        /// Property [`MinTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-minttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_ttl: Option<crate::Value<f64>>,
        /// Property [`OriginRequestPolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-originrequestpolicyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_request_policy_id: Option<crate::Value<String>>,
        /// Property [`RealtimeLogConfigArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-realtimelogconfigarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub realtime_log_config_arn: Option<crate::Value<String>>,
        /// Property [`SmoothStreaming`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-smoothstreaming).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub smooth_streaming: Option<crate::Value<bool>>,
        /// Property [`TargetOriginId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-targetoriginid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_origin_id: crate::Value<String>,
        /// Property [`TrustedKeyGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-trustedkeygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trusted_key_groups: Option<crate::ValueList<String>>,
        /// Property [`TrustedSigners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-trustedsigners).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trusted_signers: Option<crate::ValueList<String>>,
        /// Property [`ViewerProtocolPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-viewerprotocolpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub viewer_protocol_policy: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for DefaultCacheBehavior {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_methods) = self.allowed_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedMethods", allowed_methods)?;
            }
            if let Some(ref cache_policy_id) = self.cache_policy_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachePolicyId", cache_policy_id)?;
            }
            if let Some(ref cached_methods) = self.cached_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachedMethods", cached_methods)?;
            }
            if let Some(ref compress) = self.compress {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compress", compress)?;
            }
            if let Some(ref default_ttl) = self.default_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultTTL", default_ttl)?;
            }
            if let Some(ref field_level_encryption_id) = self.field_level_encryption_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldLevelEncryptionId", field_level_encryption_id)?;
            }
            if let Some(ref forwarded_values) = self.forwarded_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedValues", forwarded_values)?;
            }
            if let Some(ref function_associations) = self.function_associations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionAssociations", function_associations)?;
            }
            if let Some(ref lambda_function_associations) = self.lambda_function_associations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionAssociations", lambda_function_associations)?;
            }
            if let Some(ref max_ttl) = self.max_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxTTL", max_ttl)?;
            }
            if let Some(ref min_ttl) = self.min_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinTTL", min_ttl)?;
            }
            if let Some(ref origin_request_policy_id) = self.origin_request_policy_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginRequestPolicyId", origin_request_policy_id)?;
            }
            if let Some(ref realtime_log_config_arn) = self.realtime_log_config_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RealtimeLogConfigArn", realtime_log_config_arn)?;
            }
            if let Some(ref smooth_streaming) = self.smooth_streaming {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmoothStreaming", smooth_streaming)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetOriginId", &self.target_origin_id)?;
            if let Some(ref trusted_key_groups) = self.trusted_key_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedKeyGroups", trusted_key_groups)?;
            }
            if let Some(ref trusted_signers) = self.trusted_signers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedSigners", trusted_signers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewerProtocolPolicy", &self.viewer_protocol_policy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DefaultCacheBehavior {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultCacheBehavior, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultCacheBehavior;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultCacheBehavior")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_methods: Option<crate::ValueList<String>> = None;
                    let mut cache_policy_id: Option<crate::Value<String>> = None;
                    let mut cached_methods: Option<crate::ValueList<String>> = None;
                    let mut compress: Option<crate::Value<bool>> = None;
                    let mut default_ttl: Option<crate::Value<f64>> = None;
                    let mut field_level_encryption_id: Option<crate::Value<String>> = None;
                    let mut forwarded_values: Option<crate::Value<ForwardedValues>> = None;
                    let mut function_associations: Option<crate::ValueList<FunctionAssociation>> = None;
                    let mut lambda_function_associations: Option<crate::ValueList<LambdaFunctionAssociation>> = None;
                    let mut max_ttl: Option<crate::Value<f64>> = None;
                    let mut min_ttl: Option<crate::Value<f64>> = None;
                    let mut origin_request_policy_id: Option<crate::Value<String>> = None;
                    let mut realtime_log_config_arn: Option<crate::Value<String>> = None;
                    let mut smooth_streaming: Option<crate::Value<bool>> = None;
                    let mut target_origin_id: Option<crate::Value<String>> = None;
                    let mut trusted_key_groups: Option<crate::ValueList<String>> = None;
                    let mut trusted_signers: Option<crate::ValueList<String>> = None;
                    let mut viewer_protocol_policy: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedMethods" => {
                                allowed_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachePolicyId" => {
                                cache_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachedMethods" => {
                                cached_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Compress" => {
                                compress = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultTTL" => {
                                default_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldLevelEncryptionId" => {
                                field_level_encryption_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedValues" => {
                                forwarded_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FunctionAssociations" => {
                                function_associations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaFunctionAssociations" => {
                                lambda_function_associations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxTTL" => {
                                max_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinTTL" => {
                                min_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginRequestPolicyId" => {
                                origin_request_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RealtimeLogConfigArn" => {
                                realtime_log_config_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SmoothStreaming" => {
                                smooth_streaming = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetOriginId" => {
                                target_origin_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrustedKeyGroups" => {
                                trusted_key_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrustedSigners" => {
                                trusted_signers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ViewerProtocolPolicy" => {
                                viewer_protocol_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultCacheBehavior {
                        allowed_methods: allowed_methods,
                        cache_policy_id: cache_policy_id,
                        cached_methods: cached_methods,
                        compress: compress,
                        default_ttl: default_ttl,
                        field_level_encryption_id: field_level_encryption_id,
                        forwarded_values: forwarded_values,
                        function_associations: function_associations,
                        lambda_function_associations: lambda_function_associations,
                        max_ttl: max_ttl,
                        min_ttl: min_ttl,
                        origin_request_policy_id: origin_request_policy_id,
                        realtime_log_config_arn: realtime_log_config_arn,
                        smooth_streaming: smooth_streaming,
                        target_origin_id: target_origin_id.ok_or(::serde::de::Error::missing_field("TargetOriginId"))?,
                        trusted_key_groups: trusted_key_groups,
                        trusted_signers: trusted_signers,
                        viewer_protocol_policy: viewer_protocol_policy.ok_or(::serde::de::Error::missing_field("ViewerProtocolPolicy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.DistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DistributionConfig {
        /// Property [`Aliases`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-aliases).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aliases: Option<crate::ValueList<String>>,
        /// Property [`CNAMEs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-cnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cnam_es: Option<crate::ValueList<String>>,
        /// Property [`CacheBehaviors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-cachebehaviors).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_behaviors: Option<crate::ValueList<CacheBehavior>>,
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<crate::Value<String>>,
        /// Property [`CustomErrorResponses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-customerrorresponses).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_error_responses: Option<crate::ValueList<CustomErrorResponse>>,
        /// Property [`CustomOrigin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-customorigin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_origin: Option<crate::Value<LegacyCustomOrigin>>,
        /// Property [`DefaultCacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-defaultcachebehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_cache_behavior: Option<crate::Value<DefaultCacheBehavior>>,
        /// Property [`DefaultRootObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-defaultrootobject).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_root_object: Option<crate::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: crate::Value<bool>,
        /// Property [`HttpVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-httpversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_version: Option<crate::Value<String>>,
        /// Property [`IPV6Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-ipv6enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ipv6_enabled: Option<crate::Value<bool>>,
        /// Property [`Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-logging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging: Option<crate::Value<Logging>>,
        /// Property [`OriginGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-origingroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_groups: Option<crate::Value<OriginGroups>>,
        /// Property [`Origins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-origins).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origins: Option<crate::ValueList<Origin>>,
        /// Property [`PriceClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-priceclass).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub price_class: Option<crate::Value<String>>,
        /// Property [`Restrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-restrictions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub restrictions: Option<crate::Value<Restrictions>>,
        /// Property [`S3Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-s3origin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_origin: Option<crate::Value<LegacyS3Origin>>,
        /// Property [`ViewerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-viewercertificate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub viewer_certificate: Option<crate::Value<ViewerCertificate>>,
        /// Property [`WebACLId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-webaclid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub web_acl_id: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for DistributionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aliases) = self.aliases {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Aliases", aliases)?;
            }
            if let Some(ref cnam_es) = self.cnam_es {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CNAMEs", cnam_es)?;
            }
            if let Some(ref cache_behaviors) = self.cache_behaviors {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheBehaviors", cache_behaviors)?;
            }
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            if let Some(ref custom_error_responses) = self.custom_error_responses {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomErrorResponses", custom_error_responses)?;
            }
            if let Some(ref custom_origin) = self.custom_origin {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomOrigin", custom_origin)?;
            }
            if let Some(ref default_cache_behavior) = self.default_cache_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultCacheBehavior", default_cache_behavior)?;
            }
            if let Some(ref default_root_object) = self.default_root_object {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultRootObject", default_root_object)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref http_version) = self.http_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpVersion", http_version)?;
            }
            if let Some(ref ipv6_enabled) = self.ipv6_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPV6Enabled", ipv6_enabled)?;
            }
            if let Some(ref logging) = self.logging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logging", logging)?;
            }
            if let Some(ref origin_groups) = self.origin_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginGroups", origin_groups)?;
            }
            if let Some(ref origins) = self.origins {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Origins", origins)?;
            }
            if let Some(ref price_class) = self.price_class {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PriceClass", price_class)?;
            }
            if let Some(ref restrictions) = self.restrictions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Restrictions", restrictions)?;
            }
            if let Some(ref s3_origin) = self.s3_origin {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Origin", s3_origin)?;
            }
            if let Some(ref viewer_certificate) = self.viewer_certificate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewerCertificate", viewer_certificate)?;
            }
            if let Some(ref web_acl_id) = self.web_acl_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebACLId", web_acl_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DistributionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DistributionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DistributionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DistributionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aliases: Option<crate::ValueList<String>> = None;
                    let mut cnam_es: Option<crate::ValueList<String>> = None;
                    let mut cache_behaviors: Option<crate::ValueList<CacheBehavior>> = None;
                    let mut comment: Option<crate::Value<String>> = None;
                    let mut custom_error_responses: Option<crate::ValueList<CustomErrorResponse>> = None;
                    let mut custom_origin: Option<crate::Value<LegacyCustomOrigin>> = None;
                    let mut default_cache_behavior: Option<crate::Value<DefaultCacheBehavior>> = None;
                    let mut default_root_object: Option<crate::Value<String>> = None;
                    let mut enabled: Option<crate::Value<bool>> = None;
                    let mut http_version: Option<crate::Value<String>> = None;
                    let mut ipv6_enabled: Option<crate::Value<bool>> = None;
                    let mut logging: Option<crate::Value<Logging>> = None;
                    let mut origin_groups: Option<crate::Value<OriginGroups>> = None;
                    let mut origins: Option<crate::ValueList<Origin>> = None;
                    let mut price_class: Option<crate::Value<String>> = None;
                    let mut restrictions: Option<crate::Value<Restrictions>> = None;
                    let mut s3_origin: Option<crate::Value<LegacyS3Origin>> = None;
                    let mut viewer_certificate: Option<crate::Value<ViewerCertificate>> = None;
                    let mut web_acl_id: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Aliases" => {
                                aliases = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CNAMEs" => {
                                cnam_es = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CacheBehaviors" => {
                                cache_behaviors = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomErrorResponses" => {
                                custom_error_responses = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomOrigin" => {
                                custom_origin = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultCacheBehavior" => {
                                default_cache_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultRootObject" => {
                                default_root_object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpVersion" => {
                                http_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IPV6Enabled" => {
                                ipv6_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Logging" => {
                                logging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginGroups" => {
                                origin_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Origins" => {
                                origins = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PriceClass" => {
                                price_class = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Restrictions" => {
                                restrictions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Origin" => {
                                s3_origin = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ViewerCertificate" => {
                                viewer_certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WebACLId" => {
                                web_acl_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DistributionConfig {
                        aliases: aliases,
                        cnam_es: cnam_es,
                        cache_behaviors: cache_behaviors,
                        comment: comment,
                        custom_error_responses: custom_error_responses,
                        custom_origin: custom_origin,
                        default_cache_behavior: default_cache_behavior,
                        default_root_object: default_root_object,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        http_version: http_version,
                        ipv6_enabled: ipv6_enabled,
                        logging: logging,
                        origin_groups: origin_groups,
                        origins: origins,
                        price_class: price_class,
                        restrictions: restrictions,
                        s3_origin: s3_origin,
                        viewer_certificate: viewer_certificate,
                        web_acl_id: web_acl_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.ForwardedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html) property type.
    #[derive(Debug, Default)]
    pub struct ForwardedValues {
        /// Property [`Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html#cfn-cloudfront-distribution-forwardedvalues-cookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies: Option<crate::Value<Cookies>>,
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html#cfn-cloudfront-distribution-forwardedvalues-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<crate::ValueList<String>>,
        /// Property [`QueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html#cfn-cloudfront-distribution-forwardedvalues-querystring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string: crate::Value<bool>,
        /// Property [`QueryStringCacheKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html#cfn-cloudfront-distribution-forwardedvalues-querystringcachekeys).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string_cache_keys: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for ForwardedValues {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cookies) = self.cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cookies", cookies)?;
            }
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", &self.query_string)?;
            if let Some(ref query_string_cache_keys) = self.query_string_cache_keys {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringCacheKeys", query_string_cache_keys)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ForwardedValues {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ForwardedValues, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ForwardedValues;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ForwardedValues")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookies: Option<crate::Value<Cookies>> = None;
                    let mut headers: Option<crate::ValueList<String>> = None;
                    let mut query_string: Option<crate::Value<bool>> = None;
                    let mut query_string_cache_keys: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cookies" => {
                                cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryString" => {
                                query_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStringCacheKeys" => {
                                query_string_cache_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ForwardedValues {
                        cookies: cookies,
                        headers: headers,
                        query_string: query_string.ok_or(::serde::de::Error::missing_field("QueryString"))?,
                        query_string_cache_keys: query_string_cache_keys,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.FunctionAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-functionassociation.html) property type.
    #[derive(Debug, Default)]
    pub struct FunctionAssociation {
        /// Property [`EventType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-functionassociation.html#cfn-cloudfront-distribution-functionassociation-eventtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_type: Option<crate::Value<String>>,
        /// Property [`FunctionARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-functionassociation.html#cfn-cloudfront-distribution-functionassociation-functionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_arn: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for FunctionAssociation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref event_type) = self.event_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventType", event_type)?;
            }
            if let Some(ref function_arn) = self.function_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionARN", function_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for FunctionAssociation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionAssociation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FunctionAssociation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FunctionAssociation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event_type: Option<crate::Value<String>> = None;
                    let mut function_arn: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventType" => {
                                event_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FunctionARN" => {
                                function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FunctionAssociation {
                        event_type: event_type,
                        function_arn: function_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.GeoRestriction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-georestriction.html) property type.
    #[derive(Debug, Default)]
    pub struct GeoRestriction {
        /// Property [`Locations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-georestriction.html#cfn-cloudfront-distribution-georestriction-locations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub locations: Option<crate::ValueList<String>>,
        /// Property [`RestrictionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-georestriction.html#cfn-cloudfront-distribution-georestriction-restrictiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub restriction_type: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for GeoRestriction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref locations) = self.locations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Locations", locations)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestrictionType", &self.restriction_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for GeoRestriction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeoRestriction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeoRestriction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeoRestriction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut locations: Option<crate::ValueList<String>> = None;
                    let mut restriction_type: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Locations" => {
                                locations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RestrictionType" => {
                                restriction_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeoRestriction {
                        locations: locations,
                        restriction_type: restriction_type.ok_or(::serde::de::Error::missing_field("RestrictionType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.LambdaFunctionAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaFunctionAssociation {
        /// Property [`EventType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html#cfn-cloudfront-distribution-lambdafunctionassociation-eventtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_type: Option<crate::Value<String>>,
        /// Property [`IncludeBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html#cfn-cloudfront-distribution-lambdafunctionassociation-includebody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_body: Option<crate::Value<bool>>,
        /// Property [`LambdaFunctionARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html#cfn-cloudfront-distribution-lambdafunctionassociation-lambdafunctionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_function_arn: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for LambdaFunctionAssociation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref event_type) = self.event_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventType", event_type)?;
            }
            if let Some(ref include_body) = self.include_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeBody", include_body)?;
            }
            if let Some(ref lambda_function_arn) = self.lambda_function_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionARN", lambda_function_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for LambdaFunctionAssociation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaFunctionAssociation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaFunctionAssociation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaFunctionAssociation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event_type: Option<crate::Value<String>> = None;
                    let mut include_body: Option<crate::Value<bool>> = None;
                    let mut lambda_function_arn: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventType" => {
                                event_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeBody" => {
                                include_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaFunctionARN" => {
                                lambda_function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaFunctionAssociation {
                        event_type: event_type,
                        include_body: include_body,
                        lambda_function_arn: lambda_function_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.LegacyCustomOrigin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html) property type.
    #[derive(Debug, Default)]
    pub struct LegacyCustomOrigin {
        /// Property [`DNSName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html#cfn-cloudfront-distribution-legacycustomorigin-dnsname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns_name: crate::Value<String>,
        /// Property [`HTTPPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html#cfn-cloudfront-distribution-legacycustomorigin-httpport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_port: Option<crate::Value<u32>>,
        /// Property [`HTTPSPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html#cfn-cloudfront-distribution-legacycustomorigin-httpsport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub https_port: Option<crate::Value<u32>>,
        /// Property [`OriginProtocolPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html#cfn-cloudfront-distribution-legacycustomorigin-originprotocolpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_protocol_policy: crate::Value<String>,
        /// Property [`OriginSSLProtocols`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html#cfn-cloudfront-distribution-legacycustomorigin-originsslprotocols).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_ssl_protocols: crate::ValueList<String>,
    }

    impl crate::codec::SerializeValue for LegacyCustomOrigin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DNSName", &self.dns_name)?;
            if let Some(ref http_port) = self.http_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPPort", http_port)?;
            }
            if let Some(ref https_port) = self.https_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPSPort", https_port)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginProtocolPolicy", &self.origin_protocol_policy)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginSSLProtocols", &self.origin_ssl_protocols)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for LegacyCustomOrigin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LegacyCustomOrigin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LegacyCustomOrigin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LegacyCustomOrigin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_name: Option<crate::Value<String>> = None;
                    let mut http_port: Option<crate::Value<u32>> = None;
                    let mut https_port: Option<crate::Value<u32>> = None;
                    let mut origin_protocol_policy: Option<crate::Value<String>> = None;
                    let mut origin_ssl_protocols: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DNSName" => {
                                dns_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTPPort" => {
                                http_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTPSPort" => {
                                https_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginProtocolPolicy" => {
                                origin_protocol_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginSSLProtocols" => {
                                origin_ssl_protocols = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LegacyCustomOrigin {
                        dns_name: dns_name.ok_or(::serde::de::Error::missing_field("DNSName"))?,
                        http_port: http_port,
                        https_port: https_port,
                        origin_protocol_policy: origin_protocol_policy.ok_or(::serde::de::Error::missing_field("OriginProtocolPolicy"))?,
                        origin_ssl_protocols: origin_ssl_protocols.ok_or(::serde::de::Error::missing_field("OriginSSLProtocols"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.LegacyS3Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacys3origin.html) property type.
    #[derive(Debug, Default)]
    pub struct LegacyS3Origin {
        /// Property [`DNSName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacys3origin.html#cfn-cloudfront-distribution-legacys3origin-dnsname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns_name: crate::Value<String>,
        /// Property [`OriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacys3origin.html#cfn-cloudfront-distribution-legacys3origin-originaccessidentity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_access_identity: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for LegacyS3Origin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DNSName", &self.dns_name)?;
            if let Some(ref origin_access_identity) = self.origin_access_identity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginAccessIdentity", origin_access_identity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for LegacyS3Origin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LegacyS3Origin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LegacyS3Origin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LegacyS3Origin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_name: Option<crate::Value<String>> = None;
                    let mut origin_access_identity: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DNSName" => {
                                dns_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginAccessIdentity" => {
                                origin_access_identity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LegacyS3Origin {
                        dns_name: dns_name.ok_or(::serde::de::Error::missing_field("DNSName"))?,
                        origin_access_identity: origin_access_identity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html) property type.
    #[derive(Debug, Default)]
    pub struct Logging {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html#cfn-cloudfront-distribution-logging-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: crate::Value<String>,
        /// Property [`IncludeCookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html#cfn-cloudfront-distribution-logging-includecookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_cookies: Option<crate::Value<bool>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html#cfn-cloudfront-distribution-logging-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for Logging {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref include_cookies) = self.include_cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeCookies", include_cookies)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Logging {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Logging, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Logging;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Logging")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<crate::Value<String>> = None;
                    let mut include_cookies: Option<crate::Value<bool>> = None;
                    let mut prefix: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeCookies" => {
                                include_cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Logging {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        include_cookies: include_cookies,
                        prefix: prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html) property type.
    #[derive(Debug, Default)]
    pub struct Origin {
        /// Property [`ConnectionAttempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-connectionattempts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_attempts: Option<crate::Value<u32>>,
        /// Property [`ConnectionTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-connectiontimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_timeout: Option<crate::Value<u32>>,
        /// Property [`CustomOriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-customoriginconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_origin_config: Option<crate::Value<CustomOriginConfig>>,
        /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-domainname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_name: crate::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: crate::Value<String>,
        /// Property [`OriginCustomHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-origincustomheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_custom_headers: Option<crate::ValueList<OriginCustomHeader>>,
        /// Property [`OriginPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-originpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_path: Option<crate::Value<String>>,
        /// Property [`OriginShield`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-originshield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_shield: Option<crate::Value<OriginShield>>,
        /// Property [`S3OriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-s3originconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_origin_config: Option<crate::Value<S3OriginConfig>>,
    }

    impl crate::codec::SerializeValue for Origin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_attempts) = self.connection_attempts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionAttempts", connection_attempts)?;
            }
            if let Some(ref connection_timeout) = self.connection_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionTimeout", connection_timeout)?;
            }
            if let Some(ref custom_origin_config) = self.custom_origin_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomOriginConfig", custom_origin_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref origin_custom_headers) = self.origin_custom_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginCustomHeaders", origin_custom_headers)?;
            }
            if let Some(ref origin_path) = self.origin_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginPath", origin_path)?;
            }
            if let Some(ref origin_shield) = self.origin_shield {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginShield", origin_shield)?;
            }
            if let Some(ref s3_origin_config) = self.s3_origin_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3OriginConfig", s3_origin_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Origin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Origin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Origin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Origin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_attempts: Option<crate::Value<u32>> = None;
                    let mut connection_timeout: Option<crate::Value<u32>> = None;
                    let mut custom_origin_config: Option<crate::Value<CustomOriginConfig>> = None;
                    let mut domain_name: Option<crate::Value<String>> = None;
                    let mut id: Option<crate::Value<String>> = None;
                    let mut origin_custom_headers: Option<crate::ValueList<OriginCustomHeader>> = None;
                    let mut origin_path: Option<crate::Value<String>> = None;
                    let mut origin_shield: Option<crate::Value<OriginShield>> = None;
                    let mut s3_origin_config: Option<crate::Value<S3OriginConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionAttempts" => {
                                connection_attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectionTimeout" => {
                                connection_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomOriginConfig" => {
                                custom_origin_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DomainName" => {
                                domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginCustomHeaders" => {
                                origin_custom_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginPath" => {
                                origin_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginShield" => {
                                origin_shield = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3OriginConfig" => {
                                s3_origin_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Origin {
                        connection_attempts: connection_attempts,
                        connection_timeout: connection_timeout,
                        custom_origin_config: custom_origin_config,
                        domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        origin_custom_headers: origin_custom_headers,
                        origin_path: origin_path,
                        origin_shield: origin_shield,
                        s3_origin_config: s3_origin_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginCustomHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origincustomheader.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginCustomHeader {
        /// Property [`HeaderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origincustomheader.html#cfn-cloudfront-distribution-origincustomheader-headername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_name: crate::Value<String>,
        /// Property [`HeaderValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origincustomheader.html#cfn-cloudfront-distribution-origincustomheader-headervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_value: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for OriginCustomHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderName", &self.header_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderValue", &self.header_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OriginCustomHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginCustomHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginCustomHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginCustomHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_name: Option<crate::Value<String>> = None;
                    let mut header_value: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderName" => {
                                header_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeaderValue" => {
                                header_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginCustomHeader {
                        header_name: header_name.ok_or(::serde::de::Error::missing_field("HeaderName"))?,
                        header_value: header_value.ok_or(::serde::de::Error::missing_field("HeaderValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroup.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginGroup {
        /// Property [`FailoverCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroup.html#cfn-cloudfront-distribution-origingroup-failovercriteria).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failover_criteria: crate::Value<OriginGroupFailoverCriteria>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroup.html#cfn-cloudfront-distribution-origingroup-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: crate::Value<String>,
        /// Property [`Members`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroup.html#cfn-cloudfront-distribution-origingroup-members).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub members: crate::Value<OriginGroupMembers>,
    }

    impl crate::codec::SerializeValue for OriginGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailoverCriteria", &self.failover_criteria)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Members", &self.members)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OriginGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failover_criteria: Option<crate::Value<OriginGroupFailoverCriteria>> = None;
                    let mut id: Option<crate::Value<String>> = None;
                    let mut members: Option<crate::Value<OriginGroupMembers>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailoverCriteria" => {
                                failover_criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Members" => {
                                members = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginGroup {
                        failover_criteria: failover_criteria.ok_or(::serde::de::Error::missing_field("FailoverCriteria"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        members: members.ok_or(::serde::de::Error::missing_field("Members"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginGroupFailoverCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupfailovercriteria.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginGroupFailoverCriteria {
        /// Property [`StatusCodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupfailovercriteria.html#cfn-cloudfront-distribution-origingroupfailovercriteria-statuscodes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status_codes: crate::Value<StatusCodes>,
    }

    impl crate::codec::SerializeValue for OriginGroupFailoverCriteria {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCodes", &self.status_codes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OriginGroupFailoverCriteria {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginGroupFailoverCriteria, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginGroupFailoverCriteria;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginGroupFailoverCriteria")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut status_codes: Option<crate::Value<StatusCodes>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StatusCodes" => {
                                status_codes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginGroupFailoverCriteria {
                        status_codes: status_codes.ok_or(::serde::de::Error::missing_field("StatusCodes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginGroupMember`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmember.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginGroupMember {
        /// Property [`OriginId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmember.html#cfn-cloudfront-distribution-origingroupmember-originid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_id: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for OriginGroupMember {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginId", &self.origin_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OriginGroupMember {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginGroupMember, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginGroupMember;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginGroupMember")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut origin_id: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OriginId" => {
                                origin_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginGroupMember {
                        origin_id: origin_id.ok_or(::serde::de::Error::missing_field("OriginId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginGroupMembers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmembers.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginGroupMembers {
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmembers.html#cfn-cloudfront-distribution-origingroupmembers-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: crate::ValueList<OriginGroupMember>,
        /// Property [`Quantity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmembers.html#cfn-cloudfront-distribution-origingroupmembers-quantity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quantity: crate::Value<u32>,
    }

    impl crate::codec::SerializeValue for OriginGroupMembers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Quantity", &self.quantity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OriginGroupMembers {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginGroupMembers, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginGroupMembers;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginGroupMembers")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut items: Option<crate::ValueList<OriginGroupMember>> = None;
                    let mut quantity: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Quantity" => {
                                quantity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginGroupMembers {
                        items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                        quantity: quantity.ok_or(::serde::de::Error::missing_field("Quantity"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroups.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginGroups {
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroups.html#cfn-cloudfront-distribution-origingroups-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: Option<crate::ValueList<OriginGroup>>,
        /// Property [`Quantity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroups.html#cfn-cloudfront-distribution-origingroups-quantity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quantity: crate::Value<u32>,
    }

    impl crate::codec::SerializeValue for OriginGroups {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref items) = self.items {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", items)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Quantity", &self.quantity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OriginGroups {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginGroups, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginGroups;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginGroups")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut items: Option<crate::ValueList<OriginGroup>> = None;
                    let mut quantity: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Quantity" => {
                                quantity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginGroups {
                        items: items,
                        quantity: quantity.ok_or(::serde::de::Error::missing_field("Quantity"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginShield`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-originshield.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginShield {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-originshield.html#cfn-cloudfront-distribution-originshield-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<crate::Value<bool>>,
        /// Property [`OriginShieldRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-originshield.html#cfn-cloudfront-distribution-originshield-originshieldregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_shield_region: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for OriginShield {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref origin_shield_region) = self.origin_shield_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginShieldRegion", origin_shield_region)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OriginShield {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginShield, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginShield;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginShield")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<crate::Value<bool>> = None;
                    let mut origin_shield_region: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginShieldRegion" => {
                                origin_shield_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginShield {
                        enabled: enabled,
                        origin_shield_region: origin_shield_region,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.Restrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-restrictions.html) property type.
    #[derive(Debug, Default)]
    pub struct Restrictions {
        /// Property [`GeoRestriction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-restrictions.html#cfn-cloudfront-distribution-restrictions-georestriction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub geo_restriction: crate::Value<GeoRestriction>,
    }

    impl crate::codec::SerializeValue for Restrictions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeoRestriction", &self.geo_restriction)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Restrictions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Restrictions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Restrictions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Restrictions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut geo_restriction: Option<crate::Value<GeoRestriction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GeoRestriction" => {
                                geo_restriction = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Restrictions {
                        geo_restriction: geo_restriction.ok_or(::serde::de::Error::missing_field("GeoRestriction"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.S3OriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-s3originconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct S3OriginConfig {
        /// Property [`OriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-s3originconfig.html#cfn-cloudfront-distribution-s3originconfig-originaccessidentity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_access_identity: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for S3OriginConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref origin_access_identity) = self.origin_access_identity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginAccessIdentity", origin_access_identity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for S3OriginConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3OriginConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3OriginConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3OriginConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut origin_access_identity: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OriginAccessIdentity" => {
                                origin_access_identity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3OriginConfig {
                        origin_access_identity: origin_access_identity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.StatusCodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-statuscodes.html) property type.
    #[derive(Debug, Default)]
    pub struct StatusCodes {
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-statuscodes.html#cfn-cloudfront-distribution-statuscodes-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: crate::ValueList<u32>,
        /// Property [`Quantity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-statuscodes.html#cfn-cloudfront-distribution-statuscodes-quantity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quantity: crate::Value<u32>,
    }

    impl crate::codec::SerializeValue for StatusCodes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Quantity", &self.quantity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for StatusCodes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatusCodes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatusCodes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatusCodes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut items: Option<crate::ValueList<u32>> = None;
                    let mut quantity: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Quantity" => {
                                quantity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatusCodes {
                        items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                        quantity: quantity.ok_or(::serde::de::Error::missing_field("Quantity"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.ViewerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct ViewerCertificate {
        /// Property [`AcmCertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html#cfn-cloudfront-distribution-viewercertificate-acmcertificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub acm_certificate_arn: Option<crate::Value<String>>,
        /// Property [`CloudFrontDefaultCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html#cfn-cloudfront-distribution-viewercertificate-cloudfrontdefaultcertificate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_front_default_certificate: Option<crate::Value<bool>>,
        /// Property [`IamCertificateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html#cfn-cloudfront-distribution-viewercertificate-iamcertificateid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam_certificate_id: Option<crate::Value<String>>,
        /// Property [`MinimumProtocolVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html#cfn-cloudfront-distribution-viewercertificate-minimumprotocolversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimum_protocol_version: Option<crate::Value<String>>,
        /// Property [`SslSupportMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html#cfn-cloudfront-distribution-viewercertificate-sslsupportmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_support_method: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for ViewerCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref acm_certificate_arn) = self.acm_certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcmCertificateArn", acm_certificate_arn)?;
            }
            if let Some(ref cloud_front_default_certificate) = self.cloud_front_default_certificate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudFrontDefaultCertificate", cloud_front_default_certificate)?;
            }
            if let Some(ref iam_certificate_id) = self.iam_certificate_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamCertificateId", iam_certificate_id)?;
            }
            if let Some(ref minimum_protocol_version) = self.minimum_protocol_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumProtocolVersion", minimum_protocol_version)?;
            }
            if let Some(ref ssl_support_method) = self.ssl_support_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslSupportMethod", ssl_support_method)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ViewerCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ViewerCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ViewerCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ViewerCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut acm_certificate_arn: Option<crate::Value<String>> = None;
                    let mut cloud_front_default_certificate: Option<crate::Value<bool>> = None;
                    let mut iam_certificate_id: Option<crate::Value<String>> = None;
                    let mut minimum_protocol_version: Option<crate::Value<String>> = None;
                    let mut ssl_support_method: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AcmCertificateArn" => {
                                acm_certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudFrontDefaultCertificate" => {
                                cloud_front_default_certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IamCertificateId" => {
                                iam_certificate_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinimumProtocolVersion" => {
                                minimum_protocol_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslSupportMethod" => {
                                ssl_support_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ViewerCertificate {
                        acm_certificate_arn: acm_certificate_arn,
                        cloud_front_default_certificate: cloud_front_default_certificate,
                        iam_certificate_id: iam_certificate_id,
                        minimum_protocol_version: minimum_protocol_version,
                        ssl_support_method: ssl_support_method,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod function {
    //! Property types for the `Function` resource.

    /// The [`AWS::CloudFront::Function.FunctionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FunctionConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionconfig.html#cfn-cloudfront-function-functionconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: crate::Value<String>,
        /// Property [`Runtime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionconfig.html#cfn-cloudfront-function-functionconfig-runtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub runtime: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for FunctionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", &self.comment)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Runtime", &self.runtime)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for FunctionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FunctionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FunctionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<crate::Value<String>> = None;
                    let mut runtime: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Runtime" => {
                                runtime = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FunctionConfig {
                        comment: comment.ok_or(::serde::de::Error::missing_field("Comment"))?,
                        runtime: runtime.ok_or(::serde::de::Error::missing_field("Runtime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Function.FunctionMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionmetadata.html) property type.
    #[derive(Debug, Default)]
    pub struct FunctionMetadata {
        /// Property [`FunctionARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionmetadata.html#cfn-cloudfront-function-functionmetadata-functionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_arn: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for FunctionMetadata {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref function_arn) = self.function_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionARN", function_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for FunctionMetadata {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionMetadata, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FunctionMetadata;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FunctionMetadata")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut function_arn: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionARN" => {
                                function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FunctionMetadata {
                        function_arn: function_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod key_group {
    //! Property types for the `KeyGroup` resource.

    /// The [`AWS::CloudFront::KeyGroup.KeyGroupConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-keygroup-keygroupconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyGroupConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-keygroup-keygroupconfig.html#cfn-cloudfront-keygroup-keygroupconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<crate::Value<String>>,
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-keygroup-keygroupconfig.html#cfn-cloudfront-keygroup-keygroupconfig-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: crate::ValueList<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-keygroup-keygroupconfig.html#cfn-cloudfront-keygroup-keygroupconfig-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for KeyGroupConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for KeyGroupConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyGroupConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyGroupConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyGroupConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<crate::Value<String>> = None;
                    let mut items: Option<crate::ValueList<String>> = None;
                    let mut name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyGroupConfig {
                        comment: comment,
                        items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod origin_request_policy {
    //! Property types for the `OriginRequestPolicy` resource.

    /// The [`AWS::CloudFront::OriginRequestPolicy.CookiesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-cookiesconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CookiesConfig {
        /// Property [`CookieBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-cookiesconfig.html#cfn-cloudfront-originrequestpolicy-cookiesconfig-cookiebehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookie_behavior: crate::Value<String>,
        /// Property [`Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-cookiesconfig.html#cfn-cloudfront-originrequestpolicy-cookiesconfig-cookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for CookiesConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookieBehavior", &self.cookie_behavior)?;
            if let Some(ref cookies) = self.cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cookies", cookies)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for CookiesConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CookiesConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CookiesConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CookiesConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookie_behavior: Option<crate::Value<String>> = None;
                    let mut cookies: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CookieBehavior" => {
                                cookie_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cookies" => {
                                cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CookiesConfig {
                        cookie_behavior: cookie_behavior.ok_or(::serde::de::Error::missing_field("CookieBehavior"))?,
                        cookies: cookies,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::OriginRequestPolicy.HeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-headersconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HeadersConfig {
        /// Property [`HeaderBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-headersconfig.html#cfn-cloudfront-originrequestpolicy-headersconfig-headerbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_behavior: crate::Value<String>,
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-headersconfig.html#cfn-cloudfront-originrequestpolicy-headersconfig-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for HeadersConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderBehavior", &self.header_behavior)?;
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for HeadersConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HeadersConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HeadersConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HeadersConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_behavior: Option<crate::Value<String>> = None;
                    let mut headers: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderBehavior" => {
                                header_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HeadersConfig {
                        header_behavior: header_behavior.ok_or(::serde::de::Error::missing_field("HeaderBehavior"))?,
                        headers: headers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::OriginRequestPolicy.OriginRequestPolicyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginRequestPolicyConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<crate::Value<String>>,
        /// Property [`CookiesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig-cookiesconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies_config: crate::Value<CookiesConfig>,
        /// Property [`HeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig-headersconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers_config: crate::Value<HeadersConfig>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: crate::Value<String>,
        /// Property [`QueryStringsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig-querystringsconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_strings_config: crate::Value<QueryStringsConfig>,
    }

    impl crate::codec::SerializeValue for OriginRequestPolicyConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookiesConfig", &self.cookies_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeadersConfig", &self.headers_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringsConfig", &self.query_strings_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OriginRequestPolicyConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginRequestPolicyConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginRequestPolicyConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginRequestPolicyConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<crate::Value<String>> = None;
                    let mut cookies_config: Option<crate::Value<CookiesConfig>> = None;
                    let mut headers_config: Option<crate::Value<HeadersConfig>> = None;
                    let mut name: Option<crate::Value<String>> = None;
                    let mut query_strings_config: Option<crate::Value<QueryStringsConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CookiesConfig" => {
                                cookies_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeadersConfig" => {
                                headers_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStringsConfig" => {
                                query_strings_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginRequestPolicyConfig {
                        comment: comment,
                        cookies_config: cookies_config.ok_or(::serde::de::Error::missing_field("CookiesConfig"))?,
                        headers_config: headers_config.ok_or(::serde::de::Error::missing_field("HeadersConfig"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        query_strings_config: query_strings_config.ok_or(::serde::de::Error::missing_field("QueryStringsConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::OriginRequestPolicy.QueryStringsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-querystringsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryStringsConfig {
        /// Property [`QueryStringBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-querystringsconfig.html#cfn-cloudfront-originrequestpolicy-querystringsconfig-querystringbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string_behavior: crate::Value<String>,
        /// Property [`QueryStrings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-querystringsconfig.html#cfn-cloudfront-originrequestpolicy-querystringsconfig-querystrings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_strings: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for QueryStringsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringBehavior", &self.query_string_behavior)?;
            if let Some(ref query_strings) = self.query_strings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStrings", query_strings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for QueryStringsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryStringsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryStringsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryStringsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut query_string_behavior: Option<crate::Value<String>> = None;
                    let mut query_strings: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "QueryStringBehavior" => {
                                query_string_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStrings" => {
                                query_strings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryStringsConfig {
                        query_string_behavior: query_string_behavior.ok_or(::serde::de::Error::missing_field("QueryStringBehavior"))?,
                        query_strings: query_strings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod public_key {
    //! Property types for the `PublicKey` resource.

    /// The [`AWS::CloudFront::PublicKey.PublicKeyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-publickey-publickeyconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct PublicKeyConfig {
        /// Property [`CallerReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-publickey-publickeyconfig.html#cfn-cloudfront-publickey-publickeyconfig-callerreference).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caller_reference: crate::Value<String>,
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-publickey-publickeyconfig.html#cfn-cloudfront-publickey-publickeyconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<crate::Value<String>>,
        /// Property [`EncodedKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-publickey-publickeyconfig.html#cfn-cloudfront-publickey-publickeyconfig-encodedkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encoded_key: crate::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-publickey-publickeyconfig.html#cfn-cloudfront-publickey-publickeyconfig-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for PublicKeyConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CallerReference", &self.caller_reference)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncodedKey", &self.encoded_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for PublicKeyConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PublicKeyConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PublicKeyConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PublicKeyConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut caller_reference: Option<crate::Value<String>> = None;
                    let mut comment: Option<crate::Value<String>> = None;
                    let mut encoded_key: Option<crate::Value<String>> = None;
                    let mut name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CallerReference" => {
                                caller_reference = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncodedKey" => {
                                encoded_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PublicKeyConfig {
                        caller_reference: caller_reference.ok_or(::serde::de::Error::missing_field("CallerReference"))?,
                        comment: comment,
                        encoded_key: encoded_key.ok_or(::serde::de::Error::missing_field("EncodedKey"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod realtime_log_config {
    //! Property types for the `RealtimeLogConfig` resource.

    /// The [`AWS::CloudFront::RealtimeLogConfig.EndPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-endpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct EndPoint {
        /// Property [`KinesisStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-endpoint.html#cfn-cloudfront-realtimelogconfig-endpoint-kinesisstreamconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_stream_config: crate::Value<KinesisStreamConfig>,
        /// Property [`StreamType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-endpoint.html#cfn-cloudfront-realtimelogconfig-endpoint-streamtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_type: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for EndPoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamConfig", &self.kinesis_stream_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamType", &self.stream_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for EndPoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndPoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndPoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndPoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kinesis_stream_config: Option<crate::Value<KinesisStreamConfig>> = None;
                    let mut stream_type: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KinesisStreamConfig" => {
                                kinesis_stream_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamType" => {
                                stream_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EndPoint {
                        kinesis_stream_config: kinesis_stream_config.ok_or(::serde::de::Error::missing_field("KinesisStreamConfig"))?,
                        stream_type: stream_type.ok_or(::serde::de::Error::missing_field("StreamType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::RealtimeLogConfig.KinesisStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-kinesisstreamconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisStreamConfig {
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-kinesisstreamconfig.html#cfn-cloudfront-realtimelogconfig-kinesisstreamconfig-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: crate::Value<String>,
        /// Property [`StreamArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-kinesisstreamconfig.html#cfn-cloudfront-realtimelogconfig-kinesisstreamconfig-streamarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_arn: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for KinesisStreamConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamArn", &self.stream_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for KinesisStreamConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisStreamConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisStreamConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisStreamConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role_arn: Option<crate::Value<String>> = None;
                    let mut stream_arn: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamArn" => {
                                stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisStreamConfig {
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        stream_arn: stream_arn.ok_or(::serde::de::Error::missing_field("StreamArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod streaming_distribution {
    //! Property types for the `StreamingDistribution` resource.

    /// The [`AWS::CloudFront::StreamingDistribution.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html) property type.
    #[derive(Debug, Default)]
    pub struct Logging {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html#cfn-cloudfront-streamingdistribution-logging-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: crate::Value<String>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html#cfn-cloudfront-streamingdistribution-logging-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: crate::Value<bool>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html#cfn-cloudfront-streamingdistribution-logging-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for Logging {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", &self.prefix)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Logging {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Logging, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Logging;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Logging")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<crate::Value<String>> = None;
                    let mut enabled: Option<crate::Value<bool>> = None;
                    let mut prefix: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Logging {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        prefix: prefix.ok_or(::serde::de::Error::missing_field("Prefix"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::StreamingDistribution.S3Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-s3origin.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Origin {
        /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-s3origin.html#cfn-cloudfront-streamingdistribution-s3origin-domainname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_name: crate::Value<String>,
        /// Property [`OriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-s3origin.html#cfn-cloudfront-streamingdistribution-s3origin-originaccessidentity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_access_identity: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for S3Origin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginAccessIdentity", &self.origin_access_identity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for S3Origin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Origin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Origin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Origin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut domain_name: Option<crate::Value<String>> = None;
                    let mut origin_access_identity: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DomainName" => {
                                domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginAccessIdentity" => {
                                origin_access_identity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Origin {
                        domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                        origin_access_identity: origin_access_identity.ok_or(::serde::de::Error::missing_field("OriginAccessIdentity"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::StreamingDistribution.StreamingDistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct StreamingDistributionConfig {
        /// Property [`Aliases`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-aliases).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aliases: Option<crate::ValueList<String>>,
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: crate::Value<String>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: crate::Value<bool>,
        /// Property [`Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-logging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging: Option<crate::Value<Logging>>,
        /// Property [`PriceClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-priceclass).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub price_class: Option<crate::Value<String>>,
        /// Property [`S3Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-s3origin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_origin: crate::Value<S3Origin>,
        /// Property [`TrustedSigners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-trustedsigners).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trusted_signers: crate::Value<TrustedSigners>,
    }

    impl crate::codec::SerializeValue for StreamingDistributionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aliases) = self.aliases {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Aliases", aliases)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", &self.comment)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref logging) = self.logging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logging", logging)?;
            }
            if let Some(ref price_class) = self.price_class {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PriceClass", price_class)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Origin", &self.s3_origin)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedSigners", &self.trusted_signers)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for StreamingDistributionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamingDistributionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StreamingDistributionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StreamingDistributionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aliases: Option<crate::ValueList<String>> = None;
                    let mut comment: Option<crate::Value<String>> = None;
                    let mut enabled: Option<crate::Value<bool>> = None;
                    let mut logging: Option<crate::Value<Logging>> = None;
                    let mut price_class: Option<crate::Value<String>> = None;
                    let mut s3_origin: Option<crate::Value<S3Origin>> = None;
                    let mut trusted_signers: Option<crate::Value<TrustedSigners>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Aliases" => {
                                aliases = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Logging" => {
                                logging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PriceClass" => {
                                price_class = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Origin" => {
                                s3_origin = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrustedSigners" => {
                                trusted_signers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StreamingDistributionConfig {
                        aliases: aliases,
                        comment: comment.ok_or(::serde::de::Error::missing_field("Comment"))?,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        logging: logging,
                        price_class: price_class,
                        s3_origin: s3_origin.ok_or(::serde::de::Error::missing_field("S3Origin"))?,
                        trusted_signers: trusted_signers.ok_or(::serde::de::Error::missing_field("TrustedSigners"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::StreamingDistribution.TrustedSigners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-trustedsigners.html) property type.
    #[derive(Debug, Default)]
    pub struct TrustedSigners {
        /// Property [`AwsAccountNumbers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-trustedsigners.html#cfn-cloudfront-streamingdistribution-trustedsigners-awsaccountnumbers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_account_numbers: Option<crate::ValueList<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-trustedsigners.html#cfn-cloudfront-streamingdistribution-trustedsigners-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: crate::Value<bool>,
    }

    impl crate::codec::SerializeValue for TrustedSigners {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_account_numbers) = self.aws_account_numbers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountNumbers", aws_account_numbers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for TrustedSigners {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TrustedSigners, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TrustedSigners;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TrustedSigners")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_account_numbers: Option<crate::ValueList<String>> = None;
                    let mut enabled: Option<crate::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsAccountNumbers" => {
                                aws_account_numbers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TrustedSigners {
                        aws_account_numbers: aws_account_numbers,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
