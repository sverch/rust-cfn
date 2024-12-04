//! Types for the `DocDB` service.

/// The [`AWS::DocDB::DBCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html) resource type.
#[derive(Debug, Default)]
pub struct DBCluster {
    properties: DBClusterProperties
}

/// Properties for the `DBCluster` resource.
#[derive(Debug, Default)]
pub struct DBClusterProperties {
    /// Property [`AvailabilityZones`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-availabilityzones).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zones: Option<crate::ValueList<String>>,
    /// Property [`BackupRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-backupretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backup_retention_period: Option<crate::Value<u32>>,
    /// Property [`DBClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-dbclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_cluster_identifier: Option<crate::Value<String>>,
    /// Property [`DBClusterParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-dbclusterparametergroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_cluster_parameter_group_name: Option<crate::Value<String>>,
    /// Property [`DBSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-dbsubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_subnet_group_name: Option<crate::Value<String>>,
    /// Property [`DeletionProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-deletionprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deletion_protection: Option<crate::Value<bool>>,
    /// Property [`EnableCloudwatchLogsExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-enablecloudwatchlogsexports).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_cloudwatch_logs_exports: Option<crate::ValueList<String>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-engineversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_version: Option<crate::Value<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<crate::Value<String>>,
    /// Property [`MasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-masteruserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub master_user_password: crate::Value<String>,
    /// Property [`MasterUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-masterusername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub master_username: crate::Value<String>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-port).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub port: Option<crate::Value<u32>>,
    /// Property [`PreferredBackupWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-preferredbackupwindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_backup_window: Option<crate::Value<String>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-preferredmaintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_maintenance_window: Option<crate::Value<String>>,
    /// Property [`SnapshotIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-snapshotidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_identifier: Option<crate::Value<String>>,
    /// Property [`StorageEncrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-storageencrypted).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_encrypted: Option<crate::Value<bool>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html#cfn-docdb-dbcluster-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: Option<crate::ValueList<String>>,
}

