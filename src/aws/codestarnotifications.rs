//! Types for the `CodeStarNotifications` service.

/// The [`AWS::CodeStarNotifications::NotificationRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html) resource type.
#[derive(Debug, Default)]
pub struct NotificationRule {
    properties: NotificationRuleProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `NotificationRule` resource.
#[derive(Debug, Default)]
pub struct NotificationRuleProperties {
    /// Property [`DetailType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-detailtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub detail_type: crate::Value<String>,
    /// Property [`EventTypeIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-eventtypeids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_type_ids: crate::ValueList<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: crate::Value<String>,
    /// Property [`Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-resource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource: crate::Value<String>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<crate::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<crate::Value<crate::json::Value>>,
    /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html#cfn-codestarnotifications-notificationrule-targets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub targets: crate::ValueList<self::notification_rule::Target>,
}

impl ::serde::Serialize for NotificationRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetailType", &self.detail_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventTypeIds", &self.event_type_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resource", &self.resource)?;
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", &self.targets)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NotificationRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NotificationRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NotificationRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut detail_type: Option<crate::Value<String>> = None;
                let mut event_type_ids: Option<crate::ValueList<String>> = None;
                let mut name: Option<crate::Value<String>> = None;
                let mut resource: Option<crate::Value<String>> = None;
                let mut status: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::Value<crate::json::Value>> = None;
                let mut targets: Option<crate::ValueList<self::notification_rule::Target>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DetailType" => {
                            detail_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventTypeIds" => {
                            event_type_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Resource" => {
                            resource = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Targets" => {
                            targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NotificationRuleProperties {
                    detail_type: detail_type.ok_or(::serde::de::Error::missing_field("DetailType"))?,
                    event_type_ids: event_type_ids.ok_or(::serde::de::Error::missing_field("EventTypeIds"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    resource: resource.ok_or(::serde::de::Error::missing_field("Resource"))?,
                    status: status,
                    tags: tags,
                    targets: targets.ok_or(::serde::de::Error::missing_field("Targets"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for NotificationRule {
    type Properties = NotificationRuleProperties;
    const TYPE: &'static str = "AWS::CodeStarNotifications::NotificationRule";
    fn properties(&self) -> &NotificationRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NotificationRuleProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for NotificationRule {}

impl From<NotificationRuleProperties> for NotificationRule {
    fn from(properties: NotificationRuleProperties) -> NotificationRule {
        NotificationRule { properties, depends_on: None }
    }
}

pub mod notification_rule {
    //! Property types for the `NotificationRule` resource.

    /// The [`AWS::CodeStarNotifications::NotificationRule.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestarnotifications-notificationrule-target.html) property type.
    #[derive(Debug, Default)]
    pub struct Target {
        /// Property [`TargetAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestarnotifications-notificationrule-target.html#cfn-codestarnotifications-notificationrule-target-targetaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_address: Option<crate::Value<String>>,
        /// Property [`TargetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestarnotifications-notificationrule-target.html#cfn-codestarnotifications-notificationrule-target-targettype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_type: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for Target {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref target_address) = self.target_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetAddress", target_address)?;
            }
            if let Some(ref target_type) = self.target_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetType", target_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Target {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Target, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Target;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Target")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_address: Option<crate::Value<String>> = None;
                    let mut target_type: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetAddress" => {
                                target_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetType" => {
                                target_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Target {
                        target_address: target_address,
                        target_type: target_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
