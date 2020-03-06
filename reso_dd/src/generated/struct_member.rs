// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// [Member Resource](https://ddwiki.reso.org/display/DDW17/Member+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Member {
    /// The title or position of the member within their organization.
    ///
    /// [JobTitle](https://ddwiki.reso.org/display/DDW17/JobTitle+Field)
    #[serde(rename = "JobTitle", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,

    /// Date/time the member last logged into the source or other system.
    ///
    /// [LastLoginTimestamp](https://ddwiki.reso.org/display/DDW17/LastLoginTimestamp+Field)
    #[serde(rename = "LastLoginTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_login_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The Member's Primary Board or Association of REALTORS.
    ///
    /// [MemberAOR](https://ddwiki.reso.org/display/DDW17/MemberAOR+Field)
    #[serde(rename = "MemberAOR", skip_serializing_if = "Option::is_none")]
    pub member_aor: Option<String>,

    /// The local, well-known identifier for the member's Association of REALTORS. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [MemberAORMlsId](https://ddwiki.reso.org/display/DDW17/MemberAORMlsId+Field)
    #[serde(rename = "MemberAORMlsId", skip_serializing_if = "Option::is_none")]
    pub member_aormls_id: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the MemberAORkey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId.
    ///
    /// [MemberAORkey](https://ddwiki.reso.org/display/DDW17/MemberAORkey+Field)
    #[serde(rename = "MemberAORkey", skip_serializing_if = "Option::is_none")]
    pub member_aorkey: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the MemberAORkey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId. This is the numeric only key and used as an alternative to the MemberAORkey field.
    ///
    /// [MemberAORkeyNumeric](https://ddwiki.reso.org/display/DDW17/MemberAORkeyNumeric+Field)
    #[serde(
        rename = "MemberAORkeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_aorkey_numeric: Option<f64>,

    /// The street number, direction, name and suffix of the member.
    ///
    /// [MemberAddress1](https://ddwiki.reso.org/display/DDW17/MemberAddress1+Field)
    #[serde(rename = "MemberAddress1", skip_serializing_if = "Option::is_none")]
    pub member_address1: Option<String>,

    /// The unit/suite number of the member.
    ///
    /// [MemberAddress2](https://ddwiki.reso.org/display/DDW17/MemberAddress2+Field)
    #[serde(rename = "MemberAddress2", skip_serializing_if = "Option::is_none")]
    pub member_address2: Option<String>,

    /// The association's notes regarding the member.
    ///
    /// [MemberAssociationComments](https://ddwiki.reso.org/display/DDW17/MemberAssociationComments+Field)
    #[serde(
        rename = "MemberAssociationComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_association_comments: Option<String>,

    /// The group of addresses to which the USPS assigns the same code to aid in mail delivery. For the USPS, these codes are 9 digits: 5 numbers for the ZIP Code, one letter for the carrier route type, and 3 numbers for the carrier route number.
    ///
    /// [MemberCarrierRoute](https://ddwiki.reso.org/display/DDW17/MemberCarrierRoute+Field)
    #[serde(rename = "MemberCarrierRoute", skip_serializing_if = "Option::is_none")]
    pub member_carrier_route: Option<String>,

    /// The city of the member.
    ///
    /// [MemberCity](https://ddwiki.reso.org/display/DDW17/MemberCity+Field)
    #[serde(rename = "MemberCity", skip_serializing_if = "Option::is_none")]
    pub member_city: Option<String>,

    /// The country abbreviation in a postal address.
    ///
    /// [MemberCountry](https://ddwiki.reso.org/display/DDW17/MemberCountry+Field)
    #[serde(rename = "MemberCountry", skip_serializing_if = "Option::is_none")]
    pub member_country: Option<crate::Country>,

    /// The county or parish in which the member is addressed.
    ///
    /// [MemberCountyOrParish](https://ddwiki.reso.org/display/DDW17/MemberCountyOrParish+Field)
    #[serde(
        rename = "MemberCountyOrParish",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_county_or_parish: Option<String>,

    /// Designations and certifications acknowledging experience and expertise in various real estate sectors are awarded by NAR and each affiliated group upon completion of required courses.
    ///
    /// [MemberDesignation](https://ddwiki.reso.org/display/DDW17/MemberDesignation+Field)
    #[serde(rename = "MemberDesignation", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub member_designation: Option<Vec<crate::MemberDesignation>>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [MemberDirectPhone](https://ddwiki.reso.org/display/DDW17/MemberDirectPhone+Field)
    #[serde(rename = "MemberDirectPhone", skip_serializing_if = "Option::is_none")]
    pub member_direct_phone: Option<String>,

    /// The email address of the Member.
    ///
    /// [MemberEmail](https://ddwiki.reso.org/display/DDW17/MemberEmail+Field)
    #[serde(rename = "MemberEmail", skip_serializing_if = "Option::is_none")]
    pub member_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [MemberFax](https://ddwiki.reso.org/display/DDW17/MemberFax+Field)
    #[serde(rename = "MemberFax", skip_serializing_if = "Option::is_none")]
    pub member_fax: Option<String>,

    /// The first name of the Member.
    ///
    /// [MemberFirstName](https://ddwiki.reso.org/display/DDW17/MemberFirstName+Field)
    #[serde(rename = "MemberFirstName", skip_serializing_if = "Option::is_none")]
    pub member_first_name: Option<String>,

    /// The full name of the Member. (First Middle Last) or a alternate full name.
    ///
    /// [MemberFullName](https://ddwiki.reso.org/display/DDW17/MemberFullName+Field)
    #[serde(rename = "MemberFullName", skip_serializing_if = "Option::is_none")]
    pub member_full_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [MemberHomePhone](https://ddwiki.reso.org/display/DDW17/MemberHomePhone+Field)
    #[serde(rename = "MemberHomePhone", skip_serializing_if = "Option::is_none")]
    pub member_home_phone: Option<String>,

    /// The MemberMlsId of the Agent/Broker that this member is an assistant. The typical use will be to add the agent's ID to this field when editing the member record of the assistant.
    ///
    /// [MemberIsAssistantTo](https://ddwiki.reso.org/display/DDW17/MemberIsAssistantTo+Field)
    #[serde(
        rename = "MemberIsAssistantTo",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_is_assistant_to: Option<String>,

    /// A unique identifier for this record from the immediate source. This is a string that can include URI or other forms. Alternatively use the MemberKeyNumeric for a numeric only key field. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, see SourceSystemMemberKey and OriginatingSystemMemberKey.
    ///
    /// [MemberKey](https://ddwiki.reso.org/display/DDW17/MemberKey+Field)
    #[serde(rename = "MemberKey", skip_serializing_if = "Option::is_none")]
    pub member_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the MemberKey fields. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, see SourceSystemMemberKey and OriginatingSystemMemberKey.
    ///
    /// [MemberKeyNumeric](https://ddwiki.reso.org/display/DDW17/MemberKeyNumeric+Field)
    #[serde(rename = "MemberKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub member_key_numeric: Option<f64>,

    /// The languages the member speaks.
    ///
    /// [MemberLanguages](https://ddwiki.reso.org/display/DDW17/MemberLanguages+Field)
    #[serde(rename = "MemberLanguages", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub member_languages: Option<Vec<crate::Languages>>,

    /// The last name of the Member.
    ///
    /// [MemberLastName](https://ddwiki.reso.org/display/DDW17/MemberLastName+Field)
    #[serde(rename = "MemberLastName", skip_serializing_if = "Option::is_none")]
    pub member_last_name: Option<String>,

    /// The ID used to logon to the MLS system.
    ///
    /// [MemberLoginId](https://ddwiki.reso.org/display/DDW17/MemberLoginId+Field)
    #[serde(rename = "MemberLoginId", skip_serializing_if = "Option::is_none")]
    pub member_login_id: Option<String>,

    /// The middle name of the Member.
    ///
    /// [MemberMiddleName](https://ddwiki.reso.org/display/DDW17/MemberMiddleName+Field)
    #[serde(rename = "MemberMiddleName", skip_serializing_if = "Option::is_none")]
    pub member_middle_name: Option<String>,

    /// Does the member have access to the MLS system.
    ///
    /// [MemberMlsAccessYN](https://ddwiki.reso.org/display/DDW17/MemberMlsAccessYN+Field)
    #[serde(rename = "MemberMlsAccessYN", skip_serializing_if = "Option::is_none")]
    pub member_mls_access_yn: Option<bool>,

    /// The local, well-known identifier for the member. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [MemberMlsId](https://ddwiki.reso.org/display/DDW17/MemberMlsId+Field)
    #[serde(rename = "MemberMlsId", skip_serializing_if = "Option::is_none")]
    pub member_mls_id: Option<String>,

    /// The MLS security group or class given to the member.
    ///
    /// [MemberMlsSecurityClass](https://ddwiki.reso.org/display/DDW17/MemberMlsSecurityClass+Field)
    #[serde(
        rename = "MemberMlsSecurityClass",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_mls_security_class: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [MemberMobilePhone](https://ddwiki.reso.org/display/DDW17/MemberMobilePhone+Field)
    #[serde(rename = "MemberMobilePhone", skip_serializing_if = "Option::is_none")]
    pub member_mobile_phone: Option<String>,

    /// Prefix to the name (e.g. Dr. Mr. Ms. etc.)
    ///
    /// [MemberNamePrefix](https://ddwiki.reso.org/display/DDW17/MemberNamePrefix+Field)
    #[serde(rename = "MemberNamePrefix", skip_serializing_if = "Option::is_none")]
    pub member_name_prefix: Option<String>,

    /// Suffix to the surname (e.g. Esq., Jr., III etc.)
    ///
    /// [MemberNameSuffix](https://ddwiki.reso.org/display/DDW17/MemberNameSuffix+Field)
    #[serde(rename = "MemberNameSuffix", skip_serializing_if = "Option::is_none")]
    pub member_name_suffix: Option<String>,

    /// The national association ID of the member. i.e. in the U.S. is the NRDS number.
    ///
    /// [MemberNationalAssociationId](https://ddwiki.reso.org/display/DDW17/MemberNationalAssociationId+Field)
    #[serde(
        rename = "MemberNationalAssociationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_national_association_id: Option<String>,

    /// An alternate name used by the Member, usually as a substitute for the first name.
    ///
    /// [MemberNickname](https://ddwiki.reso.org/display/DDW17/MemberNickname+Field)
    #[serde(rename = "MemberNickname", skip_serializing_if = "Option::is_none")]
    pub member_nickname: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [MemberOfficePhone](https://ddwiki.reso.org/display/DDW17/MemberOfficePhone+Field)
    #[serde(rename = "MemberOfficePhone", skip_serializing_if = "Option::is_none")]
    pub member_office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [MemberOfficePhoneExt](https://ddwiki.reso.org/display/DDW17/MemberOfficePhoneExt+Field)
    #[serde(
        rename = "MemberOfficePhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_office_phone_ext: Option<String>,

    /// A collection of the types of other phone fields available for this member. The collection includes the type of system and other details pertinent about other phone numbers
    ///
    /// [MemberOtherPhone](https://ddwiki.reso.org/display/DDW17/MemberOtherPhone+Field)
    #[serde(rename = "MemberOtherPhone", skip_serializing_if = "Option::is_none")]
    pub member_other_phone: Option<String>,

    /// The type of "other" phone. i.e. Preferred, Office, Mobile, Direct, Home, Fax, Pager, Voicemail, Toll Free, SMS, 1, 2, 3, First, Second, Third, etc.. This is used as the list of options for the Member Other Phone repeating elements.
    ///
    /// [MemberOtherPhoneType](https://ddwiki.reso.org/display/DDW17/MemberOtherPhoneType+Field)
    #[serde(
        rename = "MemberOtherPhoneType",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_other_phone_type: Option<crate::MemberOtherPhoneType>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [MemberPager](https://ddwiki.reso.org/display/DDW17/MemberPager+Field)
    #[serde(rename = "MemberPager", skip_serializing_if = "Option::is_none")]
    pub member_pager: Option<String>,

    /// A password that the member whishes to share with other systems. Normal security considerations apply and are the responsibility of the entity utilizing this field.
    ///
    /// [MemberPassword](https://ddwiki.reso.org/display/DDW17/MemberPassword+Field)
    #[serde(rename = "MemberPassword", skip_serializing_if = "Option::is_none")]
    pub member_password: Option<String>,

    /// (Telecommunications Device for the Deaf/TeleTYpewriter) A user terminal with keyboard input and printer or display output used by the hearing and speech impaired. The device contains a modem and is used over a standard analog phone line. If a recipient does not have a corresponding terminal device, TDD/TTY users dial a relay service composed of operators who receive the typed messages, call the recipients and speak the messages to them. The operators also type the responses back to the TDD/TTY user.
    ///
    /// [MemberPhoneTTYTDD](https://ddwiki.reso.org/display/DDW17/MemberPhoneTTYTDD+Field)
    #[serde(rename = "MemberPhoneTTYTDD", skip_serializing_if = "Option::is_none")]
    pub member_phone_ttytdd: Option<String>,

    /// The postal code of the member.
    ///
    /// [MemberPostalCode](https://ddwiki.reso.org/display/DDW17/MemberPostalCode+Field)
    #[serde(rename = "MemberPostalCode", skip_serializing_if = "Option::is_none")]
    pub member_postal_code: Option<String>,

    /// The extension of the postal/zip code. i.e. +4
    ///
    /// [MemberPostalCodePlus4](https://ddwiki.reso.org/display/DDW17/MemberPostalCodePlus4+Field)
    #[serde(
        rename = "MemberPostalCodePlus4",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_postal_code_plus4: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [MemberPreferredPhone](https://ddwiki.reso.org/display/DDW17/MemberPreferredPhone+Field)
    #[serde(
        rename = "MemberPreferredPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_preferred_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [MemberPreferredPhoneExt](https://ddwiki.reso.org/display/DDW17/MemberPreferredPhoneExt+Field)
    #[serde(
        rename = "MemberPreferredPhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_preferred_phone_ext: Option<String>,

    /// A collection of the types of social media fields available for this member. The collection includes the type of system and other details pertinent about social media
    ///
    /// [MemberSocialMedia](https://ddwiki.reso.org/display/DDW17/MemberSocialMedia+Field)
    #[serde(rename = "MemberSocialMedia", skip_serializing_if = "Option::is_none")]
    pub member_social_media: Option<String>,

    /// The license of the Member. Separate multiple licenses with a comma and space.
    ///
    /// [MemberStateLicense](https://ddwiki.reso.org/display/DDW17/MemberStateLicense+Field)
    #[serde(rename = "MemberStateLicense", skip_serializing_if = "Option::is_none")]
    pub member_state_license: Option<String>,

    /// The state in which the member is licensed.
    ///
    /// [MemberStateLicenseState](https://ddwiki.reso.org/display/DDW17/MemberStateLicenseState+Field)
    #[serde(
        rename = "MemberStateLicenseState",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_state_license_state: Option<crate::StateOrProvince>,

    /// The state or province in which the member is addressed.
    ///
    /// [MemberStateOrProvince](https://ddwiki.reso.org/display/DDW17/MemberStateOrProvince+Field)
    #[serde(
        rename = "MemberStateOrProvince",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_state_or_province: Option<crate::StateOrProvince>,

    /// Is the account active, inactive or under disciplinary action.
    ///
    /// [MemberStatus](https://ddwiki.reso.org/display/DDW17/MemberStatus+Field)
    #[serde(rename = "MemberStatus", skip_serializing_if = "Option::is_none")]
    pub member_status: Option<crate::MemberStatus>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [MemberTollFreePhone](https://ddwiki.reso.org/display/DDW17/MemberTollFreePhone+Field)
    #[serde(
        rename = "MemberTollFreePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_toll_free_phone: Option<String>,

    /// The type of member. i.e. Agent, Broker, Office Manager, Appraiser, Photographer, Assistants, MLO, Realtor, Association Staff, MLS Staff, etc.
    ///
    /// [MemberType](https://ddwiki.reso.org/display/DDW17/MemberType+Field)
    #[serde(rename = "MemberType", skip_serializing_if = "Option::is_none")]
    pub member_type: Option<crate::MemberType>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [MemberVoiceMail](https://ddwiki.reso.org/display/DDW17/MemberVoiceMail+Field)
    #[serde(rename = "MemberVoiceMail", skip_serializing_if = "Option::is_none")]
    pub member_voice_mail: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [MemberVoiceMailExt](https://ddwiki.reso.org/display/DDW17/MemberVoiceMailExt+Field)
    #[serde(rename = "MemberVoiceMailExt", skip_serializing_if = "Option::is_none")]
    pub member_voice_mail_ext: Option<String>,

    /// Date/time the roster (member or office) record was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28Member%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set. This is a foreign key relating to the Office resource's OfficeKey.
    ///
    /// [OfficeKey](https://ddwiki.reso.org/display/DDW17/OfficeKey+Field)
    #[serde(rename = "OfficeKey", skip_serializing_if = "Option::is_none")]
    pub office_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set. This is a foreign key relating to the Office resource's OfficeKey.  This is the numeric only key and used as an alternative to the MemberAORkey field.
    ///
    /// [OfficeKeyNumeric](https://ddwiki.reso.org/display/DDW17/OfficeKeyNumeric+Field)
    #[serde(rename = "OfficeKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub office_key_numeric: Option<f64>,

    /// The local, well-known identifier. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [OfficeMlsId](https://ddwiki.reso.org/display/DDW17/OfficeMlsId+Field)
    #[serde(rename = "OfficeMlsId", skip_serializing_if = "Option::is_none")]
    pub office_mls_id: Option<String>,

    /// The legal name of the brokerage.
    ///
    /// [OfficeName](https://ddwiki.reso.org/display/DDW17/OfficeName+Field)
    #[serde(rename = "OfficeName", skip_serializing_if = "Option::is_none")]
    pub office_name: Option<String>,

    /// Date/time the roster (member or office) record was originally input into the source system.
    ///
    /// [OriginalEntryTimestamp](https://ddwiki.reso.org/display/DDW17/OriginalEntryTimestamp+%28Member%29+Field)
    #[serde(
        rename = "OriginalEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_entry_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider. The Originating system is the system with authoritative control over the record. For example; the name of the MLS where the member was input. In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+%28Member%29+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Originating system. The Originating system is the system with authoritative control over the record. For example, the Multiple Listing Service where the member was input. There may be cases where the Source System (how you received the record) is not the Originating System. See Source System Key for more information.
    ///
    /// [OriginatingSystemMemberKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemMemberKey+Field)
    #[serde(
        rename = "OriginatingSystemMemberKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_member_key: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the member is originally input by the member.  The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+%28Member%29+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// A list of types of sites, blog, social media, the Member URL or ID is referring to. i.e. Website, Blog, Facebook, Twitter, LinkedIn, Skype, etc., This list is used to populate the Type with repeating Social Media URL or ID types.
    ///
    /// [SocialMediaType](https://ddwiki.reso.org/display/DDW17/SocialMediaType+Field)
    #[serde(rename = "SocialMediaType", skip_serializing_if = "Option::is_none")]
    pub social_media_type: Option<crate::SocialMediaType>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider.  The source system is the system from which the record was directly received.  In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+%28Member%29+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemMemberKey](https://ddwiki.reso.org/display/DDW17/SourceSystemMemberKey+Field)
    #[serde(
        rename = "SourceSystemMemberKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_member_key: Option<String>,

    /// The name of the immediate record provider.  The system from which the record was directly received.  The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+%28Member%29+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,

    /// When permitted by the broker, the options made by the individual agent on where they would like their listings syndicated.  i.e. Zillow, Trulia, Homes.com, etc.
    ///
    /// [SyndicateTo](https://ddwiki.reso.org/display/DDW17/SyndicateTo+%28Member%29+Field)
    #[serde(rename = "SyndicateTo", skip_serializing_if = "Option::is_none")]
    pub syndicate_to: Option<String>,
}
