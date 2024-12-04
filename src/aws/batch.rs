//! Types for the `Batch` service.

/// The [`AWS::Batch::ComputeEnvironment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html) resource type.
#[derive(Debug, Default)]
pub struct ComputeEnvironment {
    properties: ComputeEnvironmentProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `ComputeEnvironment` resource.
#[derive(Debug, Default)]
pub struct ComputeEnvironmentProperties {
    /// Property [`ComputeEnvironmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html#cfn-batch-computeenvironment-computeenvironmentname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub compute_environment_name: Option<crate::Value<String>>,
    /// Property [`ComputeResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html#cfn-batch-computeenvironment-computeresources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub compute_resources: Option<crate::Value<self::compute_environment::ComputeResources>>,
    /// Property [`ServiceRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html#cfn-batch-computeenvironment-servicerole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub service_role: Option<crate::Value<String>>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html#cfn-batch-computeenvironment-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<crate::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html#cfn-batch-computeenvironment-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<crate::Value<crate::json::Value>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html#cfn-batch-computeenvironment-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: crate::Value<String>,
}

impl ::serde::Serialize for ComputeEnvironmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref compute_environment_name) = self.compute_environment_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeEnvironmentName", compute_environment_name)?;
        }
        if let Some(ref compute_resources) = self.compute_resources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeResources", compute_resources)?;
        }
        if let Some(ref service_role) = self.service_role {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRole", service_role)?;
        }
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ComputeEnvironmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ComputeEnvironmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ComputeEnvironmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ComputeEnvironmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut compute_environment_name: Option<crate::Value<String>> = None;
                let mut compute_resources: Option<crate::Value<self::compute_environment::ComputeResources>> = None;
                let mut service_role: Option<crate::Value<String>> = None;
                let mut state: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::Value<crate::json::Value>> = None;
                let mut r#type: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ComputeEnvironmentName" => {
                            compute_environment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ComputeResources" => {
                            compute_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceRole" => {
                            service_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ComputeEnvironmentProperties {
                    compute_environment_name: compute_environment_name,
                    compute_resources: compute_resources,
                    service_role: service_role,
                    state: state,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for ComputeEnvironment {
    type Properties = ComputeEnvironmentProperties;
    const TYPE: &'static str = "AWS::Batch::ComputeEnvironment";
    fn properties(&self) -> &ComputeEnvironmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ComputeEnvironmentProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for ComputeEnvironment {}

impl From<ComputeEnvironmentProperties> for ComputeEnvironment {
    fn from(properties: ComputeEnvironmentProperties) -> ComputeEnvironment {
        ComputeEnvironment { properties, depends_on: None }
    }
}

/// The [`AWS::Batch::JobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html) resource type.
#[derive(Debug, Default)]
pub struct JobDefinition {
    properties: JobDefinitionProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `JobDefinition` resource.
#[derive(Debug, Default)]
pub struct JobDefinitionProperties {
    /// Property [`ContainerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html#cfn-batch-jobdefinition-containerproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub container_properties: Option<crate::Value<self::job_definition::ContainerProperties>>,
    /// Property [`JobDefinitionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html#cfn-batch-jobdefinition-jobdefinitionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_definition_name: Option<crate::Value<String>>,
    /// Property [`NodeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html#cfn-batch-jobdefinition-nodeproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub node_properties: Option<crate::Value<self::job_definition::NodeProperties>>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html#cfn-batch-jobdefinition-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<crate::Value<crate::json::Value>>,
    /// Property [`PlatformCapabilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html#cfn-batch-jobdefinition-platformcapabilities).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub platform_capabilities: Option<crate::ValueList<String>>,
    /// Property [`PropagateTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html#cfn-batch-jobdefinition-propagatetags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub propagate_tags: Option<crate::Value<bool>>,
    /// Property [`RetryStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html#cfn-batch-jobdefinition-retrystrategy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub retry_strategy: Option<crate::Value<self::job_definition::RetryStrategy>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html#cfn-batch-jobdefinition-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<crate::Value<crate::json::Value>>,
    /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html#cfn-batch-jobdefinition-timeout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub timeout: Option<crate::Value<self::job_definition::Timeout>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html#cfn-batch-jobdefinition-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub r#type: crate::Value<String>,
}

impl ::serde::Serialize for JobDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref container_properties) = self.container_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerProperties", container_properties)?;
        }
        if let Some(ref job_definition_name) = self.job_definition_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobDefinitionName", job_definition_name)?;
        }
        if let Some(ref node_properties) = self.node_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeProperties", node_properties)?;
        }
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref platform_capabilities) = self.platform_capabilities {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformCapabilities", platform_capabilities)?;
        }
        if let Some(ref propagate_tags) = self.propagate_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropagateTags", propagate_tags)?;
        }
        if let Some(ref retry_strategy) = self.retry_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryStrategy", retry_strategy)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref timeout) = self.timeout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for JobDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<JobDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = JobDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type JobDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut container_properties: Option<crate::Value<self::job_definition::ContainerProperties>> = None;
                let mut job_definition_name: Option<crate::Value<String>> = None;
                let mut node_properties: Option<crate::Value<self::job_definition::NodeProperties>> = None;
                let mut parameters: Option<crate::Value<crate::json::Value>> = None;
                let mut platform_capabilities: Option<crate::ValueList<String>> = None;
                let mut propagate_tags: Option<crate::Value<bool>> = None;
                let mut retry_strategy: Option<crate::Value<self::job_definition::RetryStrategy>> = None;
                let mut tags: Option<crate::Value<crate::json::Value>> = None;
                let mut timeout: Option<crate::Value<self::job_definition::Timeout>> = None;
                let mut r#type: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContainerProperties" => {
                            container_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobDefinitionName" => {
                            job_definition_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NodeProperties" => {
                            node_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlatformCapabilities" => {
                            platform_capabilities = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PropagateTags" => {
                            propagate_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetryStrategy" => {
                            retry_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Timeout" => {
                            timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(JobDefinitionProperties {
                    container_properties: container_properties,
                    job_definition_name: job_definition_name,
                    node_properties: node_properties,
                    parameters: parameters,
                    platform_capabilities: platform_capabilities,
                    propagate_tags: propagate_tags,
                    retry_strategy: retry_strategy,
                    tags: tags,
                    timeout: timeout,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for JobDefinition {
    type Properties = JobDefinitionProperties;
    const TYPE: &'static str = "AWS::Batch::JobDefinition";
    fn properties(&self) -> &JobDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobDefinitionProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for JobDefinition {}

impl From<JobDefinitionProperties> for JobDefinition {
    fn from(properties: JobDefinitionProperties) -> JobDefinition {
        JobDefinition { properties, depends_on: None }
    }
}

/// The [`AWS::Batch::JobQueue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobqueue.html) resource type.
#[derive(Debug, Default)]
pub struct JobQueue {
    properties: JobQueueProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `JobQueue` resource.
#[derive(Debug, Default)]
pub struct JobQueueProperties {
    /// Property [`ComputeEnvironmentOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobqueue.html#cfn-batch-jobqueue-computeenvironmentorder).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub compute_environment_order: crate::ValueList<self::job_queue::ComputeEnvironmentOrder>,
    /// Property [`JobQueueName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobqueue.html#cfn-batch-jobqueue-jobqueuename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_queue_name: Option<crate::Value<String>>,
    /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobqueue.html#cfn-batch-jobqueue-priority).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub priority: crate::Value<u32>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobqueue.html#cfn-batch-jobqueue-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<crate::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobqueue.html#cfn-batch-jobqueue-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<crate::Value<crate::json::Value>>,
}

impl ::serde::Serialize for JobQueueProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeEnvironmentOrder", &self.compute_environment_order)?;
        if let Some(ref job_queue_name) = self.job_queue_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobQueueName", job_queue_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for JobQueueProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<JobQueueProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = JobQueueProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type JobQueueProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut compute_environment_order: Option<crate::ValueList<self::job_queue::ComputeEnvironmentOrder>> = None;
                let mut job_queue_name: Option<crate::Value<String>> = None;
                let mut priority: Option<crate::Value<u32>> = None;
                let mut state: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::Value<crate::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ComputeEnvironmentOrder" => {
                            compute_environment_order = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobQueueName" => {
                            job_queue_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Priority" => {
                            priority = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(JobQueueProperties {
                    compute_environment_order: compute_environment_order.ok_or(::serde::de::Error::missing_field("ComputeEnvironmentOrder"))?,
                    job_queue_name: job_queue_name,
                    priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                    state: state,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for JobQueue {
    type Properties = JobQueueProperties;
    const TYPE: &'static str = "AWS::Batch::JobQueue";
    fn properties(&self) -> &JobQueueProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobQueueProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for JobQueue {}

impl From<JobQueueProperties> for JobQueue {
    fn from(properties: JobQueueProperties) -> JobQueue {
        JobQueue { properties, depends_on: None }
    }
}

pub mod compute_environment {
    //! Property types for the `ComputeEnvironment` resource.

    /// The [`AWS::Batch::ComputeEnvironment.ComputeResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html) property type.
    #[derive(Debug, Default)]
    pub struct ComputeResources {
        /// Property [`AllocationStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-allocationstrategy).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub allocation_strategy: Option<crate::Value<String>>,
        /// Property [`BidPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-bidpercentage).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bid_percentage: Option<crate::Value<u32>>,
        /// Property [`DesiredvCpus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-desiredvcpus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub desiredv_cpus: Option<crate::Value<u32>>,
        /// Property [`Ec2Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-ec2configuration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ec2_configuration: Option<crate::ValueList<Ec2ConfigurationObject>>,
        /// Property [`Ec2KeyPair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-ec2keypair).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ec2_key_pair: Option<crate::Value<String>>,
        /// Property [`ImageId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-imageid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image_id: Option<crate::Value<String>>,
        /// Property [`InstanceRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-instancerole).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_role: Option<crate::Value<String>>,
        /// Property [`InstanceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-instancetypes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_types: Option<crate::ValueList<String>>,
        /// Property [`LaunchTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-launchtemplate).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub launch_template: Option<crate::Value<LaunchTemplateSpecification>>,
        /// Property [`MaxvCpus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-maxvcpus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maxv_cpus: crate::Value<u32>,
        /// Property [`MinvCpus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-minvcpus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minv_cpus: Option<crate::Value<u32>>,
        /// Property [`PlacementGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-placementgroup).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub placement_group: Option<crate::Value<String>>,
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<crate::ValueList<String>>,
        /// Property [`SpotIamFleetRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-spotiamfleetrole).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub spot_iam_fleet_role: Option<crate::Value<String>>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-subnets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnets: crate::ValueList<String>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-tags).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tags: Option<crate::Value<crate::json::Value>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html#cfn-batch-computeenvironment-computeresources-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for ComputeResources {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allocation_strategy) = self.allocation_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocationStrategy", allocation_strategy)?;
            }
            if let Some(ref bid_percentage) = self.bid_percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPercentage", bid_percentage)?;
            }
            if let Some(ref desiredv_cpus) = self.desiredv_cpus {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredvCpus", desiredv_cpus)?;
            }
            if let Some(ref ec2_configuration) = self.ec2_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2Configuration", ec2_configuration)?;
            }
            if let Some(ref ec2_key_pair) = self.ec2_key_pair {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2KeyPair", ec2_key_pair)?;
            }
            if let Some(ref image_id) = self.image_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageId", image_id)?;
            }
            if let Some(ref instance_role) = self.instance_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceRole", instance_role)?;
            }
            if let Some(ref instance_types) = self.instance_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceTypes", instance_types)?;
            }
            if let Some(ref launch_template) = self.launch_template {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplate", launch_template)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxvCpus", &self.maxv_cpus)?;
            if let Some(ref minv_cpus) = self.minv_cpus {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinvCpus", minv_cpus)?;
            }
            if let Some(ref placement_group) = self.placement_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementGroup", placement_group)?;
            }
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref spot_iam_fleet_role) = self.spot_iam_fleet_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotIamFleetRole", spot_iam_fleet_role)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ComputeResources {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComputeResources, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComputeResources;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComputeResources")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allocation_strategy: Option<crate::Value<String>> = None;
                    let mut bid_percentage: Option<crate::Value<u32>> = None;
                    let mut desiredv_cpus: Option<crate::Value<u32>> = None;
                    let mut ec2_configuration: Option<crate::ValueList<Ec2ConfigurationObject>> = None;
                    let mut ec2_key_pair: Option<crate::Value<String>> = None;
                    let mut image_id: Option<crate::Value<String>> = None;
                    let mut instance_role: Option<crate::Value<String>> = None;
                    let mut instance_types: Option<crate::ValueList<String>> = None;
                    let mut launch_template: Option<crate::Value<LaunchTemplateSpecification>> = None;
                    let mut maxv_cpus: Option<crate::Value<u32>> = None;
                    let mut minv_cpus: Option<crate::Value<u32>> = None;
                    let mut placement_group: Option<crate::Value<String>> = None;
                    let mut security_group_ids: Option<crate::ValueList<String>> = None;
                    let mut spot_iam_fleet_role: Option<crate::Value<String>> = None;
                    let mut subnets: Option<crate::ValueList<String>> = None;
                    let mut tags: Option<crate::Value<crate::json::Value>> = None;
                    let mut r#type: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllocationStrategy" => {
                                allocation_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BidPercentage" => {
                                bid_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DesiredvCpus" => {
                                desiredv_cpus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ec2Configuration" => {
                                ec2_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ec2KeyPair" => {
                                ec2_key_pair = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageId" => {
                                image_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceRole" => {
                                instance_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceTypes" => {
                                instance_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchTemplate" => {
                                launch_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxvCpus" => {
                                maxv_cpus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinvCpus" => {
                                minv_cpus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlacementGroup" => {
                                placement_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpotIamFleetRole" => {
                                spot_iam_fleet_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(ComputeResources {
                        allocation_strategy: allocation_strategy,
                        bid_percentage: bid_percentage,
                        desiredv_cpus: desiredv_cpus,
                        ec2_configuration: ec2_configuration,
                        ec2_key_pair: ec2_key_pair,
                        image_id: image_id,
                        instance_role: instance_role,
                        instance_types: instance_types,
                        launch_template: launch_template,
                        maxv_cpus: maxv_cpus.ok_or(::serde::de::Error::missing_field("MaxvCpus"))?,
                        minv_cpus: minv_cpus,
                        placement_group: placement_group,
                        security_group_ids: security_group_ids,
                        spot_iam_fleet_role: spot_iam_fleet_role,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                        tags: tags,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::ComputeEnvironment.Ec2ConfigurationObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-ec2configurationobject.html) property type.
    #[derive(Debug, Default)]
    pub struct Ec2ConfigurationObject {
        /// Property [`ImageIdOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-ec2configurationobject.html#cfn-batch-computeenvironment-ec2configurationobject-imageidoverride).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image_id_override: Option<crate::Value<String>>,
        /// Property [`ImageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-ec2configurationobject.html#cfn-batch-computeenvironment-ec2configurationobject-imagetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image_type: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for Ec2ConfigurationObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref image_id_override) = self.image_id_override {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageIdOverride", image_id_override)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageType", &self.image_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Ec2ConfigurationObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Ec2ConfigurationObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Ec2ConfigurationObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Ec2ConfigurationObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut image_id_override: Option<crate::Value<String>> = None;
                    let mut image_type: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImageIdOverride" => {
                                image_id_override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageType" => {
                                image_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Ec2ConfigurationObject {
                        image_id_override: image_id_override,
                        image_type: image_type.ok_or(::serde::de::Error::missing_field("ImageType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::ComputeEnvironment.LaunchTemplateSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-launchtemplatespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct LaunchTemplateSpecification {
        /// Property [`LaunchTemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-launchtemplatespecification.html#cfn-batch-computeenvironment-launchtemplatespecification-launchtemplateid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub launch_template_id: Option<crate::Value<String>>,
        /// Property [`LaunchTemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-launchtemplatespecification.html#cfn-batch-computeenvironment-launchtemplatespecification-launchtemplatename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub launch_template_name: Option<crate::Value<String>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-launchtemplatespecification.html#cfn-batch-computeenvironment-launchtemplatespecification-version).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub version: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for LaunchTemplateSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref launch_template_id) = self.launch_template_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplateId", launch_template_id)?;
            }
            if let Some(ref launch_template_name) = self.launch_template_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplateName", launch_template_name)?;
            }
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for LaunchTemplateSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchTemplateSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LaunchTemplateSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LaunchTemplateSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut launch_template_id: Option<crate::Value<String>> = None;
                    let mut launch_template_name: Option<crate::Value<String>> = None;
                    let mut version: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LaunchTemplateId" => {
                                launch_template_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchTemplateName" => {
                                launch_template_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LaunchTemplateSpecification {
                        launch_template_id: launch_template_id,
                        launch_template_name: launch_template_name,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod job_definition {
    //! Property types for the `JobDefinition` resource.

    /// The [`AWS::Batch::JobDefinition.AuthorizationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-authorizationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthorizationConfig {
        /// Property [`AccessPointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-authorizationconfig.html#cfn-batch-jobdefinition-authorizationconfig-accesspointid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_point_id: Option<crate::Value<String>>,
        /// Property [`Iam`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-authorizationconfig.html#cfn-batch-jobdefinition-authorizationconfig-iam).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for AuthorizationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_point_id) = self.access_point_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessPointId", access_point_id)?;
            }
            if let Some(ref iam) = self.iam {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iam", iam)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for AuthorizationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthorizationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthorizationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthorizationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_point_id: Option<crate::Value<String>> = None;
                    let mut iam: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessPointId" => {
                                access_point_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Iam" => {
                                iam = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthorizationConfig {
                        access_point_id: access_point_id,
                        iam: iam,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.ContainerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ContainerProperties {
        /// Property [`Command`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-command).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub command: Option<crate::ValueList<String>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-environment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment: Option<crate::ValueList<Environment>>,
        /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-executionrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution_role_arn: Option<crate::Value<String>>,
        /// Property [`FargatePlatformConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-fargateplatformconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fargate_platform_configuration: Option<crate::Value<FargatePlatformConfiguration>>,
        /// Property [`Image`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-image).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image: crate::Value<String>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-instancetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: Option<crate::Value<String>>,
        /// Property [`JobRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-jobrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_role_arn: Option<crate::Value<String>>,
        /// Property [`LinuxParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-linuxparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub linux_parameters: Option<crate::Value<LinuxParameters>>,
        /// Property [`LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-logconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_configuration: Option<crate::Value<LogConfiguration>>,
        /// Property [`Memory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-memory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub memory: Option<crate::Value<u32>>,
        /// Property [`MountPoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-mountpoints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mount_points: Option<crate::ValueList<MountPoints>>,
        /// Property [`NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-networkconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_configuration: Option<crate::Value<NetworkConfiguration>>,
        /// Property [`Privileged`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-privileged).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub privileged: Option<crate::Value<bool>>,
        /// Property [`ReadonlyRootFilesystem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-readonlyrootfilesystem).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub readonly_root_filesystem: Option<crate::Value<bool>>,
        /// Property [`ResourceRequirements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-resourcerequirements).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_requirements: Option<crate::ValueList<ResourceRequirement>>,
        /// Property [`Secrets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-secrets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets: Option<crate::ValueList<Secret>>,
        /// Property [`Ulimits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-ulimits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ulimits: Option<crate::ValueList<Ulimit>>,
        /// Property [`User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-user).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user: Option<crate::Value<String>>,
        /// Property [`Vcpus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-vcpus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vcpus: Option<crate::Value<u32>>,
        /// Property [`Volumes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html#cfn-batch-jobdefinition-containerproperties-volumes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volumes: Option<crate::ValueList<Volumes>>,
    }

    impl crate::codec::SerializeValue for ContainerProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref command) = self.command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Command", command)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            if let Some(ref execution_role_arn) = self.execution_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", execution_role_arn)?;
            }
            if let Some(ref fargate_platform_configuration) = self.fargate_platform_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FargatePlatformConfiguration", fargate_platform_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", &self.image)?;
            if let Some(ref instance_type) = self.instance_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", instance_type)?;
            }
            if let Some(ref job_role_arn) = self.job_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobRoleArn", job_role_arn)?;
            }
            if let Some(ref linux_parameters) = self.linux_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LinuxParameters", linux_parameters)?;
            }
            if let Some(ref log_configuration) = self.log_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogConfiguration", log_configuration)?;
            }
            if let Some(ref memory) = self.memory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", memory)?;
            }
            if let Some(ref mount_points) = self.mount_points {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountPoints", mount_points)?;
            }
            if let Some(ref network_configuration) = self.network_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfiguration", network_configuration)?;
            }
            if let Some(ref privileged) = self.privileged {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Privileged", privileged)?;
            }
            if let Some(ref readonly_root_filesystem) = self.readonly_root_filesystem {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadonlyRootFilesystem", readonly_root_filesystem)?;
            }
            if let Some(ref resource_requirements) = self.resource_requirements {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceRequirements", resource_requirements)?;
            }
            if let Some(ref secrets) = self.secrets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Secrets", secrets)?;
            }
            if let Some(ref ulimits) = self.ulimits {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ulimits", ulimits)?;
            }
            if let Some(ref user) = self.user {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "User", user)?;
            }
            if let Some(ref vcpus) = self.vcpus {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Vcpus", vcpus)?;
            }
            if let Some(ref volumes) = self.volumes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Volumes", volumes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ContainerProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContainerProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContainerProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContainerProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut command: Option<crate::ValueList<String>> = None;
                    let mut environment: Option<crate::ValueList<Environment>> = None;
                    let mut execution_role_arn: Option<crate::Value<String>> = None;
                    let mut fargate_platform_configuration: Option<crate::Value<FargatePlatformConfiguration>> = None;
                    let mut image: Option<crate::Value<String>> = None;
                    let mut instance_type: Option<crate::Value<String>> = None;
                    let mut job_role_arn: Option<crate::Value<String>> = None;
                    let mut linux_parameters: Option<crate::Value<LinuxParameters>> = None;
                    let mut log_configuration: Option<crate::Value<LogConfiguration>> = None;
                    let mut memory: Option<crate::Value<u32>> = None;
                    let mut mount_points: Option<crate::ValueList<MountPoints>> = None;
                    let mut network_configuration: Option<crate::Value<NetworkConfiguration>> = None;
                    let mut privileged: Option<crate::Value<bool>> = None;
                    let mut readonly_root_filesystem: Option<crate::Value<bool>> = None;
                    let mut resource_requirements: Option<crate::ValueList<ResourceRequirement>> = None;
                    let mut secrets: Option<crate::ValueList<Secret>> = None;
                    let mut ulimits: Option<crate::ValueList<Ulimit>> = None;
                    let mut user: Option<crate::Value<String>> = None;
                    let mut vcpus: Option<crate::Value<u32>> = None;
                    let mut volumes: Option<crate::ValueList<Volumes>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Command" => {
                                command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExecutionRoleArn" => {
                                execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FargatePlatformConfiguration" => {
                                fargate_platform_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Image" => {
                                image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobRoleArn" => {
                                job_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LinuxParameters" => {
                                linux_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogConfiguration" => {
                                log_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Memory" => {
                                memory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MountPoints" => {
                                mount_points = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkConfiguration" => {
                                network_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Privileged" => {
                                privileged = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadonlyRootFilesystem" => {
                                readonly_root_filesystem = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceRequirements" => {
                                resource_requirements = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Secrets" => {
                                secrets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ulimits" => {
                                ulimits = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "User" => {
                                user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Vcpus" => {
                                vcpus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Volumes" => {
                                volumes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContainerProperties {
                        command: command,
                        environment: environment,
                        execution_role_arn: execution_role_arn,
                        fargate_platform_configuration: fargate_platform_configuration,
                        image: image.ok_or(::serde::de::Error::missing_field("Image"))?,
                        instance_type: instance_type,
                        job_role_arn: job_role_arn,
                        linux_parameters: linux_parameters,
                        log_configuration: log_configuration,
                        memory: memory,
                        mount_points: mount_points,
                        network_configuration: network_configuration,
                        privileged: privileged,
                        readonly_root_filesystem: readonly_root_filesystem,
                        resource_requirements: resource_requirements,
                        secrets: secrets,
                        ulimits: ulimits,
                        user: user,
                        vcpus: vcpus,
                        volumes: volumes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-device.html) property type.
    #[derive(Debug, Default)]
    pub struct Device {
        /// Property [`ContainerPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-device.html#cfn-batch-jobdefinition-device-containerpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_path: Option<crate::Value<String>>,
        /// Property [`HostPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-device.html#cfn-batch-jobdefinition-device-hostpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host_path: Option<crate::Value<String>>,
        /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-device.html#cfn-batch-jobdefinition-device-permissions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub permissions: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for Device {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_path) = self.container_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPath", container_path)?;
            }
            if let Some(ref host_path) = self.host_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostPath", host_path)?;
            }
            if let Some(ref permissions) = self.permissions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Device {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Device, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Device;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Device")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_path: Option<crate::Value<String>> = None;
                    let mut host_path: Option<crate::Value<String>> = None;
                    let mut permissions: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerPath" => {
                                container_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostPath" => {
                                host_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Permissions" => {
                                permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Device {
                        container_path: container_path,
                        host_path: host_path,
                        permissions: permissions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.EfsVolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-efsvolumeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct EfsVolumeConfiguration {
        /// Property [`AuthorizationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-efsvolumeconfiguration.html#cfn-batch-jobdefinition-efsvolumeconfiguration-authorizationconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authorization_config: Option<crate::Value<AuthorizationConfig>>,
        /// Property [`FileSystemId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-efsvolumeconfiguration.html#cfn-batch-jobdefinition-efsvolumeconfiguration-filesystemid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_system_id: crate::Value<String>,
        /// Property [`RootDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-efsvolumeconfiguration.html#cfn-batch-jobdefinition-efsvolumeconfiguration-rootdirectory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub root_directory: Option<crate::Value<String>>,
        /// Property [`TransitEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-efsvolumeconfiguration.html#cfn-batch-jobdefinition-efsvolumeconfiguration-transitencryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transit_encryption: Option<crate::Value<String>>,
        /// Property [`TransitEncryptionPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-efsvolumeconfiguration.html#cfn-batch-jobdefinition-efsvolumeconfiguration-transitencryptionport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transit_encryption_port: Option<crate::Value<u32>>,
    }

    impl crate::codec::SerializeValue for EfsVolumeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authorization_config) = self.authorization_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationConfig", authorization_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemId", &self.file_system_id)?;
            if let Some(ref root_directory) = self.root_directory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootDirectory", root_directory)?;
            }
            if let Some(ref transit_encryption) = self.transit_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitEncryption", transit_encryption)?;
            }
            if let Some(ref transit_encryption_port) = self.transit_encryption_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitEncryptionPort", transit_encryption_port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for EfsVolumeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EfsVolumeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EfsVolumeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EfsVolumeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authorization_config: Option<crate::Value<AuthorizationConfig>> = None;
                    let mut file_system_id: Option<crate::Value<String>> = None;
                    let mut root_directory: Option<crate::Value<String>> = None;
                    let mut transit_encryption: Option<crate::Value<String>> = None;
                    let mut transit_encryption_port: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthorizationConfig" => {
                                authorization_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileSystemId" => {
                                file_system_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RootDirectory" => {
                                root_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransitEncryption" => {
                                transit_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransitEncryptionPort" => {
                                transit_encryption_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EfsVolumeConfiguration {
                        authorization_config: authorization_config,
                        file_system_id: file_system_id.ok_or(::serde::de::Error::missing_field("FileSystemId"))?,
                        root_directory: root_directory,
                        transit_encryption: transit_encryption,
                        transit_encryption_port: transit_encryption_port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-environment.html) property type.
    #[derive(Debug, Default)]
    pub struct Environment {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-environment.html#cfn-batch-jobdefinition-environment-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<crate::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-environment.html#cfn-batch-jobdefinition-environment-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for Environment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
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
                    let mut name: Option<crate::Value<String>> = None;
                    let mut value: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Environment {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.EvaluateOnExit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-evaluateonexit.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluateOnExit {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-evaluateonexit.html#cfn-batch-jobdefinition-evaluateonexit-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: crate::Value<String>,
        /// Property [`OnExitCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-evaluateonexit.html#cfn-batch-jobdefinition-evaluateonexit-onexitcode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_exit_code: Option<crate::Value<String>>,
        /// Property [`OnReason`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-evaluateonexit.html#cfn-batch-jobdefinition-evaluateonexit-onreason).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_reason: Option<crate::Value<String>>,
        /// Property [`OnStatusReason`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-evaluateonexit.html#cfn-batch-jobdefinition-evaluateonexit-onstatusreason).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_status_reason: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for EvaluateOnExit {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            if let Some(ref on_exit_code) = self.on_exit_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnExitCode", on_exit_code)?;
            }
            if let Some(ref on_reason) = self.on_reason {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnReason", on_reason)?;
            }
            if let Some(ref on_status_reason) = self.on_status_reason {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnStatusReason", on_status_reason)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for EvaluateOnExit {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluateOnExit, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluateOnExit;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluateOnExit")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<crate::Value<String>> = None;
                    let mut on_exit_code: Option<crate::Value<String>> = None;
                    let mut on_reason: Option<crate::Value<String>> = None;
                    let mut on_status_reason: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnExitCode" => {
                                on_exit_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnReason" => {
                                on_reason = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnStatusReason" => {
                                on_status_reason = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluateOnExit {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        on_exit_code: on_exit_code,
                        on_reason: on_reason,
                        on_status_reason: on_status_reason,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.FargatePlatformConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-fargateplatformconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct FargatePlatformConfiguration {
        /// Property [`PlatformVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-fargateplatformconfiguration.html#cfn-batch-jobdefinition-containerproperties-fargateplatformconfiguration-platformversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub platform_version: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for FargatePlatformConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref platform_version) = self.platform_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformVersion", platform_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for FargatePlatformConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FargatePlatformConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FargatePlatformConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FargatePlatformConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut platform_version: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PlatformVersion" => {
                                platform_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FargatePlatformConfiguration {
                        platform_version: platform_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.LinuxParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-linuxparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct LinuxParameters {
        /// Property [`Devices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-linuxparameters.html#cfn-batch-jobdefinition-containerproperties-linuxparameters-devices).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub devices: Option<crate::ValueList<Device>>,
        /// Property [`InitProcessEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-linuxparameters.html#cfn-batch-jobdefinition-containerproperties-linuxparameters-initprocessenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub init_process_enabled: Option<crate::Value<bool>>,
        /// Property [`MaxSwap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-linuxparameters.html#cfn-batch-jobdefinition-containerproperties-linuxparameters-maxswap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_swap: Option<crate::Value<u32>>,
        /// Property [`SharedMemorySize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-linuxparameters.html#cfn-batch-jobdefinition-containerproperties-linuxparameters-sharedmemorysize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shared_memory_size: Option<crate::Value<u32>>,
        /// Property [`Swappiness`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-linuxparameters.html#cfn-batch-jobdefinition-containerproperties-linuxparameters-swappiness).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub swappiness: Option<crate::Value<u32>>,
        /// Property [`Tmpfs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-linuxparameters.html#cfn-batch-jobdefinition-containerproperties-linuxparameters-tmpfs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tmpfs: Option<crate::ValueList<Tmpfs>>,
    }

    impl crate::codec::SerializeValue for LinuxParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref devices) = self.devices {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Devices", devices)?;
            }
            if let Some(ref init_process_enabled) = self.init_process_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitProcessEnabled", init_process_enabled)?;
            }
            if let Some(ref max_swap) = self.max_swap {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSwap", max_swap)?;
            }
            if let Some(ref shared_memory_size) = self.shared_memory_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SharedMemorySize", shared_memory_size)?;
            }
            if let Some(ref swappiness) = self.swappiness {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Swappiness", swappiness)?;
            }
            if let Some(ref tmpfs) = self.tmpfs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tmpfs", tmpfs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for LinuxParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LinuxParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LinuxParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LinuxParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut devices: Option<crate::ValueList<Device>> = None;
                    let mut init_process_enabled: Option<crate::Value<bool>> = None;
                    let mut max_swap: Option<crate::Value<u32>> = None;
                    let mut shared_memory_size: Option<crate::Value<u32>> = None;
                    let mut swappiness: Option<crate::Value<u32>> = None;
                    let mut tmpfs: Option<crate::ValueList<Tmpfs>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Devices" => {
                                devices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InitProcessEnabled" => {
                                init_process_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxSwap" => {
                                max_swap = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SharedMemorySize" => {
                                shared_memory_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Swappiness" => {
                                swappiness = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tmpfs" => {
                                tmpfs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LinuxParameters {
                        devices: devices,
                        init_process_enabled: init_process_enabled,
                        max_swap: max_swap,
                        shared_memory_size: shared_memory_size,
                        swappiness: swappiness,
                        tmpfs: tmpfs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-logconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LogConfiguration {
        /// Property [`LogDriver`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-logconfiguration.html#cfn-batch-jobdefinition-containerproperties-logconfiguration-logdriver).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_driver: crate::Value<String>,
        /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-logconfiguration.html#cfn-batch-jobdefinition-containerproperties-logconfiguration-options).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub options: Option<crate::Value<crate::json::Value>>,
        /// Property [`SecretOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-logconfiguration.html#cfn-batch-jobdefinition-containerproperties-logconfiguration-secretoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_options: Option<crate::ValueList<Secret>>,
    }

    impl crate::codec::SerializeValue for LogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDriver", &self.log_driver)?;
            if let Some(ref options) = self.options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", options)?;
            }
            if let Some(ref secret_options) = self.secret_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretOptions", secret_options)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for LogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_driver: Option<crate::Value<String>> = None;
                    let mut options: Option<crate::Value<crate::json::Value>> = None;
                    let mut secret_options: Option<crate::ValueList<Secret>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogDriver" => {
                                log_driver = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Options" => {
                                options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretOptions" => {
                                secret_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogConfiguration {
                        log_driver: log_driver.ok_or(::serde::de::Error::missing_field("LogDriver"))?,
                        options: options,
                        secret_options: secret_options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.MountPoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-mountpoints.html) property type.
    #[derive(Debug, Default)]
    pub struct MountPoints {
        /// Property [`ContainerPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-mountpoints.html#cfn-batch-jobdefinition-mountpoints-containerpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_path: Option<crate::Value<String>>,
        /// Property [`ReadOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-mountpoints.html#cfn-batch-jobdefinition-mountpoints-readonly).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_only: Option<crate::Value<bool>>,
        /// Property [`SourceVolume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-mountpoints.html#cfn-batch-jobdefinition-mountpoints-sourcevolume).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_volume: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for MountPoints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_path) = self.container_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPath", container_path)?;
            }
            if let Some(ref read_only) = self.read_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadOnly", read_only)?;
            }
            if let Some(ref source_volume) = self.source_volume {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceVolume", source_volume)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for MountPoints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MountPoints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MountPoints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MountPoints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_path: Option<crate::Value<String>> = None;
                    let mut read_only: Option<crate::Value<bool>> = None;
                    let mut source_volume: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerPath" => {
                                container_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadOnly" => {
                                read_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceVolume" => {
                                source_volume = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MountPoints {
                        container_path: container_path,
                        read_only: read_only,
                        source_volume: source_volume,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-networkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfiguration {
        /// Property [`AssignPublicIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties-networkconfiguration.html#cfn-batch-jobdefinition-containerproperties-networkconfiguration-assignpublicip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub assign_public_ip: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for NetworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref assign_public_ip) = self.assign_public_ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssignPublicIp", assign_public_ip)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for NetworkConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut assign_public_ip: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssignPublicIp" => {
                                assign_public_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfiguration {
                        assign_public_ip: assign_public_ip,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.NodeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-nodeproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct NodeProperties {
        /// Property [`MainNode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-nodeproperties.html#cfn-batch-jobdefinition-nodeproperties-mainnode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub main_node: crate::Value<u32>,
        /// Property [`NodeRangeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-nodeproperties.html#cfn-batch-jobdefinition-nodeproperties-noderangeproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub node_range_properties: crate::ValueList<NodeRangeProperty>,
        /// Property [`NumNodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-nodeproperties.html#cfn-batch-jobdefinition-nodeproperties-numnodes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_nodes: crate::Value<u32>,
    }

    impl crate::codec::SerializeValue for NodeProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MainNode", &self.main_node)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeRangeProperties", &self.node_range_properties)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumNodes", &self.num_nodes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for NodeProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NodeProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NodeProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NodeProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut main_node: Option<crate::Value<u32>> = None;
                    let mut node_range_properties: Option<crate::ValueList<NodeRangeProperty>> = None;
                    let mut num_nodes: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MainNode" => {
                                main_node = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NodeRangeProperties" => {
                                node_range_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumNodes" => {
                                num_nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NodeProperties {
                        main_node: main_node.ok_or(::serde::de::Error::missing_field("MainNode"))?,
                        node_range_properties: node_range_properties.ok_or(::serde::de::Error::missing_field("NodeRangeProperties"))?,
                        num_nodes: num_nodes.ok_or(::serde::de::Error::missing_field("NumNodes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.NodeRangeProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-noderangeproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct NodeRangeProperty {
        /// Property [`Container`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-noderangeproperty.html#cfn-batch-jobdefinition-noderangeproperty-container).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container: Option<crate::Value<ContainerProperties>>,
        /// Property [`TargetNodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-noderangeproperty.html#cfn-batch-jobdefinition-noderangeproperty-targetnodes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_nodes: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for NodeRangeProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container) = self.container {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Container", container)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetNodes", &self.target_nodes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for NodeRangeProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NodeRangeProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NodeRangeProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NodeRangeProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container: Option<crate::Value<ContainerProperties>> = None;
                    let mut target_nodes: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Container" => {
                                container = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetNodes" => {
                                target_nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NodeRangeProperty {
                        container: container,
                        target_nodes: target_nodes.ok_or(::serde::de::Error::missing_field("TargetNodes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.ResourceRequirement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-resourcerequirement.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceRequirement {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-resourcerequirement.html#cfn-batch-jobdefinition-resourcerequirement-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<crate::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-resourcerequirement.html#cfn-batch-jobdefinition-resourcerequirement-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for ResourceRequirement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ResourceRequirement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceRequirement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceRequirement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceRequirement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<crate::Value<String>> = None;
                    let mut value: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceRequirement {
                        r#type: r#type,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.RetryStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-retrystrategy.html) property type.
    #[derive(Debug, Default)]
    pub struct RetryStrategy {
        /// Property [`Attempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-retrystrategy.html#cfn-batch-jobdefinition-retrystrategy-attempts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attempts: Option<crate::Value<u32>>,
        /// Property [`EvaluateOnExit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-retrystrategy.html#cfn-batch-jobdefinition-retrystrategy-evaluateonexit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub evaluate_on_exit: Option<crate::ValueList<EvaluateOnExit>>,
    }

    impl crate::codec::SerializeValue for RetryStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attempts) = self.attempts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attempts", attempts)?;
            }
            if let Some(ref evaluate_on_exit) = self.evaluate_on_exit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluateOnExit", evaluate_on_exit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for RetryStrategy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RetryStrategy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RetryStrategy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RetryStrategy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attempts: Option<crate::Value<u32>> = None;
                    let mut evaluate_on_exit: Option<crate::ValueList<EvaluateOnExit>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attempts" => {
                                attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EvaluateOnExit" => {
                                evaluate_on_exit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RetryStrategy {
                        attempts: attempts,
                        evaluate_on_exit: evaluate_on_exit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.Secret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-secret.html) property type.
    #[derive(Debug, Default)]
    pub struct Secret {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-secret.html#cfn-batch-jobdefinition-secret-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: crate::Value<String>,
        /// Property [`ValueFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-secret.html#cfn-batch-jobdefinition-secret-valuefrom).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value_from: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for Secret {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValueFrom", &self.value_from)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Secret {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Secret, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Secret;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Secret")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<crate::Value<String>> = None;
                    let mut value_from: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValueFrom" => {
                                value_from = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Secret {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value_from: value_from.ok_or(::serde::de::Error::missing_field("ValueFrom"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-timeout.html) property type.
    #[derive(Debug, Default)]
    pub struct Timeout {
        /// Property [`AttemptDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-timeout.html#cfn-batch-jobdefinition-timeout-attemptdurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attempt_duration_seconds: Option<crate::Value<u32>>,
    }

    impl crate::codec::SerializeValue for Timeout {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attempt_duration_seconds) = self.attempt_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttemptDurationSeconds", attempt_duration_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Timeout {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Timeout, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Timeout;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Timeout")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attempt_duration_seconds: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttemptDurationSeconds" => {
                                attempt_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Timeout {
                        attempt_duration_seconds: attempt_duration_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.Tmpfs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-tmpfs.html) property type.
    #[derive(Debug, Default)]
    pub struct Tmpfs {
        /// Property [`ContainerPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-tmpfs.html#cfn-batch-jobdefinition-tmpfs-containerpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_path: crate::Value<String>,
        /// Property [`MountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-tmpfs.html#cfn-batch-jobdefinition-tmpfs-mountoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mount_options: Option<crate::ValueList<String>>,
        /// Property [`Size`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-tmpfs.html#cfn-batch-jobdefinition-tmpfs-size).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size: crate::Value<u32>,
    }

    impl crate::codec::SerializeValue for Tmpfs {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPath", &self.container_path)?;
            if let Some(ref mount_options) = self.mount_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountOptions", mount_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", &self.size)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Tmpfs {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Tmpfs, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Tmpfs;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Tmpfs")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_path: Option<crate::Value<String>> = None;
                    let mut mount_options: Option<crate::ValueList<String>> = None;
                    let mut size: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerPath" => {
                                container_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MountOptions" => {
                                mount_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Size" => {
                                size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Tmpfs {
                        container_path: container_path.ok_or(::serde::de::Error::missing_field("ContainerPath"))?,
                        mount_options: mount_options,
                        size: size.ok_or(::serde::de::Error::missing_field("Size"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.Ulimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ulimit.html) property type.
    #[derive(Debug, Default)]
    pub struct Ulimit {
        /// Property [`HardLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ulimit.html#cfn-batch-jobdefinition-ulimit-hardlimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hard_limit: crate::Value<u32>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ulimit.html#cfn-batch-jobdefinition-ulimit-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: crate::Value<String>,
        /// Property [`SoftLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ulimit.html#cfn-batch-jobdefinition-ulimit-softlimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub soft_limit: crate::Value<u32>,
    }

    impl crate::codec::SerializeValue for Ulimit {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HardLimit", &self.hard_limit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SoftLimit", &self.soft_limit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Ulimit {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Ulimit, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Ulimit;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Ulimit")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hard_limit: Option<crate::Value<u32>> = None;
                    let mut name: Option<crate::Value<String>> = None;
                    let mut soft_limit: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HardLimit" => {
                                hard_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SoftLimit" => {
                                soft_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Ulimit {
                        hard_limit: hard_limit.ok_or(::serde::de::Error::missing_field("HardLimit"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        soft_limit: soft_limit.ok_or(::serde::de::Error::missing_field("SoftLimit"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.Volumes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumes.html) property type.
    #[derive(Debug, Default)]
    pub struct Volumes {
        /// Property [`EfsVolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumes.html#cfn-batch-jobdefinition-volumes-efsvolumeconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub efs_volume_configuration: Option<crate::Value<EfsVolumeConfiguration>>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumes.html#cfn-batch-jobdefinition-volumes-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: Option<crate::Value<VolumesHost>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumes.html#cfn-batch-jobdefinition-volumes-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for Volumes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref efs_volume_configuration) = self.efs_volume_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EfsVolumeConfiguration", efs_volume_configuration)?;
            }
            if let Some(ref host) = self.host {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", host)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Volumes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Volumes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Volumes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Volumes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut efs_volume_configuration: Option<crate::Value<EfsVolumeConfiguration>> = None;
                    let mut host: Option<crate::Value<VolumesHost>> = None;
                    let mut name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EfsVolumeConfiguration" => {
                                efs_volume_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Volumes {
                        efs_volume_configuration: efs_volume_configuration,
                        host: host,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.VolumesHost`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumeshost.html) property type.
    #[derive(Debug, Default)]
    pub struct VolumesHost {
        /// Property [`SourcePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumeshost.html#cfn-batch-jobdefinition-volumeshost-sourcepath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_path: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for VolumesHost {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref source_path) = self.source_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePath", source_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for VolumesHost {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VolumesHost, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VolumesHost;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VolumesHost")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source_path: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourcePath" => {
                                source_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VolumesHost {
                        source_path: source_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod job_queue {
    //! Property types for the `JobQueue` resource.

    /// The [`AWS::Batch::JobQueue.ComputeEnvironmentOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobqueue-computeenvironmentorder.html) property type.
    #[derive(Debug, Default)]
    pub struct ComputeEnvironmentOrder {
        /// Property [`ComputeEnvironment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobqueue-computeenvironmentorder.html#cfn-batch-jobqueue-computeenvironmentorder-computeenvironment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compute_environment: crate::Value<String>,
        /// Property [`Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobqueue-computeenvironmentorder.html#cfn-batch-jobqueue-computeenvironmentorder-order).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub order: crate::Value<u32>,
    }

    impl crate::codec::SerializeValue for ComputeEnvironmentOrder {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeEnvironment", &self.compute_environment)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Order", &self.order)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ComputeEnvironmentOrder {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComputeEnvironmentOrder, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComputeEnvironmentOrder;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComputeEnvironmentOrder")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut compute_environment: Option<crate::Value<String>> = None;
                    let mut order: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComputeEnvironment" => {
                                compute_environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Order" => {
                                order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComputeEnvironmentOrder {
                        compute_environment: compute_environment.ok_or(::serde::de::Error::missing_field("ComputeEnvironment"))?,
                        order: order.ok_or(::serde::de::Error::missing_field("Order"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
