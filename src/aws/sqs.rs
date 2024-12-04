//! Types for the `SQS` service.

/// The [`AWS::SQS::Queue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html) resource type.
#[derive(Debug, Default)]
pub struct Queue {
    properties: QueueProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `Queue` resource.
#[derive(Debug, Default)]
pub struct QueueProperties {
    /// Property [`ContentBasedDeduplication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-contentbaseddeduplication).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content_based_deduplication: Option<crate::Value<bool>>,
    /// Property [`DeduplicationScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-deduplicationscope).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deduplication_scope: Option<crate::Value<String>>,
    /// Property [`DelaySeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-delayseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delay_seconds: Option<crate::Value<u32>>,
    /// Property [`FifoQueue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-fifoqueue).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub fifo_queue: Option<crate::Value<bool>>,
    /// Property [`FifoThroughputLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-fifothroughputlimit).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fifo_throughput_limit: Option<crate::Value<String>>,
    /// Property [`KmsDataKeyReusePeriodSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-kmsdatakeyreuseperiodseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_data_key_reuse_period_seconds: Option<crate::Value<u32>>,
    /// Property [`KmsMasterKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-kmsmasterkeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_master_key_id: Option<crate::Value<String>>,
    /// Property [`MaximumMessageSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-maxmesgsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_message_size: Option<crate::Value<u32>>,
    /// Property [`MessageRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-msgretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub message_retention_period: Option<crate::Value<u32>>,
    /// Property [`QueueName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub queue_name: Option<crate::Value<String>>,
    /// Property [`ReceiveMessageWaitTimeSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-receivemsgwaittime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub receive_message_wait_time_seconds: Option<crate::Value<u32>>,
    /// Property [`RedrivePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-redrive).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub redrive_policy: Option<crate::Value<crate::json::Value>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#cfn-sqs-queue-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
    /// Property [`VisibilityTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-queues.html#aws-sqs-queue-visiblitytimeout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub visibility_timeout: Option<crate::Value<u32>>,
}

impl ::serde::Serialize for QueueProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref content_based_deduplication) = self.content_based_deduplication {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentBasedDeduplication", content_based_deduplication)?;
        }
        if let Some(ref deduplication_scope) = self.deduplication_scope {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeduplicationScope", deduplication_scope)?;
        }
        if let Some(ref delay_seconds) = self.delay_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DelaySeconds", delay_seconds)?;
        }
        if let Some(ref fifo_queue) = self.fifo_queue {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FifoQueue", fifo_queue)?;
        }
        if let Some(ref fifo_throughput_limit) = self.fifo_throughput_limit {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FifoThroughputLimit", fifo_throughput_limit)?;
        }
        if let Some(ref kms_data_key_reuse_period_seconds) = self.kms_data_key_reuse_period_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsDataKeyReusePeriodSeconds", kms_data_key_reuse_period_seconds)?;
        }
        if let Some(ref kms_master_key_id) = self.kms_master_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsMasterKeyId", kms_master_key_id)?;
        }
        if let Some(ref maximum_message_size) = self.maximum_message_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumMessageSize", maximum_message_size)?;
        }
        if let Some(ref message_retention_period) = self.message_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageRetentionPeriod", message_retention_period)?;
        }
        if let Some(ref queue_name) = self.queue_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueName", queue_name)?;
        }
        if let Some(ref receive_message_wait_time_seconds) = self.receive_message_wait_time_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReceiveMessageWaitTimeSeconds", receive_message_wait_time_seconds)?;
        }
        if let Some(ref redrive_policy) = self.redrive_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedrivePolicy", redrive_policy)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref visibility_timeout) = self.visibility_timeout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VisibilityTimeout", visibility_timeout)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for QueueProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<QueueProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = QueueProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type QueueProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut content_based_deduplication: Option<crate::Value<bool>> = None;
                let mut deduplication_scope: Option<crate::Value<String>> = None;
                let mut delay_seconds: Option<crate::Value<u32>> = None;
                let mut fifo_queue: Option<crate::Value<bool>> = None;
                let mut fifo_throughput_limit: Option<crate::Value<String>> = None;
                let mut kms_data_key_reuse_period_seconds: Option<crate::Value<u32>> = None;
                let mut kms_master_key_id: Option<crate::Value<String>> = None;
                let mut maximum_message_size: Option<crate::Value<u32>> = None;
                let mut message_retention_period: Option<crate::Value<u32>> = None;
                let mut queue_name: Option<crate::Value<String>> = None;
                let mut receive_message_wait_time_seconds: Option<crate::Value<u32>> = None;
                let mut redrive_policy: Option<crate::Value<crate::json::Value>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;
                let mut visibility_timeout: Option<crate::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContentBasedDeduplication" => {
                            content_based_deduplication = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeduplicationScope" => {
                            deduplication_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DelaySeconds" => {
                            delay_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FifoQueue" => {
                            fifo_queue = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FifoThroughputLimit" => {
                            fifo_throughput_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsDataKeyReusePeriodSeconds" => {
                            kms_data_key_reuse_period_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsMasterKeyId" => {
                            kms_master_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumMessageSize" => {
                            maximum_message_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MessageRetentionPeriod" => {
                            message_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueueName" => {
                            queue_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReceiveMessageWaitTimeSeconds" => {
                            receive_message_wait_time_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RedrivePolicy" => {
                            redrive_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VisibilityTimeout" => {
                            visibility_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(QueueProperties {
                    content_based_deduplication: content_based_deduplication,
                    deduplication_scope: deduplication_scope,
                    delay_seconds: delay_seconds,
                    fifo_queue: fifo_queue,
                    fifo_throughput_limit: fifo_throughput_limit,
                    kms_data_key_reuse_period_seconds: kms_data_key_reuse_period_seconds,
                    kms_master_key_id: kms_master_key_id,
                    maximum_message_size: maximum_message_size,
                    message_retention_period: message_retention_period,
                    queue_name: queue_name,
                    receive_message_wait_time_seconds: receive_message_wait_time_seconds,
                    redrive_policy: redrive_policy,
                    tags: tags,
                    visibility_timeout: visibility_timeout,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Queue {
    type Properties = QueueProperties;
    const TYPE: &'static str = "AWS::SQS::Queue";
    fn properties(&self) -> &QueueProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut QueueProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for Queue {}

impl From<QueueProperties> for Queue {
    fn from(properties: QueueProperties) -> Queue {
        Queue { properties, depends_on: None }
    }
}

/// The [`AWS::SQS::QueuePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html) resource type.
#[derive(Debug, Default)]
pub struct QueuePolicy {
    properties: QueuePolicyProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `QueuePolicy` resource.
#[derive(Debug, Default)]
pub struct QueuePolicyProperties {
    /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html#cfn-sqs-queuepolicy-policydoc).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_document: crate::Value<crate::json::Value>,
    /// Property [`Queues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html#cfn-sqs-queuepolicy-queues).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub queues: crate::ValueList<String>,
}

impl ::serde::Serialize for QueuePolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Queues", &self.queues)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for QueuePolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<QueuePolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = QueuePolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type QueuePolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut policy_document: Option<crate::Value<crate::json::Value>> = None;
                let mut queues: Option<crate::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PolicyDocument" => {
                            policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Queues" => {
                            queues = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(QueuePolicyProperties {
                    policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                    queues: queues.ok_or(::serde::de::Error::missing_field("Queues"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for QueuePolicy {
    type Properties = QueuePolicyProperties;
    const TYPE: &'static str = "AWS::SQS::QueuePolicy";
    fn properties(&self) -> &QueuePolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut QueuePolicyProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for QueuePolicy {}

impl From<QueuePolicyProperties> for QueuePolicy {
    fn from(properties: QueuePolicyProperties) -> QueuePolicy {
        QueuePolicy { properties, depends_on: None }
    }
}
