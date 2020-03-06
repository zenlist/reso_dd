// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// [Office Resource](https://ddwiki.reso.org/display/DDW17/Office+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Office {
    /// The name of the franchise to which the broker/office is contracted.
    ///
    /// [FranchiseAffiliation](https://ddwiki.reso.org/display/DDW17/FranchiseAffiliation+Field)
    #[serde(
        rename = "FranchiseAffiliation",
        skip_serializing_if = "Option::is_none"
    )]
    pub franchise_affiliation: Option<String>,

    /// Does the Office/Broker participate in IDX.
    ///
    /// [IDXOfficeParticipationYN](https://ddwiki.reso.org/display/DDW17/IDXOfficeParticipationYN+Field)
    #[serde(
        rename = "IDXOfficeParticipationYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub idxoffice_participation_yn: Option<bool>,

    /// OfficeKey of the Main Office in a firm/company of offices.  This is a self referencing foreign key relating to this resource's OfficeKey.  This key may be the same value as the OfficeKey for a given record if the given office is the Main Office.
    ///
    /// [MainOfficeKey](https://ddwiki.reso.org/display/DDW17/MainOfficeKey+Field)
    #[serde(rename = "MainOfficeKey", skip_serializing_if = "Option::is_none")]
    pub main_office_key: Option<String>,

    /// OfficeKey of the Main Office in a firm/company of offices.  This is a self referencing foreign key relating to this resource's OfficeKey.  This key may be the same value as the OfficeKey for a given record if the given office is the Main Office.  This is the numeric only key and used as an alternative to the MainOfficeKey field.
    ///
    /// [MainOfficeKeyNumeric](https://ddwiki.reso.org/display/DDW17/MainOfficeKeyNumeric+Field)
    #[serde(
        rename = "MainOfficeKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub main_office_key_numeric: Option<f64>,

    /// OfficeMlsId of the Main Office in a firm/company of offices.
    ///
    /// [MainOfficeMlsId](https://ddwiki.reso.org/display/DDW17/MainOfficeMlsId+Field)
    #[serde(rename = "MainOfficeMlsId", skip_serializing_if = "Option::is_none")]
    pub main_office_mls_id: Option<String>,

    /// Date/time the roster (member or office) record was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28Office%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The Office's Board or Association of REALTORS.
    ///
    /// [OfficeAOR](https://ddwiki.reso.org/display/DDW17/OfficeAOR+Field)
    #[serde(rename = "OfficeAOR", skip_serializing_if = "Option::is_none")]
    pub office_aor: Option<String>,

    /// The local, well-known identifier for the office's Association of REALTORS. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [OfficeAORMlsId](https://ddwiki.reso.org/display/DDW17/OfficeAORMlsId+Field)
    #[serde(rename = "OfficeAORMlsId", skip_serializing_if = "Option::is_none")]
    pub office_aormls_id: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the OfficeAORkey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId.  This is a foreign key relating to the AOR's member management system in which the record was originated.
    ///
    /// [OfficeAORkey](https://ddwiki.reso.org/display/DDW17/OfficeAORkey+Field)
    #[serde(rename = "OfficeAORkey", skip_serializing_if = "Option::is_none")]
    pub office_aorkey: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the OfficeAORkey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId.  This is a foreign key relating to the AOR's member management system in which the record was originated.  This is the numeric only key and used as an alternative to the OfficeAORkey field.
    ///
    /// [OfficeAORkeyNumeric](https://ddwiki.reso.org/display/DDW17/OfficeAORkeyNumeric+Field)
    #[serde(
        rename = "OfficeAORkeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub office_aorkey_numeric: Option<f64>,

    /// The street number, direction, name and suffix of the office.
    ///
    /// [OfficeAddress1](https://ddwiki.reso.org/display/DDW17/OfficeAddress1+Field)
    #[serde(rename = "OfficeAddress1", skip_serializing_if = "Option::is_none")]
    pub office_address1: Option<String>,

    /// The unit/suite number of the office.
    ///
    /// [OfficeAddress2](https://ddwiki.reso.org/display/DDW17/OfficeAddress2+Field)
    #[serde(rename = "OfficeAddress2", skip_serializing_if = "Option::is_none")]
    pub office_address2: Option<String>,

    /// Notes relating to the office.
    ///
    /// [OfficeAssociationComments](https://ddwiki.reso.org/display/DDW17/OfficeAssociationComments+Field)
    #[serde(
        rename = "OfficeAssociationComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub office_association_comments: Option<String>,

    /// The level of the office in the hierarchy of Main, Branch, Stand Alone, etc.,
    ///
    /// [OfficeBranchType](https://ddwiki.reso.org/display/DDW17/OfficeBranchType+Field)
    #[serde(rename = "OfficeBranchType", skip_serializing_if = "Option::is_none")]
    pub office_branch_type: Option<crate::OfficeBranchType>,

    /// The MemberKey of the responsible/owning broker.  This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [OfficeBrokerKey](https://ddwiki.reso.org/display/DDW17/OfficeBrokerKey+Field)
    #[serde(rename = "OfficeBrokerKey", skip_serializing_if = "Option::is_none")]
    pub office_broker_key: Option<String>,

    /// The MemberKey of the responsible/owning broker.  This is a foreign key relating to the Member resource's MemberKey.  This is the numeric only key and used as an alternative to the OfficeBrokerKey field.
    ///
    /// [OfficeBrokerKeyNumeric](https://ddwiki.reso.org/display/DDW17/OfficeBrokerKeyNumeric+Field)
    #[serde(
        rename = "OfficeBrokerKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub office_broker_key_numeric: Option<f64>,

    /// The MemberMlsId of the responsible/owning broker.
    ///
    /// [OfficeBrokerMlsId](https://ddwiki.reso.org/display/DDW17/OfficeBrokerMlsId+Field)
    #[serde(rename = "OfficeBrokerMlsId", skip_serializing_if = "Option::is_none")]
    pub office_broker_mls_id: Option<String>,

    /// The city of the office.
    ///
    /// [OfficeCity](https://ddwiki.reso.org/display/DDW17/OfficeCity+Field)
    #[serde(rename = "OfficeCity", skip_serializing_if = "Option::is_none")]
    pub office_city: Option<String>,

    /// When an office/firm is a corporation, an independent license number is issued.
    ///
    /// [OfficeCorporateLicense](https://ddwiki.reso.org/display/DDW17/OfficeCorporateLicense+Field)
    #[serde(
        rename = "OfficeCorporateLicense",
        skip_serializing_if = "Option::is_none"
    )]
    pub office_corporate_license: Option<String>,

    /// The county or parish in which the offices is located.
    ///
    /// [OfficeCountyOrParish](https://ddwiki.reso.org/display/DDW17/OfficeCountyOrParish+Field)
    #[serde(
        rename = "OfficeCountyOrParish",
        skip_serializing_if = "Option::is_none"
    )]
    pub office_county_or_parish: Option<String>,

    /// The email address of the office
    ///
    /// [OfficeEmail](https://ddwiki.reso.org/display/DDW17/OfficeEmail+Field)
    #[serde(rename = "OfficeEmail", skip_serializing_if = "Option::is_none")]
    pub office_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [OfficeFax](https://ddwiki.reso.org/display/DDW17/OfficeFax+Field)
    #[serde(rename = "OfficeFax", skip_serializing_if = "Option::is_none")]
    pub office_fax: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.
    ///
    /// [OfficeKey](https://ddwiki.reso.org/display/DDW17/OfficeKey+%28Office%29+Field)
    #[serde(rename = "OfficeKey", skip_serializing_if = "Option::is_none")]
    pub office_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the OfficeKey fields.  This is the local key of the system.  When records are received from other systems, a local key is commonly applied.  If conveying the original keys from the source or originating systems, see SourceSystemOfficeKey and OriginatingSystemOfficeKey.
    ///
    /// [OfficeKeyNumeric](https://ddwiki.reso.org/display/DDW17/OfficeKeyNumeric+%28Office%29+Field)
    #[serde(rename = "OfficeKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub office_key_numeric: Option<f64>,

    /// The lead Office Manager for the given office.  This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [OfficeManagerKey](https://ddwiki.reso.org/display/DDW17/OfficeManagerKey+Field)
    #[serde(rename = "OfficeManagerKey", skip_serializing_if = "Option::is_none")]
    pub office_manager_key: Option<String>,

    /// The lead Office Manager for the given office.  This is a foreign key relating to the Member resource's MemberKey.  This is the numeric only key and used as an alternative to the OfficeManagerKey field.
    ///
    /// [OfficeManagerKeyNumeric](https://ddwiki.reso.org/display/DDW17/OfficeManagerKeyNumeric+Field)
    #[serde(
        rename = "OfficeManagerKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub office_manager_key_numeric: Option<f64>,

    /// The lead Office Manager for the given office.
    ///
    /// [OfficeManagerMlsId](https://ddwiki.reso.org/display/DDW17/OfficeManagerMlsId+Field)
    #[serde(rename = "OfficeManagerMlsId", skip_serializing_if = "Option::is_none")]
    pub office_manager_mls_id: Option<String>,

    /// The local, well-known identifier. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [OfficeMlsId](https://ddwiki.reso.org/display/DDW17/OfficeMlsId+%28Office%29+Field)
    #[serde(rename = "OfficeMlsId", skip_serializing_if = "Option::is_none")]
    pub office_mls_id: Option<String>,

    /// The legal name of the brokerage.
    ///
    /// [OfficeName](https://ddwiki.reso.org/display/DDW17/OfficeName+%28Office%29+Field)
    #[serde(rename = "OfficeName", skip_serializing_if = "Option::is_none")]
    pub office_name: Option<String>,

    /// The national association ID of the office.  i.e. in the U.S. is the NRDS number.
    ///
    /// [OfficeNationalAssociationId](https://ddwiki.reso.org/display/DDW17/OfficeNationalAssociationId+Field)
    #[serde(
        rename = "OfficeNationalAssociationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub office_national_association_id: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [OfficePhone](https://ddwiki.reso.org/display/DDW17/OfficePhone+Field)
    #[serde(rename = "OfficePhone", skip_serializing_if = "Option::is_none")]
    pub office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [OfficePhoneExt](https://ddwiki.reso.org/display/DDW17/OfficePhoneExt+Field)
    #[serde(rename = "OfficePhoneExt", skip_serializing_if = "Option::is_none")]
    pub office_phone_ext: Option<String>,

    /// The postal code of the office.
    ///
    /// [OfficePostalCode](https://ddwiki.reso.org/display/DDW17/OfficePostalCode+Field)
    #[serde(rename = "OfficePostalCode", skip_serializing_if = "Option::is_none")]
    pub office_postal_code: Option<String>,

    /// The extension of the postal/zip code.  i.e. +4
    ///
    /// [OfficePostalCodePlus4](https://ddwiki.reso.org/display/DDW17/OfficePostalCodePlus4+Field)
    #[serde(
        rename = "OfficePostalCodePlus4",
        skip_serializing_if = "Option::is_none"
    )]
    pub office_postal_code_plus4: Option<String>,

    /// A collection of the types of social media fields  available for this office. The collection includes the type of system and other details pertinent about social media
    ///
    /// [OfficeSocialMedia](https://ddwiki.reso.org/display/DDW17/OfficeSocialMedia+Field)
    #[serde(rename = "OfficeSocialMedia", skip_serializing_if = "Option::is_none")]
    pub office_social_media: Option<String>,

    /// The state or province in which the office is located.
    ///
    /// [OfficeStateOrProvince](https://ddwiki.reso.org/display/DDW17/OfficeStateOrProvince+Field)
    #[serde(
        rename = "OfficeStateOrProvince",
        skip_serializing_if = "Option::is_none"
    )]
    pub office_state_or_province: Option<crate::StateOrProvince>,

    /// Is the office active, inactive or under disciplinary action.
    ///
    /// [OfficeStatus](https://ddwiki.reso.org/display/DDW17/OfficeStatus+Field)
    #[serde(rename = "OfficeStatus", skip_serializing_if = "Option::is_none")]
    pub office_status: Option<crate::OfficeStatus>,

    /// The type of business conducted by the office.  i.e. Real Estate, Appraiser, etc.
    ///
    /// [OfficeType](https://ddwiki.reso.org/display/DDW17/OfficeType+Field)
    #[serde(rename = "OfficeType", skip_serializing_if = "Option::is_none")]
    pub office_type: Option<crate::OfficeType>,

    /// Date/time the roster (member or office) record was originally input into the source system.
    ///
    /// [OriginalEntryTimestamp](https://ddwiki.reso.org/display/DDW17/OriginalEntryTimestamp+%28Office%29+Field)
    #[serde(
        rename = "OriginalEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_entry_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the office was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+%28Office%29+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the office is originally input by the member.  The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+%28Office%29+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// The system key, a unique record identifier, from the Originating system.  The Originating system is the system with authoritative control over the record.  For example, the Multiple Listing Service where the office was input.  There may be cases where the Source System (how you received the record) is not the Originating System.  See Source System Key for more information.
    ///
    /// [OriginatingSystemOfficeKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemOfficeKey+Field)
    #[serde(
        rename = "OriginatingSystemOfficeKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_office_key: Option<String>,

    /// A list of types of sites, blog, social media, the Office URL or ID is referring to.  i.e. Website, Blog, Facebook, Twitter, LinkedIn, Skype, etc.,  This list is used to populate the Type with repeating Social Media URL or ID types.
    ///
    /// [SocialMediaType](https://ddwiki.reso.org/display/DDW17/SocialMediaType+%28Office%29+Field)
    #[serde(rename = "SocialMediaType", skip_serializing_if = "Option::is_none")]
    pub social_media_type: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider.  The source system is the system from which the record was directly received.  In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+%28Office%29+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The name of the immediate record provider.  The system from which the record was directly received.  The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+%28Office%29+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,

    /// The system key, a unique record identifier, from the Source System.  The Source System is the system from which the record was directly received.  In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemOfficeKey](https://ddwiki.reso.org/display/DDW17/SourceSystemOfficeKey+Field)
    #[serde(
        rename = "SourceSystemOfficeKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_office_key: Option<String>,

    /// A list of options allowing the broker to pass the decision of syndication choice down to the listing agents. i.e. No Agent Choice, Allow Agent Choice, Restrict Agent Choice, etc.
    ///
    /// [SyndicateAgentOption](https://ddwiki.reso.org/display/DDW17/SyndicateAgentOption+Field)
    #[serde(
        rename = "SyndicateAgentOption",
        skip_serializing_if = "Option::is_none"
    )]
    pub syndicate_agent_option: Option<String>,

    /// The principal broker's choice on where they would like their listings syndicated. i.e. Zillow, Trulia, Homes.com, etc.
    ///
    /// [SyndicateTo](https://ddwiki.reso.org/display/DDW17/SyndicateTo+%28Office%29+Field)
    #[serde(rename = "SyndicateTo", skip_serializing_if = "Option::is_none")]
    pub syndicate_to: Option<String>,
}
