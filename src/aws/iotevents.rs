//! Types for the `IoTEvents` service.

/// The [`AWS::IoTEvents::DetectorModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-detectormodel.html) resource type.
#[derive(Debug, Default)]
pub struct DetectorModel {
    properties: DetectorModelProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `DetectorModel` resource.
#[derive(Debug, Default)]
pub struct DetectorModelProperties {
    /// Property [`DetectorModelDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-detectormodel.html#cfn-iotevents-detectormodel-detectormodeldefinition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub detector_model_definition: crate::Value<self::detector_model::DetectorModelDefinition>,
    /// Property [`DetectorModelDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-detectormodel.html#cfn-iotevents-detectormodel-detectormodeldescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub detector_model_description: Option<crate::Value<String>>,
    /// Property [`DetectorModelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-detectormodel.html#cfn-iotevents-detectormodel-detectormodelname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub detector_model_name: Option<crate::Value<String>>,
    /// Property [`EvaluationMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-detectormodel.html#cfn-iotevents-detectormodel-evaluationmethod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub evaluation_method: Option<crate::Value<String>>,
    /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-detectormodel.html#cfn-iotevents-detectormodel-key).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key: Option<crate::Value<String>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-detectormodel.html#cfn-iotevents-detectormodel-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: crate::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-detectormodel.html#cfn-iotevents-detectormodel-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for DetectorModelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorModelDefinition", &self.detector_model_definition)?;
        if let Some(ref detector_model_description) = self.detector_model_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorModelDescription", detector_model_description)?;
        }
        if let Some(ref detector_model_name) = self.detector_model_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorModelName", detector_model_name)?;
        }
        if let Some(ref evaluation_method) = self.evaluation_method {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluationMethod", evaluation_method)?;
        }
        if let Some(ref key) = self.key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DetectorModelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DetectorModelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DetectorModelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DetectorModelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut detector_model_definition: Option<crate::Value<self::detector_model::DetectorModelDefinition>> = None;
                let mut detector_model_description: Option<crate::Value<String>> = None;
                let mut detector_model_name: Option<crate::Value<String>> = None;
                let mut evaluation_method: Option<crate::Value<String>> = None;
                let mut key: Option<crate::Value<String>> = None;
                let mut role_arn: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DetectorModelDefinition" => {
                            detector_model_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DetectorModelDescription" => {
                            detector_model_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DetectorModelName" => {
                            detector_model_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EvaluationMethod" => {
                            evaluation_method = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Key" => {
                            key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DetectorModelProperties {
                    detector_model_definition: detector_model_definition.ok_or(::serde::de::Error::missing_field("DetectorModelDefinition"))?,
                    detector_model_description: detector_model_description,
                    detector_model_name: detector_model_name,
                    evaluation_method: evaluation_method,
                    key: key,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DetectorModel {
    type Properties = DetectorModelProperties;
    const TYPE: &'static str = "AWS::IoTEvents::DetectorModel";
    fn properties(&self) -> &DetectorModelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DetectorModelProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for DetectorModel {}

impl From<DetectorModelProperties> for DetectorModel {
    fn from(properties: DetectorModelProperties) -> DetectorModel {
        DetectorModel { properties, depends_on: None }
    }
}

/// The [`AWS::IoTEvents::Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-input.html) resource type.
#[derive(Debug, Default)]
pub struct Input {
    properties: InputProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `Input` resource.
#[derive(Debug, Default)]
pub struct InputProperties {
    /// Property [`InputDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-input.html#cfn-iotevents-input-inputdefinition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub input_definition: crate::Value<self::input::InputDefinition>,
    /// Property [`InputDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-input.html#cfn-iotevents-input-inputdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub input_description: Option<crate::Value<String>>,
    /// Property [`InputName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-input.html#cfn-iotevents-input-inputname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub input_name: Option<crate::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-input.html#cfn-iotevents-input-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for InputProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputDefinition", &self.input_definition)?;
        if let Some(ref input_description) = self.input_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputDescription", input_description)?;
        }
        if let Some(ref input_name) = self.input_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputName", input_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InputProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InputProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InputProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InputProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut input_definition: Option<crate::Value<self::input::InputDefinition>> = None;
                let mut input_description: Option<crate::Value<String>> = None;
                let mut input_name: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InputDefinition" => {
                            input_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InputDescription" => {
                            input_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InputName" => {
                            input_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InputProperties {
                    input_definition: input_definition.ok_or(::serde::de::Error::missing_field("InputDefinition"))?,
                    input_description: input_description,
                    input_name: input_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Input {
    type Properties = InputProperties;
    const TYPE: &'static str = "AWS::IoTEvents::Input";
    fn properties(&self) -> &InputProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InputProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for Input {}

impl From<InputProperties> for Input {
    fn from(properties: InputProperties) -> Input {
        Input { properties, depends_on: None }
    }
}

pub mod detector_model {
    //! Property types for the `DetectorModel` resource.

    /// The [`AWS::IoTEvents::DetectorModel.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`ClearTimer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-cleartimer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub clear_timer: Option<crate::Value<ClearTimer>>,
        /// Property [`DynamoDB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-dynamodb).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynamo_db: Option<crate::Value<DynamoDB>>,
        /// Property [`DynamoDBv2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-dynamodbv2).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynamo_d_bv2: Option<crate::Value<DynamoDBv2>>,
        /// Property [`Firehose`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-firehose).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub firehose: Option<crate::Value<Firehose>>,
        /// Property [`IotEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-iotevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iot_events: Option<crate::Value<IotEvents>>,
        /// Property [`IotSiteWise`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-iotsitewise).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iot_site_wise: Option<crate::Value<IotSiteWise>>,
        /// Property [`IotTopicPublish`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-iottopicpublish).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iot_topic_publish: Option<crate::Value<IotTopicPublish>>,
        /// Property [`Lambda`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-lambda).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda: Option<crate::Value<Lambda>>,
        /// Property [`ResetTimer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-resettimer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reset_timer: Option<crate::Value<ResetTimer>>,
        /// Property [`SetTimer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-settimer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub set_timer: Option<crate::Value<SetTimer>>,
        /// Property [`SetVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-setvariable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub set_variable: Option<crate::Value<SetVariable>>,
        /// Property [`Sns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-sns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sns: Option<crate::Value<Sns>>,
        /// Property [`Sqs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html#cfn-iotevents-detectormodel-action-sqs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sqs: Option<crate::Value<Sqs>>,
    }

    impl crate::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref clear_timer) = self.clear_timer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClearTimer", clear_timer)?;
            }
            if let Some(ref dynamo_db) = self.dynamo_db {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamoDB", dynamo_db)?;
            }
            if let Some(ref dynamo_d_bv2) = self.dynamo_d_bv2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamoDBv2", dynamo_d_bv2)?;
            }
            if let Some(ref firehose) = self.firehose {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Firehose", firehose)?;
            }
            if let Some(ref iot_events) = self.iot_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IotEvents", iot_events)?;
            }
            if let Some(ref iot_site_wise) = self.iot_site_wise {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IotSiteWise", iot_site_wise)?;
            }
            if let Some(ref iot_topic_publish) = self.iot_topic_publish {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IotTopicPublish", iot_topic_publish)?;
            }
            if let Some(ref lambda) = self.lambda {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Lambda", lambda)?;
            }
            if let Some(ref reset_timer) = self.reset_timer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResetTimer", reset_timer)?;
            }
            if let Some(ref set_timer) = self.set_timer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SetTimer", set_timer)?;
            }
            if let Some(ref set_variable) = self.set_variable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SetVariable", set_variable)?;
            }
            if let Some(ref sns) = self.sns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sns", sns)?;
            }
            if let Some(ref sqs) = self.sqs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sqs", sqs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut clear_timer: Option<crate::Value<ClearTimer>> = None;
                    let mut dynamo_db: Option<crate::Value<DynamoDB>> = None;
                    let mut dynamo_d_bv2: Option<crate::Value<DynamoDBv2>> = None;
                    let mut firehose: Option<crate::Value<Firehose>> = None;
                    let mut iot_events: Option<crate::Value<IotEvents>> = None;
                    let mut iot_site_wise: Option<crate::Value<IotSiteWise>> = None;
                    let mut iot_topic_publish: Option<crate::Value<IotTopicPublish>> = None;
                    let mut lambda: Option<crate::Value<Lambda>> = None;
                    let mut reset_timer: Option<crate::Value<ResetTimer>> = None;
                    let mut set_timer: Option<crate::Value<SetTimer>> = None;
                    let mut set_variable: Option<crate::Value<SetVariable>> = None;
                    let mut sns: Option<crate::Value<Sns>> = None;
                    let mut sqs: Option<crate::Value<Sqs>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClearTimer" => {
                                clear_timer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DynamoDB" => {
                                dynamo_db = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DynamoDBv2" => {
                                dynamo_d_bv2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Firehose" => {
                                firehose = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IotEvents" => {
                                iot_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IotSiteWise" => {
                                iot_site_wise = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IotTopicPublish" => {
                                iot_topic_publish = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Lambda" => {
                                lambda = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResetTimer" => {
                                reset_timer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SetTimer" => {
                                set_timer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SetVariable" => {
                                set_variable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sns" => {
                                sns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sqs" => {
                                sqs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        clear_timer: clear_timer,
                        dynamo_db: dynamo_db,
                        dynamo_d_bv2: dynamo_d_bv2,
                        firehose: firehose,
                        iot_events: iot_events,
                        iot_site_wise: iot_site_wise,
                        iot_topic_publish: iot_topic_publish,
                        lambda: lambda,
                        reset_timer: reset_timer,
                        set_timer: set_timer,
                        set_variable: set_variable,
                        sns: sns,
                        sqs: sqs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.AssetPropertyTimestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertytimestamp.html) property type.
    #[derive(Debug, Default)]
    pub struct AssetPropertyTimestamp {
        /// Property [`OffsetInNanos`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertytimestamp.html#cfn-iotevents-detectormodel-assetpropertytimestamp-offsetinnanos).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub offset_in_nanos: Option<crate::Value<String>>,
        /// Property [`TimeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertytimestamp.html#cfn-iotevents-detectormodel-assetpropertytimestamp-timeinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_in_seconds: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for AssetPropertyTimestamp {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref offset_in_nanos) = self.offset_in_nanos {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OffsetInNanos", offset_in_nanos)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeInSeconds", &self.time_in_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for AssetPropertyTimestamp {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetPropertyTimestamp, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssetPropertyTimestamp;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssetPropertyTimestamp")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut offset_in_nanos: Option<crate::Value<String>> = None;
                    let mut time_in_seconds: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OffsetInNanos" => {
                                offset_in_nanos = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeInSeconds" => {
                                time_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssetPropertyTimestamp {
                        offset_in_nanos: offset_in_nanos,
                        time_in_seconds: time_in_seconds.ok_or(::serde::de::Error::missing_field("TimeInSeconds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.AssetPropertyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertyvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct AssetPropertyValue {
        /// Property [`Quality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertyvalue.html#cfn-iotevents-detectormodel-assetpropertyvalue-quality).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quality: Option<crate::Value<String>>,
        /// Property [`Timestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertyvalue.html#cfn-iotevents-detectormodel-assetpropertyvalue-timestamp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp: Option<crate::Value<AssetPropertyTimestamp>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertyvalue.html#cfn-iotevents-detectormodel-assetpropertyvalue-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: crate::Value<AssetPropertyVariant>,
    }

    impl crate::codec::SerializeValue for AssetPropertyValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref quality) = self.quality {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Quality", quality)?;
            }
            if let Some(ref timestamp) = self.timestamp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timestamp", timestamp)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for AssetPropertyValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetPropertyValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssetPropertyValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssetPropertyValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut quality: Option<crate::Value<String>> = None;
                    let mut timestamp: Option<crate::Value<AssetPropertyTimestamp>> = None;
                    let mut value: Option<crate::Value<AssetPropertyVariant>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Quality" => {
                                quality = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timestamp" => {
                                timestamp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssetPropertyValue {
                        quality: quality,
                        timestamp: timestamp,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.AssetPropertyVariant`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertyvariant.html) property type.
    #[derive(Debug, Default)]
    pub struct AssetPropertyVariant {
        /// Property [`BooleanValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertyvariant.html#cfn-iotevents-detectormodel-assetpropertyvariant-booleanvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub boolean_value: Option<crate::Value<String>>,
        /// Property [`DoubleValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertyvariant.html#cfn-iotevents-detectormodel-assetpropertyvariant-doublevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub double_value: Option<crate::Value<String>>,
        /// Property [`IntegerValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertyvariant.html#cfn-iotevents-detectormodel-assetpropertyvariant-integervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub integer_value: Option<crate::Value<String>>,
        /// Property [`StringValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertyvariant.html#cfn-iotevents-detectormodel-assetpropertyvariant-stringvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_value: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for AssetPropertyVariant {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref boolean_value) = self.boolean_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BooleanValue", boolean_value)?;
            }
            if let Some(ref double_value) = self.double_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DoubleValue", double_value)?;
            }
            if let Some(ref integer_value) = self.integer_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegerValue", integer_value)?;
            }
            if let Some(ref string_value) = self.string_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValue", string_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for AssetPropertyVariant {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetPropertyVariant, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssetPropertyVariant;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssetPropertyVariant")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut boolean_value: Option<crate::Value<String>> = None;
                    let mut double_value: Option<crate::Value<String>> = None;
                    let mut integer_value: Option<crate::Value<String>> = None;
                    let mut string_value: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BooleanValue" => {
                                boolean_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DoubleValue" => {
                                double_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntegerValue" => {
                                integer_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringValue" => {
                                string_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssetPropertyVariant {
                        boolean_value: boolean_value,
                        double_value: double_value,
                        integer_value: integer_value,
                        string_value: string_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.ClearTimer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-cleartimer.html) property type.
    #[derive(Debug, Default)]
    pub struct ClearTimer {
        /// Property [`TimerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-cleartimer.html#cfn-iotevents-detectormodel-cleartimer-timername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timer_name: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for ClearTimer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimerName", &self.timer_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ClearTimer {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClearTimer, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClearTimer;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClearTimer")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut timer_name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TimerName" => {
                                timer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClearTimer {
                        timer_name: timer_name.ok_or(::serde::de::Error::missing_field("TimerName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.DetectorModelDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-detectormodeldefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct DetectorModelDefinition {
        /// Property [`InitialStateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-detectormodeldefinition.html#cfn-iotevents-detectormodel-detectormodeldefinition-initialstatename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub initial_state_name: crate::Value<String>,
        /// Property [`States`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-detectormodeldefinition.html#cfn-iotevents-detectormodel-detectormodeldefinition-states).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub states: crate::ValueList<State>,
    }

    impl crate::codec::SerializeValue for DetectorModelDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialStateName", &self.initial_state_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "States", &self.states)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DetectorModelDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DetectorModelDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DetectorModelDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DetectorModelDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut initial_state_name: Option<crate::Value<String>> = None;
                    let mut states: Option<crate::ValueList<State>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InitialStateName" => {
                                initial_state_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "States" => {
                                states = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DetectorModelDefinition {
                        initial_state_name: initial_state_name.ok_or(::serde::de::Error::missing_field("InitialStateName"))?,
                        states: states.ok_or(::serde::de::Error::missing_field("States"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.DynamoDB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html) property type.
    #[derive(Debug, Default)]
    pub struct DynamoDB {
        /// Property [`HashKeyField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html#cfn-iotevents-detectormodel-dynamodb-hashkeyfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hash_key_field: crate::Value<String>,
        /// Property [`HashKeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html#cfn-iotevents-detectormodel-dynamodb-hashkeytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hash_key_type: Option<crate::Value<String>>,
        /// Property [`HashKeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html#cfn-iotevents-detectormodel-dynamodb-hashkeyvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hash_key_value: crate::Value<String>,
        /// Property [`Operation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html#cfn-iotevents-detectormodel-dynamodb-operation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operation: Option<crate::Value<String>>,
        /// Property [`Payload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html#cfn-iotevents-detectormodel-dynamodb-payload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload: Option<crate::Value<Payload>>,
        /// Property [`PayloadField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html#cfn-iotevents-detectormodel-dynamodb-payloadfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload_field: Option<crate::Value<String>>,
        /// Property [`RangeKeyField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html#cfn-iotevents-detectormodel-dynamodb-rangekeyfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range_key_field: Option<crate::Value<String>>,
        /// Property [`RangeKeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html#cfn-iotevents-detectormodel-dynamodb-rangekeytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range_key_type: Option<crate::Value<String>>,
        /// Property [`RangeKeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html#cfn-iotevents-detectormodel-dynamodb-rangekeyvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range_key_value: Option<crate::Value<String>>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html#cfn-iotevents-detectormodel-dynamodb-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for DynamoDB {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HashKeyField", &self.hash_key_field)?;
            if let Some(ref hash_key_type) = self.hash_key_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HashKeyType", hash_key_type)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HashKeyValue", &self.hash_key_value)?;
            if let Some(ref operation) = self.operation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Operation", operation)?;
            }
            if let Some(ref payload) = self.payload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Payload", payload)?;
            }
            if let Some(ref payload_field) = self.payload_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadField", payload_field)?;
            }
            if let Some(ref range_key_field) = self.range_key_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RangeKeyField", range_key_field)?;
            }
            if let Some(ref range_key_type) = self.range_key_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RangeKeyType", range_key_type)?;
            }
            if let Some(ref range_key_value) = self.range_key_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RangeKeyValue", range_key_value)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DynamoDB {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynamoDB, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynamoDB;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynamoDB")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hash_key_field: Option<crate::Value<String>> = None;
                    let mut hash_key_type: Option<crate::Value<String>> = None;
                    let mut hash_key_value: Option<crate::Value<String>> = None;
                    let mut operation: Option<crate::Value<String>> = None;
                    let mut payload: Option<crate::Value<Payload>> = None;
                    let mut payload_field: Option<crate::Value<String>> = None;
                    let mut range_key_field: Option<crate::Value<String>> = None;
                    let mut range_key_type: Option<crate::Value<String>> = None;
                    let mut range_key_value: Option<crate::Value<String>> = None;
                    let mut table_name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HashKeyField" => {
                                hash_key_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HashKeyType" => {
                                hash_key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HashKeyValue" => {
                                hash_key_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Operation" => {
                                operation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Payload" => {
                                payload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PayloadField" => {
                                payload_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RangeKeyField" => {
                                range_key_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RangeKeyType" => {
                                range_key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RangeKeyValue" => {
                                range_key_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynamoDB {
                        hash_key_field: hash_key_field.ok_or(::serde::de::Error::missing_field("HashKeyField"))?,
                        hash_key_type: hash_key_type,
                        hash_key_value: hash_key_value.ok_or(::serde::de::Error::missing_field("HashKeyValue"))?,
                        operation: operation,
                        payload: payload,
                        payload_field: payload_field,
                        range_key_field: range_key_field,
                        range_key_type: range_key_type,
                        range_key_value: range_key_value,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.DynamoDBv2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodbv2.html) property type.
    #[derive(Debug, Default)]
    pub struct DynamoDBv2 {
        /// Property [`Payload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodbv2.html#cfn-iotevents-detectormodel-dynamodbv2-payload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload: Option<crate::Value<Payload>>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodbv2.html#cfn-iotevents-detectormodel-dynamodbv2-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for DynamoDBv2 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref payload) = self.payload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Payload", payload)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DynamoDBv2 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynamoDBv2, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynamoDBv2;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynamoDBv2")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut payload: Option<crate::Value<Payload>> = None;
                    let mut table_name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Payload" => {
                                payload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynamoDBv2 {
                        payload: payload,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.Event`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-event.html) property type.
    #[derive(Debug, Default)]
    pub struct Event {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-event.html#cfn-iotevents-detectormodel-event-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: Option<crate::ValueList<Action>>,
        /// Property [`Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-event.html#cfn-iotevents-detectormodel-event-condition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition: Option<crate::Value<String>>,
        /// Property [`EventName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-event.html#cfn-iotevents-detectormodel-event-eventname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_name: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for Event {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref actions) = self.actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", actions)?;
            }
            if let Some(ref condition) = self.condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Condition", condition)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventName", &self.event_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Event {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Event, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Event;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Event")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<crate::ValueList<Action>> = None;
                    let mut condition: Option<crate::Value<String>> = None;
                    let mut event_name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Condition" => {
                                condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventName" => {
                                event_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Event {
                        actions: actions,
                        condition: condition,
                        event_name: event_name.ok_or(::serde::de::Error::missing_field("EventName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.Firehose`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-firehose.html) property type.
    #[derive(Debug, Default)]
    pub struct Firehose {
        /// Property [`DeliveryStreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-firehose.html#cfn-iotevents-detectormodel-firehose-deliverystreamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delivery_stream_name: crate::Value<String>,
        /// Property [`Payload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-firehose.html#cfn-iotevents-detectormodel-firehose-payload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload: Option<crate::Value<Payload>>,
        /// Property [`Separator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-firehose.html#cfn-iotevents-detectormodel-firehose-separator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub separator: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for Firehose {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamName", &self.delivery_stream_name)?;
            if let Some(ref payload) = self.payload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Payload", payload)?;
            }
            if let Some(ref separator) = self.separator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Separator", separator)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Firehose {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Firehose, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Firehose;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Firehose")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delivery_stream_name: Option<crate::Value<String>> = None;
                    let mut payload: Option<crate::Value<Payload>> = None;
                    let mut separator: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeliveryStreamName" => {
                                delivery_stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Payload" => {
                                payload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Separator" => {
                                separator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Firehose {
                        delivery_stream_name: delivery_stream_name.ok_or(::serde::de::Error::missing_field("DeliveryStreamName"))?,
                        payload: payload,
                        separator: separator,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.IotEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iotevents.html) property type.
    #[derive(Debug, Default)]
    pub struct IotEvents {
        /// Property [`InputName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iotevents.html#cfn-iotevents-detectormodel-iotevents-inputname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_name: crate::Value<String>,
        /// Property [`Payload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iotevents.html#cfn-iotevents-detectormodel-iotevents-payload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload: Option<crate::Value<Payload>>,
    }

    impl crate::codec::SerializeValue for IotEvents {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputName", &self.input_name)?;
            if let Some(ref payload) = self.payload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Payload", payload)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for IotEvents {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IotEvents, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IotEvents;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IotEvents")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input_name: Option<crate::Value<String>> = None;
                    let mut payload: Option<crate::Value<Payload>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputName" => {
                                input_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Payload" => {
                                payload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IotEvents {
                        input_name: input_name.ok_or(::serde::de::Error::missing_field("InputName"))?,
                        payload: payload,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.IotSiteWise`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iotsitewise.html) property type.
    #[derive(Debug, Default)]
    pub struct IotSiteWise {
        /// Property [`AssetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iotsitewise.html#cfn-iotevents-detectormodel-iotsitewise-assetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub asset_id: Option<crate::Value<String>>,
        /// Property [`EntryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iotsitewise.html#cfn-iotevents-detectormodel-iotsitewise-entryid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entry_id: Option<crate::Value<String>>,
        /// Property [`PropertyAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iotsitewise.html#cfn-iotevents-detectormodel-iotsitewise-propertyalias).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_alias: Option<crate::Value<String>>,
        /// Property [`PropertyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iotsitewise.html#cfn-iotevents-detectormodel-iotsitewise-propertyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_id: Option<crate::Value<String>>,
        /// Property [`PropertyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iotsitewise.html#cfn-iotevents-detectormodel-iotsitewise-propertyvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_value: crate::Value<AssetPropertyValue>,
    }

    impl crate::codec::SerializeValue for IotSiteWise {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref asset_id) = self.asset_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetId", asset_id)?;
            }
            if let Some(ref entry_id) = self.entry_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntryId", entry_id)?;
            }
            if let Some(ref property_alias) = self.property_alias {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyAlias", property_alias)?;
            }
            if let Some(ref property_id) = self.property_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyId", property_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyValue", &self.property_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for IotSiteWise {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IotSiteWise, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IotSiteWise;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IotSiteWise")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut asset_id: Option<crate::Value<String>> = None;
                    let mut entry_id: Option<crate::Value<String>> = None;
                    let mut property_alias: Option<crate::Value<String>> = None;
                    let mut property_id: Option<crate::Value<String>> = None;
                    let mut property_value: Option<crate::Value<AssetPropertyValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssetId" => {
                                asset_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntryId" => {
                                entry_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyAlias" => {
                                property_alias = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyId" => {
                                property_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyValue" => {
                                property_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IotSiteWise {
                        asset_id: asset_id,
                        entry_id: entry_id,
                        property_alias: property_alias,
                        property_id: property_id,
                        property_value: property_value.ok_or(::serde::de::Error::missing_field("PropertyValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.IotTopicPublish`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iottopicpublish.html) property type.
    #[derive(Debug, Default)]
    pub struct IotTopicPublish {
        /// Property [`MqttTopic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iottopicpublish.html#cfn-iotevents-detectormodel-iottopicpublish-mqtttopic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mqtt_topic: crate::Value<String>,
        /// Property [`Payload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iottopicpublish.html#cfn-iotevents-detectormodel-iottopicpublish-payload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload: Option<crate::Value<Payload>>,
    }

    impl crate::codec::SerializeValue for IotTopicPublish {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MqttTopic", &self.mqtt_topic)?;
            if let Some(ref payload) = self.payload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Payload", payload)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for IotTopicPublish {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IotTopicPublish, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IotTopicPublish;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IotTopicPublish")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mqtt_topic: Option<crate::Value<String>> = None;
                    let mut payload: Option<crate::Value<Payload>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MqttTopic" => {
                                mqtt_topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Payload" => {
                                payload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IotTopicPublish {
                        mqtt_topic: mqtt_topic.ok_or(::serde::de::Error::missing_field("MqttTopic"))?,
                        payload: payload,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.Lambda`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-lambda.html) property type.
    #[derive(Debug, Default)]
    pub struct Lambda {
        /// Property [`FunctionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-lambda.html#cfn-iotevents-detectormodel-lambda-functionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_arn: crate::Value<String>,
        /// Property [`Payload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-lambda.html#cfn-iotevents-detectormodel-lambda-payload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload: Option<crate::Value<Payload>>,
    }

    impl crate::codec::SerializeValue for Lambda {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionArn", &self.function_arn)?;
            if let Some(ref payload) = self.payload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Payload", payload)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Lambda {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Lambda, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Lambda;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Lambda")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut function_arn: Option<crate::Value<String>> = None;
                    let mut payload: Option<crate::Value<Payload>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionArn" => {
                                function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Payload" => {
                                payload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Lambda {
                        function_arn: function_arn.ok_or(::serde::de::Error::missing_field("FunctionArn"))?,
                        payload: payload,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.OnEnter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-onenter.html) property type.
    #[derive(Debug, Default)]
    pub struct OnEnter {
        /// Property [`Events`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-onenter.html#cfn-iotevents-detectormodel-onenter-events).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub events: Option<crate::ValueList<Event>>,
    }

    impl crate::codec::SerializeValue for OnEnter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref events) = self.events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Events", events)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OnEnter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OnEnter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OnEnter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OnEnter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut events: Option<crate::ValueList<Event>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Events" => {
                                events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OnEnter {
                        events: events,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.OnExit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-onexit.html) property type.
    #[derive(Debug, Default)]
    pub struct OnExit {
        /// Property [`Events`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-onexit.html#cfn-iotevents-detectormodel-onexit-events).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub events: Option<crate::ValueList<Event>>,
    }

    impl crate::codec::SerializeValue for OnExit {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref events) = self.events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Events", events)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OnExit {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OnExit, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OnExit;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OnExit")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut events: Option<crate::ValueList<Event>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Events" => {
                                events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OnExit {
                        events: events,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.OnInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-oninput.html) property type.
    #[derive(Debug, Default)]
    pub struct OnInput {
        /// Property [`Events`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-oninput.html#cfn-iotevents-detectormodel-oninput-events).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub events: Option<crate::ValueList<Event>>,
        /// Property [`TransitionEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-oninput.html#cfn-iotevents-detectormodel-oninput-transitionevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transition_events: Option<crate::ValueList<TransitionEvent>>,
    }

    impl crate::codec::SerializeValue for OnInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref events) = self.events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Events", events)?;
            }
            if let Some(ref transition_events) = self.transition_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitionEvents", transition_events)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OnInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OnInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OnInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OnInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut events: Option<crate::ValueList<Event>> = None;
                    let mut transition_events: Option<crate::ValueList<TransitionEvent>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Events" => {
                                events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransitionEvents" => {
                                transition_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OnInput {
                        events: events,
                        transition_events: transition_events,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.Payload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-payload.html) property type.
    #[derive(Debug, Default)]
    pub struct Payload {
        /// Property [`ContentExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-payload.html#cfn-iotevents-detectormodel-payload-contentexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_expression: crate::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-payload.html#cfn-iotevents-detectormodel-payload-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for Payload {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentExpression", &self.content_expression)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Payload {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Payload, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Payload;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Payload")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_expression: Option<crate::Value<String>> = None;
                    let mut r#type: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentExpression" => {
                                content_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Payload {
                        content_expression: content_expression.ok_or(::serde::de::Error::missing_field("ContentExpression"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.ResetTimer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-resettimer.html) property type.
    #[derive(Debug, Default)]
    pub struct ResetTimer {
        /// Property [`TimerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-resettimer.html#cfn-iotevents-detectormodel-resettimer-timername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timer_name: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for ResetTimer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimerName", &self.timer_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ResetTimer {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResetTimer, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResetTimer;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResetTimer")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut timer_name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TimerName" => {
                                timer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResetTimer {
                        timer_name: timer_name.ok_or(::serde::de::Error::missing_field("TimerName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.SetTimer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-settimer.html) property type.
    #[derive(Debug, Default)]
    pub struct SetTimer {
        /// Property [`DurationExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-settimer.html#cfn-iotevents-detectormodel-settimer-durationexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_expression: Option<crate::Value<String>>,
        /// Property [`Seconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-settimer.html#cfn-iotevents-detectormodel-settimer-seconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub seconds: Option<crate::Value<u32>>,
        /// Property [`TimerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-settimer.html#cfn-iotevents-detectormodel-settimer-timername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timer_name: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for SetTimer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_expression) = self.duration_expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationExpression", duration_expression)?;
            }
            if let Some(ref seconds) = self.seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Seconds", seconds)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimerName", &self.timer_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for SetTimer {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SetTimer, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SetTimer;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SetTimer")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut duration_expression: Option<crate::Value<String>> = None;
                    let mut seconds: Option<crate::Value<u32>> = None;
                    let mut timer_name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DurationExpression" => {
                                duration_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Seconds" => {
                                seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimerName" => {
                                timer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SetTimer {
                        duration_expression: duration_expression,
                        seconds: seconds,
                        timer_name: timer_name.ok_or(::serde::de::Error::missing_field("TimerName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.SetVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-setvariable.html) property type.
    #[derive(Debug, Default)]
    pub struct SetVariable {
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-setvariable.html#cfn-iotevents-detectormodel-setvariable-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: crate::Value<String>,
        /// Property [`VariableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-setvariable.html#cfn-iotevents-detectormodel-setvariable-variablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variable_name: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for SetVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VariableName", &self.variable_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for SetVariable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SetVariable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SetVariable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SetVariable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut value: Option<crate::Value<String>> = None;
                    let mut variable_name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VariableName" => {
                                variable_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SetVariable {
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                        variable_name: variable_name.ok_or(::serde::de::Error::missing_field("VariableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.Sns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-sns.html) property type.
    #[derive(Debug, Default)]
    pub struct Sns {
        /// Property [`Payload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-sns.html#cfn-iotevents-detectormodel-sns-payload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload: Option<crate::Value<Payload>>,
        /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-sns.html#cfn-iotevents-detectormodel-sns-targetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_arn: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for Sns {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref payload) = self.payload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Payload", payload)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", &self.target_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Sns {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Sns, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Sns;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Sns")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut payload: Option<crate::Value<Payload>> = None;
                    let mut target_arn: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Payload" => {
                                payload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetArn" => {
                                target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Sns {
                        payload: payload,
                        target_arn: target_arn.ok_or(::serde::de::Error::missing_field("TargetArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.Sqs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-sqs.html) property type.
    #[derive(Debug, Default)]
    pub struct Sqs {
        /// Property [`Payload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-sqs.html#cfn-iotevents-detectormodel-sqs-payload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload: Option<crate::Value<Payload>>,
        /// Property [`QueueUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-sqs.html#cfn-iotevents-detectormodel-sqs-queueurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_url: crate::Value<String>,
        /// Property [`UseBase64`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-sqs.html#cfn-iotevents-detectormodel-sqs-usebase64).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_base64: Option<crate::Value<bool>>,
    }

    impl crate::codec::SerializeValue for Sqs {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref payload) = self.payload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Payload", payload)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueUrl", &self.queue_url)?;
            if let Some(ref use_base64) = self.use_base64 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseBase64", use_base64)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Sqs {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Sqs, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Sqs;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Sqs")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut payload: Option<crate::Value<Payload>> = None;
                    let mut queue_url: Option<crate::Value<String>> = None;
                    let mut use_base64: Option<crate::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Payload" => {
                                payload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueUrl" => {
                                queue_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseBase64" => {
                                use_base64 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Sqs {
                        payload: payload,
                        queue_url: queue_url.ok_or(::serde::de::Error::missing_field("QueueUrl"))?,
                        use_base64: use_base64,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-state.html) property type.
    #[derive(Debug, Default)]
    pub struct State {
        /// Property [`OnEnter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-state.html#cfn-iotevents-detectormodel-state-onenter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_enter: Option<crate::Value<OnEnter>>,
        /// Property [`OnExit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-state.html#cfn-iotevents-detectormodel-state-onexit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_exit: Option<crate::Value<OnExit>>,
        /// Property [`OnInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-state.html#cfn-iotevents-detectormodel-state-oninput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_input: Option<crate::Value<OnInput>>,
        /// Property [`StateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-state.html#cfn-iotevents-detectormodel-state-statename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state_name: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for State {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref on_enter) = self.on_enter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnEnter", on_enter)?;
            }
            if let Some(ref on_exit) = self.on_exit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnExit", on_exit)?;
            }
            if let Some(ref on_input) = self.on_input {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnInput", on_input)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StateName", &self.state_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for State {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<State, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = State;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type State")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut on_enter: Option<crate::Value<OnEnter>> = None;
                    let mut on_exit: Option<crate::Value<OnExit>> = None;
                    let mut on_input: Option<crate::Value<OnInput>> = None;
                    let mut state_name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OnEnter" => {
                                on_enter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnExit" => {
                                on_exit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnInput" => {
                                on_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StateName" => {
                                state_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(State {
                        on_enter: on_enter,
                        on_exit: on_exit,
                        on_input: on_input,
                        state_name: state_name.ok_or(::serde::de::Error::missing_field("StateName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::DetectorModel.TransitionEvent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-transitionevent.html) property type.
    #[derive(Debug, Default)]
    pub struct TransitionEvent {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-transitionevent.html#cfn-iotevents-detectormodel-transitionevent-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: Option<crate::ValueList<Action>>,
        /// Property [`Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-transitionevent.html#cfn-iotevents-detectormodel-transitionevent-condition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition: crate::Value<String>,
        /// Property [`EventName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-transitionevent.html#cfn-iotevents-detectormodel-transitionevent-eventname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_name: crate::Value<String>,
        /// Property [`NextState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-transitionevent.html#cfn-iotevents-detectormodel-transitionevent-nextstate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next_state: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for TransitionEvent {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref actions) = self.actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", actions)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Condition", &self.condition)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventName", &self.event_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NextState", &self.next_state)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for TransitionEvent {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TransitionEvent, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TransitionEvent;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TransitionEvent")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<crate::ValueList<Action>> = None;
                    let mut condition: Option<crate::Value<String>> = None;
                    let mut event_name: Option<crate::Value<String>> = None;
                    let mut next_state: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Condition" => {
                                condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventName" => {
                                event_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NextState" => {
                                next_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TransitionEvent {
                        actions: actions,
                        condition: condition.ok_or(::serde::de::Error::missing_field("Condition"))?,
                        event_name: event_name.ok_or(::serde::de::Error::missing_field("EventName"))?,
                        next_state: next_state.ok_or(::serde::de::Error::missing_field("NextState"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod input {
    //! Property types for the `Input` resource.

    /// The [`AWS::IoTEvents::Input.Attribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-input-attribute.html) property type.
    #[derive(Debug, Default)]
    pub struct Attribute {
        /// Property [`JsonPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-input-attribute.html#cfn-iotevents-input-attribute-jsonpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json_path: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for Attribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JsonPath", &self.json_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Attribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Attribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Attribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Attribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut json_path: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "JsonPath" => {
                                json_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Attribute {
                        json_path: json_path.ok_or(::serde::de::Error::missing_field("JsonPath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTEvents::Input.InputDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-input-inputdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct InputDefinition {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-input-inputdefinition.html#cfn-iotevents-input-inputdefinition-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: crate::ValueList<Attribute>,
    }

    impl crate::codec::SerializeValue for InputDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", &self.attributes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for InputDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<crate::ValueList<Attribute>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputDefinition {
                        attributes: attributes.ok_or(::serde::de::Error::missing_field("Attributes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
