// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};

/// [Teams Resource](https://ddwiki.reso.org/display/DDW17/Teams+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Teams {
    /// Date/time the roster (Team or office) record was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28Teams%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Date/time the roster (Team or office) record was originally input into the source system.
    ///
    /// [OriginalEntryTimestamp](https://ddwiki.reso.org/display/DDW17/OriginalEntryTimestamp+%28Teams%29+Field)
    #[serde(
        rename = "OriginalEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_entry_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the Team was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+%28Teams%29+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Originating system.  The Originating system is the system with authoritative control over the record.  For example, the Multiple Listing Service where the Team was input.  There may be cases where the Source System (how you received the record) is not the Originating System.  See Source System Key for more information.
    ///
    /// [OriginatingSystemKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemKey+%28Teams%29+Field)
    #[serde(
        rename = "OriginatingSystemKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_key: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the Team is originally input.  The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+%28Teams%29+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// A list of types of sites, blog, social media, the Team URL or ID is referring to. i.e. Website, Blog, Facebook, Twitter, LinkedIn, Skype, etc., This list is used to populate the Type with repeating Social Media URL or ID types.
    ///
    /// [SocialMediaType](https://ddwiki.reso.org/display/DDW17/SocialMediaType+%28Teams%29+Field)
    #[serde(rename = "SocialMediaType", skip_serializing_if = "Option::is_none")]
    pub social_media_type: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider.  The source system is the system from which the record was directly received.  In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+%28Teams%29+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Source System.  The Source System is the system from which the record was directly received.  In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemKey](https://ddwiki.reso.org/display/DDW17/SourceSystemKey+%28Teams%29+Field)
    #[serde(rename = "SourceSystemKey", skip_serializing_if = "Option::is_none")]
    pub source_system_key: Option<String>,

    /// The name of the Team record provider.  The system from which the record was directly received.  The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+%28Teams%29+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,

    /// The street number, direction, name and suffix of the Team.
    ///
    /// [TeamAddress1](https://ddwiki.reso.org/display/DDW17/TeamAddress1+Field)
    #[serde(rename = "TeamAddress1", skip_serializing_if = "Option::is_none")]
    pub team_address1: Option<String>,

    /// The unit/suite number of the Team.
    ///
    /// [TeamAddress2](https://ddwiki.reso.org/display/DDW17/TeamAddress2+Field)
    #[serde(rename = "TeamAddress2", skip_serializing_if = "Option::is_none")]
    pub team_address2: Option<String>,

    /// The group of addresses to which the USPS assigns the same code to aid in mail delivery. For the USPS, these codes are 9 digits: 5 numbers for the ZIP Code, one letter for the carrier route type, and 3 numbers for the carrier route number.
    ///
    /// [TeamCarrierRoute](https://ddwiki.reso.org/display/DDW17/TeamCarrierRoute+Field)
    #[serde(rename = "TeamCarrierRoute", skip_serializing_if = "Option::is_none")]
    pub team_carrier_route: Option<String>,

    /// The city of the Team.
    ///
    /// [TeamCity](https://ddwiki.reso.org/display/DDW17/TeamCity+Field)
    #[serde(rename = "TeamCity", skip_serializing_if = "Option::is_none")]
    pub team_city: Option<String>,

    /// The country abbreviation in a postal address.
    ///
    /// [TeamCountry](https://ddwiki.reso.org/display/DDW17/TeamCountry+Field)
    #[serde(rename = "TeamCountry", skip_serializing_if = "Option::is_none")]
    pub team_country: Option<String>,

    /// The county or parish in which the Team is addressed.
    ///
    /// [TeamCountyOrParish](https://ddwiki.reso.org/display/DDW17/TeamCountyOrParish+Field)
    #[serde(rename = "TeamCountyOrParish", skip_serializing_if = "Option::is_none")]
    pub team_county_or_parish: Option<String>,

    /// A description or marketing information about the team.
    ///
    /// [TeamDescription](https://ddwiki.reso.org/display/DDW17/TeamDescription+Field)
    #[serde(rename = "TeamDescription", skip_serializing_if = "Option::is_none")]
    pub team_description: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [TeamDirectPhone](https://ddwiki.reso.org/display/DDW17/TeamDirectPhone+Field)
    #[serde(rename = "TeamDirectPhone", skip_serializing_if = "Option::is_none")]
    pub team_direct_phone: Option<String>,

    /// The email address of the Team.
    ///
    /// [TeamEmail](https://ddwiki.reso.org/display/DDW17/TeamEmail+Field)
    #[serde(rename = "TeamEmail", skip_serializing_if = "Option::is_none")]
    pub team_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [TeamFax](https://ddwiki.reso.org/display/DDW17/TeamFax+Field)
    #[serde(rename = "TeamFax", skip_serializing_if = "Option::is_none")]
    pub team_fax: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the TeamKey is the system unique identifier from the system that the record was retrieved.
    ///
    /// [TeamKey](https://ddwiki.reso.org/display/DDW17/TeamKey+Field)
    #[serde(rename = "TeamKey", skip_serializing_if = "Option::is_none")]
    pub team_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the TeamKey is the system unique identifier from the system that the record was retrieved.  This is the numeric only key and used as an alternative to the TeamKey field.
    ///
    /// [TeamKeyNumeric](https://ddwiki.reso.org/display/DDW17/TeamKeyNumeric+Field)
    #[serde(rename = "TeamKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub team_key_numeric: Option<f64>,

    /// The unique system identifier of the team's lead member.
    ///
    /// [TeamLeadKey](https://ddwiki.reso.org/display/DDW17/TeamLeadKey+Field)
    #[serde(rename = "TeamLeadKey", skip_serializing_if = "Option::is_none")]
    pub team_lead_key: Option<String>,

    /// The unique system identifier of the team's lead member.  This is the numeric only key and used as an alternative to the TeamLeadKey field.
    ///
    /// [TeamLeadKeyNumeric](https://ddwiki.reso.org/display/DDW17/TeamLeadKeyNumeric+Field)
    #[serde(rename = "TeamLeadKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub team_lead_key_numeric: Option<f64>,

    /// The ID used to logon to the MLS system.
    ///
    /// [TeamLeadLoginId](https://ddwiki.reso.org/display/DDW17/TeamLeadLoginId+Field)
    #[serde(rename = "TeamLeadLoginId", skip_serializing_if = "Option::is_none")]
    pub team_lead_login_id: Option<String>,

    /// The local, well-known identifier for the Team Lead. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [TeamLeadMlsId](https://ddwiki.reso.org/display/DDW17/TeamLeadMlsId+Field)
    #[serde(rename = "TeamLeadMlsId", skip_serializing_if = "Option::is_none")]
    pub team_lead_mls_id: Option<String>,

    /// The national association ID of the team lead.  i.e. in the U.S. is the NRDS number.
    ///
    /// [TeamLeadNationalAssociationId](https://ddwiki.reso.org/display/DDW17/TeamLeadNationalAssociationId+Field)
    #[serde(
        rename = "TeamLeadNationalAssociationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_lead_national_association_id: Option<String>,

    /// The license of the Team Lead. Separate multiple licenses with a comma and space.
    ///
    /// [TeamLeadStateLicense](https://ddwiki.reso.org/display/DDW17/TeamLeadStateLicense+Field)
    #[serde(
        rename = "TeamLeadStateLicense",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_lead_state_license: Option<String>,

    /// The state in which the Team Lead is licensed.
    ///
    /// [TeamLeadStateLicenseState](https://ddwiki.reso.org/display/DDW17/TeamLeadStateLicenseState+Field)
    #[serde(
        rename = "TeamLeadStateLicenseState",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_lead_state_license_state: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [TeamMobilePhone](https://ddwiki.reso.org/display/DDW17/TeamMobilePhone+Field)
    #[serde(rename = "TeamMobilePhone", skip_serializing_if = "Option::is_none")]
    pub team_mobile_phone: Option<String>,

    /// The name under which the team operates.  If a business this may be a DBA.
    ///
    /// [TeamName](https://ddwiki.reso.org/display/DDW17/TeamName+Field)
    #[serde(rename = "TeamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [TeamOfficePhone](https://ddwiki.reso.org/display/DDW17/TeamOfficePhone+Field)
    #[serde(rename = "TeamOfficePhone", skip_serializing_if = "Option::is_none")]
    pub team_office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [TeamOfficePhoneExt](https://ddwiki.reso.org/display/DDW17/TeamOfficePhoneExt+Field)
    #[serde(rename = "TeamOfficePhoneExt", skip_serializing_if = "Option::is_none")]
    pub team_office_phone_ext: Option<String>,

    /// The postal code of the Team.
    ///
    /// [TeamPostalCode](https://ddwiki.reso.org/display/DDW17/TeamPostalCode+Field)
    #[serde(rename = "TeamPostalCode", skip_serializing_if = "Option::is_none")]
    pub team_postal_code: Option<String>,

    /// The extension of the postal/zip code.  i.e. +4
    ///
    /// [TeamPostalCodePlus4](https://ddwiki.reso.org/display/DDW17/TeamPostalCodePlus4+Field)
    #[serde(
        rename = "TeamPostalCodePlus4",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_postal_code_plus4: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [TeamPreferredPhone](https://ddwiki.reso.org/display/DDW17/TeamPreferredPhone+Field)
    #[serde(rename = "TeamPreferredPhone", skip_serializing_if = "Option::is_none")]
    pub team_preferred_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [TeamPreferredPhoneExt](https://ddwiki.reso.org/display/DDW17/TeamPreferredPhoneExt+Field)
    #[serde(
        rename = "TeamPreferredPhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_preferred_phone_ext: Option<String>,

    /// The state or province in which the Team is addressed.
    ///
    /// [TeamStateOrProvince](https://ddwiki.reso.org/display/DDW17/TeamStateOrProvince+Field)
    #[serde(
        rename = "TeamStateOrProvince",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_state_or_province: Option<String>,

    /// Is the account active, inactive or under disciplinary action.
    ///
    /// [TeamStatus](https://ddwiki.reso.org/display/DDW17/TeamStatus+Field)
    #[serde(rename = "TeamStatus", skip_serializing_if = "Option::is_none")]
    pub team_status: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [TeamTollFreePhone](https://ddwiki.reso.org/display/DDW17/TeamTollFreePhone+Field)
    #[serde(rename = "TeamTollFreePhone", skip_serializing_if = "Option::is_none")]
    pub team_toll_free_phone: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [TeamVoiceMail](https://ddwiki.reso.org/display/DDW17/TeamVoiceMail+Field)
    #[serde(rename = "TeamVoiceMail", skip_serializing_if = "Option::is_none")]
    pub team_voice_mail: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [TeamVoiceMailExt](https://ddwiki.reso.org/display/DDW17/TeamVoiceMailExt+Field)
    #[serde(rename = "TeamVoiceMailExt", skip_serializing_if = "Option::is_none")]
    pub team_voice_mail_ext: Option<String>,

    /// A collection of the types of social media fields  available for this team. The collection includes the type of system and other details pertinent about social media
    ///
    /// [TeamsSocialMedia](https://ddwiki.reso.org/display/DDW17/TeamsSocialMedia+Field)
    #[serde(rename = "TeamsSocialMedia", skip_serializing_if = "Option::is_none")]
    pub teams_social_media: Option<String>,
}
