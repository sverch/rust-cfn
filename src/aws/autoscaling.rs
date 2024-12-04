//! Types for the `AutoScaling` service.

/// The [`AWS::AutoScaling::AutoScalingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html) resource type.
#[derive(Debug, Default)]
pub struct AutoScalingGroup {
    properties: AutoScalingGroupProperties
}

/// Properties for the `AutoScalingGroup` resource.
#[derive(Debug, Default)]
pub struct AutoScalingGroupProperties {
    /// Property [`AutoScalingGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-autoscaling-autoscalinggroup-autoscalinggroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub auto_scaling_group_name: Option<crate::Value<String>>,
    /// Property [`AvailabilityZones`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-availabilityzones).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub availability_zones: Option<crate::ValueList<String>>,
    /// Property [`CapacityRebalance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-capacityrebalance).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub capacity_rebalance: Option<crate::Value<bool>>,
    /// Property [`Context`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-context).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub context: Option<crate::Value<String>>,
    /// Property [`Cooldown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-cooldown).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cooldown: Option<crate::Value<String>>,
    /// Property [`DesiredCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-desiredcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub desired_capacity: Option<crate::Value<String>>,
    /// Property [`HealthCheckGracePeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-healthcheckgraceperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_grace_period: Option<crate::Value<u32>>,
    /// Property [`HealthCheckType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-healthchecktype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_type: Option<crate::Value<String>>,
    /// Property [`InstanceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-instanceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_id: Option<crate::Value<String>>,
    /// Property [`LaunchConfigurationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-launchconfigurationname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub launch_configuration_name: Option<crate::Value<String>>,
    /// Property [`LaunchTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-launchtemplate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub launch_template: Option<crate::Value<self::auto_scaling_group::LaunchTemplateSpecification>>,
    /// Property [`LifecycleHookSpecificationList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-autoscaling-autoscalinggroup-lifecyclehookspecificationlist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lifecycle_hook_specification_list: Option<crate::ValueList<self::auto_scaling_group::LifecycleHookSpecification>>,
    /// Property [`LoadBalancerNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-loadbalancernames).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub load_balancer_names: Option<crate::ValueList<String>>,
    /// Property [`MaxInstanceLifetime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-maxinstancelifetime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_instance_lifetime: Option<crate::Value<u32>>,
    /// Property [`MaxSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-maxsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_size: crate::Value<String>,
    /// Property [`MetricsCollection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-metricscollection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metrics_collection: Option<crate::ValueList<self::auto_scaling_group::MetricsCollection>>,
    /// Property [`MinSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-minsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_size: crate::Value<String>,
    /// Property [`MixedInstancesPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-mixedinstancespolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mixed_instances_policy: Option<crate::Value<self::auto_scaling_group::MixedInstancesPolicy>>,
    /// Property [`NewInstancesProtectedFromScaleIn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-newinstancesprotectedfromscalein).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub new_instances_protected_from_scale_in: Option<crate::Value<bool>>,
    /// Property [`NotificationConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-notificationconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_configurations: Option<crate::ValueList<self::auto_scaling_group::NotificationConfiguration>>,
    /// Property [`PlacementGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-placementgroup).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub placement_group: Option<crate::Value<String>>,
    /// Property [`ServiceLinkedRoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-autoscaling-autoscalinggroup-servicelinkedrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub service_linked_role_arn: Option<crate::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<self::auto_scaling_group::TagProperty>>,
    /// Property [`TargetGroupARNs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-targetgrouparns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_group_ar_ns: Option<crate::ValueList<String>>,
    /// Property [`TerminationPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-termpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub termination_policies: Option<crate::ValueList<String>>,
    /// Property [`VPCZoneIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html#cfn-as-group-vpczoneidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_zone_identifier: Option<crate::ValueList<String>>,
}

