// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// [HistoryTransactional Resource](https://ddwiki.reso.org/display/DDW17/HistoryTransactional+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HistoryTransactional {
    /// Description of the last major change on the listing, i.e. price reduction, back on market, etc. May be used to display on a summary view of listing results to quickly identify listings that have had major changes recently.
    ///
    /// [ChangeType](https://ddwiki.reso.org/display/DDW17/ChangeType+Field)
    #[serde(rename = "ChangeType", skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,

    /// The local, well-know identifier of the member (user) who made the change.
    ///
    /// [ChangedByMemberID](https://ddwiki.reso.org/display/DDW17/ChangedByMemberID+%28HistoryTransactional%29+Field)
    #[serde(rename = "ChangedByMemberID", skip_serializing_if = "Option::is_none")]
    pub changed_by_member_id: Option<String>,

    /// The unique identifier of the member (user) who made the change. This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [ChangedByMemberKey](https://ddwiki.reso.org/display/DDW17/ChangedByMemberKey+%28HistoryTransactional%29+Field)
    #[serde(rename = "ChangedByMemberKey", skip_serializing_if = "Option::is_none")]
    pub changed_by_member_key: Option<String>,

    /// The unique identifier of the member (user) who made the change. This is a foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the ChangedByMemberKey field.
    ///
    /// [ChangedByMemberKeyNumeric](https://ddwiki.reso.org/display/DDW17/ChangedByMemberKeyNumeric+%28HistoryTransactional%29+Field)
    #[serde(
        rename = "ChangedByMemberKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub changed_by_member_key_numeric: Option<f64>,

    /// Name of the class which this history record applies.
    ///
    /// [ClassName](https://ddwiki.reso.org/display/DDW17/ClassName+%28HistoryTransactional%29+Field)
    #[serde(rename = "ClassName", skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// The unique identifier of the field whose data is being changed.  This is a foreign key relating to the field found in the resource per the ResourceName.
    ///
    /// [FieldKey](https://ddwiki.reso.org/display/DDW17/FieldKey+Field)
    #[serde(rename = "FieldKey", skip_serializing_if = "Option::is_none")]
    pub field_key: Option<String>,

    /// The unique identifier of the field whose data is being changed.  This is a foreign key relating to the field found in the resource per the ResourceName.  This is the numeric only key and used as an alternative to the FieldKey field.
    ///
    /// [FieldKeyNumeric](https://ddwiki.reso.org/display/DDW17/FieldKeyNumeric+Field)
    #[serde(rename = "FieldKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub field_key_numeric: Option<f64>,

    /// The name of the field whose data is being changed.
    ///
    /// [FieldName](https://ddwiki.reso.org/display/DDW17/FieldName+Field)
    #[serde(rename = "FieldName", skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,

    /// A unique identifier for this record from the immediate source. This may be a number, or string that can include URI or other forms.  This is the system you are connecting to and not necessarily the original source of the record.
    ///
    /// [HistoryTransactionalKey](https://ddwiki.reso.org/display/DDW17/HistoryTransactionalKey+Field)
    #[serde(
        rename = "HistoryTransactionalKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub history_transactional_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This may be a number, or string that can include URI or other forms.  This is the system you are connecting to and not necessarily the original source of the record.  This is the numeric only key and used as an alternative to the HistoryTransactionalKey field.
    ///
    /// [HistoryTransactionalKeyNumeric](https://ddwiki.reso.org/display/DDW17/HistoryTransactionalKeyNumeric+Field)
    #[serde(
        rename = "HistoryTransactionalKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub history_transactional_key_numeric: Option<f64>,

    /// Timestamp of the last major change on the listing (see also MajorChangeType).
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28HistoryTransactional%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The new value applied to the named field.
    ///
    /// [NewValue](https://ddwiki.reso.org/display/DDW17/NewValue+Field)
    #[serde(rename = "NewValue", skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,

    /// The system key, a unique record identifier, from the Originating system.  The Originating system is the system with authoritative control over the record.  For example, the Multiple Listing Service where the History was input.  There may be cases where the Source System (how you received the record) is not the Originating System.  See Source System Key for more information.
    ///
    /// [OriginatingSystemHistoryKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemHistoryKey+Field)
    #[serde(
        rename = "OriginatingSystemHistoryKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_history_key: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the History was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+%28HistoryTransactional%29+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the History is originally input.  The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+%28HistoryTransactional%29+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// The value found in the named field prior to the change represented by this record.
    ///
    /// [PreviousValue](https://ddwiki.reso.org/display/DDW17/PreviousValue+Field)
    #[serde(rename = "PreviousValue", skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<String>,

    /// The name of the resource which this history record applies.
    ///
    /// [ResourceName](https://ddwiki.reso.org/display/DDW17/ResourceName+%28HistoryTransactional%29+Field)
    #[serde(rename = "ResourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,

    /// The well known identifier of the related record from the source resource. The value may be identical to that of the Listing Key, but the Listing ID is intended to be the value used by a human to retrieve the information about a specific listing. In a multiple originating system or a merged system, this value may not be unique and may require the use of the provider system to create a synthetic unique value.
    ///
    /// [ResourceRecordID](https://ddwiki.reso.org/display/DDW17/ResourceRecordID+%28HistoryTransactional%29+Field)
    #[serde(rename = "ResourceRecordID", skip_serializing_if = "Option::is_none")]
    pub resource_record_id: Option<String>,

    /// The primary key of the related record from the source resource. For example the ListingKey, AgentKey, OfficeKey, etc.  This is the system you are connecting to and not necessarily the original source of the record.  This is a foreign key from the resource selected in the ResourceName field.
    ///
    /// [ResourceRecordKey](https://ddwiki.reso.org/display/DDW17/ResourceRecordKey+%28HistoryTransactional%29+Field)
    #[serde(rename = "ResourceRecordKey", skip_serializing_if = "Option::is_none")]
    pub resource_record_key: Option<String>,

    /// The primary key of the related record from the source resource. For example the ListingKey, AgentKey, OfficeKey, etc.  This is the system you are connecting to and not necessarily the original source of the record.  This is a foreign key from the resource selected in the ResourceName field.  This is the numeric only key and used as an alternative to the ResourceRecordKey field.
    ///
    /// [ResourceRecordKeyNumeric](https://ddwiki.reso.org/display/DDW17/ResourceRecordKeyNumeric+%28HistoryTransactional%29+Field)
    #[serde(
        rename = "ResourceRecordKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_record_key_numeric: Option<f64>,

    /// The system key, a unique record identifier, from the Source System.  The Source System is the system from which the record was directly received.  In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemHistoryKey](https://ddwiki.reso.org/display/DDW17/SourceSystemHistoryKey+Field)
    #[serde(
        rename = "SourceSystemHistoryKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_history_key: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider.  The source system is the system from which the record was directly received.  In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+%28HistoryTransactional%29+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The name of the History record provider.  The system from which the record was directly received.  The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+%28HistoryTransactional%29+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,
}
