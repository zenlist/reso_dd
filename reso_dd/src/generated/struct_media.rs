// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};

/// [Media Resource](https://ddwiki.reso.org/display/DDW17/Media+Resource)
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Media {
    /// ID of the user, agent, member, etc., that uploaded the media this record refers to.
    ///
    /// [ChangedByMemberID](https://ddwiki.reso.org/display/DDW17/ChangedByMemberID+Field)
    #[serde(rename = "ChangedByMemberID", skip_serializing_if = "Option::is_none")]
    pub changed_by_member_id: Option<String>,

    /// The primary key of the member who uploaded the media this record refers to. This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [ChangedByMemberKey](https://ddwiki.reso.org/display/DDW17/ChangedByMemberKey+Field)
    #[serde(rename = "ChangedByMemberKey", skip_serializing_if = "Option::is_none")]
    pub changed_by_member_key: Option<String>,

    /// The primary key of the member who uploaded the media this record refers to. This is a foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the ChangedByMemberKey field.
    ///
    /// [ChangedByMemberKeyNumeric](https://ddwiki.reso.org/display/DDW17/ChangedByMemberKeyNumeric+Field)
    #[serde(
        rename = "ChangedByMemberKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub changed_by_member_key_numeric: Option<f64>,

    /// The class or table of the listing or other record the media. Residential, Lease, Agent, Office, Contact, etc.
    ///
    /// [ClassName](https://ddwiki.reso.org/display/DDW17/ClassName+Field)
    #[serde(rename = "ClassName", skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// The height of the image expressed in pixels.
    ///
    /// [ImageHeight](https://ddwiki.reso.org/display/DDW17/ImageHeight+Field)
    #[serde(rename = "ImageHeight", skip_serializing_if = "Option::is_none")]
    pub image_height: Option<f64>,

    /// When the media is an image, a list of possible matches such as kitchen, bathroom, front of structure, etc. This field may be used to identify a required image under association or MLS rules.
    ///
    /// [ImageOf](https://ddwiki.reso.org/display/DDW17/ImageOf+Field)
    #[serde(rename = "ImageOf", skip_serializing_if = "Option::is_none")]
    pub image_of: Option<String>,

    /// A text description of the size of the image.  i.e. Small, Thumbnail, Medium, Large, X-Large.  The largest image must be described as "Largest".  Thumbnail must also be included.  Pick List will remain open/extendable.
    ///
    /// [ImageSizeDescription](https://ddwiki.reso.org/display/DDW17/ImageSizeDescription+Field)
    #[serde(
        rename = "ImageSizeDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_size_description: Option<String>,

    /// The width of the image expressed in pixels.
    ///
    /// [ImageWidth](https://ddwiki.reso.org/display/DDW17/ImageWidth+Field)
    #[serde(rename = "ImageWidth", skip_serializing_if = "Option::is_none")]
    pub image_width: Option<f64>,

    /// The full robust description of the object.
    ///
    /// [LongDescription](https://ddwiki.reso.org/display/DDW17/LongDescription+Field)
    #[serde(rename = "LongDescription", skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,

    /// Category describing the , Photos, Documents, Video, Unbranded Virtual Tour, Branded Virtual Tour, Floor Plan, Logo
    ///
    /// [MediaCategory](https://ddwiki.reso.org/display/DDW17/MediaCategory+Field)
    #[serde(rename = "MediaCategory", skip_serializing_if = "Option::is_none")]
    pub media_category: Option<String>,

    /// The JavaScript or other method to embed a video, image, virtual tour or other media.
    ///
    /// [MediaHTML](https://ddwiki.reso.org/display/DDW17/MediaHTML+Field)
    #[serde(rename = "MediaHTML", skip_serializing_if = "Option::is_none")]
    pub media_html: Option<String>,

    /// A unique identifier for this record from the immediate source. This may be a number, or string that can include URI or other forms.  This is the system you are connecting to and not necessarily the original source of the record.
    ///
    /// [MediaKey](https://ddwiki.reso.org/display/DDW17/MediaKey+Field)
    #[serde(rename = "MediaKey", skip_serializing_if = "Option::is_none")]
    pub media_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This may be a number, or string that can include URI or other forms.  This is the system you are connecting to and not necessarily the original source of the record.  This is the numeric only key and used as an alternative to the MediaKey field.
    ///
    /// [MediaKeyNumeric](https://ddwiki.reso.org/display/DDW17/MediaKeyNumeric+Field)
    #[serde(rename = "MediaKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub media_key_numeric: Option<f64>,

    /// This timestamp is updated when a change to the object has been made, which may differ from a change to the Media Resource.
    ///
    /// [MediaModificationTimestamp](https://ddwiki.reso.org/display/DDW17/MediaModificationTimestamp+Field)
    #[serde(
        rename = "MediaModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub media_modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// ID of the image, supplement or other object specified by the given media record.
    ///
    /// [MediaObjectID](https://ddwiki.reso.org/display/DDW17/MediaObjectID+Field)
    #[serde(rename = "MediaObjectID", skip_serializing_if = "Option::is_none")]
    pub media_object_id: Option<String>,

    /// The status of the media item referenced by this record.  (Updated, Deleted, etc.,_
    ///
    /// [MediaStatus](https://ddwiki.reso.org/display/DDW17/MediaStatus+Field)
    #[serde(rename = "MediaStatus", skip_serializing_if = "Option::is_none")]
    pub media_status: Option<String>,

    /// Media Types as defined by IANA. http://www.iana.org/assignments/media-types/index.html. Note that the former name of MimeType, used by both IANA and RESO may still be in use by some systems/entities.
    ///
    /// [MediaType](https://ddwiki.reso.org/display/DDW17/MediaType+Field)
    #[serde(rename = "MediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,

    /// The URI to the media file referenced by this record.
    ///
    /// [MediaURL](https://ddwiki.reso.org/display/DDW17/MediaURL+Field)
    #[serde(rename = "MediaURL", skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,

    /// The transactional timestamp automatically recorded by the MLS system representing the date/time the media record was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28Media%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Only a positive integer including zero.  Element zero is the primary photo per RETS convention.
    ///
    /// [Order](https://ddwiki.reso.org/display/DDW17/Order+Field)
    #[serde(rename = "Order", skip_serializing_if = "Option::is_none")]
    pub order: Option<f64>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the Media was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+%28Media%29+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Originating system.  The Originating system is the system with authoritative control over the record.  For example, the Multiple Listing Service where the Media was input.  There may be cases where the Source System (how you received the record) is not the Originating System.  See Source System Key for more information.
    ///
    /// [OriginatingSystemMediaKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemMediaKey+Field)
    #[serde(
        rename = "OriginatingSystemMediaKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_media_key: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the Media is originally input by the member.  The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+%28Media%29+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// Public, Private, IDX, VOW, Office Only, Firm Only, Agent Only,
    ///
    /// [Permission](https://ddwiki.reso.org/display/DDW17/Permission+Field)
    #[serde(rename = "Permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,

    /// When set to true, the media record in question is the preferred photo.  This will typically mean the photo to be shown when only one of the photos is to be displayed.
    ///
    /// [PreferredPhotoYN](https://ddwiki.reso.org/display/DDW17/PreferredPhotoYN+Field)
    #[serde(rename = "PreferredPhotoYN", skip_serializing_if = "Option::is_none")]
    pub preferred_photo_yn: Option<bool>,

    /// The resource or table of the listing or other record the media relates to.  i.e. Property, Member, Office, etc.
    ///
    /// [ResourceName](https://ddwiki.reso.org/display/DDW17/ResourceName+Field)
    #[serde(rename = "ResourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,

    /// The well known identifier of the related record from the source resource. The value may be identical to that of the Listing Key, but the Listing ID is intended to be the value used by a human to retrieve the information about a specific listing. In a multiple originating system or a merged system, this value may not be unique and may require the use of the provider system to create a synthetic unique value.
    ///
    /// [ResourceRecordID](https://ddwiki.reso.org/display/DDW17/ResourceRecordID+Field)
    #[serde(rename = "ResourceRecordID", skip_serializing_if = "Option::is_none")]
    pub resource_record_id: Option<String>,

    /// The primary key of the related record from the source resource. For example the ListingKey, AgentKey, OfficeKey, TeamKey, etc. This is the system you are connecting to and not necessarily the original source of the record. This is a foreign key from the resource selected in the ResourceName field.
    ///
    /// [ResourceRecordKey](https://ddwiki.reso.org/display/DDW17/ResourceRecordKey+Field)
    #[serde(rename = "ResourceRecordKey", skip_serializing_if = "Option::is_none")]
    pub resource_record_key: Option<String>,

    /// The primary key of the related record from the source resource. For example the ListingKey, AgentKey, OfficeKey, TeamKey, etc. This is the system you are connecting to and not necessarily the original source of the record. This is a foreign key from the resource selected in the ResourceName field. This is the numeric only key and used as an alternative to the ResourceRecordKey field.
    ///
    /// [ResourceRecordKeyNumeric](https://ddwiki.reso.org/display/DDW17/ResourceRecordKeyNumeric+Field)
    #[serde(
        rename = "ResourceRecordKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_record_key_numeric: Option<f64>,

    /// The short text given to summarize the object.  Commonly used as the short description displayed under a photo.
    ///
    /// [ShortDescription](https://ddwiki.reso.org/display/DDW17/ShortDescription+Field)
    #[serde(rename = "ShortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider.  The source system is the system from which the record was directly received.  In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+%28Media%29+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemMediaKey](https://ddwiki.reso.org/display/DDW17/SourceSystemMediaKey+Field)
    #[serde(
        rename = "SourceSystemMediaKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_media_key: Option<String>,

    /// The name of the immediate record provider.  The system from which the record was directly received.  The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+%28Media%29+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,
}
