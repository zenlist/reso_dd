// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};

/// [InternetTracking Resource](https://ddwiki.reso.org/display/DDW17/InternetTracking+Resource)
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InternetTracking {
    /// The city location of the Actor as recorded by the source.
    ///
    /// [ActorCity](https://ddwiki.reso.org/display/DDW17/ActorCity+Field)
    #[serde(rename = "ActorCity", skip_serializing_if = "Option::is_none")]
    pub actor_city: Option<String>,

    /// The email address of the Actor in this event.
    ///
    /// [ActorEmail](https://ddwiki.reso.org/display/DDW17/ActorEmail+Field)
    #[serde(rename = "ActorEmail", skip_serializing_if = "Option::is_none")]
    pub actor_email: Option<String>,

    /// The local, well-known identifier the actor, provided by the source when applicable.  This value may not be unique specifically in the case of aggregation systems, this value should be the human friendly identifier from the original system.  Use of the ID may be common when the actor is an MLS or other software user.  Otherwise, use the ActorKey or ActorKeyNumeric is recommended.
    ///
    /// [ActorID](https://ddwiki.reso.org/display/DDW17/ActorID+Field)
    #[serde(rename = "ActorID", skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,

    /// The recorded IP address of the Actor in this event.  IPv4 addresses are 15 charaters and IPv6 addresses are a max of 39 characters.  IP addresses should not omit leading zeros.  i.e. 10.1.1.1 should appear as 010.001.001.001.
    ///
    /// [ActorIP](https://ddwiki.reso.org/display/DDW17/ActorIP+Field)
    #[serde(rename = "ActorIP", skip_serializing_if = "Option::is_none")]
    pub actor_ip: Option<String>,

    /// A unique identifier for this record from the immediate source. This is a string that can include URI or other forms. Alternatively use the MemberKeyNumeric for a numeric only key field. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, utilize the SourceSystemActorKey and/or the OriginatingSystemActorKey.
    ///
    /// [ActorKey](https://ddwiki.reso.org/display/DDW17/ActorKey+Field)
    #[serde(rename = "ActorKey", skip_serializing_if = "Option::is_none")]
    pub actor_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the MemberKey fields. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, utilize the SourceSystemActorKey and/or the OriginatingSystemActorKey.
    ///
    /// [ActorKeyNumeric](https://ddwiki.reso.org/display/DDW17/ActorKeyNumeric+Field)
    #[serde(rename = "ActorKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub actor_key_numeric: Option<f64>,

    /// The geographic latitude of some reference point for the location of the actor, specified in degrees and decimal parts. Positive numbers must not include the plus symbol.
    ///
    /// [ActorLatitude](https://ddwiki.reso.org/display/DDW17/ActorLatitude+Field)
    #[serde(rename = "ActorLatitude", skip_serializing_if = "Option::is_none")]
    pub actor_latitude: Option<f64>,

    /// The geographic longitude of some reference point for the location of the actor, specified in degrees and decimal parts. Positive numbers must not include the plus symbol.
    ///
    /// [ActorLongitude](https://ddwiki.reso.org/display/DDW17/ActorLongitude+Field)
    #[serde(rename = "ActorLongitude", skip_serializing_if = "Option::is_none")]
    pub actor_longitude: Option<f64>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the listing was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [ActorOriginatingSystemID](https://ddwiki.reso.org/display/DDW17/ActorOriginatingSystemID+Field)
    #[serde(
        rename = "ActorOriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_originating_system_id: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the listing is originally input by the member.  The legal name of the company.
    ///
    /// [ActorOriginatingSystemName](https://ddwiki.reso.org/display/DDW17/ActorOriginatingSystemName+Field)
    #[serde(
        rename = "ActorOriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_originating_system_name: Option<String>,

    /// The phone number of the Actor in this event.
    ///
    /// [ActorPhone](https://ddwiki.reso.org/display/DDW17/ActorPhone+Field)
    #[serde(rename = "ActorPhone", skip_serializing_if = "Option::is_none")]
    pub actor_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [ActorPhoneExt](https://ddwiki.reso.org/display/DDW17/ActorPhoneExt+Field)
    #[serde(rename = "ActorPhoneExt", skip_serializing_if = "Option::is_none")]
    pub actor_phone_ext: Option<String>,

    /// The postal code of the Actor.
    ///
    /// [ActorPostalCode](https://ddwiki.reso.org/display/DDW17/ActorPostalCode+Field)
    #[serde(rename = "ActorPostalCode", skip_serializing_if = "Option::is_none")]
    pub actor_postal_code: Option<String>,

    /// The extension of the postal/zip code. i.e. +4
    ///
    /// [ActorPostalCodePlus4](https://ddwiki.reso.org/display/DDW17/ActorPostalCodePlus4+Field)
    #[serde(
        rename = "ActorPostalCodePlus4",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_postal_code_plus4: Option<String>,

    /// A geographical region defined by the source.  For this use, the regoin is not specifically at the city, county, state, country or contenant level and this is typical in internet tracking standards.
    ///
    /// [ActorRegion](https://ddwiki.reso.org/display/DDW17/ActorRegion+Field)
    #[serde(rename = "ActorRegion", skip_serializing_if = "Option::is_none")]
    pub actor_region: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider. The source system is the system from which the record was directly received. In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [ActorSourceSystemID](https://ddwiki.reso.org/display/DDW17/ActorSourceSystemID+Field)
    #[serde(
        rename = "ActorSourceSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_source_system_id: Option<String>,

    /// The name of the immediate record provider. The system from which the record was directly received. The legal name of the company.
    ///
    /// [ActorSourceSystemName](https://ddwiki.reso.org/display/DDW17/ActorSourceSystemName+Field)
    #[serde(
        rename = "ActorSourceSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_source_system_name: Option<String>,

    /// The state or province location of the Actor as recorded by the source.
    ///
    /// [ActorStateOrProvince](https://ddwiki.reso.org/display/DDW17/ActorStateOrProvince+Field)
    #[serde(
        rename = "ActorStateOrProvince",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_state_or_province: Option<String>,

    /// A list of actor types; where the event was originated. (i.e. Agent, Consumer, Bot)  In implementation this is typically a required field
    ///
    /// [ActorType](https://ddwiki.reso.org/display/DDW17/ActorType+Field)
    #[serde(rename = "ActorType", skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<String>,

    /// The color depth of the Actor's device display
    ///
    /// [ColorDepth](https://ddwiki.reso.org/display/DDW17/ColorDepth+Field)
    #[serde(rename = "ColorDepth", skip_serializing_if = "Option::is_none")]
    pub color_depth: Option<f64>,

    /// The device type used by the Actor (mobile, desktop etc...) in this event
    ///
    /// [DeviceType](https://ddwiki.reso.org/display/DDW17/DeviceType+Field)
    #[serde(rename = "DeviceType", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,

    /// A description of the event being tracked. (i.e. "the listing was viewed")
    ///
    /// [EventDescription](https://ddwiki.reso.org/display/DDW17/EventDescription+Field)
    #[serde(rename = "EventDescription", skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,

    /// A unique identifier for this record from the immediate source. This is a string that can include URI or other forms. Alternatively use the MemberKeyNumeric for a numeric only key field. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, utilize the SourceSystemEventKey and/or the OriginatingSystemEventKey.
    ///
    /// [EventKey](https://ddwiki.reso.org/display/DDW17/EventKey+Field)
    #[serde(rename = "EventKey", skip_serializing_if = "Option::is_none")]
    pub event_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the MemberKey fields. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, utilize the SourceSystemEventKey and/or the OriginatingSystemEventKey.
    ///
    /// [EventKeyNumeric](https://ddwiki.reso.org/display/DDW17/EventKeyNumeric+Field)
    #[serde(rename = "EventKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub event_key_numeric: Option<f64>,

    /// A short description of the Event being tracked.
    ///
    /// [EventLabel](https://ddwiki.reso.org/display/DDW17/EventLabel+Field)
    #[serde(rename = "EventLabel", skip_serializing_if = "Option::is_none")]
    pub event_label: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the listing was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [EventOriginatingSystemID](https://ddwiki.reso.org/display/DDW17/EventOriginatingSystemID+Field)
    #[serde(
        rename = "EventOriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub event_originating_system_id: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the listing is originally input by the member.  The legal name of the company.
    ///
    /// [EventOriginatingSystemName](https://ddwiki.reso.org/display/DDW17/EventOriginatingSystemName+Field)
    #[serde(
        rename = "EventOriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub event_originating_system_name: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider. The source system is the system from which the record was directly received. In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [EventSourceSystemID](https://ddwiki.reso.org/display/DDW17/EventSourceSystemID+Field)
    #[serde(
        rename = "EventSourceSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub event_source_system_id: Option<String>,

    /// The name of the immediate record provider. The system from which the record was directly received. The legal name of the company.
    ///
    /// [EventSourceSystemName](https://ddwiki.reso.org/display/DDW17/EventSourceSystemName+Field)
    #[serde(
        rename = "EventSourceSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub event_source_system_name: Option<String>,

    /// A defined target of the event type.
    ///
    /// [EventTarget](https://ddwiki.reso.org/display/DDW17/EventTarget+Field)
    #[serde(rename = "EventTarget", skip_serializing_if = "Option::is_none")]
    pub event_target: Option<String>,

    /// A UTC timestamp of when the event being tracked occurred.  In implementation this is typically a required field.
    ///
    /// [EventTimestamp](https://ddwiki.reso.org/display/DDW17/EventTimestamp+Field)
    #[serde(rename = "EventTimestamp", skip_serializing_if = "Option::is_none")]
    pub event_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The type of event being tracked. In implementation this is typically a required a field.
    ///
    /// [EventType](https://ddwiki.reso.org/display/DDW17/EventType+Field)
    #[serde(rename = "EventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,

    /// An ID pertaining to the ObjectType (i.e. the MLS listing id for ObjectType.Listing).  When the ObjectIdType is a property, this should be a PUID.
    ///
    /// [ObjectID](https://ddwiki.reso.org/display/DDW17/ObjectID+Field)
    #[serde(rename = "ObjectID", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// A label that defines the ObjectID field (i.e. ObjectID is an MLS listing ID or ObjectID is a unique ID from the source...)
    ///
    /// [ObjectIdType](https://ddwiki.reso.org/display/DDW17/ObjectIdType+Field)
    #[serde(rename = "ObjectIdType", skip_serializing_if = "Option::is_none")]
    pub object_id_type: Option<String>,

    /// A unique identifier for this record from the immediate source. This is a string that can include URI or other forms. Alternatively use the MemberKeyNumeric for a numeric only key field. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, utilize the SourceSystemObjectKey and/or the OriginatingSystemObjectKey.
    ///
    /// [ObjectKey](https://ddwiki.reso.org/display/DDW17/ObjectKey+Field)
    #[serde(rename = "ObjectKey", skip_serializing_if = "Option::is_none")]
    pub object_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the MemberKey fields. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, utilize the SourceSystemObjectKey and/or the OriginatingSystemObjectKey.
    ///
    /// [ObjectKeyNumeric](https://ddwiki.reso.org/display/DDW17/ObjectKeyNumeric+Field)
    #[serde(rename = "ObjectKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub object_key_numeric: Option<f64>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the listing was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [ObjectOriginatingSystemID](https://ddwiki.reso.org/display/DDW17/ObjectOriginatingSystemID+Field)
    #[serde(
        rename = "ObjectOriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_originating_system_id: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the listing is originally input by the member.  The legal name of the company.
    ///
    /// [ObjectOriginatingSystemName](https://ddwiki.reso.org/display/DDW17/ObjectOriginatingSystemName+Field)
    #[serde(
        rename = "ObjectOriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_originating_system_name: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider. The source system is the system from which the record was directly received. In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [ObjectSourceSystemID](https://ddwiki.reso.org/display/DDW17/ObjectSourceSystemID+Field)
    #[serde(
        rename = "ObjectSourceSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_source_system_id: Option<String>,

    /// The name of the immediate record provider. The system from which the record was directly received. The legal name of the company.
    ///
    /// [ObjectSourceSystemName](https://ddwiki.reso.org/display/DDW17/ObjectSourceSystemName+Field)
    #[serde(
        rename = "ObjectSourceSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_source_system_name: Option<String>,

    /// The type of Object being tracked in this event.  In implementation this is typically a required field.
    ///
    /// [ObjectType](https://ddwiki.reso.org/display/DDW17/ObjectType+Field)
    #[serde(rename = "ObjectType", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,

    /// The URL of the tracked event.
    ///
    /// [ObjectURL](https://ddwiki.reso.org/display/DDW17/ObjectURL+Field)
    #[serde(rename = "ObjectURL", skip_serializing_if = "Option::is_none")]
    pub object_url: Option<String>,

    /// The system key, a unique record identifier, from the Originating system. The Originating system is the system with authoritative control over the record. For example, the Multiple Listing Service where the member was input. There may be cases where the Source System (how you received the record) is not the Originating System. See Source System Key for more information.
    ///
    /// [OriginatingSystemActorKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemActorKey+Field)
    #[serde(
        rename = "OriginatingSystemActorKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_actor_key: Option<String>,

    /// The system key, a unique record identifier, from the Originating system. The Originating system is the system with authoritative control over the record. For example, the Multiple Listing Service where the member was input. There may be cases where the Source System (how you received the record) is not the Originating System. See Source System Key for more information.
    ///
    /// [OriginatingSystemEventKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemEventKey+Field)
    #[serde(
        rename = "OriginatingSystemEventKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_event_key: Option<String>,

    /// The system key, a unique record identifier, from the Originating system. The Originating system is the system with authoritative control over the record. For example, the Multiple Listing Service where the member was input. There may be cases where the Source System (how you received the record) is not the Originating System. See Source System Key for more information.
    ///
    /// [OriginatingSystemObjectKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemObjectKey+Field)
    #[serde(
        rename = "OriginatingSystemObjectKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_object_key: Option<String>,

    /// The referring URL of the tracked event.
    ///
    /// [ReferringURL](https://ddwiki.reso.org/display/DDW17/ReferringURL+Field)
    #[serde(rename = "ReferringURL", skip_serializing_if = "Option::is_none")]
    pub referring_url: Option<String>,

    /// The screen height, in pixels, of the Actor's device
    ///
    /// [ScreenHeight](https://ddwiki.reso.org/display/DDW17/ScreenHeight+Field)
    #[serde(rename = "ScreenHeight", skip_serializing_if = "Option::is_none")]
    pub screen_height: Option<f64>,

    /// The screen width, in pixels, of the Actor's device
    ///
    /// [ScreenWidth](https://ddwiki.reso.org/display/DDW17/ScreenWidth+Field)
    #[serde(rename = "ScreenWidth", skip_serializing_if = "Option::is_none")]
    pub screen_width: Option<f64>,

    /// A unique session ID number, created by the source, that can be used to query data for a single session
    ///
    /// [SessionID](https://ddwiki.reso.org/display/DDW17/SessionID+Field)
    #[serde(rename = "SessionID", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemActorKey](https://ddwiki.reso.org/display/DDW17/SourceSystemActorKey+Field)
    #[serde(
        rename = "SourceSystemActorKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_actor_key: Option<String>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemEventKey](https://ddwiki.reso.org/display/DDW17/SourceSystemEventKey+Field)
    #[serde(
        rename = "SourceSystemEventKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_event_key: Option<String>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemObjectKey](https://ddwiki.reso.org/display/DDW17/SourceSystemObjectKey+Field)
    #[serde(
        rename = "SourceSystemObjectKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_object_key: Option<String>,

    /// The timezone offset is the difference, in minutes, between UTC and local time.
    ///
    /// [TimeZoneOffset](https://ddwiki.reso.org/display/DDW17/TimeZoneOffset+Field)
    #[serde(rename = "TimeZoneOffset", skip_serializing_if = "Option::is_none")]
    pub time_zone_offset: Option<f64>,

    /// The software agent acting on behalf of the user (Actor) in this event.  This is commonly conveyed by browser applications. e.g. Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; Touch; rv:11.0).  This can also be a user configurable string as seen in RETS client applications.
    ///
    /// [UserAgent](https://ddwiki.reso.org/display/DDW17/UserAgent+Field)
    #[serde(rename = "UserAgent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