impl ::serde::Serialize for DBClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref availability_zones) = self.availability_zones {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZones", availability_zones)?;
        }
        if let Some(ref backup_retention_period) = self.backup_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupRetentionPeriod", backup_retention_period)?;
        }
        if let Some(ref db_cluster_identifier) = self.db_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterIdentifier", db_cluster_identifier)?;
        }
        if let Some(ref db_cluster_parameter_group_name) = self.db_cluster_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterParameterGroupName", db_cluster_parameter_group_name)?;
        }
        if let Some(ref db_subnet_group_name) = self.db_subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupName", db_subnet_group_name)?;
        }
        if let Some(ref deletion_protection) = self.deletion_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeletionProtection", deletion_protection)?;
        }
        if let Some(ref enable_cloudwatch_logs_exports) = self.enable_cloudwatch_logs_exports {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableCloudwatchLogsExports", enable_cloudwatch_logs_exports)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", &self.master_user_password)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUsername", &self.master_username)?;
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref preferred_backup_window) = self.preferred_backup_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredBackupWindow", preferred_backup_window)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref snapshot_identifier) = self.snapshot_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotIdentifier", snapshot_identifier)?;
        }
        if let Some(ref storage_encrypted) = self.storage_encrypted {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageEncrypted", storage_encrypted)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut availability_zones: Option<crate::ValueList<String>> = None;
                let mut backup_retention_period: Option<crate::Value<u32>> = None;
                let mut db_cluster_identifier: Option<crate::Value<String>> = None;
                let mut db_cluster_parameter_group_name: Option<crate::Value<String>> = None;
                let mut db_subnet_group_name: Option<crate::Value<String>> = None;
                let mut deletion_protection: Option<crate::Value<bool>> = None;
                let mut enable_cloudwatch_logs_exports: Option<crate::ValueList<String>> = None;
                let mut engine_version: Option<crate::Value<String>> = None;
                let mut kms_key_id: Option<crate::Value<String>> = None;
                let mut master_user_password: Option<crate::Value<String>> = None;
                let mut master_username: Option<crate::Value<String>> = None;
                let mut port: Option<crate::Value<u32>> = None;
                let mut preferred_backup_window: Option<crate::Value<String>> = None;
                let mut preferred_maintenance_window: Option<crate::Value<String>> = None;
                let mut snapshot_identifier: Option<crate::Value<String>> = None;
                let mut storage_encrypted: Option<crate::Value<bool>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;
                let mut vpc_security_group_ids: Option<crate::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AvailabilityZones" => {
                            availability_zones = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupRetentionPeriod" => {
                            backup_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBClusterIdentifier" => {
                            db_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBClusterParameterGroupName" => {
                            db_cluster_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSubnetGroupName" => {
                            db_subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeletionProtection" => {
                            deletion_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableCloudwatchLogsExports" => {
                            enable_cloudwatch_logs_exports = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUserPassword" => {
                            master_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUsername" => {
                            master_username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredBackupWindow" => {
                            preferred_backup_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotIdentifier" => {
                            snapshot_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageEncrypted" => {
                            storage_encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBClusterProperties {
                    availability_zones: availability_zones,
                    backup_retention_period: backup_retention_period,
                    db_cluster_identifier: db_cluster_identifier,
                    db_cluster_parameter_group_name: db_cluster_parameter_group_name,
                    db_subnet_group_name: db_subnet_group_name,
                    deletion_protection: deletion_protection,
                    enable_cloudwatch_logs_exports: enable_cloudwatch_logs_exports,
                    engine_version: engine_version,
                    kms_key_id: kms_key_id,
                    master_user_password: master_user_password.ok_or(::serde::de::Error::missing_field("MasterUserPassword"))?,
                    master_username: master_username.ok_or(::serde::de::Error::missing_field("MasterUsername"))?,
                    port: port,
                    preferred_backup_window: preferred_backup_window,
                    preferred_maintenance_window: preferred_maintenance_window,
                    snapshot_identifier: snapshot_identifier,
                    storage_encrypted: storage_encrypted,
                    tags: tags,
                    vpc_security_group_ids: vpc_security_group_ids,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBCluster {
    type Properties = DBClusterProperties;
    const TYPE: &'static str = "AWS::DocDB::DBCluster";
    fn properties(&self) -> &DBClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBClusterProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for DBCluster {}

impl From<DBClusterProperties> for DBCluster {
    fn from(properties: DBClusterProperties) -> DBCluster {
        DBCluster { properties }
    }
}

/// The [`AWS::DocDB::DBClusterParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbclusterparametergroup.html) resource type.
#[derive(Debug, Default)]
pub struct DBClusterParameterGroup {
    properties: DBClusterParameterGroupProperties
}

/// Properties for the `DBClusterParameterGroup` resource.
#[derive(Debug, Default)]
pub struct DBClusterParameterGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbclusterparametergroup.html#cfn-docdb-dbclusterparametergroup-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: crate::Value<String>,
    /// Property [`Family`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbclusterparametergroup.html#cfn-docdb-dbclusterparametergroup-family).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub family: crate::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbclusterparametergroup.html#cfn-docdb-dbclusterparametergroup-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<crate::Value<String>>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbclusterparametergroup.html#cfn-docdb-dbclusterparametergroup-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: crate::Value<crate::json::Value>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbclusterparametergroup.html#cfn-docdb-dbclusterparametergroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for DBClusterParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Family", &self.family)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", &self.parameters)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBClusterParameterGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBClusterParameterGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBClusterParameterGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBClusterParameterGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<crate::Value<String>> = None;
                let mut family: Option<crate::Value<String>> = None;
                let mut name: Option<crate::Value<String>> = None;
                let mut parameters: Option<crate::Value<crate::json::Value>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Family" => {
                            family = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBClusterParameterGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    family: family.ok_or(::serde::de::Error::missing_field("Family"))?,
                    name: name,
                    parameters: parameters.ok_or(::serde::de::Error::missing_field("Parameters"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBClusterParameterGroup {
    type Properties = DBClusterParameterGroupProperties;
    const TYPE: &'static str = "AWS::DocDB::DBClusterParameterGroup";
    fn properties(&self) -> &DBClusterParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBClusterParameterGroupProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for DBClusterParameterGroup {}

impl From<DBClusterParameterGroupProperties> for DBClusterParameterGroup {
    fn from(properties: DBClusterParameterGroupProperties) -> DBClusterParameterGroup {
        DBClusterParameterGroup { properties }
    }
}

/// The [`AWS::DocDB::DBInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbinstance.html) resource type.
#[derive(Debug, Default)]
pub struct DBInstance {
    properties: DBInstanceProperties
}

/// Properties for the `DBInstance` resource.
#[derive(Debug, Default)]
pub struct DBInstanceProperties {
    /// Property [`AutoMinorVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbinstance.html#cfn-docdb-dbinstance-autominorversionupgrade).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_minor_version_upgrade: Option<crate::Value<bool>>,
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbinstance.html#cfn-docdb-dbinstance-availabilityzone).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zone: Option<crate::Value<String>>,
    /// Property [`DBClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbinstance.html#cfn-docdb-dbinstance-dbclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_cluster_identifier: crate::Value<String>,
    /// Property [`DBInstanceClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbinstance.html#cfn-docdb-dbinstance-dbinstanceclass).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_instance_class: crate::Value<String>,
    /// Property [`DBInstanceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbinstance.html#cfn-docdb-dbinstance-dbinstanceidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_instance_identifier: Option<crate::Value<String>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbinstance.html#cfn-docdb-dbinstance-preferredmaintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_maintenance_window: Option<crate::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbinstance.html#cfn-docdb-dbinstance-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for DBInstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_minor_version_upgrade) = self.auto_minor_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", auto_minor_version_upgrade)?;
        }
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterIdentifier", &self.db_cluster_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceClass", &self.db_instance_class)?;
        if let Some(ref db_instance_identifier) = self.db_instance_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceIdentifier", db_instance_identifier)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBInstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBInstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBInstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBInstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_minor_version_upgrade: Option<crate::Value<bool>> = None;
                let mut availability_zone: Option<crate::Value<String>> = None;
                let mut db_cluster_identifier: Option<crate::Value<String>> = None;
                let mut db_instance_class: Option<crate::Value<String>> = None;
                let mut db_instance_identifier: Option<crate::Value<String>> = None;
                let mut preferred_maintenance_window: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBClusterIdentifier" => {
                            db_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBInstanceClass" => {
                            db_instance_class = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBInstanceIdentifier" => {
                            db_instance_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBInstanceProperties {
                    auto_minor_version_upgrade: auto_minor_version_upgrade,
                    availability_zone: availability_zone,
                    db_cluster_identifier: db_cluster_identifier.ok_or(::serde::de::Error::missing_field("DBClusterIdentifier"))?,
                    db_instance_class: db_instance_class.ok_or(::serde::de::Error::missing_field("DBInstanceClass"))?,
                    db_instance_identifier: db_instance_identifier,
                    preferred_maintenance_window: preferred_maintenance_window,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBInstance {
    type Properties = DBInstanceProperties;
    const TYPE: &'static str = "AWS::DocDB::DBInstance";
    fn properties(&self) -> &DBInstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBInstanceProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for DBInstance {}

impl From<DBInstanceProperties> for DBInstance {
    fn from(properties: DBInstanceProperties) -> DBInstance {
        DBInstance { properties }
    }
}

/// The [`AWS::DocDB::DBSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbsubnetgroup.html) resource type.
#[derive(Debug, Default)]
pub struct DBSubnetGroup {
    properties: DBSubnetGroupProperties
}

/// Properties for the `DBSubnetGroup` resource.
#[derive(Debug, Default)]
pub struct DBSubnetGroupProperties {
    /// Property [`DBSubnetGroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbsubnetgroup.html#cfn-docdb-dbsubnetgroup-dbsubnetgroupdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_subnet_group_description: crate::Value<String>,
    /// Property [`DBSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbsubnetgroup.html#cfn-docdb-dbsubnetgroup-dbsubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_subnet_group_name: Option<crate::Value<String>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbsubnetgroup.html#cfn-docdb-dbsubnetgroup-subnetids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_ids: crate::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbsubnetgroup.html#cfn-docdb-dbsubnetgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for DBSubnetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupDescription", &self.db_subnet_group_description)?;
        if let Some(ref db_subnet_group_name) = self.db_subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupName", db_subnet_group_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBSubnetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBSubnetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBSubnetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBSubnetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut db_subnet_group_description: Option<crate::Value<String>> = None;
                let mut db_subnet_group_name: Option<crate::Value<String>> = None;
                let mut subnet_ids: Option<crate::ValueList<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DBSubnetGroupDescription" => {
                            db_subnet_group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSubnetGroupName" => {
                            db_subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBSubnetGroupProperties {
                    db_subnet_group_description: db_subnet_group_description.ok_or(::serde::de::Error::missing_field("DBSubnetGroupDescription"))?,
                    db_subnet_group_name: db_subnet_group_name,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBSubnetGroup {
    type Properties = DBSubnetGroupProperties;
    const TYPE: &'static str = "AWS::DocDB::DBSubnetGroup";
    fn properties(&self) -> &DBSubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSubnetGroupProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for DBSubnetGroup {}

impl From<DBSubnetGroupProperties> for DBSubnetGroup {
    fn from(properties: DBSubnetGroupProperties) -> DBSubnetGroup {
        DBSubnetGroup { properties }
    }
}
