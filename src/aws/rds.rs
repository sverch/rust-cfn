//! Types for the `RDS` service.

/// The [`AWS::RDS::DBCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html) resource type.
#[derive(Debug, Default)]
pub struct DBCluster {
    properties: DBClusterProperties
}

/// Properties for the `DBCluster` resource.
#[derive(Debug, Default)]
pub struct DBClusterProperties {
    /// Property [`AssociatedRoles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-associatedroles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub associated_roles: Option<crate::ValueList<self::db_cluster::DBClusterRole>>,
    /// Property [`AvailabilityZones`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-availabilityzones).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zones: Option<crate::ValueList<String>>,
    /// Property [`BacktrackWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-backtrackwindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backtrack_window: Option<crate::Value<u64>>,
    /// Property [`BackupRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-backuprententionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backup_retention_period: Option<crate::Value<u32>>,
    /// Property [`CopyTagsToSnapshot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-copytagstosnapshot).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub copy_tags_to_snapshot: Option<crate::Value<bool>>,
    /// Property [`DBClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-dbclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_cluster_identifier: Option<crate::Value<String>>,
    /// Property [`DBClusterParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-dbclusterparametergroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_cluster_parameter_group_name: Option<crate::Value<String>>,
    /// Property [`DBSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-dbsubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_subnet_group_name: Option<crate::Value<String>>,
    /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-databasename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub database_name: Option<crate::Value<String>>,
    /// Property [`DeletionProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-deletionprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deletion_protection: Option<crate::Value<bool>>,
    /// Property [`EnableCloudwatchLogsExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-enablecloudwatchlogsexports).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_cloudwatch_logs_exports: Option<crate::ValueList<String>>,
    /// Property [`EnableHttpEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-enablehttpendpoint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_http_endpoint: Option<crate::Value<bool>>,
    /// Property [`EnableIAMDatabaseAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-enableiamdatabaseauthentication).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_iam_database_authentication: Option<crate::Value<bool>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-engine).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub engine: crate::Value<String>,
    /// Property [`EngineMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-enginemode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_mode: Option<crate::Value<String>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-engineversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_version: Option<crate::Value<String>>,
    /// Property [`GlobalClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-globalclusteridentifier).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub global_cluster_identifier: Option<crate::Value<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<crate::Value<String>>,
    /// Property [`MasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-masteruserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub master_user_password: Option<crate::Value<String>>,
    /// Property [`MasterUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-masterusername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub master_username: Option<crate::Value<String>>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-port).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub port: Option<crate::Value<u32>>,
    /// Property [`PreferredBackupWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-preferredbackupwindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_backup_window: Option<crate::Value<String>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-preferredmaintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_maintenance_window: Option<crate::Value<String>>,
    /// Property [`ReplicationSourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-replicationsourceidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_source_identifier: Option<crate::Value<String>>,
    /// Property [`RestoreType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-restoretype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub restore_type: Option<crate::Value<String>>,
    /// Property [`ScalingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-scalingconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scaling_configuration: Option<crate::Value<self::db_cluster::ScalingConfiguration>>,
    /// Property [`SnapshotIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-snapshotidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_identifier: Option<crate::Value<String>>,
    /// Property [`SourceDBClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-sourcedbclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_db_cluster_identifier: Option<crate::Value<String>>,
    /// Property [`SourceRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-sourceregion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_region: Option<crate::Value<String>>,
    /// Property [`StorageEncrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-storageencrypted).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_encrypted: Option<crate::Value<bool>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
    /// Property [`UseLatestRestorableTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-uselatestrestorabletime).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub use_latest_restorable_time: Option<crate::Value<bool>>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: Option<crate::ValueList<String>>,
}

