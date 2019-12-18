// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};

/// [SavedSearch Resource](https://ddwiki.reso.org/display/DDW17/SavedSearch+Resource)
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SavedSearch {
    /// The class or table to which the SearchQuery criteria refers. i.e. Residential, Residential Lease, Income, Mobile, etc.
    ///
    /// [ClassName](https://ddwiki.reso.org/display/DDW17/ClassName+%28SavedSearch%29+Field)
    #[serde(rename = "ClassName", skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the MemberKey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId. This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [MemberKey](https://ddwiki.reso.org/display/DDW17/MemberKey+%28SavedSearch%29+Field)
    #[serde(rename = "MemberKey", skip_serializing_if = "Option::is_none")]
    pub member_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the MemberKey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId. This is a foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the MemberKey field.
    ///
    /// [MemberKeyNumeric](https://ddwiki.reso.org/display/DDW17/MemberKeyNumeric+%28SavedSearch%29+Field)
    #[serde(rename = "MemberKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub member_key_numeric: Option<f64>,

    /// The local, well-known identifier for the member. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [MemberMlsId](https://ddwiki.reso.org/display/DDW17/MemberMlsId+%28SavedSearch%29+Field)
    #[serde(rename = "MemberMlsId", skip_serializing_if = "Option::is_none")]
    pub member_mls_id: Option<String>,

    /// The transactional timestamp automatically recorded by the MLS system representing the date/time the saved search was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28SavedSearch%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The transactional timestamp automatically recorded by the MLS system representing the date/time the saved search was entered.
    ///
    /// [OriginalEntryTimestamp](https://ddwiki.reso.org/display/DDW17/OriginalEntryTimestamp+%28SavedSearch%29+Field)
    #[serde(
        rename = "OriginalEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_entry_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the Saved Search was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+%28SavedSearch%29+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Originating system.  The Originating system is the system with authoritative control over the record.  For example, the Multiple Listing Service where the Saved Search was input.  There may be cases where the Source System (how you received the record) is not the Originating System.  See Source System Key for more information.
    ///
    /// [OriginatingSystemKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemKey+%28SavedSearch%29+Field)
    #[serde(
        rename = "OriginatingSystemKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_key: Option<String>,

    /// Unique identifier from the originating system which is commonly a key to that system. In the case where data is passed through more than one system, this is the originating system key. This is a foreign key relating to the system where this record was originated.
    ///
    /// [OriginatingSystemMemberKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemMemberKey+%28SavedSearch%29+Field)
    #[serde(
        rename = "OriginatingSystemMemberKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_member_key: Option<String>,

    /// The name of the originating record provider. Most commonly the name of the MLS. The place where the listing is originally input by the member. The legal name of the company. To be used for display.
    ///
    /// [OriginatingSystemMemberName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemMemberName+Field)
    #[serde(
        rename = "OriginatingSystemMemberName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_member_name: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the Saved Search is originally input.  The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+%28SavedSearch%29+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// The resource to which the SearchQuery criteria refers. i.e. Property, Open House, Agent, Office, Contact, etc.
    ///
    /// [ResourceName](https://ddwiki.reso.org/display/DDW17/ResourceName+%28SavedSearch%29+Field)
    #[serde(rename = "ResourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,

    /// A textual description of the saved search input by the member who created the saved search.
    ///
    /// [SavedSearchDescription](https://ddwiki.reso.org/display/DDW17/SavedSearchDescription+Field)
    #[serde(
        rename = "SavedSearchDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub saved_search_description: Option<String>,

    /// A unique identifier for this record from the immediate source. This may be a number, or string that can include URI or other forms.  This is the system you are connecting to and not necessarily the original source of the record.
    ///
    /// [SavedSearchKey](https://ddwiki.reso.org/display/DDW17/SavedSearchKey+Field)
    #[serde(rename = "SavedSearchKey", skip_serializing_if = "Option::is_none")]
    pub saved_search_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This may be a number, or string that can include URI or other forms.  This is the system you are connecting to and not necessarily the original source of the record.  This is the numeric only key and used as an alternative to the SavedSearchKey field.
    ///
    /// [SavedSearchKeyNumeric](https://ddwiki.reso.org/display/DDW17/SavedSearchKeyNumeric+Field)
    #[serde(
        rename = "SavedSearchKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub saved_search_key_numeric: Option<f64>,

    /// The name given to the search by the member inputting the saved search.
    ///
    /// [SavedSearchName](https://ddwiki.reso.org/display/DDW17/SavedSearchName+Field)
    #[serde(rename = "SavedSearchName", skip_serializing_if = "Option::is_none")]
    pub saved_search_name: Option<String>,

    /// Is the saved search used to pass criteria to be stored and executed by the client or is the saved search a key to be passed to the host for execution.  i.e. Client Receives Criteria, Host Returns Listings.  This may be described at the record level with this field, or at some other level of implementation to be determined by RESO R&D.
    ///
    /// [SavedSearchType](https://ddwiki.reso.org/display/DDW17/SavedSearchType+Field)
    #[serde(rename = "SavedSearchType", skip_serializing_if = "Option::is_none")]
    pub saved_search_type: Option<String>,

    /// Textual representation of the search performed by the member that was saved.  It is required to present in ODATA's $filter format.  Additional formats are under review.  See additional documentation for specific requirements for this field.
    ///
    /// [SearchQuery](https://ddwiki.reso.org/display/DDW17/SearchQuery+Field)
    #[serde(rename = "SearchQuery", skip_serializing_if = "Option::is_none")]
    pub search_query: Option<String>,

    /// A free text description used to expand on the SearchQueryExceptions selections made by the host.
    ///
    /// [SearchQueryExceptionDetails](https://ddwiki.reso.org/display/DDW17/SearchQueryExceptionDetails+Field)
    #[serde(
        rename = "SearchQueryExceptionDetails",
        skip_serializing_if = "Option::is_none"
    )]
    pub search_query_exception_details: Option<String>,

    /// A list of exceptions or errors with the given search query during it's creation by the host.  Analogous to an error code this is the host's opportunity to describe an inability to fully express a saved search under the constraints of the given protocol.  i.e. $filter.  The client may use this information to bring attention to the user about a given saved search and a need to review or recreate the search.
    ///
    /// [SearchQueryExceptions](https://ddwiki.reso.org/display/DDW17/SearchQueryExceptions+Field)
    #[serde(
        rename = "SearchQueryExceptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub search_query_exceptions: Option<String>,

    /// A human readable version of the search query that is commonly used for display and may not contain all actual criteria.  For actual search criteria, use the SearchQuery field.
    ///
    /// [SearchQueryHumanReadable](https://ddwiki.reso.org/display/DDW17/SearchQueryHumanReadable+Field)
    #[serde(
        rename = "SearchQueryHumanReadable",
        skip_serializing_if = "Option::is_none"
    )]
    pub search_query_human_readable: Option<String>,

    /// A picklist of the type of query language used in the SearchQuery field. i.e. DMQL2, $filter, etc.
    ///
    /// [SearchQueryType](https://ddwiki.reso.org/display/DDW17/SearchQueryType+Field)
    #[serde(rename = "SearchQueryType", skip_serializing_if = "Option::is_none")]
    pub search_query_type: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider.  The source system is the system from which the record was directly received.  In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+%28SavedSearch%29+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Source System.  The Source System is the system from which the record was directly received.  In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemKey](https://ddwiki.reso.org/display/DDW17/SourceSystemKey+%28SavedSearch%29+Field)
    #[serde(rename = "SourceSystemKey", skip_serializing_if = "Option::is_none")]
    pub source_system_key: Option<String>,

    /// The name of the Saved Search record provider.  The system from which the record was directly received.  The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+%28SavedSearch%29+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,
}
