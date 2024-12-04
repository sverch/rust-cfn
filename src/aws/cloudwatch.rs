//! Types for the `CloudWatch` service.

/// The [`AWS::CloudWatch::Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html) resource type.
#[derive(Debug, Default)]
pub struct Alarm {
    properties: AlarmProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `Alarm` resource.
#[derive(Debug, Default)]
pub struct AlarmProperties {
    /// Property [`ActionsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-actionsenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub actions_enabled: Option<crate::Value<bool>>,
    /// Property [`AlarmActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-alarmactions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alarm_actions: Option<crate::ValueList<String>>,
    /// Property [`AlarmDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-alarmdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alarm_description: Option<crate::Value<String>>,
    /// Property [`AlarmName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-alarmname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub alarm_name: Option<crate::Value<String>>,
    /// Property [`ComparisonOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-comparisonoperator).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub comparison_operator: crate::Value<String>,
    /// Property [`DatapointsToAlarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarm-datapointstoalarm).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub datapoints_to_alarm: Option<crate::Value<u32>>,
    /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-dimension).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dimensions: Option<crate::ValueList<self::alarm::Dimension>>,
    /// Property [`EvaluateLowSampleCountPercentile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-evaluatelowsamplecountpercentile).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub evaluate_low_sample_count_percentile: Option<crate::Value<String>>,
    /// Property [`EvaluationPeriods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-evaluationperiods).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub evaluation_periods: crate::Value<u32>,
    /// Property [`ExtendedStatistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-extendedstatistic).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub extended_statistic: Option<crate::Value<String>>,
    /// Property [`InsufficientDataActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-insufficientdataactions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub insufficient_data_actions: Option<crate::ValueList<String>>,
    /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-metricname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metric_name: Option<crate::Value<String>>,
    /// Property [`Metrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarm-metrics).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metrics: Option<crate::ValueList<self::alarm::MetricDataQuery>>,
    /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-namespace).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub namespace: Option<crate::Value<String>>,
    /// Property [`OKActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-okactions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ok_actions: Option<crate::ValueList<String>>,
    /// Property [`Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-period).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub period: Option<crate::Value<u32>>,
    /// Property [`Statistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-statistic).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub statistic: Option<crate::Value<String>>,
    /// Property [`Threshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-threshold).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub threshold: Option<crate::Value<f64>>,
    /// Property [`ThresholdMetricId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-dynamic-threshold).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub threshold_metric_id: Option<crate::Value<String>>,
    /// Property [`TreatMissingData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-treatmissingdata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub treat_missing_data: Option<crate::Value<String>>,
    /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-unit).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub unit: Option<crate::Value<String>>,
}