impl ::serde::Serialize for DBClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref associated_roles) = self.associated_roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatedRoles", associated_roles)?;
        }
        if let Some(ref availability_zones) = self.availability_zones {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZones", availability_zones)?;
        }
        if let Some(ref backtrack_window) = self.backtrack_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BacktrackWindow", backtrack_window)?;
        }
        if let Some(ref backup_retention_period) = self.backup_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupRetentionPeriod", backup_retention_period)?;
        }
        if let Some(ref copy_tags_to_snapshot) = self.copy_tags_to_snapshot {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTagsToSnapshot", copy_tags_to_snapshot)?;
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
        if let Some(ref database_name) = self.database_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
        }
        if let Some(ref deletion_protection) = self.deletion_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeletionProtection", deletion_protection)?;
        }
        if let Some(ref enable_cloudwatch_logs_exports) = self.enable_cloudwatch_logs_exports {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableCloudwatchLogsExports", enable_cloudwatch_logs_exports)?;
        }
        if let Some(ref enable_http_endpoint) = self.enable_http_endpoint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableHttpEndpoint", enable_http_endpoint)?;
        }
        if let Some(ref enable_iam_database_authentication) = self.enable_iam_database_authentication {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableIAMDatabaseAuthentication", enable_iam_database_authentication)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", &self.engine)?;
        if let Some(ref engine_mode) = self.engine_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineMode", engine_mode)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref global_cluster_identifier) = self.global_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalClusterIdentifier", global_cluster_identifier)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref master_user_password) = self.master_user_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", master_user_password)?;
        }
        if let Some(ref master_username) = self.master_username {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUsername", master_username)?;
        }
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref preferred_backup_window) = self.preferred_backup_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredBackupWindow", preferred_backup_window)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref replication_source_identifier) = self.replication_source_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSourceIdentifier", replication_source_identifier)?;
        }
        if let Some(ref restore_type) = self.restore_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestoreType", restore_type)?;
        }
        if let Some(ref scaling_configuration) = self.scaling_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingConfiguration", scaling_configuration)?;
        }
        if let Some(ref snapshot_identifier) = self.snapshot_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotIdentifier", snapshot_identifier)?;
        }
        if let Some(ref source_db_cluster_identifier) = self.source_db_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDBClusterIdentifier", source_db_cluster_identifier)?;
        }
        if let Some(ref source_region) = self.source_region {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceRegion", source_region)?;
        }
        if let Some(ref storage_encrypted) = self.storage_encrypted {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageEncrypted", storage_encrypted)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref use_latest_restorable_time) = self.use_latest_restorable_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseLatestRestorableTime", use_latest_restorable_time)?;
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
                let mut associated_roles: Option<crate::ValueList<self::db_cluster::DBClusterRole>> = None;
                let mut availability_zones: Option<crate::ValueList<String>> = None;
                let mut backtrack_window: Option<crate::Value<u64>> = None;
                let mut backup_retention_period: Option<crate::Value<u32>> = None;
                let mut copy_tags_to_snapshot: Option<crate::Value<bool>> = None;
                let mut db_cluster_identifier: Option<crate::Value<String>> = None;
                let mut db_cluster_parameter_group_name: Option<crate::Value<String>> = None;
                let mut db_subnet_group_name: Option<crate::Value<String>> = None;
                let mut database_name: Option<crate::Value<String>> = None;
                let mut deletion_protection: Option<crate::Value<bool>> = None;
                let mut enable_cloudwatch_logs_exports: Option<crate::ValueList<String>> = None;
                let mut enable_http_endpoint: Option<crate::Value<bool>> = None;
                let mut enable_iam_database_authentication: Option<crate::Value<bool>> = None;
                let mut engine: Option<crate::Value<String>> = None;
                let mut engine_mode: Option<crate::Value<String>> = None;
                let mut engine_version: Option<crate::Value<String>> = None;
                let mut global_cluster_identifier: Option<crate::Value<String>> = None;
                let mut kms_key_id: Option<crate::Value<String>> = None;
                let mut master_user_password: Option<crate::Value<String>> = None;
                let mut master_username: Option<crate::Value<String>> = None;
                let mut port: Option<crate::Value<u32>> = None;
                let mut preferred_backup_window: Option<crate::Value<String>> = None;
                let mut preferred_maintenance_window: Option<crate::Value<String>> = None;
                let mut replication_source_identifier: Option<crate::Value<String>> = None;
                let mut restore_type: Option<crate::Value<String>> = None;
                let mut scaling_configuration: Option<crate::Value<self::db_cluster::ScalingConfiguration>> = None;
                let mut snapshot_identifier: Option<crate::Value<String>> = None;
                let mut source_db_cluster_identifier: Option<crate::Value<String>> = None;
                let mut source_region: Option<crate::Value<String>> = None;
                let mut storage_encrypted: Option<crate::Value<bool>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;
                let mut use_latest_restorable_time: Option<crate::Value<bool>> = None;
                let mut vpc_security_group_ids: Option<crate::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssociatedRoles" => {
                            associated_roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZones" => {
                            availability_zones = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BacktrackWindow" => {
                            backtrack_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupRetentionPeriod" => {
                            backup_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CopyTagsToSnapshot" => {
                            copy_tags_to_snapshot = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "DatabaseName" => {
                            database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeletionProtection" => {
                            deletion_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableCloudwatchLogsExports" => {
                            enable_cloudwatch_logs_exports = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableHttpEndpoint" => {
                            enable_http_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableIAMDatabaseAuthentication" => {
                            enable_iam_database_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineMode" => {
                            engine_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalClusterIdentifier" => {
                            global_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "ReplicationSourceIdentifier" => {
                            replication_source_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestoreType" => {
                            restore_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScalingConfiguration" => {
                            scaling_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotIdentifier" => {
                            snapshot_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceDBClusterIdentifier" => {
                            source_db_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceRegion" => {
                            source_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageEncrypted" => {
                            storage_encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UseLatestRestorableTime" => {
                            use_latest_restorable_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBClusterProperties {
                    associated_roles: associated_roles,
                    availability_zones: availability_zones,
                    backtrack_window: backtrack_window,
                    backup_retention_period: backup_retention_period,
                    copy_tags_to_snapshot: copy_tags_to_snapshot,
                    db_cluster_identifier: db_cluster_identifier,
                    db_cluster_parameter_group_name: db_cluster_parameter_group_name,
                    db_subnet_group_name: db_subnet_group_name,
                    database_name: database_name,
                    deletion_protection: deletion_protection,
                    enable_cloudwatch_logs_exports: enable_cloudwatch_logs_exports,
                    enable_http_endpoint: enable_http_endpoint,
                    enable_iam_database_authentication: enable_iam_database_authentication,
                    engine: engine.ok_or(::serde::de::Error::missing_field("Engine"))?,
                    engine_mode: engine_mode,
                    engine_version: engine_version,
                    global_cluster_identifier: global_cluster_identifier,
                    kms_key_id: kms_key_id,
                    master_user_password: master_user_password,
                    master_username: master_username,
                    port: port,
                    preferred_backup_window: preferred_backup_window,
                    preferred_maintenance_window: preferred_maintenance_window,
                    replication_source_identifier: replication_source_identifier,
                    restore_type: restore_type,
                    scaling_configuration: scaling_configuration,
                    snapshot_identifier: snapshot_identifier,
                    source_db_cluster_identifier: source_db_cluster_identifier,
                    source_region: source_region,
                    storage_encrypted: storage_encrypted,
                    tags: tags,
                    use_latest_restorable_time: use_latest_restorable_time,
                    vpc_security_group_ids: vpc_security_group_ids,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBCluster {
    type Properties = DBClusterProperties;
    const TYPE: &'static str = "AWS::RDS::DBCluster";
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

/// The [`AWS::RDS::DBClusterParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html) resource type.
#[derive(Debug, Default)]
pub struct DBClusterParameterGroup {
    properties: DBClusterParameterGroupProperties
}

/// Properties for the `DBClusterParameterGroup` resource.
#[derive(Debug, Default)]
pub struct DBClusterParameterGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html#cfn-rds-dbclusterparametergroup-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: crate::Value<String>,
    /// Property [`Family`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html#cfn-rds-dbclusterparametergroup-family).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub family: crate::Value<String>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html#cfn-rds-dbclusterparametergroup-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: crate::Value<crate::json::Value>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html#cfn-rds-dbclusterparametergroup-tags).
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
    const TYPE: &'static str = "AWS::RDS::DBClusterParameterGroup";
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

/// The [`AWS::RDS::DBInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html) resource type.
#[derive(Debug, Default)]
pub struct DBInstance {
    properties: DBInstanceProperties
}

/// Properties for the `DBInstance` resource.
#[derive(Debug, Default)]
pub struct DBInstanceProperties {
    /// Property [`AllocatedStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-allocatedstorage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allocated_storage: Option<crate::Value<String>>,
    /// Property [`AllowMajorVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-allowmajorversionupgrade).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_major_version_upgrade: Option<crate::Value<bool>>,
    /// Property [`AssociatedRoles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-associatedroles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub associated_roles: Option<crate::ValueList<self::db_instance::DBInstanceRole>>,
    /// Property [`AutoMinorVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-autominorversionupgrade).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub auto_minor_version_upgrade: Option<crate::Value<bool>>,
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-availabilityzone).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zone: Option<crate::Value<String>>,
    /// Property [`BackupRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-backupretentionperiod).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub backup_retention_period: Option<crate::Value<u32>>,
    /// Property [`CACertificateIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-cacertificateidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ca_certificate_identifier: Option<crate::Value<String>>,
    /// Property [`CharacterSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-charactersetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub character_set_name: Option<crate::Value<String>>,
    /// Property [`CopyTagsToSnapshot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-copytagstosnapshot).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub copy_tags_to_snapshot: Option<crate::Value<bool>>,
    /// Property [`DBClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-dbclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_cluster_identifier: Option<crate::Value<String>>,
    /// Property [`DBInstanceClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-dbinstanceclass).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_instance_class: crate::Value<String>,
    /// Property [`DBInstanceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-dbinstanceidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_instance_identifier: Option<crate::Value<String>>,
    /// Property [`DBName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-dbname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_name: Option<crate::Value<String>>,
    /// Property [`DBParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-dbparametergroupname).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub db_parameter_group_name: Option<crate::Value<String>>,
    /// Property [`DBSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-dbsecuritygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_security_groups: Option<crate::ValueList<String>>,
    /// Property [`DBSnapshotIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-dbsnapshotidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_snapshot_identifier: Option<crate::Value<String>>,
    /// Property [`DBSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-dbsubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_subnet_group_name: Option<crate::Value<String>>,
    /// Property [`DeleteAutomatedBackups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-deleteautomatedbackups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delete_automated_backups: Option<crate::Value<bool>>,
    /// Property [`DeletionProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-deletionprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deletion_protection: Option<crate::Value<bool>>,
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-domain).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain: Option<crate::Value<String>>,
    /// Property [`DomainIAMRoleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-domainiamrolename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_iam_role_name: Option<crate::Value<String>>,
    /// Property [`EnableCloudwatchLogsExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-enablecloudwatchlogsexports).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_cloudwatch_logs_exports: Option<crate::ValueList<String>>,
    /// Property [`EnableIAMDatabaseAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-enableiamdatabaseauthentication).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_iam_database_authentication: Option<crate::Value<bool>>,
    /// Property [`EnablePerformanceInsights`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-enableperformanceinsights).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_performance_insights: Option<crate::Value<bool>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-engine).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub engine: Option<crate::Value<String>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-engineversion).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub engine_version: Option<crate::Value<String>>,
    /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-iops).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iops: Option<crate::Value<u32>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<crate::Value<String>>,
    /// Property [`LicenseModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-licensemodel).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub license_model: Option<crate::Value<String>>,
    /// Property [`MasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-masteruserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub master_user_password: Option<crate::Value<String>>,
    /// Property [`MasterUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-masterusername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub master_username: Option<crate::Value<String>>,
    /// Property [`MaxAllocatedStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-maxallocatedstorage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_allocated_storage: Option<crate::Value<u32>>,
    /// Property [`MonitoringInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-monitoringinterval).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub monitoring_interval: Option<crate::Value<u32>>,
    /// Property [`MonitoringRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-monitoringrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub monitoring_role_arn: Option<crate::Value<String>>,
    /// Property [`MultiAZ`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-multiaz).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub multi_az: Option<crate::Value<bool>>,
    /// Property [`OptionGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-optiongroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub option_group_name: Option<crate::Value<String>>,
    /// Property [`PerformanceInsightsKMSKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-performanceinsightskmskeyid).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub performance_insights_kms_key_id: Option<crate::Value<String>>,
    /// Property [`PerformanceInsightsRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-performanceinsightsretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub performance_insights_retention_period: Option<crate::Value<u32>>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-port).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub port: Option<crate::Value<String>>,
    /// Property [`PreferredBackupWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-preferredbackupwindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_backup_window: Option<crate::Value<String>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-preferredmaintenancewindow).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub preferred_maintenance_window: Option<crate::Value<String>>,
    /// Property [`ProcessorFeatures`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-processorfeatures).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub processor_features: Option<crate::ValueList<self::db_instance::ProcessorFeature>>,
    /// Property [`PromotionTier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-promotiontier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub promotion_tier: Option<crate::Value<u32>>,
    /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-publiclyaccessible).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub publicly_accessible: Option<crate::Value<bool>>,
    /// Property [`SourceDBInstanceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-sourcedbinstanceidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_db_instance_identifier: Option<crate::Value<String>>,
    /// Property [`SourceRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-sourceregion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_region: Option<crate::Value<String>>,
    /// Property [`StorageEncrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-storageencrypted).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_encrypted: Option<crate::Value<bool>>,
    /// Property [`StorageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-storagetype).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub storage_type: Option<crate::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
    /// Property [`Timezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-timezone).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub timezone: Option<crate::Value<String>>,
    /// Property [`UseDefaultProcessorFeatures`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-usedefaultprocessorfeatures).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub use_default_processor_features: Option<crate::Value<bool>>,
    /// Property [`VPCSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html#cfn-rds-dbinstance-vpcsecuritygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_groups: Option<crate::ValueList<String>>,
}

