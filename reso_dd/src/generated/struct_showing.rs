// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};

/// [Showing Resource](https://ddwiki.reso.org/display/DDW17/Showing+Resource)
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Showing {
    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the listing was input.  In cases where the Originating system was not where the record was retrieved, see the Source System fields.
    ///
    /// [AgentOriginatingSystemID](https://ddwiki.reso.org/display/DDW17/AgentOriginatingSystemID+Field)
    #[serde(
        rename = "AgentOriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_originating_system_id: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the listing is originally input by the member.  The legal name of the company. In cases where the Originating system was not where the record was retrieved, see hte Source System fields.
    ///
    /// [AgentOriginatingSystemName](https://ddwiki.reso.org/display/DDW17/AgentOriginatingSystemName+Field)
    #[serde(
        rename = "AgentOriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_originating_system_name: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider. The source system is the system from which the record was directly received. In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [AgentSourceSystemID](https://ddwiki.reso.org/display/DDW17/AgentSourceSystemID+Field)
    #[serde(
        rename = "AgentSourceSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_source_system_id: Option<String>,

    /// The name of the immediate record provider. The system from which the record was directly received. The legal name of the company.  In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [AgentSourceSystemName](https://ddwiki.reso.org/display/DDW17/AgentSourceSystemName+Field)
    #[serde(
        rename = "AgentSourceSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_source_system_name: Option<String>,

    /// The well known identifier for the listing being shown. The value may be identical to that of the Listing Key, but the Listing ID is intended to be the value used by a human to retrieve the information about a specific listing. In a multiple originating system or a merged system, this value may not be unique and may require the use of the provider system to create a synthetic unique value.
    ///
    /// [ListingId](https://ddwiki.reso.org/display/DDW17/ListingId+%28Showing%29+Field)
    #[serde(rename = "ListingId", skip_serializing_if = "Option::is_none")]
    pub listing_id: Option<String>,

    /// A unique identifier for this record. This is a string that can include URI or other forms.  Alternatively use the ListingKeyNumeric for a numeric only key field.  This is the local key of the system.  When records are received from other systems, a local key is commonly applied.  If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey.
    ///
    /// [ListingKey](https://ddwiki.reso.org/display/DDW17/ListingKey+%28Showing%29+Field)
    #[serde(rename = "ListingKey", skip_serializing_if = "Option::is_none")]
    pub listing_key: Option<String>,

    /// A unique identifier for this record. This is the numeric only key and used as an alternative to the ListingKey fields.  This is the local key of the system.  When records are received from other systems, a local key is commonly applied.  If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey.
    ///
    /// [ListingKeyNumeric](https://ddwiki.reso.org/display/DDW17/ListingKeyNumeric+%28Showing%29+Field)
    #[serde(rename = "ListingKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub listing_key_numeric: Option<f64>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the listing was input.  In cases where the Originating system was not where the record was retrieved, see the Source System fields.
    ///
    /// [ListingOriginatingSystemID](https://ddwiki.reso.org/display/DDW17/ListingOriginatingSystemID+Field)
    #[serde(
        rename = "ListingOriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub listing_originating_system_id: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the listing is originally input by the member.  The legal name of the company. In cases where the Originating system was not where the record was retrieved, see the Source System fields.
    ///
    /// [ListingOriginatingSystemName](https://ddwiki.reso.org/display/DDW17/ListingOriginatingSystemName+Field)
    #[serde(
        rename = "ListingOriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub listing_originating_system_name: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider. The source system is the system from which the record was directly received. In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [ListingSourceSystemID](https://ddwiki.reso.org/display/DDW17/ListingSourceSystemID+Field)
    #[serde(
        rename = "ListingSourceSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub listing_source_system_id: Option<String>,

    /// The name of the immediate record provider. The system from which the record was directly received. The legal name of the company. In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields
    ///
    /// [ListingSourceSystemName](https://ddwiki.reso.org/display/DDW17/ListingSourceSystemName+Field)
    #[serde(
        rename = "ListingSourceSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub listing_source_system_name: Option<String>,

    /// The transactional timestamp automatically recorded by the system representing the date/time the Showing record was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28Showing%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The transactional timestamp automatically recorded by the system representing the date/time the Showing record was entered and made visible to members of the system.
    ///
    /// [OriginalEntryTimestamp](https://ddwiki.reso.org/display/DDW17/OriginalEntryTimestamp+%28Showing%29+Field)
    #[serde(
        rename = "OriginalEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_entry_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The system key, a unique record identifier, from the Originating system. The Originating system is the system with authoritative control over the record. For example, the Multiple Listing Service where the member was input. There may be cases where the Source System (how you received the record) is not the Originating System. See Source System Key for more information.
    ///
    /// [OriginatingSystemAgentKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemAgentKey+Field)
    #[serde(
        rename = "OriginatingSystemAgentKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_agent_key: Option<String>,

    /// The system key, a unique record identifier, from the Originating system. The Originating system is the system with authoritative control over the record. For example, the Multiple Listing Service where the member was input. There may be cases where the Source System (how you received the record) is not the Originating System. See Source System Key for more information.
    ///
    /// [OriginatingSystemListingKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemListingKey+Field)
    #[serde(
        rename = "OriginatingSystemListingKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_listing_key: Option<String>,

    /// The system key, a unique record identifier, from the Originating system. The Originating system is the system with authoritative control over the record. For example, the Multiple Listing Service where the member was input. There may be cases where the Source System (how you received the record) is not the Originating System. See Source System Key for more information.
    ///
    /// [OriginatingSystemShowingKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemShowingKey+Field)
    #[serde(
        rename = "OriginatingSystemShowingKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_showing_key: Option<String>,

    /// A system unique identifier of the member who has scheduled to access the property. Specifically, in aggregation systems, the ListAgentKey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId.  This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [ShowingAgentKey](https://ddwiki.reso.org/display/DDW17/ShowingAgentKey+%28Showing%29+Field)
    #[serde(rename = "ShowingAgentKey", skip_serializing_if = "Option::is_none")]
    pub showing_agent_key: Option<String>,

    /// A system unique identifier of the member who has scheduled to access the property. Specifically, in aggregation systems, the ListAgentKey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId.  This is a foreign key relating to the Member resource's MemberKey.  This is the numeric only key and used as an alternative to the ShowingAgentKey field.
    ///
    /// [ShowingAgentKeyNumeric](https://ddwiki.reso.org/display/DDW17/ShowingAgentKeyNumeric+%28Showing%29+Field)
    #[serde(
        rename = "ShowingAgentKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_agent_key_numeric: Option<f64>,

    /// The local, well-known identifier for the member who has scheduled to access the property, most commonly representing a buyer. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [ShowingAgentMlsID](https://ddwiki.reso.org/display/DDW17/ShowingAgentMlsID+%28Showing%29+Field)
    #[serde(rename = "ShowingAgentMlsID", skip_serializing_if = "Option::is_none")]
    pub showing_agent_mls_id: Option<String>,

    /// The date and time the showing ends. Where other timestamps are typically stored in UTC, showing start and end date/times are typically stored in the local time zone of the property being showed.
    ///
    /// [ShowingEndTimestamp](https://ddwiki.reso.org/display/DDW17/ShowingEndTimestamp+Field)
    #[serde(
        rename = "ShowingEndTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_end_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The well-known identifier for the showing record. The value may be identical to that of the ShowingKey, but the ShowingID is intended to be the value used by a human to retrieve the information about a specific showing. In a multiple originating system or a merged system, this value may not be unique and may require the use of the provider system to create a synthetic unique value.
    ///
    /// [ShowingId](https://ddwiki.reso.org/display/DDW17/ShowingId+Field)
    #[serde(rename = "ShowingId", skip_serializing_if = "Option::is_none")]
    pub showing_id: Option<String>,

    /// A unique identifier for this record from the immediate source. This is a string that can include URI or other forms.  Alternatively use the ShowingKeyNumeric for a numeric only key field.  This is the local key of the system.  When records are received from other systems, a local key is commonly applied.  If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey.
    ///
    /// [ShowingKey](https://ddwiki.reso.org/display/DDW17/ShowingKey+Field)
    #[serde(rename = "ShowingKey", skip_serializing_if = "Option::is_none")]
    pub showing_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the ShowingKey field.  This is the local key of the system.  When records are received from other systems, a local key is commonly applied.  If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey.
    ///
    /// [ShowingKeyNumeric](https://ddwiki.reso.org/display/DDW17/ShowingKeyNumeric+Field)
    #[serde(rename = "ShowingKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub showing_key_numeric: Option<f64>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the listing was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [ShowingOriginatingSystemID](https://ddwiki.reso.org/display/DDW17/ShowingOriginatingSystemID+Field)
    #[serde(
        rename = "ShowingOriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_originating_system_id: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the listing is originally input by the member.  The legal name of the company.
    ///
    /// [ShowingOriginatingSystemName](https://ddwiki.reso.org/display/DDW17/ShowingOriginatingSystemName+Field)
    #[serde(
        rename = "ShowingOriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_originating_system_name: Option<String>,

    /// The date/time when the showing agent submitted their request to access the property.  This is a system timestamp normally generated by a showing system, which is commonly the Originating System for showing records.
    ///
    /// [ShowingRequestedTimestamp](https://ddwiki.reso.org/display/DDW17/ShowingRequestedTimestamp+Field)
    #[serde(
        rename = "ShowingRequestedTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_requested_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider. The source system is the system from which the record was directly received. In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [ShowingSourceSystemID](https://ddwiki.reso.org/display/DDW17/ShowingSourceSystemID+Field)
    #[serde(
        rename = "ShowingSourceSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_source_system_id: Option<String>,

    /// The name of the immediate record provider. The system from which the record was directly received. The legal name of the company.
    ///
    /// [ShowingSourceSystemName](https://ddwiki.reso.org/display/DDW17/ShowingSourceSystemName+Field)
    #[serde(
        rename = "ShowingSourceSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_source_system_name: Option<String>,

    /// The date and time the showing begins.  Where other timestamps are typically stored in UTC, showing start and end date/times are typically stored in the local time zone of the property being showed.
    ///
    /// [ShowingStartTimestamp](https://ddwiki.reso.org/display/DDW17/ShowingStartTimestamp+Field)
    #[serde(
        rename = "ShowingStartTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_start_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemAgentKey](https://ddwiki.reso.org/display/DDW17/SourceSystemAgentKey+Field)
    #[serde(
        rename = "SourceSystemAgentKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_agent_key: Option<String>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemListingKey](https://ddwiki.reso.org/display/DDW17/SourceSystemListingKey+Field)
    #[serde(
        rename = "SourceSystemListingKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_listing_key: Option<String>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemShowingKey](https://ddwiki.reso.org/display/DDW17/SourceSystemShowingKey+Field)
    #[serde(
        rename = "SourceSystemShowingKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_showing_key: Option<String>,
}
