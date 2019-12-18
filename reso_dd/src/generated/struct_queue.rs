// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};

/// [Queue Resource](https://ddwiki.reso.org/display/DDW17/Queue+Resource)
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Queue {
    /// Name of the class which this queue record is referencing.
    ///
    /// [ClassName](https://ddwiki.reso.org/display/DDW17/ClassName+%28Queue%29+Field)
    #[serde(rename = "ClassName", skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// Timestamp of the last major change on the listing (see also MajorChangeType).
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28Queue%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider. The Originating system is the system with authoritative control over the record. For example; the name of the MLS where the Queue record was generated. In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+%28Queue%29+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The name of the Originating record provider. Most commonly the name of the MLS. The place where the Queue record or originally generated. The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+%28Queue%29+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// The system key, a unique record identifier, from the Originating system. The Originating system is the system with authoritative control over the record. For example, the Multiple Listing Service where the Queue record was generated. There may be cases where the Source System (how you received the record) is not the Originating System. See Source System Key for more information.
    ///
    /// [OriginatingSystemQueueKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemQueueKey+Field)
    #[serde(
        rename = "OriginatingSystemQueueKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_queue_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This may be a number, or string that can include URI or other forms. This is the system you are connecting to and not necessarily the original source of the record.
    ///
    /// [QueueTransactionKey](https://ddwiki.reso.org/display/DDW17/QueueTransactionKey+Field)
    #[serde(
        rename = "QueueTransactionKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_transaction_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This may be a number, or string that can include URI or other forms. This is the system you are connecting to and not necessarily the original source of the record. This is the numeric only key and used as an alternative to the QueueKey field.
    ///
    /// [QueueTransactionKeyNumeric](https://ddwiki.reso.org/display/DDW17/QueueTransactionKeyNumeric+Field)
    #[serde(
        rename = "QueueTransactionKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_transaction_key_numeric: Option<f64>,

    /// The type of change that the queue transaction record is representing. For example, add, change, delete, etc.
    ///
    /// [QueueTransactionType](https://ddwiki.reso.org/display/DDW17/QueueTransactionType+Field)
    #[serde(
        rename = "QueueTransactionType",
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_transaction_type: Option<String>,

    /// The name of the resource which this queue record is referencing.
    ///
    /// [ResourceName](https://ddwiki.reso.org/display/DDW17/ResourceName+%28Queue%29+Field)
    #[serde(rename = "ResourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,

    /// The well known identifier of the related record from the source resource. The value may be identical to that of the Listing Key, but the Listing ID is intended to be the value used by a human to retrieve the information about a specific listing. In a multiple originating system or a merged system, this value may not be unique and may require the use of the provider system to create a synthetic unique value.
    ///
    /// [ResourceRecordID](https://ddwiki.reso.org/display/DDW17/ResourceRecordID+%28Queue%29+Field)
    #[serde(rename = "ResourceRecordID", skip_serializing_if = "Option::is_none")]
    pub resource_record_id: Option<String>,

    /// The primary key of the related record from the source resource. For example the ListingKey, AgentKey, OfficeKey, etc. This is the system you are connecting to and not necessarily the original source of the record. This is a foreign key from the resource selected in the ResourceName field.
    ///
    /// [ResourceRecordKey](https://ddwiki.reso.org/display/DDW17/ResourceRecordKey+%28Queue%29+Field)
    #[serde(rename = "ResourceRecordKey", skip_serializing_if = "Option::is_none")]
    pub resource_record_key: Option<String>,

    /// The primary key of the related record from the source resource. For example the ListingKey, AgentKey, OfficeKey, etc. This is the system you are connecting to and not necessarily the original source of the record. This is a foreign key from the resource selected in the ResourceName field. This is the numeric only key and used as an alternative to the ResourceRecordKey field.
    ///
    /// [ResourceRecordKeyNumeric](https://ddwiki.reso.org/display/DDW17/ResourceRecordKeyNumeric+%28Queue%29+Field)
    #[serde(
        rename = "ResourceRecordKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_record_key_numeric: Option<f64>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider. The source system is the system from which the record was directly received. In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+%28Queue%29+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The name of the Queue record provider. The system from which the record was directly received. The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+%28Queue%29+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemQueueKey](https://ddwiki.reso.org/display/DDW17/SourceSystemQueueKey+Field)
    #[serde(
        rename = "SourceSystemQueueKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_queue_key: Option<String>,
}
