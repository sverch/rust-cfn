//! Types for the `CloudTrail` service.

/// The [`AWS::CloudTrail::Trail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html) resource type.
#[derive(Debug, Default)]
pub struct Trail {
    properties: TrailProperties
}

/// Properties for the `Trail` resource.
#[derive(Debug, Default)]
pub struct TrailProperties {
    /// Property [`CloudWatchLogsLogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-cloudwatchlogsloggrouparn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cloud_watch_logs_log_group_arn: Option<crate::Value<String>>,
    /// Property [`CloudWatchLogsRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-cloudwatchlogsrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cloud_watch_logs_role_arn: Option<crate::Value<String>>,
    /// Property [`EnableLogFileValidation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-enablelogfilevalidation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_log_file_validation: Option<crate::Value<bool>>,
    /// Property [`EventSelectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-eventselectors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_selectors: Option<crate::ValueList<self::trail::EventSelector>>,
    /// Property [`IncludeGlobalServiceEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-includeglobalserviceevents).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub include_global_service_events: Option<crate::Value<bool>>,
    /// Property [`IsLogging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-islogging).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_logging: crate::Value<bool>,
    /// Property [`IsMultiRegionTrail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-ismultiregiontrail).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_multi_region_trail: Option<crate::Value<bool>>,
    /// Property [`KMSKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-kmskeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_id: Option<crate::Value<String>>,
    /// Property [`S3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-s3bucketname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_bucket_name: crate::Value<String>,
    /// Property [`S3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-s3keyprefix).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_key_prefix: Option<crate::Value<String>>,
    /// Property [`SnsTopicName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-snstopicname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sns_topic_name: Option<crate::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
    /// Property [`TrailName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html#cfn-cloudtrail-trail-trailname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub trail_name: Option<crate::Value<String>>,
}

impl ::serde::Serialize for TrailProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cloud_watch_logs_log_group_arn) = self.cloud_watch_logs_log_group_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsLogGroupArn", cloud_watch_logs_log_group_arn)?;
        }
        if let Some(ref cloud_watch_logs_role_arn) = self.cloud_watch_logs_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsRoleArn", cloud_watch_logs_role_arn)?;
        }
        if let Some(ref enable_log_file_validation) = self.enable_log_file_validation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableLogFileValidation", enable_log_file_validation)?;
        }
        if let Some(ref event_selectors) = self.event_selectors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSelectors", event_selectors)?;
        }
        if let Some(ref include_global_service_events) = self.include_global_service_events {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeGlobalServiceEvents", include_global_service_events)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsLogging", &self.is_logging)?;
        if let Some(ref is_multi_region_trail) = self.is_multi_region_trail {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsMultiRegionTrail", is_multi_region_trail)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSKeyId", kms_key_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", &self.s3_bucket_name)?;
        if let Some(ref s3_key_prefix) = self.s3_key_prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyPrefix", s3_key_prefix)?;
        }
        if let Some(ref sns_topic_name) = self.sns_topic_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicName", sns_topic_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref trail_name) = self.trail_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrailName", trail_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TrailProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TrailProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TrailProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TrailProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cloud_watch_logs_log_group_arn: Option<crate::Value<String>> = None;
                let mut cloud_watch_logs_role_arn: Option<crate::Value<String>> = None;
                let mut enable_log_file_validation: Option<crate::Value<bool>> = None;
                let mut event_selectors: Option<crate::ValueList<self::trail::EventSelector>> = None;
                let mut include_global_service_events: Option<crate::Value<bool>> = None;
                let mut is_logging: Option<crate::Value<bool>> = None;
                let mut is_multi_region_trail: Option<crate::Value<bool>> = None;
                let mut kms_key_id: Option<crate::Value<String>> = None;
                let mut s3_bucket_name: Option<crate::Value<String>> = None;
                let mut s3_key_prefix: Option<crate::Value<String>> = None;
                let mut sns_topic_name: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;
                let mut trail_name: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CloudWatchLogsLogGroupArn" => {
                            cloud_watch_logs_log_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CloudWatchLogsRoleArn" => {
                            cloud_watch_logs_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableLogFileValidation" => {
                            enable_log_file_validation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventSelectors" => {
                            event_selectors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IncludeGlobalServiceEvents" => {
                            include_global_service_events = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsLogging" => {
                            is_logging = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsMultiRegionTrail" => {
                            is_multi_region_trail = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KMSKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3BucketName" => {
                            s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3KeyPrefix" => {
                            s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnsTopicName" => {
                            sns_topic_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrailName" => {
                            trail_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TrailProperties {
                    cloud_watch_logs_log_group_arn: cloud_watch_logs_log_group_arn,
                    cloud_watch_logs_role_arn: cloud_watch_logs_role_arn,
                    enable_log_file_validation: enable_log_file_validation,
                    event_selectors: event_selectors,
                    include_global_service_events: include_global_service_events,
                    is_logging: is_logging.ok_or(::serde::de::Error::missing_field("IsLogging"))?,
                    is_multi_region_trail: is_multi_region_trail,
                    kms_key_id: kms_key_id,
                    s3_bucket_name: s3_bucket_name.ok_or(::serde::de::Error::missing_field("S3BucketName"))?,
                    s3_key_prefix: s3_key_prefix,
                    sns_topic_name: sns_topic_name,
                    tags: tags,
                    trail_name: trail_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Trail {
    type Properties = TrailProperties;
    const TYPE: &'static str = "AWS::CloudTrail::Trail";
    fn properties(&self) -> &TrailProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TrailProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for Trail {}

impl From<TrailProperties> for Trail {
    fn from(properties: TrailProperties) -> Trail {
        Trail { properties }
    }
}

pub mod trail {
    //! Property types for the `Trail` resource.

    /// The [`AWS::CloudTrail::Trail.DataResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-dataresource.html) property type.
    #[derive(Debug, Default)]
    pub struct DataResource {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-dataresource.html#cfn-cloudtrail-trail-dataresource-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: crate::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-dataresource.html#cfn-cloudtrail-trail-dataresource-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for DataResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DataResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<crate::Value<String>> = None;
                    let mut values: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataResource {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudTrail::Trail.EventSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html) property type.
    #[derive(Debug, Default)]
    pub struct EventSelector {
        /// Property [`DataResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html#cfn-cloudtrail-trail-eventselector-dataresources).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_resources: Option<crate::ValueList<DataResource>>,
        /// Property [`IncludeManagementEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html#cfn-cloudtrail-trail-eventselector-includemanagementevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_management_events: Option<crate::Value<bool>>,
        /// Property [`ReadWriteType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html#cfn-cloudtrail-trail-eventselector-readwritetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_write_type: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for EventSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_resources) = self.data_resources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataResources", data_resources)?;
            }
            if let Some(ref include_management_events) = self.include_management_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeManagementEvents", include_management_events)?;
            }
            if let Some(ref read_write_type) = self.read_write_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadWriteType", read_write_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for EventSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_resources: Option<crate::ValueList<DataResource>> = None;
                    let mut include_management_events: Option<crate::Value<bool>> = None;
                    let mut read_write_type: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataResources" => {
                                data_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeManagementEvents" => {
                                include_management_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadWriteType" => {
                                read_write_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventSelector {
                        data_resources: data_resources,
                        include_management_events: include_management_events,
                        read_write_type: read_write_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
