// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};

/// [OUID Resource](https://ddwiki.reso.org/display/DDW17/OUID+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OUID {
    /// The local, well-know identifier of the member (user) who made the change.
    ///
    /// [ChangedByMemberID](https://ddwiki.reso.org/display/DDW17/ChangedByMemberID+%28OUID%29+Field)
    #[serde(rename = "ChangedByMemberID", skip_serializing_if = "Option::is_none")]
    pub changed_by_member_id: Option<String>,

    /// The unique identifier of the member (user) who made the change. This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [ChangedByMemberKey](https://ddwiki.reso.org/display/DDW17/ChangedByMemberKey+%28OUID%29+Field)
    #[serde(rename = "ChangedByMemberKey", skip_serializing_if = "Option::is_none")]
    pub changed_by_member_key: Option<String>,

    /// The unique identifier of the member (user) who made the change. This is a foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the ChangedByMemberKey field.
    ///
    /// [ChangedByMemberKeyNumeric](https://ddwiki.reso.org/display/DDW17/ChangedByMemberKeyNumeric+%28OUID%29+Field)
    #[serde(
        rename = "ChangedByMemberKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub changed_by_member_key_numeric: Option<f64>,

    /// Date/time the Organization record was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28OUID%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The Organization's Primary Board or Association of REALTORS if applicable.
    ///
    /// [OrganizationAOR](https://ddwiki.reso.org/display/DDW17/OrganizationAOR+Field)
    #[serde(rename = "OrganizationAOR", skip_serializing_if = "Option::is_none")]
    pub organization_aor: Option<String>,

    /// The street number, direction, name and suffix of the organization.
    ///
    /// [OrganizationAddress1](https://ddwiki.reso.org/display/DDW17/OrganizationAddress1+Field)
    #[serde(
        rename = "OrganizationAddress1",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_address1: Option<String>,

    /// The unit/suite number of the organization.
    ///
    /// [OrganizationAddress2](https://ddwiki.reso.org/display/DDW17/OrganizationAddress2+Field)
    #[serde(
        rename = "OrganizationAddress2",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_address2: Option<String>,

    /// The OUID for the Organization's Association of REALTORS if applicable.
    ///
    /// [OrganizationAorOuid](https://ddwiki.reso.org/display/DDW17/OrganizationAorOuid+Field)
    #[serde(
        rename = "OrganizationAorOuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_aor_ouid: Option<String>,

    /// The OrganizationUniqueIdKey of the AOR from the system serving the OUID resource.
    ///
    /// [OrganizationAorOuidKey](https://ddwiki.reso.org/display/DDW17/OrganizationAorOuidKey+Field)
    #[serde(
        rename = "OrganizationAorOuidKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_aor_ouid_key: Option<String>,

    /// The OrganizationUniqueIdKey of the AOR from the system serving the OUID resource.  This is the numeric only key and used as an alternative to the OrganizationAorOuidKey field.
    ///
    /// [OrganizationAorOuidKeyNumeric](https://ddwiki.reso.org/display/DDW17/OrganizationAorOuidKeyNumeric+Field)
    #[serde(
        rename = "OrganizationAorOuidKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_aor_ouid_key_numeric: Option<f64>,

    /// The group of addresses to which the USPS assigns the same code to aid in mail delivery. For the USPS, these codes are 9 digits: 5 numbers for the ZIP Code, one letter for the carrier route type, and 3 numbers for the carrier route number.
    ///
    /// [OrganizationCarrierRoute](https://ddwiki.reso.org/display/DDW17/OrganizationCarrierRoute+Field)
    #[serde(
        rename = "OrganizationCarrierRoute",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_carrier_route: Option<String>,

    /// The city of the organization.
    ///
    /// [OrganizationCity](https://ddwiki.reso.org/display/DDW17/OrganizationCity+Field)
    #[serde(rename = "OrganizationCity", skip_serializing_if = "Option::is_none")]
    pub organization_city: Option<String>,

    /// Comments or notes about the Organization.
    ///
    /// [OrganizationComments](https://ddwiki.reso.org/display/DDW17/OrganizationComments+Field)
    #[serde(
        rename = "OrganizationComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_comments: Option<String>,

    /// The email address of the Organization Contact.
    ///
    /// [OrganizationContactEmail](https://ddwiki.reso.org/display/DDW17/OrganizationContactEmail+Field)
    #[serde(
        rename = "OrganizationContactEmail",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_contact_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [OrganizationContactFax](https://ddwiki.reso.org/display/DDW17/OrganizationContactFax+Field)
    #[serde(
        rename = "OrganizationContactFax",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_contact_fax: Option<String>,

    /// The first name of the Organization Contact.
    ///
    /// [OrganizationContactFirstName](https://ddwiki.reso.org/display/DDW17/OrganizationContactFirstName+Field)
    #[serde(
        rename = "OrganizationContactFirstName",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_contact_first_name: Option<String>,

    /// The full name of the Organization Contact. (First Middle Last) or a alternate full name.
    ///
    /// [OrganizationContactFullName](https://ddwiki.reso.org/display/DDW17/OrganizationContactFullName+Field)
    #[serde(
        rename = "OrganizationContactFullName",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_contact_full_name: Option<String>,

    /// The title or position of the Organization Contact.
    ///
    /// [OrganizationContactJobTitle](https://ddwiki.reso.org/display/DDW17/OrganizationContactJobTitle+Field)
    #[serde(
        rename = "OrganizationContactJobTitle",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_contact_job_title: Option<String>,

    /// The last name of the Organization Contact.
    ///
    /// [OrganizationContactLastName](https://ddwiki.reso.org/display/DDW17/OrganizationContactLastName+Field)
    #[serde(
        rename = "OrganizationContactLastName",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_contact_last_name: Option<String>,

    /// The middle name of the Organization Contact.
    ///
    /// [OrganizationContactMiddleName](https://ddwiki.reso.org/display/DDW17/OrganizationContactMiddleName+Field)
    #[serde(
        rename = "OrganizationContactMiddleName",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_contact_middle_name: Option<String>,

    /// Prefix to the name of the Organization Contact. (e.g. Dr. Mr. Ms. etc.)
    ///
    /// [OrganizationContactNamePrefix](https://ddwiki.reso.org/display/DDW17/OrganizationContactNamePrefix+Field)
    #[serde(
        rename = "OrganizationContactNamePrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_contact_name_prefix: Option<String>,

    /// Suffix to the surname (e.g. Esq.,  Jr.,  III etc.) of the Organization Contact.
    ///
    /// [OrganizationContactNameSuffix](https://ddwiki.reso.org/display/DDW17/OrganizationContactNameSuffix+Field)
    #[serde(
        rename = "OrganizationContactNameSuffix",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_contact_name_suffix: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [OrganizationContactPhone](https://ddwiki.reso.org/display/DDW17/OrganizationContactPhone+Field)
    #[serde(
        rename = "OrganizationContactPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_contact_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [OrganizationContactPhoneExt](https://ddwiki.reso.org/display/DDW17/OrganizationContactPhoneExt+Field)
    #[serde(
        rename = "OrganizationContactPhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_contact_phone_ext: Option<String>,

    /// The country abbreviation in a postal address.
    ///
    /// [OrganizationCountry](https://ddwiki.reso.org/display/DDW17/OrganizationCountry+Field)
    #[serde(
        rename = "OrganizationCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_country: Option<String>,

    /// The county or parish in which the organization is addressed.
    ///
    /// [OrganizationCountyOrParish](https://ddwiki.reso.org/display/DDW17/OrganizationCountyOrParish+Field)
    #[serde(
        rename = "OrganizationCountyOrParish",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_county_or_parish: Option<String>,

    /// The total number of active members in the Organization if applicable.
    ///
    /// [OrganizationMemberCount](https://ddwiki.reso.org/display/DDW17/OrganizationMemberCount+Field)
    #[serde(
        rename = "OrganizationMemberCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_member_count: Option<f64>,

    /// If the organization is an MLS it is likely they already have an ID or code based on their name or an abbreviation.  This field supports the continued use/reference to that legacy code.
    ///
    /// [OrganizationMlsCode](https://ddwiki.reso.org/display/DDW17/OrganizationMlsCode+Field)
    #[serde(
        rename = "OrganizationMlsCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_mls_code: Option<String>,

    /// If the organization uses an MLS system, this is the textual name of the vendor.
    ///
    /// [OrganizationMlsVendorName](https://ddwiki.reso.org/display/DDW17/OrganizationMlsVendorName+Field)
    #[serde(
        rename = "OrganizationMlsVendorName",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_mls_vendor_name: Option<String>,

    /// If the organization uses an MLS system, this is that vendor's OUID.
    ///
    /// [OrganizationMlsVendorOuid](https://ddwiki.reso.org/display/DDW17/OrganizationMlsVendorOuid+Field)
    #[serde(
        rename = "OrganizationMlsVendorOuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_mls_vendor_ouid: Option<String>,

    /// The textual name of the organization represented by a given OUID record.
    ///
    /// [OrganizationName](https://ddwiki.reso.org/display/DDW17/OrganizationName+Field)
    #[serde(rename = "OrganizationName", skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,

    /// The national association ID of the Organization if applicable.  i.e. in the U.S. is the NRDS number.
    ///
    /// [OrganizationNationalAssociationId](https://ddwiki.reso.org/display/DDW17/OrganizationNationalAssociationId+Field)
    #[serde(
        rename = "OrganizationNationalAssociationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_national_association_id: Option<String>,

    /// The postal code of the organization.
    ///
    /// [OrganizationPostalCode](https://ddwiki.reso.org/display/DDW17/OrganizationPostalCode+Field)
    #[serde(
        rename = "OrganizationPostalCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_postal_code: Option<String>,

    /// The extension of the postal/zip code.  i.e. +4
    ///
    /// [OrganizationPostalCodePlus4](https://ddwiki.reso.org/display/DDW17/OrganizationPostalCodePlus4+Field)
    #[serde(
        rename = "OrganizationPostalCodePlus4",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_postal_code_plus4: Option<String>,

    /// A collection of the types of social media fields available for this organization. The collection includes the type of system and other details pertinent about social media
    ///
    /// [OrganizationSocialMedia](https://ddwiki.reso.org/display/DDW17/OrganizationSocialMedia+Field)
    #[serde(
        rename = "OrganizationSocialMedia",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_social_media: Option<String>,

    /// A list of types of sites, blog, social media, the Organization URL or ID is referring to.  i.e. Website, Blog, Facebook, Twitter, LinkedIn, Skype, etc.,  This list is used to populate the Type with repeating Social Media URL or ID types.
    ///
    /// [OrganizationSocialMediaType](https://ddwiki.reso.org/display/DDW17/OrganizationSocialMediaType+Field)
    #[serde(
        rename = "OrganizationSocialMediaType",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_social_media_type: Option<String>,

    /// The license of the Organization if applicable. Separate multiple licenses with a comma and space.
    ///
    /// [OrganizationStateLicense](https://ddwiki.reso.org/display/DDW17/OrganizationStateLicense+Field)
    #[serde(
        rename = "OrganizationStateLicense",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_state_license: Option<String>,

    /// The state in which the Organization is licensed if applicable.
    ///
    /// [OrganizationStateLicenseState](https://ddwiki.reso.org/display/DDW17/OrganizationStateLicenseState+Field)
    #[serde(
        rename = "OrganizationStateLicenseState",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_state_license_state: Option<String>,

    /// The state or province in which the organization is addressed.
    ///
    /// [OrganizationStateOrProvince](https://ddwiki.reso.org/display/DDW17/OrganizationStateOrProvince+Field)
    #[serde(
        rename = "OrganizationStateOrProvince",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_state_or_province: Option<String>,

    /// Is the Organization active or inactive. 1 or true is active, 0 or false is inactive. This field is not nullable.
    ///
    /// [OrganizationStatus](https://ddwiki.reso.org/display/DDW17/OrganizationStatus+Field)
    #[serde(rename = "OrganizationStatus", skip_serializing_if = "Option::is_none")]
    pub organization_status: Option<bool>,

    /// The date/time of when the Organization Status was last changed.
    ///
    /// [OrganizationStatusChangeTimestamp](https://ddwiki.reso.org/display/DDW17/OrganizationStatusChangeTimestamp+Field)
    #[serde(
        rename = "OrganizationStatusChangeTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_status_change_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The type of organization.  i.e. MLS, Vendor, Association, etc.  This is not a substitute or alternative for the Office resource, however it may be that a brokerage has a record in this table for a non-listing purpose.
    ///
    /// [OrganizationType](https://ddwiki.reso.org/display/DDW17/OrganizationType+Field)
    #[serde(rename = "OrganizationType", skip_serializing_if = "Option::is_none")]
    pub organization_type: Option<String>,

    /// The OUID.  This is the unique ID assigned to organizations included in the OUID resource.  Assignment of OUIDs will be centralized and may not be created by systems hosting the OUID resource.  Contact info@RESO.org for information on obtaining an OUID.
    ///
    /// [OrganizationUniqueId](https://ddwiki.reso.org/display/DDW17/OrganizationUniqueId+Field)
    #[serde(
        rename = "OrganizationUniqueId",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_unique_id: Option<String>,

    /// The key field used by the system hosting a table of OUIDs.  This key is likely to be unique to each hosting system and is not meant to be a universal ID for an organization, but rather a key used by clients of the hosting system.  The actual OUID is the Organization Unique ID field.
    ///
    /// [OrganizationUniqueIdKey](https://ddwiki.reso.org/display/DDW17/OrganizationUniqueIdKey+Field)
    #[serde(
        rename = "OrganizationUniqueIdKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_unique_id_key: Option<String>,

    /// The key field used by the system hosting a table of OUIDs.  This key is likely to be unique to each hosting system and is not meant to be a universal ID for an organization, but rather a key used by clients of the hosting system.  The actual OUID is the Organization Unique ID field.  This is the numeric only key and used as an alternative to the OrganizationUniqueIdKey field.
    ///
    /// [OrganizationUniqueIdKeyNumeric](https://ddwiki.reso.org/display/DDW17/OrganizationUniqueIdKeyNumeric+Field)
    #[serde(
        rename = "OrganizationUniqueIdKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_unique_id_key_numeric: Option<f64>,

    /// Date/time the Organization record was originally input into the source system.
    ///
    /// [OriginalEntryTimestamp](https://ddwiki.reso.org/display/DDW17/OriginalEntryTimestamp+%28OUID%29+Field)
    #[serde(
        rename = "OriginalEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_entry_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,
}