impl ::serde::Serialize for DBInstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allocated_storage) = self.allocated_storage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocatedStorage", allocated_storage)?;
        }
        if let Some(ref allow_major_version_upgrade) = self.allow_major_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowMajorVersionUpgrade", allow_major_version_upgrade)?;
        }
        if let Some(ref associated_roles) = self.associated_roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatedRoles", associated_roles)?;
        }
        if let Some(ref auto_minor_version_upgrade) = self.auto_minor_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", auto_minor_version_upgrade)?;
        }
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        if let Some(ref backup_retention_period) = self.backup_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupRetentionPeriod", backup_retention_period)?;
        }
        if let Some(ref ca_certificate_identifier) = self.ca_certificate_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CACertificateIdentifier", ca_certificate_identifier)?;
        }
        if let Some(ref character_set_name) = self.character_set_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CharacterSetName", character_set_name)?;
        }
        if let Some(ref copy_tags_to_snapshot) = self.copy_tags_to_snapshot {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTagsToSnapshot", copy_tags_to_snapshot)?;
        }
        if let Some(ref db_cluster_identifier) = self.db_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterIdentifier", db_cluster_identifier)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceClass", &self.db_instance_class)?;
        if let Some(ref db_instance_identifier) = self.db_instance_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceIdentifier", db_instance_identifier)?;
        }
        if let Some(ref db_name) = self.db_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBName", db_name)?;
        }
        if let Some(ref db_parameter_group_name) = self.db_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBParameterGroupName", db_parameter_group_name)?;
        }
        if let Some(ref db_security_groups) = self.db_security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroups", db_security_groups)?;
        }
        if let Some(ref db_snapshot_identifier) = self.db_snapshot_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSnapshotIdentifier", db_snapshot_identifier)?;
        }
        if let Some(ref db_subnet_group_name) = self.db_subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupName", db_subnet_group_name)?;
        }
        if let Some(ref delete_automated_backups) = self.delete_automated_backups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteAutomatedBackups", delete_automated_backups)?;
        }
        if let Some(ref deletion_protection) = self.deletion_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeletionProtection", deletion_protection)?;
        }
        if let Some(ref domain) = self.domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", domain)?;
        }
        if let Some(ref domain_iam_role_name) = self.domain_iam_role_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainIAMRoleName", domain_iam_role_name)?;
        }
        if let Some(ref enable_cloudwatch_logs_exports) = self.enable_cloudwatch_logs_exports {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableCloudwatchLogsExports", enable_cloudwatch_logs_exports)?;
        }
        if let Some(ref enable_iam_database_authentication) = self.enable_iam_database_authentication {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableIAMDatabaseAuthentication", enable_iam_database_authentication)?;
        }
        if let Some(ref enable_performance_insights) = self.enable_performance_insights {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnablePerformanceInsights", enable_performance_insights)?;
        }
        if let Some(ref engine) = self.engine {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", engine)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref iops) = self.iops {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref license_model) = self.license_model {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LicenseModel", license_model)?;
        }
        if let Some(ref master_user_password) = self.master_user_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", master_user_password)?;
        }
        if let Some(ref master_username) = self.master_username {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUsername", master_username)?;
        }
        if let Some(ref max_allocated_storage) = self.max_allocated_storage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxAllocatedStorage", max_allocated_storage)?;
        }
        if let Some(ref monitoring_interval) = self.monitoring_interval {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringInterval", monitoring_interval)?;
        }
        if let Some(ref monitoring_role_arn) = self.monitoring_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringRoleArn", monitoring_role_arn)?;
        }
        if let Some(ref multi_az) = self.multi_az {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiAZ", multi_az)?;
        }
        if let Some(ref option_group_name) = self.option_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionGroupName", option_group_name)?;
        }
        if let Some(ref performance_insights_kms_key_id) = self.performance_insights_kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceInsightsKMSKeyId", performance_insights_kms_key_id)?;
        }
        if let Some(ref performance_insights_retention_period) = self.performance_insights_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceInsightsRetentionPeriod", performance_insights_retention_period)?;
        }
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref preferred_backup_window) = self.preferred_backup_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredBackupWindow", preferred_backup_window)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref processor_features) = self.processor_features {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessorFeatures", processor_features)?;
        }
        if let Some(ref promotion_tier) = self.promotion_tier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PromotionTier", promotion_tier)?;
        }
        if let Some(ref publicly_accessible) = self.publicly_accessible {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", publicly_accessible)?;
        }
        if let Some(ref source_db_instance_identifier) = self.source_db_instance_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDBInstanceIdentifier", source_db_instance_identifier)?;
        }
        if let Some(ref source_region) = self.source_region {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceRegion", source_region)?;
        }
        if let Some(ref storage_encrypted) = self.storage_encrypted {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageEncrypted", storage_encrypted)?;
        }
        if let Some(ref storage_type) = self.storage_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageType", storage_type)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref timezone) = self.timezone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timezone", timezone)?;
        }
        if let Some(ref use_default_processor_features) = self.use_default_processor_features {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseDefaultProcessorFeatures", use_default_processor_features)?;
        }
        if let Some(ref vpc_security_groups) = self.vpc_security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCSecurityGroups", vpc_security_groups)?;
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
                let mut allocated_storage: Option<crate::Value<String>> = None;
                let mut allow_major_version_upgrade: Option<crate::Value<bool>> = None;
                let mut associated_roles: Option<crate::ValueList<self::db_instance::DBInstanceRole>> = None;
                let mut auto_minor_version_upgrade: Option<crate::Value<bool>> = None;
                let mut availability_zone: Option<crate::Value<String>> = None;
                let mut backup_retention_period: Option<crate::Value<u32>> = None;
                let mut ca_certificate_identifier: Option<crate::Value<String>> = None;
                let mut character_set_name: Option<crate::Value<String>> = None;
                let mut copy_tags_to_snapshot: Option<crate::Value<bool>> = None;
                let mut db_cluster_identifier: Option<crate::Value<String>> = None;
                let mut db_instance_class: Option<crate::Value<String>> = None;
                let mut db_instance_identifier: Option<crate::Value<String>> = None;
                let mut db_name: Option<crate::Value<String>> = None;
                let mut db_parameter_group_name: Option<crate::Value<String>> = None;
                let mut db_security_groups: Option<crate::ValueList<String>> = None;
                let mut db_snapshot_identifier: Option<crate::Value<String>> = None;
                let mut db_subnet_group_name: Option<crate::Value<String>> = None;
                let mut delete_automated_backups: Option<crate::Value<bool>> = None;
                let mut deletion_protection: Option<crate::Value<bool>> = None;
                let mut domain: Option<crate::Value<String>> = None;
                let mut domain_iam_role_name: Option<crate::Value<String>> = None;
                let mut enable_cloudwatch_logs_exports: Option<crate::ValueList<String>> = None;
                let mut enable_iam_database_authentication: Option<crate::Value<bool>> = None;
                let mut enable_performance_insights: Option<crate::Value<bool>> = None;
                let mut engine: Option<crate::Value<String>> = None;
                let mut engine_version: Option<crate::Value<String>> = None;
                let mut iops: Option<crate::Value<u32>> = None;
                let mut kms_key_id: Option<crate::Value<String>> = None;
                let mut license_model: Option<crate::Value<String>> = None;
                let mut master_user_password: Option<crate::Value<String>> = None;
                let mut master_username: Option<crate::Value<String>> = None;
                let mut max_allocated_storage: Option<crate::Value<u32>> = None;
                let mut monitoring_interval: Option<crate::Value<u32>> = None;
                let mut monitoring_role_arn: Option<crate::Value<String>> = None;
                let mut multi_az: Option<crate::Value<bool>> = None;
                let mut option_group_name: Option<crate::Value<String>> = None;
                let mut performance_insights_kms_key_id: Option<crate::Value<String>> = None;
                let mut performance_insights_retention_period: Option<crate::Value<u32>> = None;
                let mut port: Option<crate::Value<String>> = None;
                let mut preferred_backup_window: Option<crate::Value<String>> = None;
                let mut preferred_maintenance_window: Option<crate::Value<String>> = None;
                let mut processor_features: Option<crate::ValueList<self::db_instance::ProcessorFeature>> = None;
                let mut promotion_tier: Option<crate::Value<u32>> = None;
                let mut publicly_accessible: Option<crate::Value<bool>> = None;
                let mut source_db_instance_identifier: Option<crate::Value<String>> = None;
                let mut source_region: Option<crate::Value<String>> = None;
                let mut storage_encrypted: Option<crate::Value<bool>> = None;
                let mut storage_type: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;
                let mut timezone: Option<crate::Value<String>> = None;
                let mut use_default_processor_features: Option<crate::Value<bool>> = None;
                let mut vpc_security_groups: Option<crate::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllocatedStorage" => {
                            allocated_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AllowMajorVersionUpgrade" => {
                            allow_major_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssociatedRoles" => {
                            associated_roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupRetentionPeriod" => {
                            backup_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CACertificateIdentifier" => {
                            ca_certificate_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CharacterSetName" => {
                            character_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CopyTagsToSnapshot" => {
                            copy_tags_to_snapshot = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "DBName" => {
                            db_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBParameterGroupName" => {
                            db_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSecurityGroups" => {
                            db_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSnapshotIdentifier" => {
                            db_snapshot_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSubnetGroupName" => {
                            db_subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeleteAutomatedBackups" => {
                            delete_automated_backups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeletionProtection" => {
                            deletion_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainIAMRoleName" => {
                            domain_iam_role_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableCloudwatchLogsExports" => {
                            enable_cloudwatch_logs_exports = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableIAMDatabaseAuthentication" => {
                            enable_iam_database_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnablePerformanceInsights" => {
                            enable_performance_insights = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Iops" => {
                            iops = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LicenseModel" => {
                            license_model = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUserPassword" => {
                            master_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUsername" => {
                            master_username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxAllocatedStorage" => {
                            max_allocated_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitoringInterval" => {
                            monitoring_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitoringRoleArn" => {
                            monitoring_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MultiAZ" => {
                            multi_az = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OptionGroupName" => {
                            option_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PerformanceInsightsKMSKeyId" => {
                            performance_insights_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PerformanceInsightsRetentionPeriod" => {
                            performance_insights_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "ProcessorFeatures" => {
                            processor_features = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PromotionTier" => {
                            promotion_tier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceDBInstanceIdentifier" => {
                            source_db_instance_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceRegion" => {
                            source_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageEncrypted" => {
                            storage_encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageType" => {
                            storage_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Timezone" => {
                            timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UseDefaultProcessorFeatures" => {
                            use_default_processor_features = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VPCSecurityGroups" => {
                            vpc_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBInstanceProperties {
                    allocated_storage: allocated_storage,
                    allow_major_version_upgrade: allow_major_version_upgrade,
                    associated_roles: associated_roles,
                    auto_minor_version_upgrade: auto_minor_version_upgrade,
                    availability_zone: availability_zone,
                    backup_retention_period: backup_retention_period,
                    ca_certificate_identifier: ca_certificate_identifier,
                    character_set_name: character_set_name,
                    copy_tags_to_snapshot: copy_tags_to_snapshot,
                    db_cluster_identifier: db_cluster_identifier,
                    db_instance_class: db_instance_class.ok_or(::serde::de::Error::missing_field("DBInstanceClass"))?,
                    db_instance_identifier: db_instance_identifier,
                    db_name: db_name,
                    db_parameter_group_name: db_parameter_group_name,
                    db_security_groups: db_security_groups,
                    db_snapshot_identifier: db_snapshot_identifier,
                    db_subnet_group_name: db_subnet_group_name,
                    delete_automated_backups: delete_automated_backups,
                    deletion_protection: deletion_protection,
                    domain: domain,
                    domain_iam_role_name: domain_iam_role_name,
                    enable_cloudwatch_logs_exports: enable_cloudwatch_logs_exports,
                    enable_iam_database_authentication: enable_iam_database_authentication,
                    enable_performance_insights: enable_performance_insights,
                    engine: engine,
                    engine_version: engine_version,
                    iops: iops,
                    kms_key_id: kms_key_id,
                    license_model: license_model,
                    master_user_password: master_user_password,
                    master_username: master_username,
                    max_allocated_storage: max_allocated_storage,
                    monitoring_interval: monitoring_interval,
                    monitoring_role_arn: monitoring_role_arn,
                    multi_az: multi_az,
                    option_group_name: option_group_name,
                    performance_insights_kms_key_id: performance_insights_kms_key_id,
                    performance_insights_retention_period: performance_insights_retention_period,
                    port: port,
                    preferred_backup_window: preferred_backup_window,
                    preferred_maintenance_window: preferred_maintenance_window,
                    processor_features: processor_features,
                    promotion_tier: promotion_tier,
                    publicly_accessible: publicly_accessible,
                    source_db_instance_identifier: source_db_instance_identifier,
                    source_region: source_region,
                    storage_encrypted: storage_encrypted,
                    storage_type: storage_type,
                    tags: tags,
                    timezone: timezone,
                    use_default_processor_features: use_default_processor_features,
                    vpc_security_groups: vpc_security_groups,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBInstance {
    type Properties = DBInstanceProperties;
    const TYPE: &'static str = "AWS::RDS::DBInstance";
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

/// The [`AWS::RDS::DBParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbparametergroup.html) resource type.
#[derive(Debug, Default)]
pub struct DBParameterGroup {
    properties: DBParameterGroupProperties
}

/// Properties for the `DBParameterGroup` resource.
#[derive(Debug, Default)]
pub struct DBParameterGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbparametergroup.html#cfn-rds-dbparametergroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: crate::Value<String>,
    /// Property [`Family`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbparametergroup.html#cfn-rds-dbparametergroup-family).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub family: crate::Value<String>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbparametergroup.html#cfn-rds-dbparametergroup-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<crate::ValueMap<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbparametergroup.html#cfn-rds-dbparametergroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for DBParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Family", &self.family)?;
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBParameterGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBParameterGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBParameterGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBParameterGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<crate::Value<String>> = None;
                let mut family: Option<crate::Value<String>> = None;
                let mut parameters: Option<crate::ValueMap<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Family" => {
                            family = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(DBParameterGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    family: family.ok_or(::serde::de::Error::missing_field("Family"))?,
                    parameters: parameters,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBParameterGroup {
    type Properties = DBParameterGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBParameterGroup";
    fn properties(&self) -> &DBParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBParameterGroupProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for DBParameterGroup {}

impl From<DBParameterGroupProperties> for DBParameterGroup {
    fn from(properties: DBParameterGroupProperties) -> DBParameterGroup {
        DBParameterGroup { properties }
    }
}

/// The [`AWS::RDS::DBProxy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html) resource type.
#[derive(Debug, Default)]
pub struct DBProxy {
    properties: DBProxyProperties
}

/// Properties for the `DBProxy` resource.
#[derive(Debug, Default)]
pub struct DBProxyProperties {
    /// Property [`Auth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-auth).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auth: crate::ValueList<self::db_proxy::AuthFormat>,
    /// Property [`DBProxyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-dbproxyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_proxy_name: crate::Value<String>,
    /// Property [`DebugLogging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-debuglogging).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub debug_logging: Option<crate::Value<bool>>,
    /// Property [`EngineFamily`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-enginefamily).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_family: crate::Value<String>,
    /// Property [`IdleClientTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-idleclienttimeout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub idle_client_timeout: Option<crate::Value<u32>>,
    /// Property [`RequireTLS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-requiretls).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub require_tls: Option<crate::Value<bool>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: crate::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<self::db_proxy::TagFormat>>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: Option<crate::ValueList<String>>,
    /// Property [`VpcSubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-vpcsubnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_subnet_ids: crate::ValueList<String>,
}

impl ::serde::Serialize for DBProxyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Auth", &self.auth)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBProxyName", &self.db_proxy_name)?;
        if let Some(ref debug_logging) = self.debug_logging {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DebugLogging", debug_logging)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineFamily", &self.engine_family)?;
        if let Some(ref idle_client_timeout) = self.idle_client_timeout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdleClientTimeout", idle_client_timeout)?;
        }
        if let Some(ref require_tls) = self.require_tls {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireTLS", require_tls)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSubnetIds", &self.vpc_subnet_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBProxyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBProxyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBProxyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBProxyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auth: Option<crate::ValueList<self::db_proxy::AuthFormat>> = None;
                let mut db_proxy_name: Option<crate::Value<String>> = None;
                let mut debug_logging: Option<crate::Value<bool>> = None;
                let mut engine_family: Option<crate::Value<String>> = None;
                let mut idle_client_timeout: Option<crate::Value<u32>> = None;
                let mut require_tls: Option<crate::Value<bool>> = None;
                let mut role_arn: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<self::db_proxy::TagFormat>> = None;
                let mut vpc_security_group_ids: Option<crate::ValueList<String>> = None;
                let mut vpc_subnet_ids: Option<crate::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Auth" => {
                            auth = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBProxyName" => {
                            db_proxy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DebugLogging" => {
                            debug_logging = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineFamily" => {
                            engine_family = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdleClientTimeout" => {
                            idle_client_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequireTLS" => {
                            require_tls = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSubnetIds" => {
                            vpc_subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBProxyProperties {
                    auth: auth.ok_or(::serde::de::Error::missing_field("Auth"))?,
                    db_proxy_name: db_proxy_name.ok_or(::serde::de::Error::missing_field("DBProxyName"))?,
                    debug_logging: debug_logging,
                    engine_family: engine_family.ok_or(::serde::de::Error::missing_field("EngineFamily"))?,
                    idle_client_timeout: idle_client_timeout,
                    require_tls: require_tls,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                    vpc_security_group_ids: vpc_security_group_ids,
                    vpc_subnet_ids: vpc_subnet_ids.ok_or(::serde::de::Error::missing_field("VpcSubnetIds"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBProxy {
    type Properties = DBProxyProperties;
    const TYPE: &'static str = "AWS::RDS::DBProxy";
    fn properties(&self) -> &DBProxyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBProxyProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for DBProxy {}

impl From<DBProxyProperties> for DBProxy {
    fn from(properties: DBProxyProperties) -> DBProxy {
        DBProxy { properties }
    }
}

/// The [`AWS::RDS::DBProxyEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html) resource type.
#[derive(Debug, Default)]
pub struct DBProxyEndpoint {
    properties: DBProxyEndpointProperties
}

/// Properties for the `DBProxyEndpoint` resource.
#[derive(Debug, Default)]
pub struct DBProxyEndpointProperties {
    /// Property [`DBProxyEndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-dbproxyendpointname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_proxy_endpoint_name: crate::Value<String>,
    /// Property [`DBProxyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-dbproxyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_proxy_name: crate::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<self::db_proxy_endpoint::TagFormat>>,
    /// Property [`TargetRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-targetrole).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_role: Option<crate::Value<String>>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: Option<crate::ValueList<String>>,
    /// Property [`VpcSubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-vpcsubnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_subnet_ids: crate::ValueList<String>,
}

impl ::serde::Serialize for DBProxyEndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBProxyEndpointName", &self.db_proxy_endpoint_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBProxyName", &self.db_proxy_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target_role) = self.target_role {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetRole", target_role)?;
        }
        if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSubnetIds", &self.vpc_subnet_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBProxyEndpointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBProxyEndpointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBProxyEndpointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBProxyEndpointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut db_proxy_endpoint_name: Option<crate::Value<String>> = None;
                let mut db_proxy_name: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<self::db_proxy_endpoint::TagFormat>> = None;
                let mut target_role: Option<crate::Value<String>> = None;
                let mut vpc_security_group_ids: Option<crate::ValueList<String>> = None;
                let mut vpc_subnet_ids: Option<crate::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DBProxyEndpointName" => {
                            db_proxy_endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBProxyName" => {
                            db_proxy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetRole" => {
                            target_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSubnetIds" => {
                            vpc_subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBProxyEndpointProperties {
                    db_proxy_endpoint_name: db_proxy_endpoint_name.ok_or(::serde::de::Error::missing_field("DBProxyEndpointName"))?,
                    db_proxy_name: db_proxy_name.ok_or(::serde::de::Error::missing_field("DBProxyName"))?,
                    tags: tags,
                    target_role: target_role,
                    vpc_security_group_ids: vpc_security_group_ids,
                    vpc_subnet_ids: vpc_subnet_ids.ok_or(::serde::de::Error::missing_field("VpcSubnetIds"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBProxyEndpoint {
    type Properties = DBProxyEndpointProperties;
    const TYPE: &'static str = "AWS::RDS::DBProxyEndpoint";
    fn properties(&self) -> &DBProxyEndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBProxyEndpointProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for DBProxyEndpoint {}

impl From<DBProxyEndpointProperties> for DBProxyEndpoint {
    fn from(properties: DBProxyEndpointProperties) -> DBProxyEndpoint {
        DBProxyEndpoint { properties }
    }
}

/// The [`AWS::RDS::DBProxyTargetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html) resource type.
#[derive(Debug, Default)]
pub struct DBProxyTargetGroup {
    properties: DBProxyTargetGroupProperties
}

/// Properties for the `DBProxyTargetGroup` resource.
#[derive(Debug, Default)]
pub struct DBProxyTargetGroupProperties {
    /// Property [`ConnectionPoolConfigurationInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connection_pool_configuration_info: Option<crate::Value<self::db_proxy_target_group::ConnectionPoolConfigurationInfoFormat>>,
    /// Property [`DBClusterIdentifiers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html#cfn-rds-dbproxytargetgroup-dbclusteridentifiers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_cluster_identifiers: Option<crate::ValueList<String>>,
    /// Property [`DBInstanceIdentifiers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html#cfn-rds-dbproxytargetgroup-dbinstanceidentifiers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_instance_identifiers: Option<crate::ValueList<String>>,
    /// Property [`DBProxyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html#cfn-rds-dbproxytargetgroup-dbproxyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_proxy_name: crate::Value<String>,
    /// Property [`TargetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html#cfn-rds-dbproxytargetgroup-targetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_group_name: crate::Value<String>,
}

impl ::serde::Serialize for DBProxyTargetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref connection_pool_configuration_info) = self.connection_pool_configuration_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionPoolConfigurationInfo", connection_pool_configuration_info)?;
        }
        if let Some(ref db_cluster_identifiers) = self.db_cluster_identifiers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterIdentifiers", db_cluster_identifiers)?;
        }
        if let Some(ref db_instance_identifiers) = self.db_instance_identifiers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceIdentifiers", db_instance_identifiers)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBProxyName", &self.db_proxy_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupName", &self.target_group_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBProxyTargetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBProxyTargetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBProxyTargetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBProxyTargetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connection_pool_configuration_info: Option<crate::Value<self::db_proxy_target_group::ConnectionPoolConfigurationInfoFormat>> = None;
                let mut db_cluster_identifiers: Option<crate::ValueList<String>> = None;
                let mut db_instance_identifiers: Option<crate::ValueList<String>> = None;
                let mut db_proxy_name: Option<crate::Value<String>> = None;
                let mut target_group_name: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectionPoolConfigurationInfo" => {
                            connection_pool_configuration_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBClusterIdentifiers" => {
                            db_cluster_identifiers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBInstanceIdentifiers" => {
                            db_instance_identifiers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBProxyName" => {
                            db_proxy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetGroupName" => {
                            target_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBProxyTargetGroupProperties {
                    connection_pool_configuration_info: connection_pool_configuration_info,
                    db_cluster_identifiers: db_cluster_identifiers,
                    db_instance_identifiers: db_instance_identifiers,
                    db_proxy_name: db_proxy_name.ok_or(::serde::de::Error::missing_field("DBProxyName"))?,
                    target_group_name: target_group_name.ok_or(::serde::de::Error::missing_field("TargetGroupName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBProxyTargetGroup {
    type Properties = DBProxyTargetGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBProxyTargetGroup";
    fn properties(&self) -> &DBProxyTargetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBProxyTargetGroupProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for DBProxyTargetGroup {}

impl From<DBProxyTargetGroupProperties> for DBProxyTargetGroup {
    fn from(properties: DBProxyTargetGroupProperties) -> DBProxyTargetGroup {
        DBProxyTargetGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html) resource type.
#[derive(Debug, Default)]
pub struct DBSecurityGroup {
    properties: DBSecurityGroupProperties
}

/// Properties for the `DBSecurityGroup` resource.
#[derive(Debug, Default)]
pub struct DBSecurityGroupProperties {
    /// Property [`DBSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html#cfn-rds-dbsecuritygroup-dbsecuritygroupingress).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_security_group_ingress: crate::ValueList<self::db_security_group::Ingress>,
    /// Property [`EC2VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html#cfn-rds-dbsecuritygroup-ec2vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_vpc_id: Option<crate::Value<String>>,
    /// Property [`GroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html#cfn-rds-dbsecuritygroup-groupdescription).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub group_description: crate::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html#cfn-rds-dbsecuritygroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for DBSecurityGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroupIngress", &self.db_security_group_ingress)?;
        if let Some(ref ec2_vpc_id) = self.ec2_vpc_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2VpcId", ec2_vpc_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupDescription", &self.group_description)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBSecurityGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBSecurityGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBSecurityGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBSecurityGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut db_security_group_ingress: Option<crate::ValueList<self::db_security_group::Ingress>> = None;
                let mut ec2_vpc_id: Option<crate::Value<String>> = None;
                let mut group_description: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DBSecurityGroupIngress" => {
                            db_security_group_ingress = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2VpcId" => {
                            ec2_vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GroupDescription" => {
                            group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBSecurityGroupProperties {
                    db_security_group_ingress: db_security_group_ingress.ok_or(::serde::de::Error::missing_field("DBSecurityGroupIngress"))?,
                    ec2_vpc_id: ec2_vpc_id,
                    group_description: group_description.ok_or(::serde::de::Error::missing_field("GroupDescription"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBSecurityGroup {
    type Properties = DBSecurityGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBSecurityGroup";
    fn properties(&self) -> &DBSecurityGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSecurityGroupProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for DBSecurityGroup {}

impl From<DBSecurityGroupProperties> for DBSecurityGroup {
    fn from(properties: DBSecurityGroupProperties) -> DBSecurityGroup {
        DBSecurityGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html) resource type.
#[derive(Debug, Default)]
pub struct DBSecurityGroupIngress {
    properties: DBSecurityGroupIngressProperties
}

/// Properties for the `DBSecurityGroupIngress` resource.
#[derive(Debug, Default)]
pub struct DBSecurityGroupIngressProperties {
    /// Property [`CIDRIP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html#cfn-rds-securitygroup-ingress-cidrip).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cidrip: Option<crate::Value<String>>,
    /// Property [`DBSecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html#cfn-rds-securitygroup-ingress-dbsecuritygroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_security_group_name: crate::Value<String>,
    /// Property [`EC2SecurityGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html#cfn-rds-securitygroup-ingress-ec2securitygroupid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ec2_security_group_id: Option<crate::Value<String>>,
    /// Property [`EC2SecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html#cfn-rds-securitygroup-ingress-ec2securitygroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ec2_security_group_name: Option<crate::Value<String>>,
    /// Property [`EC2SecurityGroupOwnerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html#cfn-rds-securitygroup-ingress-ec2securitygroupownerid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ec2_security_group_owner_id: Option<crate::Value<String>>,
}

impl ::serde::Serialize for DBSecurityGroupIngressProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cidrip) = self.cidrip {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CIDRIP", cidrip)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroupName", &self.db_security_group_name)?;
        if let Some(ref ec2_security_group_id) = self.ec2_security_group_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupId", ec2_security_group_id)?;
        }
        if let Some(ref ec2_security_group_name) = self.ec2_security_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupName", ec2_security_group_name)?;
        }
        if let Some(ref ec2_security_group_owner_id) = self.ec2_security_group_owner_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupOwnerId", ec2_security_group_owner_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBSecurityGroupIngressProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBSecurityGroupIngressProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBSecurityGroupIngressProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBSecurityGroupIngressProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cidrip: Option<crate::Value<String>> = None;
                let mut db_security_group_name: Option<crate::Value<String>> = None;
                let mut ec2_security_group_id: Option<crate::Value<String>> = None;
                let mut ec2_security_group_name: Option<crate::Value<String>> = None;
                let mut ec2_security_group_owner_id: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CIDRIP" => {
                            cidrip = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSecurityGroupName" => {
                            db_security_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2SecurityGroupId" => {
                            ec2_security_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2SecurityGroupName" => {
                            ec2_security_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2SecurityGroupOwnerId" => {
                            ec2_security_group_owner_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBSecurityGroupIngressProperties {
                    cidrip: cidrip,
                    db_security_group_name: db_security_group_name.ok_or(::serde::de::Error::missing_field("DBSecurityGroupName"))?,
                    ec2_security_group_id: ec2_security_group_id,
                    ec2_security_group_name: ec2_security_group_name,
                    ec2_security_group_owner_id: ec2_security_group_owner_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for DBSecurityGroupIngress {
    type Properties = DBSecurityGroupIngressProperties;
    const TYPE: &'static str = "AWS::RDS::DBSecurityGroupIngress";
    fn properties(&self) -> &DBSecurityGroupIngressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSecurityGroupIngressProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for DBSecurityGroupIngress {}

impl From<DBSecurityGroupIngressProperties> for DBSecurityGroupIngress {
    fn from(properties: DBSecurityGroupIngressProperties) -> DBSecurityGroupIngress {
        DBSecurityGroupIngress { properties }
    }
}

/// The [`AWS::RDS::DBSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnet-group.html) resource type.
#[derive(Debug, Default)]
pub struct DBSubnetGroup {
    properties: DBSubnetGroupProperties
}

/// Properties for the `DBSubnetGroup` resource.
#[derive(Debug, Default)]
pub struct DBSubnetGroupProperties {
    /// Property [`DBSubnetGroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnet-group.html#cfn-rds-dbsubnetgroup-dbsubnetgroupdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_subnet_group_description: crate::Value<String>,
    /// Property [`DBSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnet-group.html#cfn-rds-dbsubnetgroup-dbsubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_subnet_group_name: Option<crate::Value<String>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnet-group.html#cfn-rds-dbsubnetgroup-subnetids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_ids: crate::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnet-group.html#cfn-rds-dbsubnetgroup-tags).
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
    const TYPE: &'static str = "AWS::RDS::DBSubnetGroup";
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

/// The [`AWS::RDS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html) resource type.
#[derive(Debug, Default)]
pub struct EventSubscription {
    properties: EventSubscriptionProperties
}

/// Properties for the `EventSubscription` resource.
#[derive(Debug, Default)]
pub struct EventSubscriptionProperties {
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<crate::Value<bool>>,
    /// Property [`EventCategories`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-eventcategories).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_categories: Option<crate::ValueList<String>>,
    /// Property [`SnsTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-snstopicarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sns_topic_arn: crate::Value<String>,
    /// Property [`SourceIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-sourceids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_ids: Option<crate::ValueList<String>>,
    /// Property [`SourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-sourcetype).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub source_type: Option<crate::Value<String>>,
}

impl ::serde::Serialize for EventSubscriptionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref event_categories) = self.event_categories {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventCategories", event_categories)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicArn", &self.sns_topic_arn)?;
        if let Some(ref source_ids) = self.source_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIds", source_ids)?;
        }
        if let Some(ref source_type) = self.source_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceType", source_type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventSubscriptionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSubscriptionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventSubscriptionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventSubscriptionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut enabled: Option<crate::Value<bool>> = None;
                let mut event_categories: Option<crate::ValueList<String>> = None;
                let mut sns_topic_arn: Option<crate::Value<String>> = None;
                let mut source_ids: Option<crate::ValueList<String>> = None;
                let mut source_type: Option<crate::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventCategories" => {
                            event_categories = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnsTopicArn" => {
                            sns_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceIds" => {
                            source_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceType" => {
                            source_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventSubscriptionProperties {
                    enabled: enabled,
                    event_categories: event_categories,
                    sns_topic_arn: sns_topic_arn.ok_or(::serde::de::Error::missing_field("SnsTopicArn"))?,
                    source_ids: source_ids,
                    source_type: source_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for EventSubscription {
    type Properties = EventSubscriptionProperties;
    const TYPE: &'static str = "AWS::RDS::EventSubscription";
    fn properties(&self) -> &EventSubscriptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventSubscriptionProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for EventSubscription {}

impl From<EventSubscriptionProperties> for EventSubscription {
    fn from(properties: EventSubscriptionProperties) -> EventSubscription {
        EventSubscription { properties }
    }
}

/// The [`AWS::RDS::GlobalCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html) resource type.
#[derive(Debug, Default)]
pub struct GlobalCluster {
    properties: GlobalClusterProperties
}

/// Properties for the `GlobalCluster` resource.
#[derive(Debug, Default)]
pub struct GlobalClusterProperties {
    /// Property [`DeletionProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-deletionprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deletion_protection: Option<crate::Value<bool>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-engine).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine: Option<crate::Value<String>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-engineversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_version: Option<crate::Value<String>>,
    /// Property [`GlobalClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-globalclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub global_cluster_identifier: Option<crate::Value<String>>,
    /// Property [`SourceDBClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-sourcedbclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_db_cluster_identifier: Option<crate::Value<String>>,
    /// Property [`StorageEncrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-storageencrypted).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_encrypted: Option<crate::Value<bool>>,
}

impl ::serde::Serialize for GlobalClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref deletion_protection) = self.deletion_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeletionProtection", deletion_protection)?;
        }
        if let Some(ref engine) = self.engine {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", engine)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref global_cluster_identifier) = self.global_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalClusterIdentifier", global_cluster_identifier)?;
        }
        if let Some(ref source_db_cluster_identifier) = self.source_db_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDBClusterIdentifier", source_db_cluster_identifier)?;
        }
        if let Some(ref storage_encrypted) = self.storage_encrypted {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageEncrypted", storage_encrypted)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GlobalClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GlobalClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GlobalClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GlobalClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut deletion_protection: Option<crate::Value<bool>> = None;
                let mut engine: Option<crate::Value<String>> = None;
                let mut engine_version: Option<crate::Value<String>> = None;
                let mut global_cluster_identifier: Option<crate::Value<String>> = None;
                let mut source_db_cluster_identifier: Option<crate::Value<String>> = None;
                let mut storage_encrypted: Option<crate::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeletionProtection" => {
                            deletion_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalClusterIdentifier" => {
                            global_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceDBClusterIdentifier" => {
                            source_db_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageEncrypted" => {
                            storage_encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GlobalClusterProperties {
                    deletion_protection: deletion_protection,
                    engine: engine,
                    engine_version: engine_version,
                    global_cluster_identifier: global_cluster_identifier,
                    source_db_cluster_identifier: source_db_cluster_identifier,
                    storage_encrypted: storage_encrypted,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for GlobalCluster {
    type Properties = GlobalClusterProperties;
    const TYPE: &'static str = "AWS::RDS::GlobalCluster";
    fn properties(&self) -> &GlobalClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GlobalClusterProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for GlobalCluster {}

impl From<GlobalClusterProperties> for GlobalCluster {
    fn from(properties: GlobalClusterProperties) -> GlobalCluster {
        GlobalCluster { properties }
    }
}

/// The [`AWS::RDS::OptionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html) resource type.
#[derive(Debug, Default)]
pub struct OptionGroup {
    properties: OptionGroupProperties
}

/// Properties for the `OptionGroup` resource.
#[derive(Debug, Default)]
pub struct OptionGroupProperties {
    /// Property [`EngineName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html#cfn-rds-optiongroup-enginename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_name: crate::Value<String>,
    /// Property [`MajorEngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html#cfn-rds-optiongroup-majorengineversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub major_engine_version: crate::Value<String>,
    /// Property [`OptionConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html#cfn-rds-optiongroup-optionconfigurations).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub option_configurations: crate::ValueList<self::option_group::OptionConfiguration>,
    /// Property [`OptionGroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html#cfn-rds-optiongroup-optiongroupdescription).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub option_group_description: crate::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html#cfn-rds-optiongroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<crate::ValueList<crate::Tag>>,
}

impl ::serde::Serialize for OptionGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineName", &self.engine_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MajorEngineVersion", &self.major_engine_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionConfigurations", &self.option_configurations)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionGroupDescription", &self.option_group_description)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for OptionGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<OptionGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = OptionGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type OptionGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut engine_name: Option<crate::Value<String>> = None;
                let mut major_engine_version: Option<crate::Value<String>> = None;
                let mut option_configurations: Option<crate::ValueList<self::option_group::OptionConfiguration>> = None;
                let mut option_group_description: Option<crate::Value<String>> = None;
                let mut tags: Option<crate::ValueList<crate::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EngineName" => {
                            engine_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MajorEngineVersion" => {
                            major_engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OptionConfigurations" => {
                            option_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OptionGroupDescription" => {
                            option_group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(OptionGroupProperties {
                    engine_name: engine_name.ok_or(::serde::de::Error::missing_field("EngineName"))?,
                    major_engine_version: major_engine_version.ok_or(::serde::de::Error::missing_field("MajorEngineVersion"))?,
                    option_configurations: option_configurations.ok_or(::serde::de::Error::missing_field("OptionConfigurations"))?,
                    option_group_description: option_group_description.ok_or(::serde::de::Error::missing_field("OptionGroupDescription"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl crate::Resource for OptionGroup {
    type Properties = OptionGroupProperties;
    const TYPE: &'static str = "AWS::RDS::OptionGroup";
    fn properties(&self) -> &OptionGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OptionGroupProperties {
        &mut self.properties
    }
}

impl crate::private::Sealed for OptionGroup {}

impl From<OptionGroupProperties> for OptionGroup {
    fn from(properties: OptionGroupProperties) -> OptionGroup {
        OptionGroup { properties }
    }
}

pub mod db_cluster {
    //! Property types for the `DBCluster` resource.

    /// The [`AWS::RDS::DBCluster.DBClusterRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-dbclusterrole.html) property type.
    #[derive(Debug, Default)]
    pub struct DBClusterRole {
        /// Property [`FeatureName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-dbclusterrole.html#cfn-rds-dbcluster-dbclusterrole-featurename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub feature_name: Option<crate::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-dbclusterrole.html#cfn-rds-dbcluster-dbclusterrole-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for DBClusterRole {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref feature_name) = self.feature_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureName", feature_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DBClusterRole {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DBClusterRole, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DBClusterRole;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DBClusterRole")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut feature_name: Option<crate::Value<String>> = None;
                    let mut role_arn: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FeatureName" => {
                                feature_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DBClusterRole {
                        feature_name: feature_name,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBCluster.ScalingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ScalingConfiguration {
        /// Property [`AutoPause`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html#cfn-rds-dbcluster-scalingconfiguration-autopause).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_pause: Option<crate::Value<bool>>,
        /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html#cfn-rds-dbcluster-scalingconfiguration-maxcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_capacity: Option<crate::Value<u32>>,
        /// Property [`MinCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html#cfn-rds-dbcluster-scalingconfiguration-mincapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_capacity: Option<crate::Value<u32>>,
        /// Property [`SecondsUntilAutoPause`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html#cfn-rds-dbcluster-scalingconfiguration-secondsuntilautopause).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub seconds_until_auto_pause: Option<crate::Value<u32>>,
    }

    impl crate::codec::SerializeValue for ScalingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_pause) = self.auto_pause {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoPause", auto_pause)?;
            }
            if let Some(ref max_capacity) = self.max_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", max_capacity)?;
            }
            if let Some(ref min_capacity) = self.min_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinCapacity", min_capacity)?;
            }
            if let Some(ref seconds_until_auto_pause) = self.seconds_until_auto_pause {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondsUntilAutoPause", seconds_until_auto_pause)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ScalingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_pause: Option<crate::Value<bool>> = None;
                    let mut max_capacity: Option<crate::Value<u32>> = None;
                    let mut min_capacity: Option<crate::Value<u32>> = None;
                    let mut seconds_until_auto_pause: Option<crate::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoPause" => {
                                auto_pause = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxCapacity" => {
                                max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinCapacity" => {
                                min_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondsUntilAutoPause" => {
                                seconds_until_auto_pause = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingConfiguration {
                        auto_pause: auto_pause,
                        max_capacity: max_capacity,
                        min_capacity: min_capacity,
                        seconds_until_auto_pause: seconds_until_auto_pause,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod db_instance {
    //! Property types for the `DBInstance` resource.

    /// The [`AWS::RDS::DBInstance.DBInstanceRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-dbinstancerole.html) property type.
    #[derive(Debug, Default)]
    pub struct DBInstanceRole {
        /// Property [`FeatureName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-dbinstancerole.html#cfn-rds-dbinstance-dbinstancerole-featurename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub feature_name: crate::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-dbinstancerole.html#cfn-rds-dbinstance-dbinstancerole-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: crate::Value<String>,
    }

    impl crate::codec::SerializeValue for DBInstanceRole {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureName", &self.feature_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for DBInstanceRole {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DBInstanceRole, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DBInstanceRole;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DBInstanceRole")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut feature_name: Option<crate::Value<String>> = None;
                    let mut role_arn: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FeatureName" => {
                                feature_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DBInstanceRole {
                        feature_name: feature_name.ok_or(::serde::de::Error::missing_field("FeatureName"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBInstance.ProcessorFeature`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-processorfeature.html) property type.
    #[derive(Debug, Default)]
    pub struct ProcessorFeature {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-processorfeature.html#cfn-rds-dbinstance-processorfeature-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<crate::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-processorfeature.html#cfn-rds-dbinstance-processorfeature-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for ProcessorFeature {
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

    impl crate::codec::DeserializeValue for ProcessorFeature {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProcessorFeature, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProcessorFeature;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProcessorFeature")
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

                    Ok(ProcessorFeature {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod db_proxy {
    //! Property types for the `DBProxy` resource.

    /// The [`AWS::RDS::DBProxy.AuthFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthFormat {
        /// Property [`AuthScheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html#cfn-rds-dbproxy-authformat-authscheme).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth_scheme: Option<crate::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html#cfn-rds-dbproxy-authformat-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<crate::Value<String>>,
        /// Property [`IAMAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html#cfn-rds-dbproxy-authformat-iamauth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam_auth: Option<crate::Value<String>>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html#cfn-rds-dbproxy-authformat-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: Option<crate::Value<String>>,
        /// Property [`UserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html#cfn-rds-dbproxy-authformat-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_name: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for AuthFormat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auth_scheme) = self.auth_scheme {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthScheme", auth_scheme)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref iam_auth) = self.iam_auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IAMAuth", iam_auth)?;
            }
            if let Some(ref secret_arn) = self.secret_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", secret_arn)?;
            }
            if let Some(ref user_name) = self.user_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", user_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for AuthFormat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthFormat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthFormat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthFormat")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auth_scheme: Option<crate::Value<String>> = None;
                    let mut description: Option<crate::Value<String>> = None;
                    let mut iam_auth: Option<crate::Value<String>> = None;
                    let mut secret_arn: Option<crate::Value<String>> = None;
                    let mut user_name: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthScheme" => {
                                auth_scheme = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IAMAuth" => {
                                iam_auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserName" => {
                                user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthFormat {
                        auth_scheme: auth_scheme,
                        description: description,
                        iam_auth: iam_auth,
                        secret_arn: secret_arn,
                        user_name: user_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBProxy.TagFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-tagformat.html) property type.
    #[derive(Debug, Default)]
    pub struct TagFormat {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-tagformat.html#cfn-rds-dbproxy-tagformat-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<crate::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-tagformat.html#cfn-rds-dbproxy-tagformat-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for TagFormat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for TagFormat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagFormat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagFormat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagFormat")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<crate::Value<String>> = None;
                    let mut value: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TagFormat {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod db_proxy_endpoint {
    //! Property types for the `DBProxyEndpoint` resource.

    /// The [`AWS::RDS::DBProxyEndpoint.TagFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxyendpoint-tagformat.html) property type.
    #[derive(Debug, Default)]
    pub struct TagFormat {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxyendpoint-tagformat.html#cfn-rds-dbproxyendpoint-tagformat-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<crate::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxyendpoint-tagformat.html#cfn-rds-dbproxyendpoint-tagformat-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for TagFormat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for TagFormat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagFormat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagFormat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagFormat")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<crate::Value<String>> = None;
                    let mut value: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TagFormat {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod db_proxy_target_group {
    //! Property types for the `DBProxyTargetGroup` resource.

    /// The [`AWS::RDS::DBProxyTargetGroup.ConnectionPoolConfigurationInfoFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectionPoolConfigurationInfoFormat {
        /// Property [`ConnectionBorrowTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat-connectionborrowtimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_borrow_timeout: Option<crate::Value<u32>>,
        /// Property [`InitQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat-initquery).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub init_query: Option<crate::Value<String>>,
        /// Property [`MaxConnectionsPercent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat-maxconnectionspercent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_connections_percent: Option<crate::Value<u32>>,
        /// Property [`MaxIdleConnectionsPercent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat-maxidleconnectionspercent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_idle_connections_percent: Option<crate::Value<u32>>,
        /// Property [`SessionPinningFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat-sessionpinningfilters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_pinning_filters: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for ConnectionPoolConfigurationInfoFormat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_borrow_timeout) = self.connection_borrow_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionBorrowTimeout", connection_borrow_timeout)?;
            }
            if let Some(ref init_query) = self.init_query {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitQuery", init_query)?;
            }
            if let Some(ref max_connections_percent) = self.max_connections_percent {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConnectionsPercent", max_connections_percent)?;
            }
            if let Some(ref max_idle_connections_percent) = self.max_idle_connections_percent {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxIdleConnectionsPercent", max_idle_connections_percent)?;
            }
            if let Some(ref session_pinning_filters) = self.session_pinning_filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionPinningFilters", session_pinning_filters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for ConnectionPoolConfigurationInfoFormat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionPoolConfigurationInfoFormat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectionPoolConfigurationInfoFormat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectionPoolConfigurationInfoFormat")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_borrow_timeout: Option<crate::Value<u32>> = None;
                    let mut init_query: Option<crate::Value<String>> = None;
                    let mut max_connections_percent: Option<crate::Value<u32>> = None;
                    let mut max_idle_connections_percent: Option<crate::Value<u32>> = None;
                    let mut session_pinning_filters: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionBorrowTimeout" => {
                                connection_borrow_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InitQuery" => {
                                init_query = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxConnectionsPercent" => {
                                max_connections_percent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxIdleConnectionsPercent" => {
                                max_idle_connections_percent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionPinningFilters" => {
                                session_pinning_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectionPoolConfigurationInfoFormat {
                        connection_borrow_timeout: connection_borrow_timeout,
                        init_query: init_query,
                        max_connections_percent: max_connections_percent,
                        max_idle_connections_percent: max_idle_connections_percent,
                        session_pinning_filters: session_pinning_filters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod db_security_group {
    //! Property types for the `DBSecurityGroup` resource.

    /// The [`AWS::RDS::DBSecurityGroup.Ingress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html) property type.
    #[derive(Debug, Default)]
    pub struct Ingress {
        /// Property [`CIDRIP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html#cfn-rds-securitygroup-cidrip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cidrip: Option<crate::Value<String>>,
        /// Property [`EC2SecurityGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html#cfn-rds-securitygroup-ec2securitygroupid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ec2_security_group_id: Option<crate::Value<String>>,
        /// Property [`EC2SecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html#cfn-rds-securitygroup-ec2securitygroupname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ec2_security_group_name: Option<crate::Value<String>>,
        /// Property [`EC2SecurityGroupOwnerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html#cfn-rds-securitygroup-ec2securitygroupownerid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ec2_security_group_owner_id: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for Ingress {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cidrip) = self.cidrip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CIDRIP", cidrip)?;
            }
            if let Some(ref ec2_security_group_id) = self.ec2_security_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupId", ec2_security_group_id)?;
            }
            if let Some(ref ec2_security_group_name) = self.ec2_security_group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupName", ec2_security_group_name)?;
            }
            if let Some(ref ec2_security_group_owner_id) = self.ec2_security_group_owner_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupOwnerId", ec2_security_group_owner_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for Ingress {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Ingress, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Ingress;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Ingress")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cidrip: Option<crate::Value<String>> = None;
                    let mut ec2_security_group_id: Option<crate::Value<String>> = None;
                    let mut ec2_security_group_name: Option<crate::Value<String>> = None;
                    let mut ec2_security_group_owner_id: Option<crate::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CIDRIP" => {
                                cidrip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EC2SecurityGroupId" => {
                                ec2_security_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EC2SecurityGroupName" => {
                                ec2_security_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EC2SecurityGroupOwnerId" => {
                                ec2_security_group_owner_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Ingress {
                        cidrip: cidrip,
                        ec2_security_group_id: ec2_security_group_id,
                        ec2_security_group_name: ec2_security_group_name,
                        ec2_security_group_owner_id: ec2_security_group_owner_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod option_group {
    //! Property types for the `OptionGroup` resource.

    /// The [`AWS::RDS::OptionGroup.OptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations.html) property type.
    #[derive(Debug, Default)]
    pub struct OptionConfiguration {
        /// Property [`DBSecurityGroupMemberships`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations.html#cfn-rds-optiongroup-optionconfigurations-dbsecuritygroupmemberships).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub db_security_group_memberships: Option<crate::ValueList<String>>,
        /// Property [`OptionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations.html#cfn-rds-optiongroup-optionconfigurations-optionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub option_name: crate::Value<String>,
        /// Property [`OptionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations.html#cfn-rds-optiongroup-optionconfigurations-optionsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub option_settings: Option<crate::ValueList<OptionSetting>>,
        /// Property [`OptionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations.html#cfn-rds-optiongroup-optionconfiguration-optionversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub option_version: Option<crate::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations.html#cfn-rds-optiongroup-optionconfigurations-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<crate::Value<u32>>,
        /// Property [`VpcSecurityGroupMemberships`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations.html#cfn-rds-optiongroup-optionconfigurations-vpcsecuritygroupmemberships).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_security_group_memberships: Option<crate::ValueList<String>>,
    }

    impl crate::codec::SerializeValue for OptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref db_security_group_memberships) = self.db_security_group_memberships {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroupMemberships", db_security_group_memberships)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionName", &self.option_name)?;
            if let Some(ref option_settings) = self.option_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionSettings", option_settings)?;
            }
            if let Some(ref option_version) = self.option_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionVersion", option_version)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref vpc_security_group_memberships) = self.vpc_security_group_memberships {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupMemberships", vpc_security_group_memberships)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl crate::codec::DeserializeValue for OptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut db_security_group_memberships: Option<crate::ValueList<String>> = None;
                    let mut option_name: Option<crate::Value<String>> = None;
                    let mut option_settings: Option<crate::ValueList<OptionSetting>> = None;
                    let mut option_version: Option<crate::Value<String>> = None;
                    let mut port: Option<crate::Value<u32>> = None;
                    let mut vpc_security_group_memberships: Option<crate::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DBSecurityGroupMemberships" => {
                                db_security_group_memberships = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OptionName" => {
                                option_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OptionSettings" => {
                                option_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OptionVersion" => {
                                option_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcSecurityGroupMemberships" => {
                                vpc_security_group_memberships = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OptionConfiguration {
                        db_security_group_memberships: db_security_group_memberships,
                        option_name: option_name.ok_or(::serde::de::Error::missing_field("OptionName"))?,
                        option_settings: option_settings,
                        option_version: option_version,
                        port: port,
                        vpc_security_group_memberships: vpc_security_group_memberships,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::OptionGroup.OptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations-optionsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct OptionSetting {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations-optionsettings.html#cfn-rds-optiongroup-optionconfigurations-optionsettings-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<crate::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations-optionsettings.html#cfn-rds-optiongroup-optionconfigurations-optionsettings-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<crate::Value<String>>,
    }

    impl crate::codec::SerializeValue for OptionSetting {
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

    impl crate::codec::DeserializeValue for OptionSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OptionSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OptionSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OptionSetting")
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

                    Ok(OptionSetting {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
