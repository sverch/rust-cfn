//! Types for the `Lambda` service.

/// The [`AWS::Lambda::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html) resource type.
#[derive(Debug, Default)]
pub struct Alias {
    properties: AliasProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `Alias` resource.
#[derive(Debug, Default)]
pub struct AliasProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<crate::Value<String>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: crate::Value<String>,
    /// Property [`FunctionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-functionversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_version: crate::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: crate::Value<String>,
    /// Property [`ProvisionedConcurrencyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-provisionedconcurrencyconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioned_concurrency_config: Option<crate::Value<self::alias::ProvisionedConcurrencyConfiguration>>,
    /// Property [`RoutingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-routingconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub routing_config: Option<crate::Value<self::alias::AliasRoutingConfiguration>>,
}

impl ::serde::Serialize for AliasProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionVersion", &self.function_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref provisioned_concurrency_config) = self.provisioned_concurrency_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedConcurrencyConfig", provisioned_concurrency_config)?;
        }
        if let Some(ref routing_config) = self.routing_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoutingConfig", routing_config)?;
        }
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
                let mut description: Option<crate::Value<String>> = None;
                let mut function_name: Option<crate::Value<String>> = None;
                let mut function_version: Option<crate::Value<String>> = None;
                let mut name: Option<crate::Value<String>> = None;
                let mut provisioned_concurrency_config: Option<crate::Value<self::alias::ProvisionedConcurrencyConfiguration>> = None;
                let mut routing_config: Option<crate::Value<self::alias::AliasRoutingConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionVersion" => {
                            function_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisionedConcurrencyConfig" => {
                            provisioned_concurrency_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoutingConfig" => {
                            routing_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AliasProperties {
                    description: description,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    function_version: function_version.ok_or(::serde::de::Error::missing_field("FunctionVersion"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    provisioned_concurrency_config: provisioned_concurrency_config,
                    routing_config: routing_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Alias {
    type Properties = AliasProperties;
    const TYPE: &'static str = "AWS::Lambda::Alias";
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

/// The [`AWS::Lambda::CodeSigningConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-codesigningconfig.html) resource type.
#[derive(Debug, Default)]
pub struct CodeSigningConfig {
    properties: CodeSigningConfigProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `CodeSigningConfig` resource.
#[derive(Debug, Default)]
pub struct CodeSigningConfigProperties {
    /// Property [`AllowedPublishers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-codesigningconfig.html#cfn-lambda-codesigningconfig-allowedpublishers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allowed_publishers: crate::Value<self::code_signing_config::AllowedPublishers>,
    /// Property [`CodeSigningPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-codesigningconfig.html#cfn-lambda-codesigningconfig-codesigningpolicies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub code_signing_policies: Option<crate::Value<self::code_signing_config::CodeSigningPolicies>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-codesigningconfig.html#cfn-lambda-codesigningconfig-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<crate::Value<String>>,
}

impl ::serde::Serialize for CodeSigningConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedPublishers", &self.allowed_publishers)?;
        if let Some(ref code_signing_policies) = self.code_signing_policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeSigningPolicies", code_signing_policies)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CodeSigningConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CodeSigningConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CodeSigningConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CodeSigningConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allowed_publishers: Option<crate::Value<self::code_signing_config::AllowedPublishers>> = None;
                let mut code_signing_policies: Option<crate::Value<self::code_signing_config::CodeSigningPolicies>> = None;
                let mut description: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowedPublishers" => {
                            allowed_publishers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CodeSigningPolicies" => {
                            code_signing_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CodeSigningConfigProperties {
                    allowed_publishers: allowed_publishers.ok_or(::serde::de::Error::missing_field("AllowedPublishers"))?,
                    code_signing_policies: code_signing_policies,
                    description: description,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for CodeSigningConfig {
    type Properties = CodeSigningConfigProperties;
    const TYPE: &'static str = "AWS::Lambda::CodeSigningConfig";
    fn properties(&self) -> &CodeSigningConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CodeSigningConfigProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for CodeSigningConfig {}

impl From<CodeSigningConfigProperties> for CodeSigningConfig {
    fn from(properties: CodeSigningConfigProperties) -> CodeSigningConfig {
        CodeSigningConfig { properties, depends_on: None }
    }
}

/// The [`AWS::Lambda::EventInvokeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html) resource type.
#[derive(Debug, Default)]
pub struct EventInvokeConfig {
    properties: EventInvokeConfigProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `EventInvokeConfig` resource.
#[derive(Debug, Default)]
pub struct EventInvokeConfigProperties {
    /// Property [`DestinationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html#cfn-lambda-eventinvokeconfig-destinationconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destination_config: Option<crate::Value<self::event_invoke_config::DestinationConfig>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html#cfn-lambda-eventinvokeconfig-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: crate::Value<String>,
    /// Property [`MaximumEventAgeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html#cfn-lambda-eventinvokeconfig-maximumeventageinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_event_age_in_seconds: Option<crate::Value<u32>>,
    /// Property [`MaximumRetryAttempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html#cfn-lambda-eventinvokeconfig-maximumretryattempts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_retry_attempts: Option<crate::Value<u32>>,
    /// Property [`Qualifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html#cfn-lambda-eventinvokeconfig-qualifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub qualifier: crate::Value<String>,
}

impl ::serde::Serialize for EventInvokeConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref destination_config) = self.destination_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationConfig", destination_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        if let Some(ref maximum_event_age_in_seconds) = self.maximum_event_age_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumEventAgeInSeconds", maximum_event_age_in_seconds)?;
        }
        if let Some(ref maximum_retry_attempts) = self.maximum_retry_attempts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumRetryAttempts", maximum_retry_attempts)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Qualifier", &self.qualifier)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventInvokeConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventInvokeConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventInvokeConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventInvokeConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut destination_config: Option<crate::Value<self::event_invoke_config::DestinationConfig>> = None;
                let mut function_name: Option<crate::Value<String>> = None;
                let mut maximum_event_age_in_seconds: Option<crate::Value<u32>> = None;
                let mut maximum_retry_attempts: Option<crate::Value<u32>> = None;
                let mut qualifier: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DestinationConfig" => {
                            destination_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumEventAgeInSeconds" => {
                            maximum_event_age_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumRetryAttempts" => {
                            maximum_retry_attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Qualifier" => {
                            qualifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventInvokeConfigProperties {
                    destination_config: destination_config,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    maximum_event_age_in_seconds: maximum_event_age_in_seconds,
                    maximum_retry_attempts: maximum_retry_attempts,
                    qualifier: qualifier.ok_or(::serde::de::Error::missing_field("Qualifier"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for EventInvokeConfig {
    type Properties = EventInvokeConfigProperties;
    const TYPE: &'static str = "AWS::Lambda::EventInvokeConfig";
    fn properties(&self) -> &EventInvokeConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventInvokeConfigProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for EventInvokeConfig {}

impl From<EventInvokeConfigProperties> for EventInvokeConfig {
    fn from(properties: EventInvokeConfigProperties) -> EventInvokeConfig {
        EventInvokeConfig { properties, depends_on: None }
    }
}

/// The [`AWS::Lambda::EventSourceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html) resource type.
#[derive(Debug, Default)]
pub struct EventSourceMapping {
    properties: EventSourceMappingProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `EventSourceMapping` resource.
#[derive(Debug, Default)]
pub struct EventSourceMappingProperties {
    /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-batchsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub batch_size: Option<crate::Value<u32>>,
    /// Property [`BisectBatchOnFunctionError`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-bisectbatchonfunctionerror).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bisect_batch_on_function_error: Option<crate::Value<bool>>,
    /// Property [`DestinationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-destinationconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destination_config: Option<crate::Value<self::event_source_mapping::DestinationConfig>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<crate::Value<bool>>,
    /// Property [`EventSourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-eventsourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_source_arn: Option<crate::Value<String>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-functionname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_name: crate::Value<String>,
    /// Property [`FunctionResponseTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-functionresponsetypes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_response_types: Option<crate::ValueList<String>>,
    /// Property [`MaximumBatchingWindowInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-maximumbatchingwindowinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_batching_window_in_seconds: Option<crate::Value<u32>>,
    /// Property [`MaximumRecordAgeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-maximumrecordageinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_record_age_in_seconds: Option<crate::Value<u32>>,
    /// Property [`MaximumRetryAttempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-maximumretryattempts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_retry_attempts: Option<crate::Value<u32>>,
    /// Property [`ParallelizationFactor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-parallelizationfactor).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parallelization_factor: Option<crate::Value<u32>>,
    /// Property [`Queues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-queues).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub queues: Option<crate::ValueList<String>>,
    /// Property [`SelfManagedEventSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-selfmanagedeventsource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub self_managed_event_source: Option<crate::Value<self::event_source_mapping::SelfManagedEventSource>>,
    /// Property [`SourceAccessConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-sourceaccessconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_access_configurations: Option<crate::ValueList<self::event_source_mapping::SourceAccessConfiguration>>,
    /// Property [`StartingPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-startingposition).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub starting_position: Option<crate::Value<String>>,
    /// Property [`StartingPositionTimestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-startingpositiontimestamp).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub starting_position_timestamp: Option<crate::Value<f64>>,
    /// Property [`Topics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-topics).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub topics: Option<crate::ValueList<String>>,
    /// Property [`TumblingWindowInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-tumblingwindowinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tumbling_window_in_seconds: Option<crate::Value<u32>>,
}

impl ::serde::Serialize for EventSourceMappingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref batch_size) = self.batch_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", batch_size)?;
        }
        if let Some(ref bisect_batch_on_function_error) = self.bisect_batch_on_function_error {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BisectBatchOnFunctionError", bisect_batch_on_function_error)?;
        }
        if let Some(ref destination_config) = self.destination_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationConfig", destination_config)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref event_source_arn) = self.event_source_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSourceArn", event_source_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        if let Some(ref function_response_types) = self.function_response_types {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionResponseTypes", function_response_types)?;
        }
        if let Some(ref maximum_batching_window_in_seconds) = self.maximum_batching_window_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumBatchingWindowInSeconds", maximum_batching_window_in_seconds)?;
        }
        if let Some(ref maximum_record_age_in_seconds) = self.maximum_record_age_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumRecordAgeInSeconds", maximum_record_age_in_seconds)?;
        }
        if let Some(ref maximum_retry_attempts) = self.maximum_retry_attempts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumRetryAttempts", maximum_retry_attempts)?;
        }
        if let Some(ref parallelization_factor) = self.parallelization_factor {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParallelizationFactor", parallelization_factor)?;
        }
        if let Some(ref queues) = self.queues {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Queues", queues)?;
        }
        if let Some(ref self_managed_event_source) = self.self_managed_event_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelfManagedEventSource", self_managed_event_source)?;
        }
        if let Some(ref source_access_configurations) = self.source_access_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceAccessConfigurations", source_access_configurations)?;
        }
        if let Some(ref starting_position) = self.starting_position {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartingPosition", starting_position)?;
        }
        if let Some(ref starting_position_timestamp) = self.starting_position_timestamp {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartingPositionTimestamp", starting_position_timestamp)?;
        }
        if let Some(ref topics) = self.topics {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Topics", topics)?;
        }
        if let Some(ref tumbling_window_in_seconds) = self.tumbling_window_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TumblingWindowInSeconds", tumbling_window_in_seconds)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventSourceMappingProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSourceMappingProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventSourceMappingProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventSourceMappingProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut batch_size: Option<crate::Value<u32>> = None;
                let mut bisect_batch_on_function_error: Option<crate::Value<bool>> = None;
                let mut destination_config: Option<crate::Value<self::event_source_mapping::DestinationConfig>> = None;
                let mut enabled: Option<crate::Value<bool>> = None;
                let mut event_source_arn: Option<crate::Value<String>> = None;
                let mut function_name: Option<crate::Value<String>> = None;
                let mut function_response_types: Option<crate::ValueList<String>> = None;
                let mut maximum_batching_window_in_seconds: Option<crate::Value<u32>> = None;
                let mut maximum_record_age_in_seconds: Option<crate::Value<u32>> = None;
                let mut maximum_retry_attempts: Option<crate::Value<u32>> = None;
                let mut parallelization_factor: Option<crate::Value<u32>> = None;
                let mut queues: Option<crate::ValueList<String>> = None;
                let mut self_managed_event_source: Option<crate::Value<self::event_source_mapping::SelfManagedEventSource>> = None;
                let mut source_access_configurations: Option<crate::ValueList<self::event_source_mapping::SourceAccessConfiguration>> = None;
                let mut starting_position: Option<crate::Value<String>> = None;
                let mut starting_position_timestamp: Option<crate::Value<f64>> = None;
                let mut topics: Option<crate::ValueList<String>> = None;
                let mut tumbling_window_in_seconds: Option<crate::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BatchSize" => {
                            batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BisectBatchOnFunctionError" => {
                            bisect_batch_on_function_error = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DestinationConfig" => {
                            destination_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventSourceArn" => {
                            event_source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionResponseTypes" => {
                            function_response_types = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumBatchingWindowInSeconds" => {
                            maximum_batching_window_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumRecordAgeInSeconds" => {
                            maximum_record_age_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumRetryAttempts" => {
                            maximum_retry_attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParallelizationFactor" => {
                            parallelization_factor = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Queues" => {
                            queues = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SelfManagedEventSource" => {
                            self_managed_event_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceAccessConfigurations" => {
                            source_access_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartingPosition" => {
                            starting_position = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartingPositionTimestamp" => {
                            starting_position_timestamp = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Topics" => {
                            topics = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TumblingWindowInSeconds" => {
                            tumbling_window_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventSourceMappingProperties {
                    batch_size: batch_size,
                    bisect_batch_on_function_error: bisect_batch_on_function_error,
                    destination_config: destination_config,
                    enabled: enabled,
                    event_source_arn: event_source_arn,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    function_response_types: function_response_types,
                    maximum_batching_window_in_seconds: maximum_batching_window_in_seconds,
                    maximum_record_age_in_seconds: maximum_record_age_in_seconds,
                    maximum_retry_attempts: maximum_retry_attempts,
                    parallelization_factor: parallelization_factor,
                    queues: queues,
                    self_managed_event_source: self_managed_event_source,
                    source_access_configurations: source_access_configurations,
                    starting_position: starting_position,
                    starting_position_timestamp: starting_position_timestamp,
                    topics: topics,
                    tumbling_window_in_seconds: tumbling_window_in_seconds,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for EventSourceMapping {
    type Properties = EventSourceMappingProperties;
    const TYPE: &'static str = "AWS::Lambda::EventSourceMapping";
    fn properties(&self) -> &EventSourceMappingProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventSourceMappingProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for EventSourceMapping {}

impl From<EventSourceMappingProperties> for EventSourceMapping {
    fn from(properties: EventSourceMappingProperties) -> EventSourceMapping {
        EventSourceMapping { properties, depends_on: None }
    }
}

/// The [`AWS::Lambda::Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html) resource type.
#[derive(Debug, Default)]
pub struct Function {
    properties: FunctionProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `Function` resource.
#[derive(Debug, Default)]
pub struct FunctionProperties {
    /// Property [`Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-code).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub code: crate::Value<self::function::Code>,
    /// Property [`CodeSigningConfigArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-codesigningconfigarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub code_signing_config_arn: Option<crate::Value<String>>,
    /// Property [`DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-deadletterconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dead_letter_config: Option<crate::Value<self::function::DeadLetterConfig>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<crate::Value<String>>,
    /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-environment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment: Option<crate::Value<self::function::Environment>>,
    /// Property [`FileSystemConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-filesystemconfigs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub file_system_configs: Option<crate::ValueList<self::function::FileSystemConfig>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: Option<crate::Value<String>>,
    /// Property [`Handler`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-handler).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub handler: Option<crate::Value<String>>,
    /// Property [`ImageConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-imageconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_config: Option<crate::Value<self::function::ImageConfig>>,
    /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-kmskeyarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_arn: Option<crate::Value<String>>,
    /// Property [`Layers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-layers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub layers: Option<crate::ValueList<String>>,
    /// Property [`MemorySize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-memorysize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub memory_size: Option<crate::Value<u32>>,
    /// Property [`PackageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-packagetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub package_type: Option<crate::Value<String>>,
    /// Property [`ReservedConcurrentExecutions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-reservedconcurrentexecutions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub reserved_concurrent_executions: Option<crate::Value<u32>>,
    /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-role).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role: crate::Value<String>,
    /// Property [`Runtime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-runtime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub runtime: Option<crate::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
    /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-timeout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub timeout: Option<crate::Value<u32>>,
    /// Property [`TracingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-tracingconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tracing_config: Option<crate::Value<self::function::TracingConfig>>,
    /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-vpcconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_config: Option<crate::Value<self::function::VpcConfig>>,
}

impl ::serde::Serialize for FunctionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Code", &self.code)?;
        if let Some(ref code_signing_config_arn) = self.code_signing_config_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeSigningConfigArn", code_signing_config_arn)?;
        }
        if let Some(ref dead_letter_config) = self.dead_letter_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeadLetterConfig", dead_letter_config)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref environment) = self.environment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
        }
        if let Some(ref file_system_configs) = self.file_system_configs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemConfigs", file_system_configs)?;
        }
        if let Some(ref function_name) = self.function_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", function_name)?;
        }
        if let Some(ref handler) = self.handler {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Handler", handler)?;
        }
        if let Some(ref image_config) = self.image_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageConfig", image_config)?;
        }
        if let Some(ref kms_key_arn) = self.kms_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
        }
        if let Some(ref layers) = self.layers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Layers", layers)?;
        }
        if let Some(ref memory_size) = self.memory_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemorySize", memory_size)?;
        }
        if let Some(ref package_type) = self.package_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackageType", package_type)?;
        }
        if let Some(ref reserved_concurrent_executions) = self.reserved_concurrent_executions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReservedConcurrentExecutions", reserved_concurrent_executions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
        if let Some(ref runtime) = self.runtime {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Runtime", runtime)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref timeout) = self.timeout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
        }
        if let Some(ref tracing_config) = self.tracing_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TracingConfig", tracing_config)?;
        }
        if let Some(ref vpc_config) = self.vpc_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
        }
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
                let mut code: Option<crate::Value<self::function::Code>> = None;
                let mut code_signing_config_arn: Option<crate::Value<String>> = None;
                let mut dead_letter_config: Option<crate::Value<self::function::DeadLetterConfig>> = None;
                let mut description: Option<crate::Value<String>> = None;
                let mut environment: Option<crate::Value<self::function::Environment>> = None;
                let mut file_system_configs: Option<crate::ValueList<self::function::FileSystemConfig>> = None;
                let mut function_name: Option<crate::Value<String>> = None;
                let mut handler: Option<crate::Value<String>> = None;
                let mut image_config: Option<crate::Value<self::function::ImageConfig>> = None;
                let mut kms_key_arn: Option<crate::Value<String>> = None;
                let mut layers: Option<crate::ValueList<String>> = None;
                let mut memory_size: Option<crate::Value<u32>> = None;
                let mut package_type: Option<crate::Value<String>> = None;
                let mut reserved_concurrent_executions: Option<crate::Value<u32>> = None;
                let mut role: Option<crate::Value<String>> = None;
                let mut runtime: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;
                let mut timeout: Option<crate::Value<u32>> = None;
                let mut tracing_config: Option<crate::Value<self::function::TracingConfig>> = None;
                let mut vpc_config: Option<crate::Value<self::function::VpcConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Code" => {
                            code = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CodeSigningConfigArn" => {
                            code_signing_config_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeadLetterConfig" => {
                            dead_letter_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Environment" => {
                            environment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileSystemConfigs" => {
                            file_system_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Handler" => {
                            handler = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageConfig" => {
                            image_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyArn" => {
                            kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Layers" => {
                            layers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MemorySize" => {
                            memory_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PackageType" => {
                            package_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReservedConcurrentExecutions" => {
                            reserved_concurrent_executions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Role" => {
                            role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Runtime" => {
                            runtime = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Timeout" => {
                            timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TracingConfig" => {
                            tracing_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcConfig" => {
                            vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FunctionProperties {
                    code: code.ok_or(::serde::de::Error::missing_field("Code"))?,
                    code_signing_config_arn: code_signing_config_arn,
                    dead_letter_config: dead_letter_config,
                    description: description,
                    environment: environment,
                    file_system_configs: file_system_configs,
                    function_name: function_name,
                    handler: handler,
                    image_config: image_config,
                    kms_key_arn: kms_key_arn,
                    layers: layers,
                    memory_size: memory_size,
                    package_type: package_type,
                    reserved_concurrent_executions: reserved_concurrent_executions,
                    role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
                    runtime: runtime,
                    tags: tags,
                    timeout: timeout,
                    tracing_config: tracing_config,
                    vpc_config: vpc_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Function {
    type Properties = FunctionProperties;
    const TYPE: &'static str = "AWS::Lambda::Function";
    fn properties(&self) -> &FunctionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FunctionProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for Function {}

impl From<FunctionProperties> for Function {
    fn from(properties: FunctionProperties) -> Function {
        Function { properties, depends_on: None }
    }
}

/// The [`AWS::Lambda::LayerVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html) resource type.
#[derive(Debug, Default)]
pub struct LayerVersion {
    properties: LayerVersionProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `LayerVersion` resource.
#[derive(Debug, Default)]
pub struct LayerVersionProperties {
    /// Property [`CompatibleRuntimes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html#cfn-lambda-layerversion-compatibleruntimes).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub compatible_runtimes: Option<crate::ValueList<String>>,
    /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html#cfn-lambda-layerversion-content).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub content: crate::Value<self::layer_version::Content>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html#cfn-lambda-layerversion-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<crate::Value<String>>,
    /// Property [`LayerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html#cfn-lambda-layerversion-layername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub layer_name: Option<crate::Value<String>>,
    /// Property [`LicenseInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html#cfn-lambda-layerversion-licenseinfo).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub license_info: Option<crate::Value<String>>,
}

impl ::serde::Serialize for LayerVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref compatible_runtimes) = self.compatible_runtimes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompatibleRuntimes", compatible_runtimes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref layer_name) = self.layer_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LayerName", layer_name)?;
        }
        if let Some(ref license_info) = self.license_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LicenseInfo", license_info)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LayerVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LayerVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LayerVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LayerVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut compatible_runtimes: Option<crate::ValueList<String>> = None;
                let mut content: Option<crate::Value<self::layer_version::Content>> = None;
                let mut description: Option<crate::Value<String>> = None;
                let mut layer_name: Option<crate::Value<String>> = None;
                let mut license_info: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CompatibleRuntimes" => {
                            compatible_runtimes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Content" => {
                            content = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LayerName" => {
                            layer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LicenseInfo" => {
                            license_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LayerVersionProperties {
                    compatible_runtimes: compatible_runtimes,
                    content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                    description: description,
                    layer_name: layer_name,
                    license_info: license_info,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for LayerVersion {
    type Properties = LayerVersionProperties;
    const TYPE: &'static str = "AWS::Lambda::LayerVersion";
    fn properties(&self) -> &LayerVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LayerVersionProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for LayerVersion {}

impl From<LayerVersionProperties> for LayerVersion {
    fn from(properties: LayerVersionProperties) -> LayerVersion {
        LayerVersion { properties, depends_on: None }
    }
}

/// The [`AWS::Lambda::LayerVersionPermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html) resource type.
#[derive(Debug, Default)]
pub struct LayerVersionPermission {
    properties: LayerVersionPermissionProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `LayerVersionPermission` resource.
#[derive(Debug, Default)]
pub struct LayerVersionPermissionProperties {
    /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html#cfn-lambda-layerversionpermission-action).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub action: crate::Value<String>,
    /// Property [`LayerVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html#cfn-lambda-layerversionpermission-layerversionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub layer_version_arn: crate::Value<String>,
    /// Property [`OrganizationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html#cfn-lambda-layerversionpermission-organizationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub organization_id: Option<crate::Value<String>>,
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html#cfn-lambda-layerversionpermission-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: crate::Value<String>,
}

impl ::serde::Serialize for LayerVersionPermissionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LayerVersionArn", &self.layer_version_arn)?;
        if let Some(ref organization_id) = self.organization_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationId", organization_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LayerVersionPermissionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LayerVersionPermissionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LayerVersionPermissionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LayerVersionPermissionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action: Option<crate::Value<String>> = None;
                let mut layer_version_arn: Option<crate::Value<String>> = None;
                let mut organization_id: Option<crate::Value<String>> = None;
                let mut principal: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Action" => {
                            action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LayerVersionArn" => {
                            layer_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OrganizationId" => {
                            organization_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LayerVersionPermissionProperties {
                    action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                    layer_version_arn: layer_version_arn.ok_or(::serde::de::Error::missing_field("LayerVersionArn"))?,
                    organization_id: organization_id,
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for LayerVersionPermission {
    type Properties = LayerVersionPermissionProperties;
    const TYPE: &'static str = "AWS::Lambda::LayerVersionPermission";
    fn properties(&self) -> &LayerVersionPermissionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LayerVersionPermissionProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for LayerVersionPermission {}

impl From<LayerVersionPermissionProperties> for LayerVersionPermission {
    fn from(properties: LayerVersionPermissionProperties) -> LayerVersionPermission {
        LayerVersionPermission { properties, depends_on: None }
    }
}

/// The [`AWS::Lambda::Permission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html) resource type.
#[derive(Debug, Default)]
pub struct Permission {
    properties: PermissionProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `Permission` resource.
#[derive(Debug, Default)]
pub struct PermissionProperties {
    /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-action).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub action: crate::Value<String>,
    /// Property [`EventSourceToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-eventsourcetoken).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_source_token: Option<crate::Value<String>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: crate::Value<String>,
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: crate::Value<String>,
    /// Property [`SourceAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-sourceaccount).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_account: Option<crate::Value<String>>,
    /// Property [`SourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-sourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_arn: Option<crate::Value<String>>,
}

impl ::serde::Serialize for PermissionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
        if let Some(ref event_source_token) = self.event_source_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSourceToken", event_source_token)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        if let Some(ref source_account) = self.source_account {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceAccount", source_account)?;
        }
        if let Some(ref source_arn) = self.source_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceArn", source_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PermissionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PermissionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PermissionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PermissionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action: Option<crate::Value<String>> = None;
                let mut event_source_token: Option<crate::Value<String>> = None;
                let mut function_name: Option<crate::Value<String>> = None;
                let mut principal: Option<crate::Value<String>> = None;
                let mut source_account: Option<crate::Value<String>> = None;
                let mut source_arn: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Action" => {
                            action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventSourceToken" => {
                            event_source_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceAccount" => {
                            source_account = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceArn" => {
                            source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PermissionProperties {
                    action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                    event_source_token: event_source_token,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    source_account: source_account,
                    source_arn: source_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Permission {
    type Properties = PermissionProperties;
    const TYPE: &'static str = "AWS::Lambda::Permission";
    fn properties(&self) -> &PermissionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PermissionProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for Permission {}

impl From<PermissionProperties> for Permission {
    fn from(properties: PermissionProperties) -> Permission {
        Permission { properties, depends_on: None }
    }
}

/// The [`AWS::Lambda::Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html) resource type.
#[derive(Debug, Default)]
pub struct Version {
    properties: VersionProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `Version` resource.
#[derive(Debug, Default)]
pub struct VersionProperties {
    /// Property [`CodeSha256`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-codesha256).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub code_sha256: Option<crate::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<crate::Value<String>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: crate::Value<String>,
    /// Property [`ProvisionedConcurrencyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-provisionedconcurrencyconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioned_concurrency_config: Option<crate::Value<self::version::ProvisionedConcurrencyConfiguration>>,
}

impl ::serde::Serialize for VersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref code_sha256) = self.code_sha256 {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeSha256", code_sha256)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        if let Some(ref provisioned_concurrency_config) = self.provisioned_concurrency_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedConcurrencyConfig", provisioned_concurrency_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut code_sha256: Option<crate::Value<String>> = None;
                let mut description: Option<crate::Value<String>> = None;
                let mut function_name: Option<crate::Value<String>> = None;
                let mut provisioned_concurrency_config: Option<crate::Value<self::version::ProvisionedConcurrencyConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CodeSha256" => {
                            code_sha256 = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisionedConcurrencyConfig" => {
                            provisioned_concurrency_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VersionProperties {
                    code_sha256: code_sha256,
                    description: description,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    provisioned_concurrency_config: provisioned_concurrency_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Version {
    type Properties = VersionProperties;
    const TYPE: &'static str = "AWS::Lambda::Version";
    fn properties(&self) -> &VersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VersionProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for Version {}

impl From<VersionProperties> for Version {
    fn from(properties: VersionProperties) -> Version {
        Version { properties, depends_on: None }
    }
}

pub mod alias {
    //! Property types for the `Alias` resource.

    /// The [`AWS::Lambda::Alias.AliasRoutingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-aliasroutingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AliasRoutingConfiguration {
        /// Property [`AdditionalVersionWeights`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-aliasroutingconfiguration.html#cfn-lambda-alias-aliasroutingconfiguration-additionalversionweights).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub additional_version_weights: crate::ValueList<VersionWeight>,
    }

    impl crate::codec::SerializeValue for AliasRoutingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalVersionWeights", &self.additional_version_weights)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for AliasRoutingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AliasRoutingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AliasRoutingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AliasRoutingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut additional_version_weights: Option<crate::ValueList<VersionWeight>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdditionalVersionWeights" => {
                                additional_version_weights = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AliasRoutingConfiguration {
                        additional_version_weights: additional_version_weights.ok_or(::serde::de::Error::missing_field("AdditionalVersionWeights"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Alias.ProvisionedConcurrencyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-provisionedconcurrencyconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisionedConcurrencyConfiguration {
        /// Property [`ProvisionedConcurrentExecutions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-provisionedconcurrencyconfiguration.html#cfn-lambda-alias-provisionedconcurrencyconfiguration-provisionedconcurrentexecutions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provisioned_concurrent_executions: crate::Value<u32>,
    }

    impl crate::codec::SerializeValue for ProvisionedConcurrencyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedConcurrentExecutions", &self.provisioned_concurrent_executions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ProvisionedConcurrencyConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisionedConcurrencyConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisionedConcurrencyConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisionedConcurrencyConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut provisioned_concurrent_executions: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ProvisionedConcurrentExecutions" => {
                                provisioned_concurrent_executions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProvisionedConcurrencyConfiguration {
                        provisioned_concurrent_executions: provisioned_concurrent_executions.ok_or(::serde::de::Error::missing_field("ProvisionedConcurrentExecutions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Alias.VersionWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-versionweight.html) property type.
    #[derive(Debug, Default)]
    pub struct VersionWeight {
        /// Property [`FunctionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-versionweight.html#cfn-lambda-alias-versionweight-functionversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_version: crate::Value<String>,
        /// Property [`FunctionWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-versionweight.html#cfn-lambda-alias-versionweight-functionweight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_weight: crate::Value<f64>,
    }

    impl crate::codec::SerializeValue for VersionWeight {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionVersion", &self.function_version)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionWeight", &self.function_weight)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for VersionWeight {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VersionWeight, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VersionWeight;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VersionWeight")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut function_version: Option<crate::Value<String>> = None;
                    let mut function_weight: Option<crate::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionVersion" => {
                                function_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FunctionWeight" => {
                                function_weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VersionWeight {
                        function_version: function_version.ok_or(::serde::de::Error::missing_field("FunctionVersion"))?,
                        function_weight: function_weight.ok_or(::serde::de::Error::missing_field("FunctionWeight"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod code_signing_config {
    //! Property types for the `CodeSigningConfig` resource.

    /// The [`AWS::Lambda::CodeSigningConfig.AllowedPublishers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-codesigningconfig-allowedpublishers.html) property type.
    #[derive(Debug, Default)]
    pub struct AllowedPublishers {
        /// Property [`SigningProfileVersionArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-codesigningconfig-allowedpublishers.html#cfn-lambda-codesigningconfig-allowedpublishers-signingprofileversionarns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub signing_profile_version_arns: crate::ValueList<String>,
    }

    impl crate::codec::SerializeValue for AllowedPublishers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SigningProfileVersionArns", &self.signing_profile_version_arns)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for AllowedPublishers {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AllowedPublishers, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AllowedPublishers;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AllowedPublishers")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut signing_profile_version_arns: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SigningProfileVersionArns" => {
                                signing_profile_version_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AllowedPublishers {
                        signing_profile_version_arns: signing_profile_version_arns.ok_or(::serde::de::Error::missing_field("SigningProfileVersionArns"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::CodeSigningConfig.CodeSigningPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-codesigningconfig-codesigningpolicies.html) property type.
    #[derive(Debug, Default)]
    pub struct CodeSigningPolicies {
        /// Property [`UntrustedArtifactOnDeployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-codesigningconfig-codesigningpolicies.html#cfn-lambda-codesigningconfig-codesigningpolicies-untrustedartifactondeployment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub untrusted_artifact_on_deployment: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for CodeSigningPolicies {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UntrustedArtifactOnDeployment", &self.untrusted_artifact_on_deployment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for CodeSigningPolicies {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CodeSigningPolicies, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CodeSigningPolicies;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CodeSigningPolicies")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut untrusted_artifact_on_deployment: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "UntrustedArtifactOnDeployment" => {
                                untrusted_artifact_on_deployment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CodeSigningPolicies {
                        untrusted_artifact_on_deployment: untrusted_artifact_on_deployment.ok_or(::serde::de::Error::missing_field("UntrustedArtifactOnDeployment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod event_invoke_config {
    //! Property types for the `EventInvokeConfig` resource.

    /// The [`AWS::Lambda::EventInvokeConfig.DestinationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-destinationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DestinationConfig {
        /// Property [`OnFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-destinationconfig.html#cfn-lambda-eventinvokeconfig-destinationconfig-onfailure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_failure: Option<crate::Value<OnFailure>>,
        /// Property [`OnSuccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-destinationconfig.html#cfn-lambda-eventinvokeconfig-destinationconfig-onsuccess).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_success: Option<crate::Value<OnSuccess>>,
    }

    impl crate::codec::SerializeValue for DestinationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref on_failure) = self.on_failure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnFailure", on_failure)?;
            }
            if let Some(ref on_success) = self.on_success {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnSuccess", on_success)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DestinationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DestinationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DestinationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut on_failure: Option<crate::Value<OnFailure>> = None;
                    let mut on_success: Option<crate::Value<OnSuccess>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OnFailure" => {
                                on_failure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnSuccess" => {
                                on_success = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DestinationConfig {
                        on_failure: on_failure,
                        on_success: on_success,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventInvokeConfig.OnFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-destinationconfig-onfailure.html) property type.
    #[derive(Debug, Default)]
    pub struct OnFailure {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-destinationconfig-onfailure.html#cfn-lambda-eventinvokeconfig-destinationconfig-onfailure-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for OnFailure {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OnFailure {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OnFailure, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OnFailure;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OnFailure")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OnFailure {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventInvokeConfig.OnSuccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-destinationconfig-onsuccess.html) property type.
    #[derive(Debug, Default)]
    pub struct OnSuccess {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-destinationconfig-onsuccess.html#cfn-lambda-eventinvokeconfig-destinationconfig-onsuccess-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for OnSuccess {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OnSuccess {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OnSuccess, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OnSuccess;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OnSuccess")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OnSuccess {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod event_source_mapping {
    //! Property types for the `EventSourceMapping` resource.

    /// The [`AWS::Lambda::EventSourceMapping.DestinationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-destinationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DestinationConfig {
        /// Property [`OnFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-destinationconfig.html#cfn-lambda-eventsourcemapping-destinationconfig-onfailure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_failure: Option<crate::Value<OnFailure>>,
    }

    impl crate::codec::SerializeValue for DestinationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref on_failure) = self.on_failure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnFailure", on_failure)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DestinationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DestinationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DestinationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut on_failure: Option<crate::Value<OnFailure>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OnFailure" => {
                                on_failure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DestinationConfig {
                        on_failure: on_failure,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.Endpoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-endpoints.html) property type.
    #[derive(Debug, Default)]
    pub struct Endpoints {
        /// Property [`KafkaBootstrapServers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-endpoints.html#cfn-lambda-eventsourcemapping-endpoints-kafkabootstrapservers).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kafka_bootstrap_servers: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for Endpoints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kafka_bootstrap_servers) = self.kafka_bootstrap_servers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KafkaBootstrapServers", kafka_bootstrap_servers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Endpoints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Endpoints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Endpoints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Endpoints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kafka_bootstrap_servers: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KafkaBootstrapServers" => {
                                kafka_bootstrap_servers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Endpoints {
                        kafka_bootstrap_servers: kafka_bootstrap_servers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.OnFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-onfailure.html) property type.
    #[derive(Debug, Default)]
    pub struct OnFailure {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-onfailure.html#cfn-lambda-eventsourcemapping-onfailure-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for OnFailure {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OnFailure {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OnFailure, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OnFailure;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OnFailure")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OnFailure {
                        destination: destination,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.SelfManagedEventSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-selfmanagedeventsource.html) property type.
    #[derive(Debug, Default)]
    pub struct SelfManagedEventSource {
        /// Property [`Endpoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-selfmanagedeventsource.html#cfn-lambda-eventsourcemapping-selfmanagedeventsource-endpoints).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoints: Option<crate::Value<Endpoints>>,
    }

    impl crate::codec::SerializeValue for SelfManagedEventSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref endpoints) = self.endpoints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoints", endpoints)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for SelfManagedEventSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SelfManagedEventSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SelfManagedEventSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SelfManagedEventSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoints: Option<crate::Value<Endpoints>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Endpoints" => {
                                endpoints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SelfManagedEventSource {
                        endpoints: endpoints,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.SourceAccessConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-sourceaccessconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceAccessConfiguration {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-sourceaccessconfiguration.html#cfn-lambda-eventsourcemapping-sourceaccessconfiguration-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<crate::Value<String>>,
        /// Property [`URI`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-sourceaccessconfiguration.html#cfn-lambda-eventsourcemapping-sourceaccessconfiguration-uri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uri: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for SourceAccessConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            if let Some(ref uri) = self.uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "URI", uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for SourceAccessConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceAccessConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceAccessConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceAccessConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<crate::Value<String>> = None;
                    let mut uri: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "URI" => {
                                uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceAccessConfiguration {
                        r#type: r#type,
                        uri: uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod function {
    //! Property types for the `Function` resource.

    /// The [`AWS::Lambda::Function.Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html) property type.
    #[derive(Debug, Default)]
    pub struct Code {
        /// Property [`ImageUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html#cfn-lambda-function-code-imageuri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_uri: Option<crate::Value<String>>,
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html#cfn-lambda-function-code-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: Option<crate::Value<String>>,
        /// Property [`S3Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html#cfn-lambda-function-code-s3key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key: Option<crate::Value<String>>,
        /// Property [`S3ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html#cfn-lambda-function-code-s3objectversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_object_version: Option<crate::Value<String>>,
        /// Property [`ZipFile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html#cfn-lambda-function-code-zipfile).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zip_file: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for Code {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref image_uri) = self.image_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUri", image_uri)?;
            }
            if let Some(ref s3_bucket) = self.s3_bucket {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", s3_bucket)?;
            }
            if let Some(ref s3_key) = self.s3_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", s3_key)?;
            }
            if let Some(ref s3_object_version) = self.s3_object_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ObjectVersion", s3_object_version)?;
            }
            if let Some(ref zip_file) = self.zip_file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ZipFile", zip_file)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Code {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Code, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Code;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Code")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut image_uri: Option<crate::Value<String>> = None;
                    let mut s3_bucket: Option<crate::Value<String>> = None;
                    let mut s3_key: Option<crate::Value<String>> = None;
                    let mut s3_object_version: Option<crate::Value<String>> = None;
                    let mut zip_file: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImageUri" => {
                                image_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Key" => {
                                s3_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3ObjectVersion" => {
                                s3_object_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ZipFile" => {
                                zip_file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Code {
                        image_uri: image_uri,
                        s3_bucket: s3_bucket,
                        s3_key: s3_key,
                        s3_object_version: s3_object_version,
                        zip_file: zip_file,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-deadletterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DeadLetterConfig {
        /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-deadletterconfig.html#cfn-lambda-function-deadletterconfig-targetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_arn: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for DeadLetterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref target_arn) = self.target_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", target_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DeadLetterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeadLetterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeadLetterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeadLetterConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_arn: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetArn" => {
                                target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeadLetterConfig {
                        target_arn: target_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-environment.html) property type.
    #[derive(Debug, Default)]
    pub struct Environment {
        /// Property [`Variables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-environment.html#cfn-lambda-function-environment-variables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variables: Option<crate::ValueMap<String>>,
    }

    impl crate::codec::SerializeValue for Environment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref variables) = self.variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", variables)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Environment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Environment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Environment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Environment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut variables: Option<crate::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Variables" => {
                                variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Environment {
                        variables: variables,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.FileSystemConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-filesystemconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FileSystemConfig {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-filesystemconfig.html#cfn-lambda-function-filesystemconfig-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: crate::Value<String>,
        /// Property [`LocalMountPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-filesystemconfig.html#cfn-lambda-function-filesystemconfig-localmountpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub local_mount_path: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for FileSystemConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalMountPath", &self.local_mount_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for FileSystemConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FileSystemConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FileSystemConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FileSystemConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<crate::Value<String>> = None;
                    let mut local_mount_path: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocalMountPath" => {
                                local_mount_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FileSystemConfig {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        local_mount_path: local_mount_path.ok_or(::serde::de::Error::missing_field("LocalMountPath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.ImageConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-imageconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ImageConfig {
        /// Property [`Command`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-imageconfig.html#cfn-lambda-function-imageconfig-command).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub command: Option<crate::ValueList<String>>,
        /// Property [`EntryPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-imageconfig.html#cfn-lambda-function-imageconfig-entrypoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entry_point: Option<crate::ValueList<String>>,
        /// Property [`WorkingDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-imageconfig.html#cfn-lambda-function-imageconfig-workingdirectory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub working_directory: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for ImageConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref command) = self.command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Command", command)?;
            }
            if let Some(ref entry_point) = self.entry_point {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntryPoint", entry_point)?;
            }
            if let Some(ref working_directory) = self.working_directory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkingDirectory", working_directory)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ImageConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImageConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImageConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut command: Option<crate::ValueList<String>> = None;
                    let mut entry_point: Option<crate::ValueList<String>> = None;
                    let mut working_directory: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Command" => {
                                command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntryPoint" => {
                                entry_point = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkingDirectory" => {
                                working_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImageConfig {
                        command: command,
                        entry_point: entry_point,
                        working_directory: working_directory,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.TracingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-tracingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TracingConfig {
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-tracingconfig.html#cfn-lambda-function-tracingconfig-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for TracingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mode) = self.mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for TracingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TracingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TracingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TracingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mode: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TracingConfig {
                        mode: mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html#cfn-lambda-function-vpcconfig-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<crate::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html#cfn-lambda-function-vpcconfig-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<crate::ValueList<String>> = None;
                    let mut subnet_ids: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod layer_version {
    //! Property types for the `LayerVersion` resource.

    /// The [`AWS::Lambda::LayerVersion.Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-layerversion-content.html) property type.
    #[derive(Debug, Default)]
    pub struct Content {
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-layerversion-content.html#cfn-lambda-layerversion-content-s3bucket).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_bucket: crate::Value<String>,
        /// Property [`S3Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-layerversion-content.html#cfn-lambda-layerversion-content-s3key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_key: crate::Value<String>,
        /// Property [`S3ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-layerversion-content.html#cfn-lambda-layerversion-content-s3objectversion).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_object_version: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for Content {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", &self.s3_key)?;
            if let Some(ref s3_object_version) = self.s3_object_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ObjectVersion", s3_object_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Content {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Content, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Content;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Content")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_bucket: Option<crate::Value<String>> = None;
                    let mut s3_key: Option<crate::Value<String>> = None;
                    let mut s3_object_version: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Key" => {
                                s3_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3ObjectVersion" => {
                                s3_object_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Content {
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                        s3_key: s3_key.ok_or(::serde::de::Error::missing_field("S3Key"))?,
                        s3_object_version: s3_object_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod version {
    //! Property types for the `Version` resource.

    /// The [`AWS::Lambda::Version.ProvisionedConcurrencyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-version-provisionedconcurrencyconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisionedConcurrencyConfiguration {
        /// Property [`ProvisionedConcurrentExecutions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-version-provisionedconcurrencyconfiguration.html#cfn-lambda-version-provisionedconcurrencyconfiguration-provisionedconcurrentexecutions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provisioned_concurrent_executions: crate::Value<u32>,
    }

    impl crate::codec::SerializeValue for ProvisionedConcurrencyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedConcurrentExecutions", &self.provisioned_concurrent_executions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ProvisionedConcurrencyConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisionedConcurrencyConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisionedConcurrencyConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisionedConcurrencyConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut provisioned_concurrent_executions: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ProvisionedConcurrentExecutions" => {
                                provisioned_concurrent_executions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProvisionedConcurrencyConfiguration {
                        provisioned_concurrent_executions: provisioned_concurrent_executions.ok_or(::serde::de::Error::missing_field("ProvisionedConcurrentExecutions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
