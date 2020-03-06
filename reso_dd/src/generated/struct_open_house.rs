// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// [OpenHouse Resource](https://ddwiki.reso.org/display/DDW17/OpenHouse+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OpenHouse {
    /// Indicates whether or not the OpenHouse requires an appointment.
    ///
    /// [AppointmentRequiredYN](https://ddwiki.reso.org/display/DDW17/AppointmentRequiredYN+Field)
    #[serde(
        rename = "AppointmentRequiredYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub appointment_required_yn: Option<bool>,

    /// The well known identifier for the listing related to this Open House. The value may be identical to that of the Listing Key, but the Listing ID is intended to be the value used by a human to retrieve the information about a specific listing. In a multiple originating system or a merged system, this value may not be unique and may require the use of the provider system to create a synthetic unique value.
    ///
    /// [ListingId](https://ddwiki.reso.org/display/DDW17/ListingId+%28OpenHouse%29+Field)
    #[serde(rename = "ListingId", skip_serializing_if = "Option::is_none")]
    pub listing_id: Option<String>,

    /// A unique identifier for the listing record related to this Open House. This may be a number, or string that can include URI or other forms.  This is the system you are connecting to and not necessarily the original source of the record.   This may be a foreign key from the resource selected in the ResourceName field.
    ///
    /// [ListingKey](https://ddwiki.reso.org/display/DDW17/ListingKey+%28OpenHouse%29+Field)
    #[serde(rename = "ListingKey", skip_serializing_if = "Option::is_none")]
    pub listing_key: Option<String>,

    /// A unique identifier for the listing record related to this Open House. This may be a number, or string that can include URI or other forms.  This is the system you are connecting to and not necessarily the original source of the record.   This may be a foreign key from the resource selected in the ResourceName field.  This is the numeric only key and used as an alternative to the ListingKey field.
    ///
    /// [ListingKeyNumeric](https://ddwiki.reso.org/display/DDW17/ListingKeyNumeric+%28OpenHouse%29+Field)
    #[serde(rename = "ListingKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub listing_key_numeric: Option<f64>,

    /// The transactional timestamp automatically recorded by the MLS system representing the date/time the Open House was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28OpenHouse%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Will the open house be attended by a licensed agent? Options are attended by agent, attended by the seller or unattended.
    ///
    /// [OpenHouseAttendedBy](https://ddwiki.reso.org/display/DDW17/OpenHouseAttendedBy+Field)
    #[serde(
        rename = "OpenHouseAttendedBy",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_house_attended_by: Option<crate::Attended>,

    /// The date on which the open house will occur.
    ///
    /// [OpenHouseDate](https://ddwiki.reso.org/display/DDW17/OpenHouseDate+Field)
    #[serde(rename = "OpenHouseDate", skip_serializing_if = "Option::is_none")]
    pub open_house_date: Option<chrono::NaiveDate>,

    /// The time the open house ends.
    ///
    /// [OpenHouseEndTime](https://ddwiki.reso.org/display/DDW17/OpenHouseEndTime+Field)
    #[serde(rename = "OpenHouseEndTime", skip_serializing_if = "Option::is_none")]
    pub open_house_end_time: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The well-known identifier for the Open House resource. The value may be identical to that of the OpenHouseKey, but the OpenHouseId intended to be the value used by a human to retrieve the information about a specific open house. In a multiple originating system or a merged system, this value may not be unique and may require the use of the provider system to create a synthetic unique value.
    ///
    /// [OpenHouseId](https://ddwiki.reso.org/display/DDW17/OpenHouseId+Field)
    #[serde(rename = "OpenHouseId", skip_serializing_if = "Option::is_none")]
    pub open_house_id: Option<String>,

    /// A unique identifier for this record from the immediate source. This may be a number, or string that can include URI or other forms.  This is the system you are connecting to and not necessarily the original source of the record.
    ///
    /// [OpenHouseKey](https://ddwiki.reso.org/display/DDW17/OpenHouseKey+Field)
    #[serde(rename = "OpenHouseKey", skip_serializing_if = "Option::is_none")]
    pub open_house_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This may be a number, or string that can include URI or other forms.  This is the system you are connecting to and not necessarily the original source of the record.  This is the numeric only key and used as an alternative to the OpenHouseKey field.
    ///
    /// [OpenHouseKeyNumeric](https://ddwiki.reso.org/display/DDW17/OpenHouseKeyNumeric+Field)
    #[serde(
        rename = "OpenHouseKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_house_key_numeric: Option<f64>,

    /// Comments, instructions or information about the open house.
    ///
    /// [OpenHouseRemarks](https://ddwiki.reso.org/display/DDW17/OpenHouseRemarks+Field)
    #[serde(rename = "OpenHouseRemarks", skip_serializing_if = "Option::is_none")]
    pub open_house_remarks: Option<String>,

    /// The time the open house begins.
    ///
    /// [OpenHouseStartTime](https://ddwiki.reso.org/display/DDW17/OpenHouseStartTime+Field)
    #[serde(rename = "OpenHouseStartTime", skip_serializing_if = "Option::is_none")]
    pub open_house_start_time: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Status of the open house, i.e. Active, Cancelled, Ended.
    ///
    /// [OpenHouseStatus](https://ddwiki.reso.org/display/DDW17/OpenHouseStatus+Field)
    #[serde(rename = "OpenHouseStatus", skip_serializing_if = "Option::is_none")]
    pub open_house_status: Option<crate::OpenHouseStatus>,

    /// The type of open house.  i.e. Public, Broker, Office, Association, Private (invitation or targeted publication).
    ///
    /// [OpenHouseType](https://ddwiki.reso.org/display/DDW17/OpenHouseType+Field)
    #[serde(rename = "OpenHouseType", skip_serializing_if = "Option::is_none")]
    pub open_house_type: Option<crate::OpenHouseType>,

    /// The transactional timestamp automatically recorded by the MLS system representing the date/time the Open House was entered and made visible to members of the MLS.
    ///
    /// [OriginalEntryTimestamp](https://ddwiki.reso.org/display/DDW17/OriginalEntryTimestamp+%28OpenHouse%29+Field)
    #[serde(
        rename = "OriginalEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_entry_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the Open House was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+%28OpenHouse%29+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Originating system.  The Originating system is the system with authoritative control over the record.  For example, the Multiple Listing Service where the Open House was input.  There may be cases where the Source System (how you received the record) is not the Originating System.  See Source System Key for more information.
    ///
    /// [OriginatingSystemKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemKey+%28OpenHouse%29+Field)
    #[serde(
        rename = "OriginatingSystemKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_key: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the Open House is originally input.  The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+%28OpenHouse%29+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// A description of the refreshments that will be served at the open house.
    ///
    /// [Refreshments](https://ddwiki.reso.org/display/DDW17/Refreshments+Field)
    #[serde(rename = "Refreshments", skip_serializing_if = "Option::is_none")]
    pub refreshments: Option<String>,

    /// The first name of the showing agent.
    ///
    /// [ShowingAgentFirstName](https://ddwiki.reso.org/display/DDW17/ShowingAgentFirstName+Field)
    #[serde(
        rename = "ShowingAgentFirstName",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_agent_first_name: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the ListAgentKey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId. This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [ShowingAgentKey](https://ddwiki.reso.org/display/DDW17/ShowingAgentKey+Field)
    #[serde(rename = "ShowingAgentKey", skip_serializing_if = "Option::is_none")]
    pub showing_agent_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the ListAgentKey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId. This is a foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the ShowingAgentKey field.
    ///
    /// [ShowingAgentKeyNumeric](https://ddwiki.reso.org/display/DDW17/ShowingAgentKeyNumeric+Field)
    #[serde(
        rename = "ShowingAgentKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_agent_key_numeric: Option<f64>,

    /// The last name of the showing agent.
    ///
    /// [ShowingAgentLastName](https://ddwiki.reso.org/display/DDW17/ShowingAgentLastName+Field)
    #[serde(
        rename = "ShowingAgentLastName",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_agent_last_name: Option<String>,

    /// The local, well-known identifier for the member. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [ShowingAgentMlsID](https://ddwiki.reso.org/display/DDW17/ShowingAgentMlsID+Field)
    #[serde(rename = "ShowingAgentMlsID", skip_serializing_if = "Option::is_none")]
    pub showing_agent_mls_id: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider.  The source system is the system from which the record was directly received.  In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+%28OpenHouse%29+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemKey](https://ddwiki.reso.org/display/DDW17/SourceSystemKey+%28OpenHouse%29+Field)
    #[serde(rename = "SourceSystemKey", skip_serializing_if = "Option::is_none")]
    pub source_system_key: Option<String>,

    /// The name of the Open House record provider.  The system from which the record was directly received.  The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+%28OpenHouse%29+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,
}