impl ::serde::Serialize for AutoScalingGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_scaling_group_name) = self.auto_scaling_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroupName", auto_scaling_group_name)?;
        }
        if let Some(ref availability_zones) = self.availability_zones {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZones", availability_zones)?;
        }
        if let Some(ref capacity_rebalance) = self.capacity_rebalance {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityRebalance", capacity_rebalance)?;
        }
        if let Some(ref context) = self.context {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Context", context)?;
        }
        if let Some(ref cooldown) = self.cooldown {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cooldown", cooldown)?;
        }
        if let Some(ref desired_capacity) = self.desired_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredCapacity", desired_capacity)?;
        }
        if let Some(ref health_check_grace_period) = self.health_check_grace_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckGracePeriod", health_check_grace_period)?;
        }
        if let Some(ref health_check_type) = self.health_check_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckType", health_check_type)?;
        }
        if let Some(ref instance_id) = self.instance_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", instance_id)?;
        }
        if let Some(ref launch_configuration_name) = self.launch_configuration_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchConfigurationName", launch_configuration_name)?;
        }
        if let Some(ref launch_template) = self.launch_template {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplate", launch_template)?;
        }
        if let Some(ref lifecycle_hook_specification_list) = self.lifecycle_hook_specification_list {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleHookSpecificationList", lifecycle_hook_specification_list)?;
        }
        if let Some(ref load_balancer_names) = self.load_balancer_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerNames", load_balancer_names)?;
        }
        if let Some(ref max_instance_lifetime) = self.max_instance_lifetime {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxInstanceLifetime", max_instance_lifetime)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSize", &self.max_size)?;
        if let Some(ref metrics_collection) = self.metrics_collection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricsCollection", metrics_collection)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSize", &self.min_size)?;
        if let Some(ref mixed_instances_policy) = self.mixed_instances_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MixedInstancesPolicy", mixed_instances_policy)?;
        }
        if let Some(ref new_instances_protected_from_scale_in) = self.new_instances_protected_from_scale_in {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NewInstancesProtectedFromScaleIn", new_instances_protected_from_scale_in)?;
        }
        if let Some(ref notification_configurations) = self.notification_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationConfigurations", notification_configurations)?;
        }
        if let Some(ref placement_group) = self.placement_group {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementGroup", placement_group)?;
        }
        if let Some(ref service_linked_role_arn) = self.service_linked_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceLinkedRoleARN", service_linked_role_arn)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target_group_ar_ns) = self.target_group_ar_ns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupARNs", target_group_ar_ns)?;
        }
        if let Some(ref termination_policies) = self.termination_policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TerminationPolicies", termination_policies)?;
        }
        if let Some(ref vpc_zone_identifier) = self.vpc_zone_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCZoneIdentifier", vpc_zone_identifier)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AutoScalingGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoScalingGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AutoScalingGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AutoScalingGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_scaling_group_name: Option<crate::Value<String>> = None;
                let mut availability_zones: Option<crate::ValueList<String>> = None;
                let mut capacity_rebalance: Option<crate::Value<bool>> = None;
                let mut context: Option<crate::Value<String>> = None;
                let mut cooldown: Option<crate::Value<String>> = None;
                let mut desired_capacity: Option<crate::Value<String>> = None;
                let mut health_check_grace_period: Option<crate::Value<u32>> = None;
                let mut health_check_type: Option<crate::Value<String>> = None;
                let mut instance_id: Option<crate::Value<String>> = None;
                let mut launch_configuration_name: Option<crate::Value<String>> = None;
                let mut launch_template: Option<crate::Value<self::auto_scaling_group::LaunchTemplateSpecification>> = None;
                let mut lifecycle_hook_specification_list: Option<crate::ValueList<self::auto_scaling_group::LifecycleHookSpecification>> = None;
                let mut load_balancer_names: Option<crate::ValueList<String>> = None;
                let mut max_instance_lifetime: Option<crate::Value<u32>> = None;
                let mut max_size: Option<crate::Value<String>> = None;
                let mut metrics_collection: Option<crate::ValueList<self::auto_scaling_group::MetricsCollection>> = None;
                let mut min_size: Option<crate::Value<String>> = None;
                let mut mixed_instances_policy: Option<crate::Value<self::auto_scaling_group::MixedInstancesPolicy>> = None;
                let mut new_instances_protected_from_scale_in: Option<crate::Value<bool>> = None;
                let mut notification_configurations: Option<crate::ValueList<self::auto_scaling_group::NotificationConfiguration>> = None;
                let mut placement_group: Option<crate::Value<String>> = None;
                let mut service_linked_role_arn: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<self::auto_scaling_group::TagProperty>> = None;
                let mut target_group_ar_ns: Option<crate::ValueList<String>> = None;
                let mut termination_policies: Option<crate::ValueList<String>> = None;
                let mut vpc_zone_identifier: Option<crate::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingGroupName" => {
                            auto_scaling_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZones" => {
                            availability_zones = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CapacityRebalance" => {
                            capacity_rebalance = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Context" => {
                            context = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cooldown" => {
                            cooldown = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DesiredCapacity" => {
                            desired_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckGracePeriod" => {
                            health_check_grace_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckType" => {
                            health_check_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceId" => {
                            instance_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LaunchConfigurationName" => {
                            launch_configuration_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LaunchTemplate" => {
                            launch_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LifecycleHookSpecificationList" => {
                            lifecycle_hook_specification_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoadBalancerNames" => {
                            load_balancer_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxInstanceLifetime" => {
                            max_instance_lifetime = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxSize" => {
                            max_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricsCollection" => {
                            metrics_collection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinSize" => {
                            min_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MixedInstancesPolicy" => {
                            mixed_instances_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NewInstancesProtectedFromScaleIn" => {
                            new_instances_protected_from_scale_in = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationConfigurations" => {
                            notification_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlacementGroup" => {
                            placement_group = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceLinkedRoleARN" => {
                            service_linked_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetGroupARNs" => {
                            target_group_ar_ns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TerminationPolicies" => {
                            termination_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VPCZoneIdentifier" => {
                            vpc_zone_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AutoScalingGroupProperties {
                    auto_scaling_group_name: auto_scaling_group_name,
                    availability_zones: availability_zones,
                    capacity_rebalance: capacity_rebalance,
                    context: context,
                    cooldown: cooldown,
                    desired_capacity: desired_capacity,
                    health_check_grace_period: health_check_grace_period,
                    health_check_type: health_check_type,
                    instance_id: instance_id,
                    launch_configuration_name: launch_configuration_name,
                    launch_template: launch_template,
                    lifecycle_hook_specification_list: lifecycle_hook_specification_list,
                    load_balancer_names: load_balancer_names,
                    max_instance_lifetime: max_instance_lifetime,
                    max_size: max_size.ok_or(::serde::de::Error::missing_field("MaxSize"))?,
                    metrics_collection: metrics_collection,
                    min_size: min_size.ok_or(::serde::de::Error::missing_field("MinSize"))?,
                    mixed_instances_policy: mixed_instances_policy,
                    new_instances_protected_from_scale_in: new_instances_protected_from_scale_in,
                    notification_configurations: notification_configurations,
                    placement_group: placement_group,
                    service_linked_role_arn: service_linked_role_arn,
                    tags: tags,
                    target_group_ar_ns: target_group_ar_ns,
                    termination_policies: termination_policies,
                    vpc_zone_identifier: vpc_zone_identifier,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for AutoScalingGroup {
    type Properties = AutoScalingGroupProperties;
    const TYPE: &'static str = "AWS::AutoScaling::AutoScalingGroup";
    fn properties(&self) -> &AutoScalingGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AutoScalingGroupProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for AutoScalingGroup {}

impl From<AutoScalingGroupProperties> for AutoScalingGroup {
    fn from(properties: AutoScalingGroupProperties) -> AutoScalingGroup {
        AutoScalingGroup { properties }
    }
}

/// The [`AWS::AutoScaling::LaunchConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html) resource type.
#[derive(Debug, Default)]
pub struct LaunchConfiguration {
    properties: LaunchConfigurationProperties
}

/// Properties for the `LaunchConfiguration` resource.
#[derive(Debug, Default)]
pub struct LaunchConfigurationProperties {
    /// Property [`AssociatePublicIpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cf-as-launchconfig-associatepubip).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub associate_public_ip_address: Option<crate::Value<bool>>,
    /// Property [`BlockDeviceMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-blockdevicemappings).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub block_device_mappings: Option<crate::ValueList<self::launch_configuration::BlockDeviceMapping>>,
    /// Property [`ClassicLinkVPCId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-classiclinkvpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub classic_link_vpc_id: Option<crate::Value<String>>,
    /// Property [`ClassicLinkVPCSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-classiclinkvpcsecuritygroups).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub classic_link_vpc_security_groups: Option<crate::ValueList<String>>,
    /// Property [`EbsOptimized`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-ebsoptimized).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ebs_optimized: Option<crate::Value<bool>>,
    /// Property [`IamInstanceProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-iaminstanceprofile).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub iam_instance_profile: Option<crate::Value<String>>,
    /// Property [`ImageId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-imageid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub image_id: crate::Value<String>,
    /// Property [`InstanceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-instanceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_id: Option<crate::Value<String>>,
    /// Property [`InstanceMonitoring`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-instancemonitoring).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_monitoring: Option<crate::Value<bool>>,
    /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-instancetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_type: crate::Value<String>,
    /// Property [`KernelId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-kernelid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kernel_id: Option<crate::Value<String>>,
    /// Property [`KeyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-keyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key_name: Option<crate::Value<String>>,
    /// Property [`LaunchConfigurationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-autoscaling-launchconfig-launchconfigurationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub launch_configuration_name: Option<crate::Value<String>>,
    /// Property [`MetadataOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-autoscaling-launchconfig-metadataoptions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub metadata_options: Option<crate::Value<self::launch_configuration::MetadataOptions>>,
    /// Property [`PlacementTenancy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-placementtenancy).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub placement_tenancy: Option<crate::Value<String>>,
    /// Property [`RamDiskId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-ramdiskid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ram_disk_id: Option<crate::Value<String>>,
    /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-securitygroups).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_groups: Option<crate::ValueList<String>>,
    /// Property [`SpotPrice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-spotprice).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub spot_price: Option<crate::Value<String>>,
    /// Property [`UserData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html#cfn-as-launchconfig-userdata).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_data: Option<crate::Value<String>>,
}

impl ::serde::Serialize for LaunchConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref associate_public_ip_address) = self.associate_public_ip_address {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatePublicIpAddress", associate_public_ip_address)?;
        }
        if let Some(ref block_device_mappings) = self.block_device_mappings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDeviceMappings", block_device_mappings)?;
        }
        if let Some(ref classic_link_vpc_id) = self.classic_link_vpc_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClassicLinkVPCId", classic_link_vpc_id)?;
        }
        if let Some(ref classic_link_vpc_security_groups) = self.classic_link_vpc_security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClassicLinkVPCSecurityGroups", classic_link_vpc_security_groups)?;
        }
        if let Some(ref ebs_optimized) = self.ebs_optimized {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", ebs_optimized)?;
        }
        if let Some(ref iam_instance_profile) = self.iam_instance_profile {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamInstanceProfile", iam_instance_profile)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageId", &self.image_id)?;
        if let Some(ref instance_id) = self.instance_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", instance_id)?;
        }
        if let Some(ref instance_monitoring) = self.instance_monitoring {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceMonitoring", instance_monitoring)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        if let Some(ref kernel_id) = self.kernel_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KernelId", kernel_id)?;
        }
        if let Some(ref key_name) = self.key_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyName", key_name)?;
        }
        if let Some(ref launch_configuration_name) = self.launch_configuration_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchConfigurationName", launch_configuration_name)?;
        }
        if let Some(ref metadata_options) = self.metadata_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetadataOptions", metadata_options)?;
        }
        if let Some(ref placement_tenancy) = self.placement_tenancy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementTenancy", placement_tenancy)?;
        }
        if let Some(ref ram_disk_id) = self.ram_disk_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RamDiskId", ram_disk_id)?;
        }
        if let Some(ref security_groups) = self.security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
        }
        if let Some(ref spot_price) = self.spot_price {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotPrice", spot_price)?;
        }
        if let Some(ref user_data) = self.user_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserData", user_data)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LaunchConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LaunchConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LaunchConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut associate_public_ip_address: Option<crate::Value<bool>> = None;
                let mut block_device_mappings: Option<crate::ValueList<self::launch_configuration::BlockDeviceMapping>> = None;
                let mut classic_link_vpc_id: Option<crate::Value<String>> = None;
                let mut classic_link_vpc_security_groups: Option<crate::ValueList<String>> = None;
                let mut ebs_optimized: Option<crate::Value<bool>> = None;
                let mut iam_instance_profile: Option<crate::Value<String>> = None;
                let mut image_id: Option<crate::Value<String>> = None;
                let mut instance_id: Option<crate::Value<String>> = None;
                let mut instance_monitoring: Option<crate::Value<bool>> = None;
                let mut instance_type: Option<crate::Value<String>> = None;
                let mut kernel_id: Option<crate::Value<String>> = None;
                let mut key_name: Option<crate::Value<String>> = None;
                let mut launch_configuration_name: Option<crate::Value<String>> = None;
                let mut metadata_options: Option<crate::Value<self::launch_configuration::MetadataOptions>> = None;
                let mut placement_tenancy: Option<crate::Value<String>> = None;
                let mut ram_disk_id: Option<crate::Value<String>> = None;
                let mut security_groups: Option<crate::ValueList<String>> = None;
                let mut spot_price: Option<crate::Value<String>> = None;
                let mut user_data: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssociatePublicIpAddress" => {
                            associate_public_ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BlockDeviceMappings" => {
                            block_device_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClassicLinkVPCId" => {
                            classic_link_vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClassicLinkVPCSecurityGroups" => {
                            classic_link_vpc_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EbsOptimized" => {
                            ebs_optimized = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IamInstanceProfile" => {
                            iam_instance_profile = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageId" => {
                            image_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceId" => {
                            instance_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceMonitoring" => {
                            instance_monitoring = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceType" => {
                            instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KernelId" => {
                            kernel_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyName" => {
                            key_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LaunchConfigurationName" => {
                            launch_configuration_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetadataOptions" => {
                            metadata_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlacementTenancy" => {
                            placement_tenancy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RamDiskId" => {
                            ram_disk_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroups" => {
                            security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SpotPrice" => {
                            spot_price = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserData" => {
                            user_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LaunchConfigurationProperties {
                    associate_public_ip_address: associate_public_ip_address,
                    block_device_mappings: block_device_mappings,
                    classic_link_vpc_id: classic_link_vpc_id,
                    classic_link_vpc_security_groups: classic_link_vpc_security_groups,
                    ebs_optimized: ebs_optimized,
                    iam_instance_profile: iam_instance_profile,
                    image_id: image_id.ok_or(::serde::de::Error::missing_field("ImageId"))?,
                    instance_id: instance_id,
                    instance_monitoring: instance_monitoring,
                    instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                    kernel_id: kernel_id,
                    key_name: key_name,
                    launch_configuration_name: launch_configuration_name,
                    metadata_options: metadata_options,
                    placement_tenancy: placement_tenancy,
                    ram_disk_id: ram_disk_id,
                    security_groups: security_groups,
                    spot_price: spot_price,
                    user_data: user_data,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for LaunchConfiguration {
    type Properties = LaunchConfigurationProperties;
    const TYPE: &'static str = "AWS::AutoScaling::LaunchConfiguration";
    fn properties(&self) -> &LaunchConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LaunchConfigurationProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for LaunchConfiguration {}

impl From<LaunchConfigurationProperties> for LaunchConfiguration {
    fn from(properties: LaunchConfigurationProperties) -> LaunchConfiguration {
        LaunchConfiguration { properties }
    }
}

/// The [`AWS::AutoScaling::LifecycleHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-lifecyclehook.html) resource type.
#[derive(Debug, Default)]
pub struct LifecycleHook {
    properties: LifecycleHookProperties
}

/// Properties for the `LifecycleHook` resource.
#[derive(Debug, Default)]
pub struct LifecycleHookProperties {
    /// Property [`AutoScalingGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-lifecyclehook.html#cfn-as-lifecyclehook-autoscalinggroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub auto_scaling_group_name: crate::Value<String>,
    /// Property [`DefaultResult`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-lifecyclehook.html#cfn-as-lifecyclehook-defaultresult).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_result: Option<crate::Value<String>>,
    /// Property [`HeartbeatTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-lifecyclehook.html#cfn-as-lifecyclehook-heartbeattimeout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub heartbeat_timeout: Option<crate::Value<u32>>,
    /// Property [`LifecycleHookName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-lifecyclehook.html#cfn-autoscaling-lifecyclehook-lifecyclehookname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub lifecycle_hook_name: Option<crate::Value<String>>,
    /// Property [`LifecycleTransition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-lifecyclehook.html#cfn-as-lifecyclehook-lifecycletransition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lifecycle_transition: crate::Value<String>,
    /// Property [`NotificationMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-lifecyclehook.html#cfn-as-lifecyclehook-notificationmetadata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_metadata: Option<crate::Value<String>>,
    /// Property [`NotificationTargetARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-lifecyclehook.html#cfn-as-lifecyclehook-notificationtargetarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_target_arn: Option<crate::Value<String>>,
    /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-lifecyclehook.html#cfn-as-lifecyclehook-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<crate::Value<String>>,
}

impl ::serde::Serialize for LifecycleHookProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroupName", &self.auto_scaling_group_name)?;
        if let Some(ref default_result) = self.default_result {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultResult", default_result)?;
        }
        if let Some(ref heartbeat_timeout) = self.heartbeat_timeout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeartbeatTimeout", heartbeat_timeout)?;
        }
        if let Some(ref lifecycle_hook_name) = self.lifecycle_hook_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleHookName", lifecycle_hook_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleTransition", &self.lifecycle_transition)?;
        if let Some(ref notification_metadata) = self.notification_metadata {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationMetadata", notification_metadata)?;
        }
        if let Some(ref notification_target_arn) = self.notification_target_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTargetARN", notification_target_arn)?;
        }
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", role_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LifecycleHookProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LifecycleHookProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LifecycleHookProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LifecycleHookProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_scaling_group_name: Option<crate::Value<String>> = None;
                let mut default_result: Option<crate::Value<String>> = None;
                let mut heartbeat_timeout: Option<crate::Value<u32>> = None;
                let mut lifecycle_hook_name: Option<crate::Value<String>> = None;
                let mut lifecycle_transition: Option<crate::Value<String>> = None;
                let mut notification_metadata: Option<crate::Value<String>> = None;
                let mut notification_target_arn: Option<crate::Value<String>> = None;
                let mut role_arn: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingGroupName" => {
                            auto_scaling_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultResult" => {
                            default_result = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HeartbeatTimeout" => {
                            heartbeat_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LifecycleHookName" => {
                            lifecycle_hook_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LifecycleTransition" => {
                            lifecycle_transition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationMetadata" => {
                            notification_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationTargetARN" => {
                            notification_target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleARN" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LifecycleHookProperties {
                    auto_scaling_group_name: auto_scaling_group_name.ok_or(::serde::de::Error::missing_field("AutoScalingGroupName"))?,
                    default_result: default_result,
                    heartbeat_timeout: heartbeat_timeout,
                    lifecycle_hook_name: lifecycle_hook_name,
                    lifecycle_transition: lifecycle_transition.ok_or(::serde::de::Error::missing_field("LifecycleTransition"))?,
                    notification_metadata: notification_metadata,
                    notification_target_arn: notification_target_arn,
                    role_arn: role_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for LifecycleHook {
    type Properties = LifecycleHookProperties;
    const TYPE: &'static str = "AWS::AutoScaling::LifecycleHook";
    fn properties(&self) -> &LifecycleHookProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LifecycleHookProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for LifecycleHook {}

impl From<LifecycleHookProperties> for LifecycleHook {
    fn from(properties: LifecycleHookProperties) -> LifecycleHook {
        LifecycleHook { properties }
    }
}

/// The [`AWS::AutoScaling::ScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html) resource type.
#[derive(Debug, Default)]
pub struct ScalingPolicy {
    properties: ScalingPolicyProperties
}

/// Properties for the `ScalingPolicy` resource.
#[derive(Debug, Default)]
pub struct ScalingPolicyProperties {
    /// Property [`AdjustmentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html#cfn-as-scalingpolicy-adjustmenttype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub adjustment_type: Option<crate::Value<String>>,
    /// Property [`AutoScalingGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html#cfn-as-scalingpolicy-autoscalinggroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_scaling_group_name: crate::Value<String>,
    /// Property [`Cooldown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html#cfn-as-scalingpolicy-cooldown).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cooldown: Option<crate::Value<String>>,
    /// Property [`EstimatedInstanceWarmup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html#cfn-as-scalingpolicy-estimatedinstancewarmup).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub estimated_instance_warmup: Option<crate::Value<u32>>,
    /// Property [`MetricAggregationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html#cfn-as-scalingpolicy-metricaggregationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metric_aggregation_type: Option<crate::Value<String>>,
    /// Property [`MinAdjustmentMagnitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html#cfn-as-scalingpolicy-minadjustmentmagnitude).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_adjustment_magnitude: Option<crate::Value<u32>>,
    /// Property [`PolicyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html#cfn-as-scalingpolicy-policytype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_type: Option<crate::Value<String>>,
    /// Property [`ScalingAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html#cfn-as-scalingpolicy-scalingadjustment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scaling_adjustment: Option<crate::Value<u32>>,
    /// Property [`StepAdjustments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html#cfn-as-scalingpolicy-stepadjustments).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub step_adjustments: Option<crate::ValueList<self::scaling_policy::StepAdjustment>>,
    /// Property [`TargetTrackingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html#cfn-autoscaling-scalingpolicy-targettrackingconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_tracking_configuration: Option<crate::Value<self::scaling_policy::TargetTrackingConfiguration>>,
}

impl ::serde::Serialize for ScalingPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref adjustment_type) = self.adjustment_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdjustmentType", adjustment_type)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroupName", &self.auto_scaling_group_name)?;
        if let Some(ref cooldown) = self.cooldown {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cooldown", cooldown)?;
        }
        if let Some(ref estimated_instance_warmup) = self.estimated_instance_warmup {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EstimatedInstanceWarmup", estimated_instance_warmup)?;
        }
        if let Some(ref metric_aggregation_type) = self.metric_aggregation_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricAggregationType", metric_aggregation_type)?;
        }
        if let Some(ref min_adjustment_magnitude) = self.min_adjustment_magnitude {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinAdjustmentMagnitude", min_adjustment_magnitude)?;
        }
        if let Some(ref policy_type) = self.policy_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyType", policy_type)?;
        }
        if let Some(ref scaling_adjustment) = self.scaling_adjustment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingAdjustment", scaling_adjustment)?;
        }
        if let Some(ref step_adjustments) = self.step_adjustments {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StepAdjustments", step_adjustments)?;
        }
        if let Some(ref target_tracking_configuration) = self.target_tracking_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetTrackingConfiguration", target_tracking_configuration)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScalingPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScalingPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScalingPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut adjustment_type: Option<crate::Value<String>> = None;
                let mut auto_scaling_group_name: Option<crate::Value<String>> = None;
                let mut cooldown: Option<crate::Value<String>> = None;
                let mut estimated_instance_warmup: Option<crate::Value<u32>> = None;
                let mut metric_aggregation_type: Option<crate::Value<String>> = None;
                let mut min_adjustment_magnitude: Option<crate::Value<u32>> = None;
                let mut policy_type: Option<crate::Value<String>> = None;
                let mut scaling_adjustment: Option<crate::Value<u32>> = None;
                let mut step_adjustments: Option<crate::ValueList<self::scaling_policy::StepAdjustment>> = None;
                let mut target_tracking_configuration: Option<crate::Value<self::scaling_policy::TargetTrackingConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdjustmentType" => {
                            adjustment_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoScalingGroupName" => {
                            auto_scaling_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cooldown" => {
                            cooldown = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EstimatedInstanceWarmup" => {
                            estimated_instance_warmup = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricAggregationType" => {
                            metric_aggregation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinAdjustmentMagnitude" => {
                            min_adjustment_magnitude = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyType" => {
                            policy_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScalingAdjustment" => {
                            scaling_adjustment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StepAdjustments" => {
                            step_adjustments = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetTrackingConfiguration" => {
                            target_tracking_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ScalingPolicyProperties {
                    adjustment_type: adjustment_type,
                    auto_scaling_group_name: auto_scaling_group_name.ok_or(::serde::de::Error::missing_field("AutoScalingGroupName"))?,
                    cooldown: cooldown,
                    estimated_instance_warmup: estimated_instance_warmup,
                    metric_aggregation_type: metric_aggregation_type,
                    min_adjustment_magnitude: min_adjustment_magnitude,
                    policy_type: policy_type,
                    scaling_adjustment: scaling_adjustment,
                    step_adjustments: step_adjustments,
                    target_tracking_configuration: target_tracking_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for ScalingPolicy {
    type Properties = ScalingPolicyProperties;
    const TYPE: &'static str = "AWS::AutoScaling::ScalingPolicy";
    fn properties(&self) -> &ScalingPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScalingPolicyProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for ScalingPolicy {}

impl From<ScalingPolicyProperties> for ScalingPolicy {
    fn from(properties: ScalingPolicyProperties) -> ScalingPolicy {
        ScalingPolicy { properties }
    }
}

/// The [`AWS::AutoScaling::ScheduledAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-scheduledaction.html) resource type.
#[derive(Debug, Default)]
pub struct ScheduledAction {
    properties: ScheduledActionProperties
}

/// Properties for the `ScheduledAction` resource.
#[derive(Debug, Default)]
pub struct ScheduledActionProperties {
    /// Property [`AutoScalingGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-scheduledaction.html#cfn-as-scheduledaction-asgname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub auto_scaling_group_name: crate::Value<String>,
    /// Property [`DesiredCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-scheduledaction.html#cfn-as-scheduledaction-desiredcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub desired_capacity: Option<crate::Value<u32>>,
    /// Property [`EndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-scheduledaction.html#cfn-as-scheduledaction-endtime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub end_time: Option<crate::Value<String>>,
    /// Property [`MaxSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-scheduledaction.html#cfn-as-scheduledaction-maxsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_size: Option<crate::Value<u32>>,
    /// Property [`MinSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-scheduledaction.html#cfn-as-scheduledaction-minsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_size: Option<crate::Value<u32>>,
    /// Property [`Recurrence`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-scheduledaction.html#cfn-as-scheduledaction-recurrence).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub recurrence: Option<crate::Value<String>>,
    /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-scheduledaction.html#cfn-as-scheduledaction-starttime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub start_time: Option<crate::Value<String>>,
    /// Property [`TimeZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-scheduledaction.html#cfn-as-scheduledaction-timezone).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub time_zone: Option<crate::Value<String>>,
}

impl ::serde::Serialize for ScheduledActionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroupName", &self.auto_scaling_group_name)?;
        if let Some(ref desired_capacity) = self.desired_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredCapacity", desired_capacity)?;
        }
        if let Some(ref end_time) = self.end_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTime", end_time)?;
        }
        if let Some(ref max_size) = self.max_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSize", max_size)?;
        }
        if let Some(ref min_size) = self.min_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSize", min_size)?;
        }
        if let Some(ref recurrence) = self.recurrence {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Recurrence", recurrence)?;
        }
        if let Some(ref start_time) = self.start_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", start_time)?;
        }
        if let Some(ref time_zone) = self.time_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeZone", time_zone)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScheduledActionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduledActionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScheduledActionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScheduledActionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_scaling_group_name: Option<crate::Value<String>> = None;
                let mut desired_capacity: Option<crate::Value<u32>> = None;
                let mut end_time: Option<crate::Value<String>> = None;
                let mut max_size: Option<crate::Value<u32>> = None;
                let mut min_size: Option<crate::Value<u32>> = None;
                let mut recurrence: Option<crate::Value<String>> = None;
                let mut start_time: Option<crate::Value<String>> = None;
                let mut time_zone: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingGroupName" => {
                            auto_scaling_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DesiredCapacity" => {
                            desired_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndTime" => {
                            end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxSize" => {
                            max_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinSize" => {
                            min_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Recurrence" => {
                            recurrence = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartTime" => {
                            start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeZone" => {
                            time_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ScheduledActionProperties {
                    auto_scaling_group_name: auto_scaling_group_name.ok_or(::serde::de::Error::missing_field("AutoScalingGroupName"))?,
                    desired_capacity: desired_capacity,
                    end_time: end_time,
                    max_size: max_size,
                    min_size: min_size,
                    recurrence: recurrence,
                    start_time: start_time,
                    time_zone: time_zone,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for ScheduledAction {
    type Properties = ScheduledActionProperties;
    const TYPE: &'static str = "AWS::AutoScaling::ScheduledAction";
    fn properties(&self) -> &ScheduledActionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScheduledActionProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for ScheduledAction {}

impl From<ScheduledActionProperties> for ScheduledAction {
    fn from(properties: ScheduledActionProperties) -> ScheduledAction {
        ScheduledAction { properties }
    }
}

/// The [`AWS::AutoScaling::WarmPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscaling-warmpool.html) resource type.
#[derive(Debug, Default)]
pub struct WarmPool {
    properties: WarmPoolProperties
}

/// Properties for the `WarmPool` resource.
#[derive(Debug, Default)]
pub struct WarmPoolProperties {
    /// Property [`AutoScalingGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscaling-warmpool.html#cfn-autoscaling-warmpool-autoscalinggroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub auto_scaling_group_name: crate::Value<String>,
    /// Property [`MaxGroupPreparedCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscaling-warmpool.html#cfn-autoscaling-warmpool-maxgrouppreparedcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_group_prepared_capacity: Option<crate::Value<u32>>,
    /// Property [`MinSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscaling-warmpool.html#cfn-autoscaling-warmpool-minsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_size: Option<crate::Value<u32>>,
    /// Property [`PoolState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscaling-warmpool.html#cfn-autoscaling-warmpool-poolstate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pool_state: Option<crate::Value<String>>,
}

impl ::serde::Serialize for WarmPoolProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroupName", &self.auto_scaling_group_name)?;
        if let Some(ref max_group_prepared_capacity) = self.max_group_prepared_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxGroupPreparedCapacity", max_group_prepared_capacity)?;
        }
        if let Some(ref min_size) = self.min_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSize", min_size)?;
        }
        if let Some(ref pool_state) = self.pool_state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PoolState", pool_state)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WarmPoolProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WarmPoolProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WarmPoolProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WarmPoolProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_scaling_group_name: Option<crate::Value<String>> = None;
                let mut max_group_prepared_capacity: Option<crate::Value<u32>> = None;
                let mut min_size: Option<crate::Value<u32>> = None;
                let mut pool_state: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingGroupName" => {
                            auto_scaling_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxGroupPreparedCapacity" => {
                            max_group_prepared_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinSize" => {
                            min_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PoolState" => {
                            pool_state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WarmPoolProperties {
                    auto_scaling_group_name: auto_scaling_group_name.ok_or(::serde::de::Error::missing_field("AutoScalingGroupName"))?,
                    max_group_prepared_capacity: max_group_prepared_capacity,
                    min_size: min_size,
                    pool_state: pool_state,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for WarmPool {
    type Properties = WarmPoolProperties;
    const TYPE: &'static str = "AWS::AutoScaling::WarmPool";
    fn properties(&self) -> &WarmPoolProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WarmPoolProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for WarmPool {}

impl From<WarmPoolProperties> for WarmPool {
    fn from(properties: WarmPoolProperties) -> WarmPool {
        WarmPool { properties }
    }
}

pub mod auto_scaling_group {
    //! Property types for the `AutoScalingGroup` resource.

    /// The [`AWS::AutoScaling::AutoScalingGroup.InstancesDistribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-instancesdistribution.html) property type.
    #[derive(Debug, Default)]
    pub struct InstancesDistribution {
        /// Property [`OnDemandAllocationStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-instancesdistribution.html#cfn-autoscaling-autoscalinggroup-instancesdistribution-ondemandallocationstrategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_demand_allocation_strategy: Option<crate::Value<String>>,
        /// Property [`OnDemandBaseCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-instancesdistribution.html#cfn-autoscaling-autoscalinggroup-instancesdistribution-ondemandbasecapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_demand_base_capacity: Option<crate::Value<u32>>,
        /// Property [`OnDemandPercentageAboveBaseCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-instancesdistribution.html#cfn-autoscaling-autoscalinggroup-instancesdistribution-ondemandpercentageabovebasecapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_demand_percentage_above_base_capacity: Option<crate::Value<u32>>,
        /// Property [`SpotAllocationStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-instancesdistribution.html#cfn-autoscaling-autoscalinggroup-instancesdistribution-spotallocationstrategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub spot_allocation_strategy: Option<crate::Value<String>>,
        /// Property [`SpotInstancePools`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-instancesdistribution.html#cfn-autoscaling-autoscalinggroup-instancesdistribution-spotinstancepools).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub spot_instance_pools: Option<crate::Value<u32>>,
        /// Property [`SpotMaxPrice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-instancesdistribution.html#cfn-autoscaling-autoscalinggroup-instancesdistribution-spotmaxprice).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub spot_max_price: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for InstancesDistribution {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref on_demand_allocation_strategy) = self.on_demand_allocation_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnDemandAllocationStrategy", on_demand_allocation_strategy)?;
            }
            if let Some(ref on_demand_base_capacity) = self.on_demand_base_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnDemandBaseCapacity", on_demand_base_capacity)?;
            }
            if let Some(ref on_demand_percentage_above_base_capacity) = self.on_demand_percentage_above_base_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnDemandPercentageAboveBaseCapacity", on_demand_percentage_above_base_capacity)?;
            }
            if let Some(ref spot_allocation_strategy) = self.spot_allocation_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotAllocationStrategy", spot_allocation_strategy)?;
            }
            if let Some(ref spot_instance_pools) = self.spot_instance_pools {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotInstancePools", spot_instance_pools)?;
            }
            if let Some(ref spot_max_price) = self.spot_max_price {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotMaxPrice", spot_max_price)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for InstancesDistribution {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstancesDistribution, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstancesDistribution;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstancesDistribution")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut on_demand_allocation_strategy: Option<crate::Value<String>> = None;
                    let mut on_demand_base_capacity: Option<crate::Value<u32>> = None;
                    let mut on_demand_percentage_above_base_capacity: Option<crate::Value<u32>> = None;
                    let mut spot_allocation_strategy: Option<crate::Value<String>> = None;
                    let mut spot_instance_pools: Option<crate::Value<u32>> = None;
                    let mut spot_max_price: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OnDemandAllocationStrategy" => {
                                on_demand_allocation_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnDemandBaseCapacity" => {
                                on_demand_base_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnDemandPercentageAboveBaseCapacity" => {
                                on_demand_percentage_above_base_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpotAllocationStrategy" => {
                                spot_allocation_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpotInstancePools" => {
                                spot_instance_pools = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpotMaxPrice" => {
                                spot_max_price = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InstancesDistribution {
                        on_demand_allocation_strategy: on_demand_allocation_strategy,
                        on_demand_base_capacity: on_demand_base_capacity,
                        on_demand_percentage_above_base_capacity: on_demand_percentage_above_base_capacity,
                        spot_allocation_strategy: spot_allocation_strategy,
                        spot_instance_pools: spot_instance_pools,
                        spot_max_price: spot_max_price,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.LaunchTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-launchtemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct LaunchTemplate {
        /// Property [`LaunchTemplateSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-launchtemplate.html#cfn-as-group-launchtemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_template_specification: crate::Value<LaunchTemplateSpecification>,
        /// Property [`Overrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-launchtemplate.html#cfn-as-mixedinstancespolicy-overrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub overrides: Option<crate::ValueList<LaunchTemplateOverrides>>,
    }

    impl crate::codec::SerializeValue for LaunchTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplateSpecification", &self.launch_template_specification)?;
            if let Some(ref overrides) = self.overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Overrides", overrides)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for LaunchTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LaunchTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LaunchTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut launch_template_specification: Option<crate::Value<LaunchTemplateSpecification>> = None;
                    let mut overrides: Option<crate::ValueList<LaunchTemplateOverrides>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LaunchTemplateSpecification" => {
                                launch_template_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Overrides" => {
                                overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LaunchTemplate {
                        launch_template_specification: launch_template_specification.ok_or(::serde::de::Error::missing_field("LaunchTemplateSpecification"))?,
                        overrides: overrides,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.LaunchTemplateOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-launchtemplateoverrides.html) property type.
    #[derive(Debug, Default)]
    pub struct LaunchTemplateOverrides {
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-launchtemplateoverrides.html#cfn-autoscaling-autoscalinggroup-launchtemplateoverrides-instancetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: Option<crate::Value<String>>,
        /// Property [`LaunchTemplateSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-launchtemplateoverrides.html#cfn-autoscaling-autoscalinggroup-launchtemplateoverrides-launchtemplatespecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_template_specification: Option<crate::Value<LaunchTemplateSpecification>>,
        /// Property [`WeightedCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy-launchtemplateoverrides.html#cfn-autoscaling-autoscalinggroup-launchtemplateoverrides-weightedcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weighted_capacity: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for LaunchTemplateOverrides {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref instance_type) = self.instance_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", instance_type)?;
            }
            if let Some(ref launch_template_specification) = self.launch_template_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplateSpecification", launch_template_specification)?;
            }
            if let Some(ref weighted_capacity) = self.weighted_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeightedCapacity", weighted_capacity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for LaunchTemplateOverrides {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchTemplateOverrides, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LaunchTemplateOverrides;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LaunchTemplateOverrides")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_type: Option<crate::Value<String>> = None;
                    let mut launch_template_specification: Option<crate::Value<LaunchTemplateSpecification>> = None;
                    let mut weighted_capacity: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchTemplateSpecification" => {
                                launch_template_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WeightedCapacity" => {
                                weighted_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LaunchTemplateOverrides {
                        instance_type: instance_type,
                        launch_template_specification: launch_template_specification,
                        weighted_capacity: weighted_capacity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.LaunchTemplateSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-launchtemplatespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct LaunchTemplateSpecification {
        /// Property [`LaunchTemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-launchtemplatespecification.html#cfn-autoscaling-autoscalinggroup-launchtemplatespecification-launchtemplateid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_template_id: Option<crate::Value<String>>,
        /// Property [`LaunchTemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-launchtemplatespecification.html#cfn-autoscaling-autoscalinggroup-launchtemplatespecification-launchtemplatename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_template_name: Option<crate::Value<String>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-launchtemplatespecification.html#cfn-autoscaling-autoscalinggroup-launchtemplatespecification-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: crate::Value<String>,
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
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
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
                        version: version.ok_or(::serde::de::Error::missing_field("Version"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.LifecycleHookSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-lifecyclehookspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct LifecycleHookSpecification {
        /// Property [`DefaultResult`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-lifecyclehookspecification.html#cfn-autoscaling-autoscalinggroup-lifecyclehookspecification-defaultresult).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_result: Option<crate::Value<String>>,
        /// Property [`HeartbeatTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-lifecyclehookspecification.html#cfn-autoscaling-autoscalinggroup-lifecyclehookspecification-heartbeattimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub heartbeat_timeout: Option<crate::Value<u32>>,
        /// Property [`LifecycleHookName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-lifecyclehookspecification.html#cfn-autoscaling-autoscalinggroup-lifecyclehookspecification-lifecyclehookname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lifecycle_hook_name: crate::Value<String>,
        /// Property [`LifecycleTransition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-lifecyclehookspecification.html#cfn-autoscaling-autoscalinggroup-lifecyclehookspecification-lifecycletransition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lifecycle_transition: crate::Value<String>,
        /// Property [`NotificationMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-lifecyclehookspecification.html#cfn-autoscaling-autoscalinggroup-lifecyclehookspecification-notificationmetadata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_metadata: Option<crate::Value<String>>,
        /// Property [`NotificationTargetARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-lifecyclehookspecification.html#cfn-autoscaling-autoscalinggroup-lifecyclehookspecification-notificationtargetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_target_arn: Option<crate::Value<String>>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-lifecyclehookspecification.html#cfn-autoscaling-autoscalinggroup-lifecyclehookspecification-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for LifecycleHookSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_result) = self.default_result {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultResult", default_result)?;
            }
            if let Some(ref heartbeat_timeout) = self.heartbeat_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeartbeatTimeout", heartbeat_timeout)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleHookName", &self.lifecycle_hook_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleTransition", &self.lifecycle_transition)?;
            if let Some(ref notification_metadata) = self.notification_metadata {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationMetadata", notification_metadata)?;
            }
            if let Some(ref notification_target_arn) = self.notification_target_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTargetARN", notification_target_arn)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for LifecycleHookSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LifecycleHookSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LifecycleHookSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LifecycleHookSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_result: Option<crate::Value<String>> = None;
                    let mut heartbeat_timeout: Option<crate::Value<u32>> = None;
                    let mut lifecycle_hook_name: Option<crate::Value<String>> = None;
                    let mut lifecycle_transition: Option<crate::Value<String>> = None;
                    let mut notification_metadata: Option<crate::Value<String>> = None;
                    let mut notification_target_arn: Option<crate::Value<String>> = None;
                    let mut role_arn: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultResult" => {
                                default_result = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeartbeatTimeout" => {
                                heartbeat_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LifecycleHookName" => {
                                lifecycle_hook_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LifecycleTransition" => {
                                lifecycle_transition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotificationMetadata" => {
                                notification_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotificationTargetARN" => {
                                notification_target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleARN" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LifecycleHookSpecification {
                        default_result: default_result,
                        heartbeat_timeout: heartbeat_timeout,
                        lifecycle_hook_name: lifecycle_hook_name.ok_or(::serde::de::Error::missing_field("LifecycleHookName"))?,
                        lifecycle_transition: lifecycle_transition.ok_or(::serde::de::Error::missing_field("LifecycleTransition"))?,
                        notification_metadata: notification_metadata,
                        notification_target_arn: notification_target_arn,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.MetricsCollection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-metricscollection.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricsCollection {
        /// Property [`Granularity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-metricscollection.html#cfn-as-metricscollection-granularity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub granularity: crate::Value<String>,
        /// Property [`Metrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-metricscollection.html#cfn-as-metricscollection-metrics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metrics: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for MetricsCollection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Granularity", &self.granularity)?;
            if let Some(ref metrics) = self.metrics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Metrics", metrics)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for MetricsCollection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricsCollection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricsCollection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricsCollection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut granularity: Option<crate::Value<String>> = None;
                    let mut metrics: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Granularity" => {
                                granularity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Metrics" => {
                                metrics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricsCollection {
                        granularity: granularity.ok_or(::serde::de::Error::missing_field("Granularity"))?,
                        metrics: metrics,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.MixedInstancesPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-group-mixedinstancespolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct MixedInstancesPolicy {
        /// Property [`InstancesDistribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-group-mixedinstancespolicy.html#cfn-as-mixedinstancespolicy-instancesdistribution).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instances_distribution: Option<crate::Value<InstancesDistribution>>,
        /// Property [`LaunchTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-group-mixedinstancespolicy.html#cfn-as-mixedinstancespolicy-launchtemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_template: crate::Value<LaunchTemplate>,
    }

    impl crate::codec::SerializeValue for MixedInstancesPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref instances_distribution) = self.instances_distribution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstancesDistribution", instances_distribution)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchTemplate", &self.launch_template)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for MixedInstancesPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MixedInstancesPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MixedInstancesPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MixedInstancesPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instances_distribution: Option<crate::Value<InstancesDistribution>> = None;
                    let mut launch_template: Option<crate::Value<LaunchTemplate>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstancesDistribution" => {
                                instances_distribution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchTemplate" => {
                                launch_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MixedInstancesPolicy {
                        instances_distribution: instances_distribution,
                        launch_template: launch_template.ok_or(::serde::de::Error::missing_field("LaunchTemplate"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.NotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-notificationconfigurations.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationConfiguration {
        /// Property [`NotificationTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-notificationconfigurations.html#cfn-as-group-notificationconfigurations-notificationtypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_types: Option<crate::ValueList<String>>,
        /// Property [`TopicARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-notificationconfigurations.html#cfn-autoscaling-autoscalinggroup-notificationconfigurations-topicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_arn: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for NotificationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref notification_types) = self.notification_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTypes", notification_types)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicARN", &self.topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for NotificationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut notification_types: Option<crate::ValueList<String>> = None;
                    let mut topic_arn: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NotificationTypes" => {
                                notification_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicARN" => {
                                topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationConfiguration {
                        notification_types: notification_types,
                        topic_arn: topic_arn.ok_or(::serde::de::Error::missing_field("TopicARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.TagProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-tags.html) property type.
    #[derive(Debug, Default)]
    pub struct TagProperty {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-tags.html#cfn-as-tags-Key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: crate::Value<String>,
        /// Property [`PropagateAtLaunch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-tags.html#cfn-as-tags-PropagateAtLaunch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub propagate_at_launch: crate::Value<bool>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-tags.html#cfn-as-tags-Value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for TagProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropagateAtLaunch", &self.propagate_at_launch)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for TagProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<crate::Value<String>> = None;
                    let mut propagate_at_launch: Option<crate::Value<bool>> = None;
                    let mut value: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropagateAtLaunch" => {
                                propagate_at_launch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TagProperty {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        propagate_at_launch: propagate_at_launch.ok_or(::serde::de::Error::missing_field("PropagateAtLaunch"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod launch_configuration {
    //! Property types for the `LaunchConfiguration` resource.

    /// The [`AWS::AutoScaling::LaunchConfiguration.BlockDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-template.html) property type.
    #[derive(Debug, Default)]
    pub struct BlockDevice {
        /// Property [`DeleteOnTermination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-template.html#cfn-as-launchconfig-blockdev-template-deleteonterm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delete_on_termination: Option<crate::Value<bool>>,
        /// Property [`Encrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-template.html#cfn-as-launchconfig-blockdev-template-encrypted).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encrypted: Option<crate::Value<bool>>,
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-template.html#cfn-as-launchconfig-blockdev-template-iops).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iops: Option<crate::Value<u32>>,
        /// Property [`SnapshotId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-template.html#cfn-as-launchconfig-blockdev-template-snapshotid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snapshot_id: Option<crate::Value<String>>,
        /// Property [`Throughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-template.html#cfn-as-launchconfig-blockdev-template-throughput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throughput: Option<crate::Value<u32>>,
        /// Property [`VolumeSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-template.html#cfn-as-launchconfig-blockdev-template-volumesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_size: Option<crate::Value<u32>>,
        /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-template.html#cfn-as-launchconfig-blockdev-template-volumetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_type: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for BlockDevice {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delete_on_termination) = self.delete_on_termination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOnTermination", delete_on_termination)?;
            }
            if let Some(ref encrypted) = self.encrypted {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", encrypted)?;
            }
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
            }
            if let Some(ref snapshot_id) = self.snapshot_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotId", snapshot_id)?;
            }
            if let Some(ref throughput) = self.throughput {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Throughput", throughput)?;
            }
            if let Some(ref volume_size) = self.volume_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSize", volume_size)?;
            }
            if let Some(ref volume_type) = self.volume_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", volume_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for BlockDevice {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlockDevice, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlockDevice;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlockDevice")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delete_on_termination: Option<crate::Value<bool>> = None;
                    let mut encrypted: Option<crate::Value<bool>> = None;
                    let mut iops: Option<crate::Value<u32>> = None;
                    let mut snapshot_id: Option<crate::Value<String>> = None;
                    let mut throughput: Option<crate::Value<u32>> = None;
                    let mut volume_size: Option<crate::Value<u32>> = None;
                    let mut volume_type: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteOnTermination" => {
                                delete_on_termination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Encrypted" => {
                                encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Iops" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnapshotId" => {
                                snapshot_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Throughput" => {
                                throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSize" => {
                                volume_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeType" => {
                                volume_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BlockDevice {
                        delete_on_termination: delete_on_termination,
                        encrypted: encrypted,
                        iops: iops,
                        snapshot_id: snapshot_id,
                        throughput: throughput,
                        volume_size: volume_size,
                        volume_type: volume_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::LaunchConfiguration.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-mapping.html) property type.
    #[derive(Debug, Default)]
    pub struct BlockDeviceMapping {
        /// Property [`DeviceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-mapping.html#cfn-as-launchconfig-blockdev-mapping-devicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_name: crate::Value<String>,
        /// Property [`Ebs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-mapping.html#cfn-as-launchconfig-blockdev-mapping-ebs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebs: Option<crate::Value<BlockDevice>>,
        /// Property [`NoDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-mapping.html#cfn-as-launchconfig-blockdev-mapping-nodevice).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_device: Option<crate::Value<bool>>,
        /// Property [`VirtualName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-mapping.html#cfn-as-launchconfig-blockdev-mapping-virtualname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub virtual_name: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for BlockDeviceMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceName", &self.device_name)?;
            if let Some(ref ebs) = self.ebs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ebs", ebs)?;
            }
            if let Some(ref no_device) = self.no_device {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoDevice", no_device)?;
            }
            if let Some(ref virtual_name) = self.virtual_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualName", virtual_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for BlockDeviceMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlockDeviceMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlockDeviceMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlockDeviceMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device_name: Option<crate::Value<String>> = None;
                    let mut ebs: Option<crate::Value<BlockDevice>> = None;
                    let mut no_device: Option<crate::Value<bool>> = None;
                    let mut virtual_name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeviceName" => {
                                device_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ebs" => {
                                ebs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoDevice" => {
                                no_device = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VirtualName" => {
                                virtual_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BlockDeviceMapping {
                        device_name: device_name.ok_or(::serde::de::Error::missing_field("DeviceName"))?,
                        ebs: ebs,
                        no_device: no_device,
                        virtual_name: virtual_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::LaunchConfiguration.MetadataOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-launchconfig-metadataoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct MetadataOptions {
        /// Property [`HttpEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-launchconfig-metadataoptions.html#cfn-autoscaling-launchconfig-metadataoptions-httpendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_endpoint: Option<crate::Value<String>>,
        /// Property [`HttpPutResponseHopLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-launchconfig-metadataoptions.html#cfn-autoscaling-launchconfig-metadataoptions-httpputresponsehoplimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_put_response_hop_limit: Option<crate::Value<u32>>,
        /// Property [`HttpTokens`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-launchconfig-metadataoptions.html#cfn-autoscaling-launchconfig-metadataoptions-httptokens).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_tokens: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for MetadataOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref http_endpoint) = self.http_endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpEndpoint", http_endpoint)?;
            }
            if let Some(ref http_put_response_hop_limit) = self.http_put_response_hop_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpPutResponseHopLimit", http_put_response_hop_limit)?;
            }
            if let Some(ref http_tokens) = self.http_tokens {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpTokens", http_tokens)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for MetadataOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetadataOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetadataOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetadataOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut http_endpoint: Option<crate::Value<String>> = None;
                    let mut http_put_response_hop_limit: Option<crate::Value<u32>> = None;
                    let mut http_tokens: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HttpEndpoint" => {
                                http_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpPutResponseHopLimit" => {
                                http_put_response_hop_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpTokens" => {
                                http_tokens = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetadataOptions {
                        http_endpoint: http_endpoint,
                        http_put_response_hop_limit: http_put_response_hop_limit,
                        http_tokens: http_tokens,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod scaling_policy {
    //! Property types for the `ScalingPolicy` resource.

    /// The [`AWS::AutoScaling::ScalingPolicy.CustomizedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-customizedmetricspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomizedMetricSpecification {
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-customizedmetricspecification.html#cfn-autoscaling-scalingpolicy-customizedmetricspecification-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: Option<crate::ValueList<MetricDimension>>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-customizedmetricspecification.html#cfn-autoscaling-scalingpolicy-customizedmetricspecification-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: crate::Value<String>,
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-customizedmetricspecification.html#cfn-autoscaling-scalingpolicy-customizedmetricspecification-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: crate::Value<String>,
        /// Property [`Statistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-customizedmetricspecification.html#cfn-autoscaling-scalingpolicy-customizedmetricspecification-statistic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statistic: crate::Value<String>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-customizedmetricspecification.html#cfn-autoscaling-scalingpolicy-customizedmetricspecification-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for CustomizedMetricSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dimensions) = self.dimensions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", dimensions)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", &self.statistic)?;
            if let Some(ref unit) = self.unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for CustomizedMetricSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomizedMetricSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomizedMetricSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomizedMetricSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimensions: Option<crate::ValueList<MetricDimension>> = None;
                    let mut metric_name: Option<crate::Value<String>> = None;
                    let mut namespace: Option<crate::Value<String>> = None;
                    let mut statistic: Option<crate::Value<String>> = None;
                    let mut unit: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Dimensions" => {
                                dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Statistic" => {
                                statistic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomizedMetricSpecification {
                        dimensions: dimensions,
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                        statistic: statistic.ok_or(::serde::de::Error::missing_field("Statistic"))?,
                        unit: unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::ScalingPolicy.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-metricdimension.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricDimension {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-metricdimension.html#cfn-autoscaling-scalingpolicy-metricdimension-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: crate::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-metricdimension.html#cfn-autoscaling-scalingpolicy-metricdimension-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for MetricDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for MetricDimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricDimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricDimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricDimension")
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

                    Ok(MetricDimension {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::ScalingPolicy.PredefinedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predefinedmetricspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct PredefinedMetricSpecification {
        /// Property [`PredefinedMetricType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predefinedmetricspecification.html#cfn-autoscaling-scalingpolicy-predefinedmetricspecification-predefinedmetrictype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predefined_metric_type: crate::Value<String>,
        /// Property [`ResourceLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predefinedmetricspecification.html#cfn-autoscaling-scalingpolicy-predefinedmetricspecification-resourcelabel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_label: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for PredefinedMetricSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredefinedMetricType", &self.predefined_metric_type)?;
            if let Some(ref resource_label) = self.resource_label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceLabel", resource_label)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for PredefinedMetricSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PredefinedMetricSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PredefinedMetricSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PredefinedMetricSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut predefined_metric_type: Option<crate::Value<String>> = None;
                    let mut resource_label: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PredefinedMetricType" => {
                                predefined_metric_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceLabel" => {
                                resource_label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PredefinedMetricSpecification {
                        predefined_metric_type: predefined_metric_type.ok_or(::serde::de::Error::missing_field("PredefinedMetricType"))?,
                        resource_label: resource_label,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::ScalingPolicy.StepAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-stepadjustments.html) property type.
    #[derive(Debug, Default)]
    pub struct StepAdjustment {
        /// Property [`MetricIntervalLowerBound`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-stepadjustments.html#cfn-autoscaling-scalingpolicy-stepadjustment-metricintervallowerbound).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_interval_lower_bound: Option<crate::Value<f64>>,
        /// Property [`MetricIntervalUpperBound`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-stepadjustments.html#cfn-autoscaling-scalingpolicy-stepadjustment-metricintervalupperbound).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_interval_upper_bound: Option<crate::Value<f64>>,
        /// Property [`ScalingAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-stepadjustments.html#cfn-autoscaling-scalingpolicy-stepadjustment-scalingadjustment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scaling_adjustment: crate::Value<u32>,
    }

    impl crate::codec::SerializeValue for StepAdjustment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref metric_interval_lower_bound) = self.metric_interval_lower_bound {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricIntervalLowerBound", metric_interval_lower_bound)?;
            }
            if let Some(ref metric_interval_upper_bound) = self.metric_interval_upper_bound {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricIntervalUpperBound", metric_interval_upper_bound)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingAdjustment", &self.scaling_adjustment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for StepAdjustment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StepAdjustment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StepAdjustment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StepAdjustment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut metric_interval_lower_bound: Option<crate::Value<f64>> = None;
                    let mut metric_interval_upper_bound: Option<crate::Value<f64>> = None;
                    let mut scaling_adjustment: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MetricIntervalLowerBound" => {
                                metric_interval_lower_bound = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricIntervalUpperBound" => {
                                metric_interval_upper_bound = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScalingAdjustment" => {
                                scaling_adjustment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StepAdjustment {
                        metric_interval_lower_bound: metric_interval_lower_bound,
                        metric_interval_upper_bound: metric_interval_upper_bound,
                        scaling_adjustment: scaling_adjustment.ok_or(::serde::de::Error::missing_field("ScalingAdjustment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::ScalingPolicy.TargetTrackingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetTrackingConfiguration {
        /// Property [`CustomizedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingconfiguration.html#cfn-autoscaling-scalingpolicy-targettrackingconfiguration-customizedmetricspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub customized_metric_specification: Option<crate::Value<CustomizedMetricSpecification>>,
        /// Property [`DisableScaleIn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingconfiguration.html#cfn-autoscaling-scalingpolicy-targettrackingconfiguration-disablescalein).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disable_scale_in: Option<crate::Value<bool>>,
        /// Property [`PredefinedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingconfiguration.html#cfn-autoscaling-scalingpolicy-targettrackingconfiguration-predefinedmetricspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predefined_metric_specification: Option<crate::Value<PredefinedMetricSpecification>>,
        /// Property [`TargetValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingconfiguration.html#cfn-autoscaling-scalingpolicy-targettrackingconfiguration-targetvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_value: crate::Value<f64>,
    }

    impl crate::codec::SerializeValue for TargetTrackingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref customized_metric_specification) = self.customized_metric_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomizedMetricSpecification", customized_metric_specification)?;
            }
            if let Some(ref disable_scale_in) = self.disable_scale_in {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableScaleIn", disable_scale_in)?;
            }
            if let Some(ref predefined_metric_specification) = self.predefined_metric_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredefinedMetricSpecification", predefined_metric_specification)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetValue", &self.target_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for TargetTrackingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetTrackingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetTrackingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetTrackingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut customized_metric_specification: Option<crate::Value<CustomizedMetricSpecification>> = None;
                    let mut disable_scale_in: Option<crate::Value<bool>> = None;
                    let mut predefined_metric_specification: Option<crate::Value<PredefinedMetricSpecification>> = None;
                    let mut target_value: Option<crate::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomizedMetricSpecification" => {
                                customized_metric_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisableScaleIn" => {
                                disable_scale_in = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PredefinedMetricSpecification" => {
                                predefined_metric_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetValue" => {
                                target_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetTrackingConfiguration {
                        customized_metric_specification: customized_metric_specification,
                        disable_scale_in: disable_scale_in,
                        predefined_metric_specification: predefined_metric_specification,
                        target_value: target_value.ok_or(::serde::de::Error::missing_field("TargetValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
