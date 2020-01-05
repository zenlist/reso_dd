// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// [ContactListings Resource](https://ddwiki.reso.org/display/DDW17/ContactListings+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ContactListings {
    /// When True, one or more of the agent notes are unread.
    ///
    /// [AgentNotesUnreadYN](https://ddwiki.reso.org/display/DDW17/AgentNotesUnreadYN+Field)
    #[serde(rename = "AgentNotesUnreadYN", skip_serializing_if = "Option::is_none")]
    pub agent_notes_unread_yn: Option<bool>,

    /// The name of the class where the listing record is located.
    ///
    /// [ClassName](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246771)
    #[serde(rename = "ClassName", skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// This is the foreign key relating to the Contact resource.  A unique identifier for this record from the immediate source. This is a string that can include URI or other forms.  Alternatively use the ContactKeyNumeric for a numeric only key field.  This is the local key of the system.  When records are received from other systems, a local key is commonly applied.  If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey variants.
    ///
    /// [ContactKey](https://ddwiki.reso.org/display/DDW17/ContactKey+%28ContactListings%29+Field)
    #[serde(rename = "ContactKey", skip_serializing_if = "Option::is_none")]
    pub contact_key: Option<String>,

    /// This is the foreign key relating to the Contact resource.  A unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the ContactKey fields.  This is the local key of the system.  When records are received from other systems, a local key is commonly applied.  If conveying the original keys from the source or originating systems, see SourceSystemKeyNumeric and OriginatingSystemKeyNumeric variants.
    ///
    /// [ContactKeyNumeric](https://ddwiki.reso.org/display/DDW17/ContactKeyNumeric+%28ContactListings%29+Field)
    #[serde(rename = "ContactKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub contact_key_numeric: Option<f64>,

    /// The contacts preference selection on the given listing.  Favorite, Possibility or Discard.
    ///
    /// [ContactListingPreference](https://ddwiki.reso.org/display/DDW17/ContactListingPreference+Field)
    #[serde(
        rename = "ContactListingPreference",
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_listing_preference: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set..
    ///
    /// [ContactListingsKey](https://ddwiki.reso.org/display/DDW17/ContactListingsKey+Field)
    #[serde(rename = "ContactListingsKey", skip_serializing_if = "Option::is_none")]
    pub contact_listings_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This may be a number, or string that can include URI or other forms. This is the system you are connecting to and not necessarily the original source of the record. This is the numeric only key and used as an alternative to the ContactListingsKey field.
    ///
    /// [ContactListingsKeyNumeric](https://ddwiki.reso.org/display/DDW17/ContactListingsKeyNumeric+Field)
    #[serde(
        rename = "ContactListingsKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_listings_key_numeric: Option<f64>,

    /// This is a foreign key refering to the Contact Resource's local, well-known identifier for the contact. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system and is used by the Contact to logon to a client portal in that system.
    ///
    /// [ContactLoginId](https://ddwiki.reso.org/display/DDW17/ContactLoginId+%28ContactListings%29+Field)
    #[serde(rename = "ContactLoginId", skip_serializing_if = "Option::is_none")]
    pub contact_login_id: Option<String>,

    /// When True, one or more of the contacts notes are unread.
    ///
    /// [ContactNotesUnreadYN](https://ddwiki.reso.org/display/DDW17/ContactNotesUnreadYN+Field)
    #[serde(
        rename = "ContactNotesUnreadYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_notes_unread_yn: Option<bool>,

    /// If True, the email was a direct email sent to the client by the member.  If False the email was an auto email.
    ///
    /// [DirectEmailYN](https://ddwiki.reso.org/display/DDW17/DirectEmailYN+Field)
    #[serde(rename = "DirectEmailYN", skip_serializing_if = "Option::is_none")]
    pub direct_email_yn: Option<bool>,

    /// The Date/Time the Member last added or updated a ListingNote.
    ///
    /// [LastAgentNoteTimestamp](https://ddwiki.reso.org/display/DDW17/LastAgentNoteTimestamp+Field)
    #[serde(
        rename = "LastAgentNoteTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_agent_note_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The Date/Time the Contact last added or updated a ListingNote.
    ///
    /// [LastContactNoteTimestamp](https://ddwiki.reso.org/display/DDW17/LastContactNoteTimestamp+Field)
    #[serde(
        rename = "LastContactNoteTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_contact_note_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The well known identifier for the listing. The value may be identical to that of the Listing Key, but the Listing ID is intended to be the value used by a human to retrieve the information about a specific listing. In a multiple originating system or a merged system, this value may not be unique and may require the use of the provider system to create a synthetic unique value.
    ///
    /// [ListingId](https://ddwiki.reso.org/display/DDW17/ListingId+%28ContactListings%29+Field)
    #[serde(rename = "ListingId", skip_serializing_if = "Option::is_none")]
    pub listing_id: Option<String>,

    /// This is the foreign key related to the Property Resource's unique identifier for this record from the immediate source. This is a string that can include URI or other forms. Alternatively use the ListingKeyNumeric for a numeric only key field. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey.
    ///
    /// [ListingKey](https://ddwiki.reso.org/display/DDW17/ListingKey+%28ContactListings%29+Field)
    #[serde(rename = "ListingKey", skip_serializing_if = "Option::is_none")]
    pub listing_key: Option<String>,

    /// This is the foreign key related to the Property Resource's unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the ListingKey fields. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey.
    ///
    /// [ListingKeyNumeric](https://ddwiki.reso.org/display/DDW17/ListingKeyNumeric+%28ContactListings%29+Field)
    #[serde(rename = "ListingKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub listing_key_numeric: Option<f64>,

    /// The last time the listing was updated. This does not refer to the ContactListing record, but changes to the referenced listing.
    ///
    /// [ListingModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ListingModificationTimestamp+Field)
    #[serde(
        rename = "ListingModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub listing_modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The notes input by the Member and/or the Contact in reference to the given listing. This is a complex data type referencing the separate resource called ContactListingNotes.
    ///
    /// [ListingNotes](https://ddwiki.reso.org/display/DDW17/ListingNotes+Field)
    #[serde(rename = "ListingNotes", skip_serializing_if = "Option::is_none")]
    pub listing_notes: Option<String>,

    /// The Date/Time the listing was sent to the contact.
    ///
    /// [ListingSentTimestamp](https://ddwiki.reso.org/display/DDW17/ListingSentTimestamp+Field)
    #[serde(
        rename = "ListingSentTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub listing_sent_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// When True, the Client has viewed the listing.  This is typically when the client clicks to view a detailed report, rather than seen in a one line  or thumbnail display.
    ///
    /// [ListingViewedYN](https://ddwiki.reso.org/display/DDW17/ListingViewedYN+Field)
    #[serde(rename = "ListingViewedYN", skip_serializing_if = "Option::is_none")]
    pub listing_viewed_yn: Option<bool>,

    /// The Date/Time that the ContactListing record was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28ContactListings%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The Date/Time the listing was last viewed by the Contact.
    ///
    /// [PortalLastVisitedTimestamp](https://ddwiki.reso.org/display/DDW17/PortalLastVisitedTimestamp+Field)
    #[serde(
        rename = "PortalLastVisitedTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub portal_last_visited_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The name of the resource where the listing record is located.
    ///
    /// [ResourceName](https://ddwiki.reso.org/display/DDW17/ResourceName+%28ContactListings%29+Field)
    #[serde(rename = "ResourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