impl ::serde::Serialize for AlarmProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref actions_enabled) = self.actions_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionsEnabled", actions_enabled)?;
        }
        if let Some(ref alarm_actions) = self.alarm_actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmActions", alarm_actions)?;
        }
        if let Some(ref alarm_description) = self.alarm_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmDescription", alarm_description)?;
        }
        if let Some(ref alarm_name) = self.alarm_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmName", alarm_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
        if let Some(ref datapoints_to_alarm) = self.datapoints_to_alarm {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatapointsToAlarm", datapoints_to_alarm)?;
        }
        if let Some(ref dimensions) = self.dimensions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", dimensions)?;
        }
        if let Some(ref evaluate_low_sample_count_percentile) = self.evaluate_low_sample_count_percentile {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluateLowSampleCountPercentile", evaluate_low_sample_count_percentile)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluationPeriods", &self.evaluation_periods)?;
        if let Some(ref extended_statistic) = self.extended_statistic {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtendedStatistic", extended_statistic)?;
        }
        if let Some(ref insufficient_data_actions) = self.insufficient_data_actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsufficientDataActions", insufficient_data_actions)?;
        }
        if let Some(ref metric_name) = self.metric_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", metric_name)?;
        }
        if let Some(ref metrics) = self.metrics {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Metrics", metrics)?;
        }
        if let Some(ref namespace) = self.namespace {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", namespace)?;
        }
        if let Some(ref ok_actions) = self.ok_actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OKActions", ok_actions)?;
        }
        if let Some(ref period) = self.period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", period)?;
        }
        if let Some(ref statistic) = self.statistic {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", statistic)?;
        }
        if let Some(ref threshold) = self.threshold {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", threshold)?;
        }
        if let Some(ref threshold_metric_id) = self.threshold_metric_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThresholdMetricId", threshold_metric_id)?;
        }
        if let Some(ref treat_missing_data) = self.treat_missing_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TreatMissingData", treat_missing_data)?;
        }
        if let Some(ref unit) = self.unit {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AlarmProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AlarmProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AlarmProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AlarmProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions_enabled: Option<crate::Value<bool>> = None;
                let mut alarm_actions: Option<crate::ValueList<String>> = None;
                let mut alarm_description: Option<crate::Value<String>> = None;
                let mut alarm_name: Option<crate::Value<String>> = None;
                let mut comparison_operator: Option<crate::Value<String>> = None;
                let mut datapoints_to_alarm: Option<crate::Value<u32>> = None;
                let mut dimensions: Option<crate::ValueList<self::alarm::Dimension>> = None;
                let mut evaluate_low_sample_count_percentile: Option<crate::Value<String>> = None;
                let mut evaluation_periods: Option<crate::Value<u32>> = None;
                let mut extended_statistic: Option<crate::Value<String>> = None;
                let mut insufficient_data_actions: Option<crate::ValueList<String>> = None;
                let mut metric_name: Option<crate::Value<String>> = None;
                let mut metrics: Option<crate::ValueList<self::alarm::MetricDataQuery>> = None;
                let mut namespace: Option<crate::Value<String>> = None;
                let mut ok_actions: Option<crate::ValueList<String>> = None;
                let mut period: Option<crate::Value<u32>> = None;
                let mut statistic: Option<crate::Value<String>> = None;
                let mut threshold: Option<crate::Value<f64>> = None;
                let mut threshold_metric_id: Option<crate::Value<String>> = None;
                let mut treat_missing_data: Option<crate::Value<String>> = None;
                let mut unit: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActionsEnabled" => {
                            actions_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlarmActions" => {
                            alarm_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlarmDescription" => {
                            alarm_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlarmName" => {
                            alarm_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ComparisonOperator" => {
                            comparison_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatapointsToAlarm" => {
                            datapoints_to_alarm = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Dimensions" => {
                            dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EvaluateLowSampleCountPercentile" => {
                            evaluate_low_sample_count_percentile = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EvaluationPeriods" => {
                            evaluation_periods = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExtendedStatistic" => {
                            extended_statistic = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InsufficientDataActions" => {
                            insufficient_data_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricName" => {
                            metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Metrics" => {
                            metrics = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Namespace" => {
                            namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OKActions" => {
                            ok_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Period" => {
                            period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Statistic" => {
                            statistic = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Threshold" => {
                            threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThresholdMetricId" => {
                            threshold_metric_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TreatMissingData" => {
                            treat_missing_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Unit" => {
                            unit = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AlarmProperties {
                    actions_enabled: actions_enabled,
                    alarm_actions: alarm_actions,
                    alarm_description: alarm_description,
                    alarm_name: alarm_name,
                    comparison_operator: comparison_operator.ok_or(::serde::de::Error::missing_field("ComparisonOperator"))?,
                    datapoints_to_alarm: datapoints_to_alarm,
                    dimensions: dimensions,
                    evaluate_low_sample_count_percentile: evaluate_low_sample_count_percentile,
                    evaluation_periods: evaluation_periods.ok_or(::serde::de::Error::missing_field("EvaluationPeriods"))?,
                    extended_statistic: extended_statistic,
                    insufficient_data_actions: insufficient_data_actions,
                    metric_name: metric_name,
                    metrics: metrics,
                    namespace: namespace,
                    ok_actions: ok_actions,
                    period: period,
                    statistic: statistic,
                    threshold: threshold,
                    threshold_metric_id: threshold_metric_id,
                    treat_missing_data: treat_missing_data,
                    unit: unit,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Alarm {
    type Properties = AlarmProperties;
    const TYPE: &'static str = "AWS::CloudWatch::Alarm";
    fn properties(&self) -> &AlarmProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AlarmProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for Alarm {}

impl From<AlarmProperties> for Alarm {
    fn from(properties: AlarmProperties) -> Alarm {
        Alarm { properties, depends_on: None }
    }
}

/// The [`AWS::CloudWatch::AnomalyDetector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-anomalydetector.html) resource type.
#[derive(Debug, Default)]
pub struct AnomalyDetector {
    properties: AnomalyDetectorProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `AnomalyDetector` resource.
#[derive(Debug, Default)]
pub struct AnomalyDetectorProperties {
    /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-anomalydetector.html#cfn-cloudwatch-anomalydetector-configuration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration: Option<crate::Value<self::anomaly_detector::Configuration>>,
    /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-anomalydetector.html#cfn-cloudwatch-anomalydetector-dimensions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dimensions: Option<crate::ValueList<self::anomaly_detector::Dimension>>,
    /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-anomalydetector.html#cfn-cloudwatch-anomalydetector-metricname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub metric_name: crate::Value<String>,
    /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-anomalydetector.html#cfn-cloudwatch-anomalydetector-namespace).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub namespace: crate::Value<String>,
    /// Property [`Stat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-anomalydetector.html#cfn-cloudwatch-anomalydetector-stat).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stat: crate::Value<String>,
}

impl ::serde::Serialize for AnomalyDetectorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref configuration) = self.configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", configuration)?;
        }
        if let Some(ref dimensions) = self.dimensions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", dimensions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stat", &self.stat)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AnomalyDetectorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AnomalyDetectorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AnomalyDetectorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AnomalyDetectorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut configuration: Option<crate::Value<self::anomaly_detector::Configuration>> = None;
                let mut dimensions: Option<crate::ValueList<self::anomaly_detector::Dimension>> = None;
                let mut metric_name: Option<crate::Value<String>> = None;
                let mut namespace: Option<crate::Value<String>> = None;
                let mut stat: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Configuration" => {
                            configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Dimensions" => {
                            dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricName" => {
                            metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Namespace" => {
                            namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Stat" => {
                            stat = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AnomalyDetectorProperties {
                    configuration: configuration,
                    dimensions: dimensions,
                    metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                    namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                    stat: stat.ok_or(::serde::de::Error::missing_field("Stat"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for AnomalyDetector {
    type Properties = AnomalyDetectorProperties;
    const TYPE: &'static str = "AWS::CloudWatch::AnomalyDetector";
    fn properties(&self) -> &AnomalyDetectorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AnomalyDetectorProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for AnomalyDetector {}

impl From<AnomalyDetectorProperties> for AnomalyDetector {
    fn from(properties: AnomalyDetectorProperties) -> AnomalyDetector {
        AnomalyDetector { properties, depends_on: None }
    }
}

/// The [`AWS::CloudWatch::CompositeAlarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html) resource type.
#[derive(Debug, Default)]
pub struct CompositeAlarm {
    properties: CompositeAlarmProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `CompositeAlarm` resource.
#[derive(Debug, Default)]
pub struct CompositeAlarmProperties {
    /// Property [`ActionsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html#cfn-cloudwatch-compositealarm-actionsenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub actions_enabled: Option<crate::Value<bool>>,
    /// Property [`AlarmActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html#cfn-cloudwatch-compositealarm-alarmactions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alarm_actions: Option<crate::ValueList<String>>,
    /// Property [`AlarmDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html#cfn-cloudwatch-compositealarm-alarmdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alarm_description: Option<crate::Value<String>>,
    /// Property [`AlarmName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html#cfn-cloudwatch-compositealarm-alarmname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub alarm_name: crate::Value<String>,
    /// Property [`AlarmRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html#cfn-cloudwatch-compositealarm-alarmrule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alarm_rule: crate::Value<String>,
    /// Property [`InsufficientDataActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html#cfn-cloudwatch-compositealarm-insufficientdataactions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub insufficient_data_actions: Option<crate::ValueList<String>>,
    /// Property [`OKActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html#cfn-cloudwatch-compositealarm-okactions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ok_actions: Option<crate::ValueList<String>>,
}

impl ::serde::Serialize for CompositeAlarmProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref actions_enabled) = self.actions_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionsEnabled", actions_enabled)?;
        }
        if let Some(ref alarm_actions) = self.alarm_actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmActions", alarm_actions)?;
        }
        if let Some(ref alarm_description) = self.alarm_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmDescription", alarm_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmName", &self.alarm_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmRule", &self.alarm_rule)?;
        if let Some(ref insufficient_data_actions) = self.insufficient_data_actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsufficientDataActions", insufficient_data_actions)?;
        }
        if let Some(ref ok_actions) = self.ok_actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OKActions", ok_actions)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CompositeAlarmProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CompositeAlarmProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CompositeAlarmProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CompositeAlarmProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions_enabled: Option<crate::Value<bool>> = None;
                let mut alarm_actions: Option<crate::ValueList<String>> = None;
                let mut alarm_description: Option<crate::Value<String>> = None;
                let mut alarm_name: Option<crate::Value<String>> = None;
                let mut alarm_rule: Option<crate::Value<String>> = None;
                let mut insufficient_data_actions: Option<crate::ValueList<String>> = None;
                let mut ok_actions: Option<crate::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActionsEnabled" => {
                            actions_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlarmActions" => {
                            alarm_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlarmDescription" => {
                            alarm_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlarmName" => {
                            alarm_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlarmRule" => {
                            alarm_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InsufficientDataActions" => {
                            insufficient_data_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OKActions" => {
                            ok_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CompositeAlarmProperties {
                    actions_enabled: actions_enabled,
                    alarm_actions: alarm_actions,
                    alarm_description: alarm_description,
                    alarm_name: alarm_name.ok_or(::serde::de::Error::missing_field("AlarmName"))?,
                    alarm_rule: alarm_rule.ok_or(::serde::de::Error::missing_field("AlarmRule"))?,
                    insufficient_data_actions: insufficient_data_actions,
                    ok_actions: ok_actions,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for CompositeAlarm {
    type Properties = CompositeAlarmProperties;
    const TYPE: &'static str = "AWS::CloudWatch::CompositeAlarm";
    fn properties(&self) -> &CompositeAlarmProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CompositeAlarmProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for CompositeAlarm {}

impl From<CompositeAlarmProperties> for CompositeAlarm {
    fn from(properties: CompositeAlarmProperties) -> CompositeAlarm {
        CompositeAlarm { properties, depends_on: None }
    }
}

/// The [`AWS::CloudWatch::Dashboard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-dashboard.html) resource type.
#[derive(Debug, Default)]
pub struct Dashboard {
    properties: DashboardProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `Dashboard` resource.
#[derive(Debug, Default)]
pub struct DashboardProperties {
    /// Property [`DashboardBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-dashboard.html#cfn-cloudwatch-dashboard-dashboardbody).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dashboard_body: crate::Value<String>,
    /// Property [`DashboardName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-dashboard.html#cfn-cloudwatch-dashboard-dashboardname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dashboard_name: Option<crate::Value<String>>,
}

impl ::serde::Serialize for DashboardProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardBody", &self.dashboard_body)?;
        if let Some(ref dashboard_name) = self.dashboard_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardName", dashboard_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DashboardProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DashboardProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DashboardProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DashboardProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dashboard_body: Option<crate::Value<String>> = None;
                let mut dashboard_name: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DashboardBody" => {
                            dashboard_body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DashboardName" => {
                            dashboard_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DashboardProperties {
                    dashboard_body: dashboard_body.ok_or(::serde::de::Error::missing_field("DashboardBody"))?,
                    dashboard_name: dashboard_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for Dashboard {
    type Properties = DashboardProperties;
    const TYPE: &'static str = "AWS::CloudWatch::Dashboard";
    fn properties(&self) -> &DashboardProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DashboardProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for Dashboard {}

impl From<DashboardProperties> for Dashboard {
    fn from(properties: DashboardProperties) -> Dashboard {
        Dashboard { properties, depends_on: None }
    }
}

/// The [`AWS::CloudWatch::InsightRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-insightrule.html) resource type.
#[derive(Debug, Default)]
pub struct InsightRule {
    properties: InsightRuleProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `InsightRule` resource.
#[derive(Debug, Default)]
pub struct InsightRuleProperties {
    /// Property [`RuleBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-insightrule.html#cfn-cloudwatch-insightrule-rulebody).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule_body: crate::Value<String>,
    /// Property [`RuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-insightrule.html#cfn-cloudwatch-insightrule-rulename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rule_name: crate::Value<String>,
    /// Property [`RuleState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-insightrule.html#cfn-cloudwatch-insightrule-rulestate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule_state: crate::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-insightrule.html#cfn-cloudwatch-insightrule-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::Value<self::insight_rule::Tags>>,
}

impl ::serde::Serialize for InsightRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleBody", &self.rule_body)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleName", &self.rule_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleState", &self.rule_state)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InsightRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InsightRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InsightRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InsightRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut rule_body: Option<crate::Value<String>> = None;
                let mut rule_name: Option<crate::Value<String>> = None;
                let mut rule_state: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::Value<self::insight_rule::Tags>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "RuleBody" => {
                            rule_body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleName" => {
                            rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleState" => {
                            rule_state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InsightRuleProperties {
                    rule_body: rule_body.ok_or(::serde::de::Error::missing_field("RuleBody"))?,
                    rule_name: rule_name.ok_or(::serde::de::Error::missing_field("RuleName"))?,
                    rule_state: rule_state.ok_or(::serde::de::Error::missing_field("RuleState"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for InsightRule {
    type Properties = InsightRuleProperties;
    const TYPE: &'static str = "AWS::CloudWatch::InsightRule";
    fn properties(&self) -> &InsightRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InsightRuleProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for InsightRule {}

impl From<InsightRuleProperties> for InsightRule {
    fn from(properties: InsightRuleProperties) -> InsightRule {
        InsightRule { properties, depends_on: None }
    }
}

/// The [`AWS::CloudWatch::MetricStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-metricstream.html) resource type.
#[derive(Debug, Default)]
pub struct MetricStream {
    properties: MetricStreamProperties,
    depends_on: Option<crate::DependsOn>,
}

/// Properties for the `MetricStream` resource.
#[derive(Debug, Default)]
pub struct MetricStreamProperties {
    /// Property [`ExcludeFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-metricstream.html#cfn-cloudwatch-metricstream-excludefilters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub exclude_filters: Option<crate::ValueList<self::metric_stream::MetricStreamFilter>>,
    /// Property [`FirehoseArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-metricstream.html#cfn-cloudwatch-metricstream-firehosearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub firehose_arn: crate::Value<String>,
    /// Property [`IncludeFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-metricstream.html#cfn-cloudwatch-metricstream-includefilters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub include_filters: Option<crate::ValueList<self::metric_stream::MetricStreamFilter>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-metricstream.html#cfn-cloudwatch-metricstream-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<crate::Value<String>>,
    /// Property [`OutputFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-metricstream.html#cfn-cloudwatch-metricstream-outputformat).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub output_format: crate::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-metricstream.html#cfn-cloudwatch-metricstream-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: crate::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-metricstream.html#cfn-cloudwatch-metricstream-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for MetricStreamProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref exclude_filters) = self.exclude_filters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeFilters", exclude_filters)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirehoseArn", &self.firehose_arn)?;
        if let Some(ref include_filters) = self.include_filters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeFilters", include_filters)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputFormat", &self.output_format)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MetricStreamProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricStreamProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MetricStreamProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MetricStreamProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut exclude_filters: Option<crate::ValueList<self::metric_stream::MetricStreamFilter>> = None;
                let mut firehose_arn: Option<crate::Value<String>> = None;
                let mut include_filters: Option<crate::ValueList<self::metric_stream::MetricStreamFilter>> = None;
                let mut name: Option<crate::Value<String>> = None;
                let mut output_format: Option<crate::Value<String>> = None;
                let mut role_arn: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ExcludeFilters" => {
                            exclude_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FirehoseArn" => {
                            firehose_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IncludeFilters" => {
                            include_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OutputFormat" => {
                            output_format = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(MetricStreamProperties {
                    exclude_filters: exclude_filters,
                    firehose_arn: firehose_arn.ok_or(::serde::de::Error::missing_field("FirehoseArn"))?,
                    include_filters: include_filters,
                    name: name,
                    output_format: output_format.ok_or(::serde::de::Error::missing_field("OutputFormat"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for MetricStream {
    type Properties = MetricStreamProperties;
    const TYPE: &'static str = "AWS::CloudWatch::MetricStream";
    fn properties(&self) -> &MetricStreamProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MetricStreamProperties {
        &mut self.properties
    }
    fn depends_on(&self) -> &Option<crate::DependsOn> {
        &self.depends_on
    }
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn> {
        &mut self.depends_on
    }
}

impl crate::private::Sealed for MetricStream {}

impl From<MetricStreamProperties> for MetricStream {
    fn from(properties: MetricStreamProperties) -> MetricStream {
        MetricStream { properties, depends_on: None }
    }
}

pub mod alarm {
    //! Property types for the `Alarm` resource.

    /// The [`AWS::CloudWatch::Alarm.Dimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-dimension.html) property type.
    #[derive(Debug, Default)]
    pub struct Dimension {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-dimension.html#cfn-cloudwatch-alarm-dimension-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: crate::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-dimension.html#cfn-cloudwatch-alarm-dimension-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for Dimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Dimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Dimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Dimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Dimension")
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

                    Ok(Dimension {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudWatch::Alarm.Metric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metric.html) property type.
    #[derive(Debug, Default)]
    pub struct Metric {
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metric.html#cfn-cloudwatch-alarm-metric-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: Option<crate::ValueList<Dimension>>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metric.html#cfn-cloudwatch-alarm-metric-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: Option<crate::Value<String>>,
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metric.html#cfn-cloudwatch-alarm-metric-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for Metric {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dimensions) = self.dimensions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", dimensions)?;
            }
            if let Some(ref metric_name) = self.metric_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", metric_name)?;
            }
            if let Some(ref namespace) = self.namespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", namespace)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Metric {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Metric, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Metric;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Metric")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimensions: Option<crate::ValueList<Dimension>> = None;
                    let mut metric_name: Option<crate::Value<String>> = None;
                    let mut namespace: Option<crate::Value<String>> = None;

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
                            _ => {}
                        }
                    }

                    Ok(Metric {
                        dimensions: dimensions,
                        metric_name: metric_name,
                        namespace: namespace,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudWatch::Alarm.MetricDataQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricdataquery.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricDataQuery {
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricdataquery.html#cfn-cloudwatch-alarm-metricdataquery-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: Option<crate::Value<String>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricdataquery.html#cfn-cloudwatch-alarm-metricdataquery-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: crate::Value<String>,
        /// Property [`Label`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricdataquery.html#cfn-cloudwatch-alarm-metricdataquery-label).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub label: Option<crate::Value<String>>,
        /// Property [`MetricStat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricdataquery.html#cfn-cloudwatch-alarm-metricdataquery-metricstat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_stat: Option<crate::Value<MetricStat>>,
        /// Property [`Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricdataquery.html#cfn-cloudwatch-alarm-metricdataquery-period).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub period: Option<crate::Value<u32>>,
        /// Property [`ReturnData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricdataquery.html#cfn-cloudwatch-alarm-metricdataquery-returndata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub return_data: Option<crate::Value<bool>>,
    }

    impl crate::codec::SerializeValue for MetricDataQuery {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref expression) = self.expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", expression)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref label) = self.label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Label", label)?;
            }
            if let Some(ref metric_stat) = self.metric_stat {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricStat", metric_stat)?;
            }
            if let Some(ref period) = self.period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", period)?;
            }
            if let Some(ref return_data) = self.return_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReturnData", return_data)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for MetricDataQuery {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricDataQuery, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricDataQuery;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricDataQuery")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut expression: Option<crate::Value<String>> = None;
                    let mut id: Option<crate::Value<String>> = None;
                    let mut label: Option<crate::Value<String>> = None;
                    let mut metric_stat: Option<crate::Value<MetricStat>> = None;
                    let mut period: Option<crate::Value<u32>> = None;
                    let mut return_data: Option<crate::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Label" => {
                                label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricStat" => {
                                metric_stat = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Period" => {
                                period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReturnData" => {
                                return_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricDataQuery {
                        expression: expression,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        label: label,
                        metric_stat: metric_stat,
                        period: period,
                        return_data: return_data,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudWatch::Alarm.MetricStat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricstat.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricStat {
        /// Property [`Metric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricstat.html#cfn-cloudwatch-alarm-metricstat-metric).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric: crate::Value<Metric>,
        /// Property [`Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricstat.html#cfn-cloudwatch-alarm-metricstat-period).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub period: crate::Value<u32>,
        /// Property [`Stat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricstat.html#cfn-cloudwatch-alarm-metricstat-stat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stat: crate::Value<String>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricstat.html#cfn-cloudwatch-alarm-metricstat-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for MetricStat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Metric", &self.metric)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", &self.period)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stat", &self.stat)?;
            if let Some(ref unit) = self.unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for MetricStat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricStat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricStat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricStat")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut metric: Option<crate::Value<Metric>> = None;
                    let mut period: Option<crate::Value<u32>> = None;
                    let mut stat: Option<crate::Value<String>> = None;
                    let mut unit: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Metric" => {
                                metric = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Period" => {
                                period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Stat" => {
                                stat = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricStat {
                        metric: metric.ok_or(::serde::de::Error::missing_field("Metric"))?,
                        period: period.ok_or(::serde::de::Error::missing_field("Period"))?,
                        stat: stat.ok_or(::serde::de::Error::missing_field("Stat"))?,
                        unit: unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod anomaly_detector {
    //! Property types for the `AnomalyDetector` resource.

    /// The [`AWS::CloudWatch::AnomalyDetector.Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-configuration.html) property type.
    #[derive(Debug, Default)]
    pub struct Configuration {
        /// Property [`ExcludedTimeRanges`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-configuration.html#cfn-cloudwatch-anomalydetector-configuration-excludedtimeranges).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded_time_ranges: Option<crate::ValueList<Range>>,
        /// Property [`MetricTimeZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-configuration.html#cfn-cloudwatch-anomalydetector-configuration-metrictimezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_time_zone: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for Configuration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref excluded_time_ranges) = self.excluded_time_ranges {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedTimeRanges", excluded_time_ranges)?;
            }
            if let Some(ref metric_time_zone) = self.metric_time_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricTimeZone", metric_time_zone)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Configuration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Configuration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Configuration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Configuration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut excluded_time_ranges: Option<crate::ValueList<Range>> = None;
                    let mut metric_time_zone: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExcludedTimeRanges" => {
                                excluded_time_ranges = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricTimeZone" => {
                                metric_time_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Configuration {
                        excluded_time_ranges: excluded_time_ranges,
                        metric_time_zone: metric_time_zone,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudWatch::AnomalyDetector.Dimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-dimension.html) property type.
    #[derive(Debug, Default)]
    pub struct Dimension {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-dimension.html#cfn-cloudwatch-anomalydetector-dimension-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: crate::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-dimension.html#cfn-cloudwatch-anomalydetector-dimension-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for Dimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Dimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Dimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Dimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Dimension")
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

                    Ok(Dimension {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudWatch::AnomalyDetector.Range`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-range.html) property type.
    #[derive(Debug, Default)]
    pub struct Range {
        /// Property [`EndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-range.html#cfn-cloudwatch-anomalydetector-range-endtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_time: crate::Value<String>,
        /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-range.html#cfn-cloudwatch-anomalydetector-range-starttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_time: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for Range {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTime", &self.end_time)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", &self.start_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Range {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Range, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Range;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Range")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end_time: Option<crate::Value<String>> = None;
                    let mut start_time: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndTime" => {
                                end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTime" => {
                                start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Range {
                        end_time: end_time.ok_or(::serde::de::Error::missing_field("EndTime"))?,
                        start_time: start_time.ok_or(::serde::de::Error::missing_field("StartTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod insight_rule {
    //! Property types for the `InsightRule` resource.

    /// The [`AWS::CloudWatch::InsightRule.Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-insightrule-tags.html) property type.
    #[derive(Debug, Default)]
    pub struct Tags {
    }

    impl crate::codec::SerializeValue for Tags {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Tags {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Tags, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Tags;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Tags")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Tags {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod metric_stream {
    //! Property types for the `MetricStream` resource.

    /// The [`AWS::CloudWatch::MetricStream.MetricStreamFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-metricstream-metricstreamfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricStreamFilter {
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-metricstream-metricstreamfilter.html#cfn-cloudwatch-metricstream-metricstreamfilter-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for MetricStreamFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for MetricStreamFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricStreamFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricStreamFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricStreamFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut namespace: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricStreamFilter {
                        namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
