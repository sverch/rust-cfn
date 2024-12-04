//! Types for the `KMS` service.

/// The [`AWS::KMS::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-alias.html) resource type.
#[derive(Debug, Default)]
pub struct Alias {
    properties: AliasProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `Alias` resource.
#[derive(Debug, Default)]
pub struct AliasProperties {
    /// Property [`AliasName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-alias.html#cfn-kms-alias-aliasname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub alias_name: crate::Value<String>,
    /// Property [`TargetKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-alias.html#cfn-kms-alias-targetkeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_key_id: crate::Value<String>,
}

impl ::serde::Serialize for AliasProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AliasName", &self.alias_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetKeyId", &self.target_key_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AliasProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AliasProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AliasProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AliasProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut alias_name: Option<crate::Value<String>> = None;
                let mut target_key_id: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AliasName" => {
                            alias_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetKeyId" => {
                            target_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AliasProperties {
                    alias_name: alias_name.ok_or(::serde::de::Error::missing_field("AliasName"))?,
                    target_key_id: target_key_id.ok_or(::serde::de::Error::missing_field("TargetKeyId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Alias {
    type Properties = AliasProperties;
    const TYPE: &'static str = "AWS::KMS::Alias";
    fn properties(&self) -> &AliasProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AliasProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for Alias {}

impl From<AliasProperties> for Alias {
    fn from(properties: AliasProperties) -> Alias {
        Alias { properties, depends_on: None }
    }
}

/// The [`AWS::KMS::Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html) resource type.
#[derive(Debug, Default)]
pub struct Key {
    properties: KeyProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `Key` resource.
#[derive(Debug, Default)]
pub struct KeyProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html#cfn-kms-key-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<crate::Value<String>>,
    /// Property [`EnableKeyRotation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html#cfn-kms-key-enablekeyrotation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_key_rotation: Option<crate::Value<bool>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html#cfn-kms-key-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<crate::Value<bool>>,
    /// Property [`KeyPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html#cfn-kms-key-keypolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub key_policy: crate::Value<crate::json::Value>,
    /// Property [`KeySpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html#cfn-kms-key-keyspec).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub key_spec: Option<crate::Value<String>>,
    /// Property [`KeyUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html#cfn-kms-key-keyusage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub key_usage: Option<crate::Value<String>>,
    /// Property [`MultiRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html#cfn-kms-key-multiregion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub multi_region: Option<crate::Value<bool>>,
    /// Property [`PendingWindowInDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html#cfn-kms-key-pendingwindowindays).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pending_window_in_days: Option<crate::Value<u32>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html#cfn-kms-key-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for KeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref enable_key_rotation) = self.enable_key_rotation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableKeyRotation", enable_key_rotation)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPolicy", &self.key_policy)?;
        if let Some(ref key_spec) = self.key_spec {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySpec", key_spec)?;
        }
        if let Some(ref key_usage) = self.key_usage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyUsage", key_usage)?;
        }
        if let Some(ref multi_region) = self.multi_region {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiRegion", multi_region)?;
        }
        if let Some(ref pending_window_in_days) = self.pending_window_in_days {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PendingWindowInDays", pending_window_in_days)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for KeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = KeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type KeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<crate::Value<String>> = None;
                let mut enable_key_rotation: Option<crate::Value<bool>> = None;
                let mut enabled: Option<crate::Value<bool>> = None;
                let mut key_policy: Option<crate::Value<crate::json::Value>> = None;
                let mut key_spec: Option<crate::Value<String>> = None;
                let mut key_usage: Option<crate::Value<String>> = None;
                let mut multi_region: Option<crate::Value<bool>> = None;
                let mut pending_window_in_days: Option<crate::Value<u32>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableKeyRotation" => {
                            enable_key_rotation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyPolicy" => {
                            key_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeySpec" => {
                            key_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyUsage" => {
                            key_usage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MultiRegion" => {
                            multi_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PendingWindowInDays" => {
                            pending_window_in_days = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(KeyProperties {
                    description: description,
                    enable_key_rotation: enable_key_rotation,
                    enabled: enabled,
                    key_policy: key_policy.ok_or(::serde::de::Error::missing_field("KeyPolicy"))?,
                    key_spec: key_spec,
                    key_usage: key_usage,
                    multi_region: multi_region,
                    pending_window_in_days: pending_window_in_days,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Key {
    type Properties = KeyProperties;
    const TYPE: &'static str = "AWS::KMS::Key";
    fn properties(&self) -> &KeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut KeyProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for Key {}

impl From<KeyProperties> for Key {
    fn from(properties: KeyProperties) -> Key {
        Key { properties, depends_on: None }
    }
}

/// The [`AWS::KMS::ReplicaKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-replicakey.html) resource type.
#[derive(Debug, Default)]
pub struct ReplicaKey {
    properties: ReplicaKeyProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `ReplicaKey` resource.
#[derive(Debug, Default)]
pub struct ReplicaKeyProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-replicakey.html#cfn-kms-replicakey-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<crate::Value<String>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-replicakey.html#cfn-kms-replicakey-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<crate::Value<bool>>,
    /// Property [`KeyPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-replicakey.html#cfn-kms-replicakey-keypolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub key_policy: crate::Value<crate::json::Value>,
    /// Property [`PendingWindowInDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-replicakey.html#cfn-kms-replicakey-pendingwindowindays).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pending_window_in_days: Option<crate::Value<u32>>,
    /// Property [`PrimaryKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-replicakey.html#cfn-kms-replicakey-primarykeyarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub primary_key_arn: crate::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-replicakey.html#cfn-kms-replicakey-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for ReplicaKeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPolicy", &self.key_policy)?;
        if let Some(ref pending_window_in_days) = self.pending_window_in_days {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PendingWindowInDays", pending_window_in_days)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryKeyArn", &self.primary_key_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicaKeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicaKeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicaKeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicaKeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<crate::Value<String>> = None;
                let mut enabled: Option<crate::Value<bool>> = None;
                let mut key_policy: Option<crate::Value<crate::json::Value>> = None;
                let mut pending_window_in_days: Option<crate::Value<u32>> = None;
                let mut primary_key_arn: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyPolicy" => {
                            key_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PendingWindowInDays" => {
                            pending_window_in_days = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrimaryKeyArn" => {
                            primary_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReplicaKeyProperties {
                    description: description,
                    enabled: enabled,
                    key_policy: key_policy.ok_or(::serde::de::Error::missing_field("KeyPolicy"))?,
                    pending_window_in_days: pending_window_in_days,
                    primary_key_arn: primary_key_arn.ok_or(::serde::de::Error::missing_field("PrimaryKeyArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for ReplicaKey {
    type Properties = ReplicaKeyProperties;
    const TYPE: &'static str = "AWS::KMS::ReplicaKey";
    fn properties(&self) -> &ReplicaKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicaKeyProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for ReplicaKey {}

impl From<ReplicaKeyProperties> for ReplicaKey {
    fn from(properties: ReplicaKeyProperties) -> ReplicaKey {
        ReplicaKey { properties, depends_on: None }
    }
}
