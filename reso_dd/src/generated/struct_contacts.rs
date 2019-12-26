// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};

/// [Contacts Resource](https://ddwiki.reso.org/display/DDW17/Contacts+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Contacts {
    /// The wedding anniversary of the contact; month, day and year.
    ///
    /// [Anniversary](https://ddwiki.reso.org/display/DDW17/Anniversary+Field)
    #[serde(rename = "Anniversary", skip_serializing_if = "Option::is_none")]
    pub anniversary: Option<chrono::NaiveDate>,

    /// Email address of the contact's assistant.
    ///
    /// [AssistantEmail](https://ddwiki.reso.org/display/DDW17/AssistantEmail+Field)
    #[serde(rename = "AssistantEmail", skip_serializing_if = "Option::is_none")]
    pub assistant_email: Option<String>,

    /// Name of the contact's assistant.
    ///
    /// [AssistantName](https://ddwiki.reso.org/display/DDW17/AssistantName+Field)
    #[serde(rename = "AssistantName", skip_serializing_if = "Option::is_none")]
    pub assistant_name: Option<String>,

    /// Phone number of the contact's assistant.
    ///
    /// [AssistantPhone](https://ddwiki.reso.org/display/DDW17/AssistantPhone+Field)
    #[serde(rename = "AssistantPhone", skip_serializing_if = "Option::is_none")]
    pub assistant_phone: Option<String>,

    /// Phone number extension of the contact's assistant.
    ///
    /// [AssistantPhoneExt](https://ddwiki.reso.org/display/DDW17/AssistantPhoneExt+Field)
    #[serde(rename = "AssistantPhoneExt", skip_serializing_if = "Option::is_none")]
    pub assistant_phone_ext: Option<String>,

    /// The birthday of the contact; month, day and year.
    ///
    /// [Birthdate](https://ddwiki.reso.org/display/DDW17/Birthdate+Field)
    #[serde(rename = "Birthdate", skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<chrono::NaiveDate>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [BusinessFax](https://ddwiki.reso.org/display/DDW17/BusinessFax+Field)
    #[serde(rename = "BusinessFax", skip_serializing_if = "Option::is_none")]
    pub business_fax: Option<String>,

    /// A list of the names of the contact's children in a comma separated list.
    ///
    /// [Children](https://ddwiki.reso.org/display/DDW17/Children+Field)
    #[serde(rename = "Children", skip_serializing_if = "Option::is_none")]
    pub children: Option<String>,

    /// The contact's company or employer.
    ///
    /// [Company](https://ddwiki.reso.org/display/DDW17/Company+Field)
    #[serde(rename = "Company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the ContactKey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId.
    ///
    /// [ContactKey](https://ddwiki.reso.org/display/DDW17/ContactKey+Field)
    #[serde(rename = "ContactKey", skip_serializing_if = "Option::is_none")]
    pub contact_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the ContactKey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId.  This is the numeric only key and used as an alternative to the ContactKey field.
    ///
    /// [ContactKeyNumeric](https://ddwiki.reso.org/display/DDW17/ContactKeyNumeric+Field)
    #[serde(rename = "ContactKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub contact_key_numeric: Option<f64>,

    /// The local, well-known identifier for the contact. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system and is used by the Contact to logon to a client portal in that system.
    ///
    /// [ContactLoginId](https://ddwiki.reso.org/display/DDW17/ContactLoginId+Field)
    #[serde(rename = "ContactLoginId", skip_serializing_if = "Option::is_none")]
    pub contact_login_id: Option<String>,

    /// A client password that the member wishes to share with other systems. Normal security considerations apply and are the responsibility of the entity utilizing this field.
    ///
    /// [ContactPassword](https://ddwiki.reso.org/display/DDW17/ContactPassword+Field)
    #[serde(rename = "ContactPassword", skip_serializing_if = "Option::is_none")]
    pub contact_password: Option<String>,

    /// The status of the contact. Active, Inactive, On Vacation, Deleted, etc.,
    ///
    /// [ContactStatus](https://ddwiki.reso.org/display/DDW17/ContactStatus+Field)
    #[serde(rename = "ContactStatus", skip_serializing_if = "Option::is_none")]
    pub contact_status: Option<String>,

    /// The type of contact. i.e. Business, Friend, Family, Prospect, Ready to Buy, etc.
    ///
    /// [ContactType](https://ddwiki.reso.org/display/DDW17/ContactType+Field)
    #[serde(rename = "ContactType", skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<String>,

    /// A collection of the types of other phone fields  available for Contacts. The collection includes the type of system and other details pertinent about other phone numbers
    ///
    /// [ContactsOtherPhone](https://ddwiki.reso.org/display/DDW17/ContactsOtherPhone+Field)
    #[serde(rename = "ContactsOtherPhone", skip_serializing_if = "Option::is_none")]
    pub contacts_other_phone: Option<String>,

    /// A collection of the types of social media fields  available for this contact. The collection includes the type of system and other details pertinent about social media
    ///
    /// [ContactsSocialMedia](https://ddwiki.reso.org/display/DDW17/ContactsSocialMedia+Field)
    #[serde(
        rename = "ContactsSocialMedia",
        skip_serializing_if = "Option::is_none"
    )]
    pub contacts_social_media: Option<String>,

    /// The department in which the contact works.
    ///
    /// [Department](https://ddwiki.reso.org/display/DDW17/Department+Field)
    #[serde(rename = "Department", skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [DirectPhone](https://ddwiki.reso.org/display/DDW17/DirectPhone+Field)
    #[serde(rename = "DirectPhone", skip_serializing_if = "Option::is_none")]
    pub direct_phone: Option<String>,

    /// The preferred Email address of the contact.
    ///
    /// [Email](https://ddwiki.reso.org/display/DDW17/Email+Field)
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The secondary email address of the contact.
    ///
    /// [Email2](https://ddwiki.reso.org/display/DDW17/Email2+Field)
    #[serde(rename = "Email2", skip_serializing_if = "Option::is_none")]
    pub email2: Option<String>,

    /// The tertiary email address of the contact.
    ///
    /// [Email3](https://ddwiki.reso.org/display/DDW17/Email3+Field)
    #[serde(rename = "Email3", skip_serializing_if = "Option::is_none")]
    pub email3: Option<String>,

    /// The first name of the Contact.
    ///
    /// [FirstName](https://ddwiki.reso.org/display/DDW17/FirstName+Field)
    #[serde(rename = "FirstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// The full name of the Contact. (First Middle Last) or a alternate full name.
    ///
    /// [FullName](https://ddwiki.reso.org/display/DDW17/FullName+Field)
    #[serde(rename = "FullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,

    /// The street number, direction, name and suffix of the contact's home.
    ///
    /// [HomeAddress1](https://ddwiki.reso.org/display/DDW17/HomeAddress1+Field)
    #[serde(rename = "HomeAddress1", skip_serializing_if = "Option::is_none")]
    pub home_address1: Option<String>,

    /// The unit/suite number of the contact's home.
    ///
    /// [HomeAddress2](https://ddwiki.reso.org/display/DDW17/HomeAddress2+Field)
    #[serde(rename = "HomeAddress2", skip_serializing_if = "Option::is_none")]
    pub home_address2: Option<String>,

    /// The group of addresses to which the USPS assigns the same code to aid in mail delivery. For the USPS, these codes are 9 digits: 5 numbers for the ZIP Code, one letter for the carrier route type, and 3 numbers for the carrier route number.
    ///
    /// [HomeCarrierRoute](https://ddwiki.reso.org/display/DDW17/HomeCarrierRoute+Field)
    #[serde(rename = "HomeCarrierRoute", skip_serializing_if = "Option::is_none")]
    pub home_carrier_route: Option<String>,

    /// The city of the contact's home.
    ///
    /// [HomeCity](https://ddwiki.reso.org/display/DDW17/HomeCity+Field)
    #[serde(rename = "HomeCity", skip_serializing_if = "Option::is_none")]
    pub home_city: Option<String>,

    /// The country abbreviation in a postal address.
    ///
    /// [HomeCountry](https://ddwiki.reso.org/display/DDW17/HomeCountry+Field)
    #[serde(rename = "HomeCountry", skip_serializing_if = "Option::is_none")]
    pub home_country: Option<String>,

    /// The county or parish in which the contact's home is addressed.
    ///
    /// [HomeCountyOrParish](https://ddwiki.reso.org/display/DDW17/HomeCountyOrParish+Field)
    #[serde(rename = "HomeCountyOrParish", skip_serializing_if = "Option::is_none")]
    pub home_county_or_parish: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [HomeFax](https://ddwiki.reso.org/display/DDW17/HomeFax+Field)
    #[serde(rename = "HomeFax", skip_serializing_if = "Option::is_none")]
    pub home_fax: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [HomePhone](https://ddwiki.reso.org/display/DDW17/HomePhone+Field)
    #[serde(rename = "HomePhone", skip_serializing_if = "Option::is_none")]
    pub home_phone: Option<String>,

    /// The postal code of the contact's home.
    ///
    /// [HomePostalCode](https://ddwiki.reso.org/display/DDW17/HomePostalCode+Field)
    #[serde(rename = "HomePostalCode", skip_serializing_if = "Option::is_none")]
    pub home_postal_code: Option<String>,

    /// The extension of the postal/zip code.  i.e. +4
    ///
    /// [HomePostalCodePlus4](https://ddwiki.reso.org/display/DDW17/HomePostalCodePlus4+Field)
    #[serde(
        rename = "HomePostalCodePlus4",
        skip_serializing_if = "Option::is_none"
    )]
    pub home_postal_code_plus4: Option<String>,

    /// The state or province in which the contact's home is addressed.
    ///
    /// [HomeStateOrProvince](https://ddwiki.reso.org/display/DDW17/HomeStateOrProvince+Field)
    #[serde(
        rename = "HomeStateOrProvince",
        skip_serializing_if = "Option::is_none"
    )]
    pub home_state_or_province: Option<String>,

    /// The title or position of the contact within their organization.
    ///
    /// [JobTitle](https://ddwiki.reso.org/display/DDW17/JobTitle+%28Contacts%29+Field)
    #[serde(rename = "JobTitle", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,

    /// The languages spoken by the contact.
    ///
    /// [Language](https://ddwiki.reso.org/display/DDW17/Language+Field)
    #[serde(rename = "Language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// The last name of the Contact.
    ///
    /// [LastName](https://ddwiki.reso.org/display/DDW17/LastName+Field)
    #[serde(rename = "LastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// The source or person that provided the contact.
    ///
    /// [LeadSource](https://ddwiki.reso.org/display/DDW17/LeadSource+Field)
    #[serde(rename = "LeadSource", skip_serializing_if = "Option::is_none")]
    pub lead_source: Option<String>,

    /// The middle name of the Contact.
    ///
    /// [MiddleName](https://ddwiki.reso.org/display/DDW17/MiddleName+Field)
    #[serde(rename = "MiddleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [MobilePhone](https://ddwiki.reso.org/display/DDW17/MobilePhone+Field)
    #[serde(rename = "MobilePhone", skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,

    /// Date/time the contact record was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28Contacts%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Prefix to the name (e.g. Dr. Mr. Ms. etc.)
    ///
    /// [NamePrefix](https://ddwiki.reso.org/display/DDW17/NamePrefix+Field)
    #[serde(rename = "NamePrefix", skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,

    /// Suffix to the surname (e.g. Esq.,  Jr.,  III etc.)
    ///
    /// [NameSuffix](https://ddwiki.reso.org/display/DDW17/NameSuffix+Field)
    #[serde(rename = "NameSuffix", skip_serializing_if = "Option::is_none")]
    pub name_suffix: Option<String>,

    /// An alternate name used by the Contact, usually as a substitute for the first name.
    ///
    /// [Nickname](https://ddwiki.reso.org/display/DDW17/Nickname+Field)
    #[serde(rename = "Nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// Notes about the client.
    ///
    /// [Notes](https://ddwiki.reso.org/display/DDW17/Notes+Field)
    #[serde(rename = "Notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [OfficePhone](https://ddwiki.reso.org/display/DDW17/OfficePhone+%28Contacts%29+Field)
    #[serde(rename = "OfficePhone", skip_serializing_if = "Option::is_none")]
    pub office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [OfficePhoneExt](https://ddwiki.reso.org/display/DDW17/OfficePhoneExt+%28Contacts%29+Field)
    #[serde(rename = "OfficePhoneExt", skip_serializing_if = "Option::is_none")]
    pub office_phone_ext: Option<String>,

    /// Date/time the contact record was originally input into the source system.
    ///
    /// [OriginalEntryTimestamp](https://ddwiki.reso.org/display/DDW17/OriginalEntryTimestamp+%28Contacts%29+Field)
    #[serde(
        rename = "OriginalEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_entry_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The system key, a unique record identifier, from the Originating system.  The Originating system is the system with authoritative control over the record.  For example, the Multiple Listing Service where the Contact was input.  There may be cases where the Source System (how you received the record) is not the Originating System.  See Source System Key for more information.
    ///
    /// [OriginatingSystemContactKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemContactKey+Field)
    #[serde(
        rename = "OriginatingSystemContactKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_contact_key: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the Contact was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+%28Contacts%29+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the Contact is originally input by the member.  The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+%28Contacts%29+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// The other street number, direction, name and suffix of the contact.
    ///
    /// [OtherAddress1](https://ddwiki.reso.org/display/DDW17/OtherAddress1+Field)
    #[serde(rename = "OtherAddress1", skip_serializing_if = "Option::is_none")]
    pub other_address1: Option<String>,

    /// The other unit/suite number of the contact.
    ///
    /// [OtherAddress2](https://ddwiki.reso.org/display/DDW17/OtherAddress2+Field)
    #[serde(rename = "OtherAddress2", skip_serializing_if = "Option::is_none")]
    pub other_address2: Option<String>,

    /// The group of addresses to which the USPS assigns the same code to aid in mail delivery. For the USPS, these codes are 9 digits: 5 numbers for the ZIP Code, one letter for the carrier route type, and 3 numbers for the carrier route number.
    ///
    /// [OtherCarrierRoute](https://ddwiki.reso.org/display/DDW17/OtherCarrierRoute+Field)
    #[serde(rename = "OtherCarrierRoute", skip_serializing_if = "Option::is_none")]
    pub other_carrier_route: Option<String>,

    /// The other city of the contact.
    ///
    /// [OtherCity](https://ddwiki.reso.org/display/DDW17/OtherCity+Field)
    #[serde(rename = "OtherCity", skip_serializing_if = "Option::is_none")]
    pub other_city: Option<String>,

    /// The other country abbreviation in a postal address.
    ///
    /// [OtherCountry](https://ddwiki.reso.org/display/DDW17/OtherCountry+Field)
    #[serde(rename = "OtherCountry", skip_serializing_if = "Option::is_none")]
    pub other_country: Option<String>,

    /// The other county or parish in which contact is addressed.
    ///
    /// [OtherCountyOrParish](https://ddwiki.reso.org/display/DDW17/OtherCountyOrParish+Field)
    #[serde(
        rename = "OtherCountyOrParish",
        skip_serializing_if = "Option::is_none"
    )]
    pub other_county_or_parish: Option<String>,

    /// The type of "other" phone that does not already exist in the given phone fields or if a second of any type of phone field is needed. i.e. HomePhone2, BrothersPhone, etc. This is used as the list of options for the Other Phone repeating elements.
    ///
    /// [OtherPhoneType](https://ddwiki.reso.org/display/DDW17/OtherPhoneType+Field)
    #[serde(rename = "OtherPhoneType", skip_serializing_if = "Option::is_none")]
    pub other_phone_type: Option<String>,

    /// The other postal code of the contact.
    ///
    /// [OtherPostalCode](https://ddwiki.reso.org/display/DDW17/OtherPostalCode+Field)
    #[serde(rename = "OtherPostalCode", skip_serializing_if = "Option::is_none")]
    pub other_postal_code: Option<String>,

    /// The other extension of the postal/zip code.  i.e. +4
    ///
    /// [OtherPostalCodePlus4](https://ddwiki.reso.org/display/DDW17/OtherPostalCodePlus4+Field)
    #[serde(
        rename = "OtherPostalCodePlus4",
        skip_serializing_if = "Option::is_none"
    )]
    pub other_postal_code_plus4: Option<String>,

    /// The other state or province in which the contact is addressed.
    ///
    /// [OtherStateOrProvince](https://ddwiki.reso.org/display/DDW17/OtherStateOrProvince+Field)
    #[serde(
        rename = "OtherStateOrProvince",
        skip_serializing_if = "Option::is_none"
    )]
    pub other_state_or_province: Option<String>,

    /// The local, well-known identifier for the member owning the contact.
    ///
    /// [OwnerMemberID](https://ddwiki.reso.org/display/DDW17/OwnerMemberID+Field)
    #[serde(rename = "OwnerMemberID", skip_serializing_if = "Option::is_none")]
    pub owner_member_id: Option<String>,

    /// The unique identifier (key) of the member owning the contact. This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [OwnerMemberKey](https://ddwiki.reso.org/display/DDW17/OwnerMemberKey+Field)
    #[serde(rename = "OwnerMemberKey", skip_serializing_if = "Option::is_none")]
    pub owner_member_key: Option<String>,

    /// The unique identifier (key) of the member owning the contact. This is a foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the OwnerMemberKey field.
    ///
    /// [OwnerMemberKeyNumeric](https://ddwiki.reso.org/display/DDW17/OwnerMemberKeyNumeric+Field)
    #[serde(
        rename = "OwnerMemberKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_member_key_numeric: Option<f64>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [Pager](https://ddwiki.reso.org/display/DDW17/Pager+Field)
    #[serde(rename = "Pager", skip_serializing_if = "Option::is_none")]
    pub pager: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [PhoneTTYTDD](https://ddwiki.reso.org/display/DDW17/PhoneTTYTDD+Field)
    #[serde(rename = "PhoneTTYTDD", skip_serializing_if = "Option::is_none")]
    pub phone_ttytdd: Option<String>,

    /// A list of the address options Home, Work and Other used to determine the address preferred by the client.
    ///
    /// [PreferredAddress](https://ddwiki.reso.org/display/DDW17/PreferredAddress+Field)
    #[serde(rename = "PreferredAddress", skip_serializing_if = "Option::is_none")]
    pub preferred_address: Option<String>,

    /// A list of the phone options Office, Mobile, Direct, Voicemail, Other used to determine the phone preferred by the client.
    ///
    /// [PreferredPhone](https://ddwiki.reso.org/display/DDW17/PreferredPhone+Field)
    #[serde(rename = "PreferredPhone", skip_serializing_if = "Option::is_none")]
    pub preferred_phone: Option<String>,

    /// Name of the person who referred the contact.
    ///
    /// [ReferredBy](https://ddwiki.reso.org/display/DDW17/ReferredBy+Field)
    #[serde(rename = "ReferredBy", skip_serializing_if = "Option::is_none")]
    pub referred_by: Option<String>,

    /// A list of types of sites, blog, social media, the Contact URL or ID is referring to. i.e. Website, Blog, Facebook, Twitter, LinkedIn, Skype, etc., This list is used to populate the Type with repeating Social Media URL or ID types.
    ///
    /// [SocialMediaType](https://ddwiki.reso.org/display/DDW17/SocialMediaType+%28Contacts%29+Field)
    #[serde(rename = "SocialMediaType", skip_serializing_if = "Option::is_none")]
    pub social_media_type: Option<String>,

    /// The system key, a unique record identifier, from the Source System.  The Source System is the system from which the record was directly received.  In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemContactKey](https://ddwiki.reso.org/display/DDW17/SourceSystemContactKey+Field)
    #[serde(
        rename = "SourceSystemContactKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_contact_key: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider.  The source system is the system from which the record was directly received.  In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+%28Contacts%29+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The name of the immediate record provider.  The system from which the record was directly received.  The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+%28Contacts%29+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,

    /// The contact's spouse or partner.
    ///
    /// [SpousePartnerName](https://ddwiki.reso.org/display/DDW17/SpousePartnerName+Field)
    #[serde(rename = "SpousePartnerName", skip_serializing_if = "Option::is_none")]
    pub spouse_partner_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [TollFreePhone](https://ddwiki.reso.org/display/DDW17/TollFreePhone+Field)
    #[serde(rename = "TollFreePhone", skip_serializing_if = "Option::is_none")]
    pub toll_free_phone: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [VoiceMail](https://ddwiki.reso.org/display/DDW17/VoiceMail+Field)
    #[serde(rename = "VoiceMail", skip_serializing_if = "Option::is_none")]
    pub voice_mail: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [VoiceMailExt](https://ddwiki.reso.org/display/DDW17/VoiceMailExt+Field)
    #[serde(rename = "VoiceMailExt", skip_serializing_if = "Option::is_none")]
    pub voice_mail_ext: Option<String>,

    /// The street number, direction, name and suffix of the contact's work.
    ///
    /// [WorkAddress1](https://ddwiki.reso.org/display/DDW17/WorkAddress1+Field)
    #[serde(rename = "WorkAddress1", skip_serializing_if = "Option::is_none")]
    pub work_address1: Option<String>,

    /// The unit/suite number of the contact's work.
    ///
    /// [WorkAddress2](https://ddwiki.reso.org/display/DDW17/WorkAddress2+Field)
    #[serde(rename = "WorkAddress2", skip_serializing_if = "Option::is_none")]
    pub work_address2: Option<String>,

    /// The group of addresses to which the USPS assigns the same code to aid in mail delivery. For the USPS, these codes are 9 digits: 5 numbers for the ZIP Code, one letter for the carrier route type, and 3 numbers for the carrier route number.
    ///
    /// [WorkCarrierRoute](https://ddwiki.reso.org/display/DDW17/WorkCarrierRoute+Field)
    #[serde(rename = "WorkCarrierRoute", skip_serializing_if = "Option::is_none")]
    pub work_carrier_route: Option<String>,

    /// The city of the contact's work.
    ///
    /// [WorkCity](https://ddwiki.reso.org/display/DDW17/WorkCity+Field)
    #[serde(rename = "WorkCity", skip_serializing_if = "Option::is_none")]
    pub work_city: Option<String>,

    /// The country abbreviation in a postal address.
    ///
    /// [WorkCountry](https://ddwiki.reso.org/display/DDW17/WorkCountry+Field)
    #[serde(rename = "WorkCountry", skip_serializing_if = "Option::is_none")]
    pub work_country: Option<String>,

    /// The county or parish in which the contact's work is addressed.
    ///
    /// [WorkCountyOrParish](https://ddwiki.reso.org/display/DDW17/WorkCountyOrParish+Field)
    #[serde(rename = "WorkCountyOrParish", skip_serializing_if = "Option::is_none")]
    pub work_county_or_parish: Option<String>,

    /// The postal code of the contact's work.
    ///
    /// [WorkPostalCode](https://ddwiki.reso.org/display/DDW17/WorkPostalCode+Field)
    #[serde(rename = "WorkPostalCode", skip_serializing_if = "Option::is_none")]
    pub work_postal_code: Option<String>,

    /// The extension of the postal/zip code.  i.e. +4
    ///
    /// [WorkPostalCodePlus4](https://ddwiki.reso.org/display/DDW17/WorkPostalCodePlus4+Field)
    #[serde(
        rename = "WorkPostalCodePlus4",
        skip_serializing_if = "Option::is_none"
    )]
    pub work_postal_code_plus4: Option<String>,

    /// The state or province in which the contact's work is addressed.
    ///
    /// [WorkStateOrProvince](https://ddwiki.reso.org/display/DDW17/WorkStateOrProvince+Field)
    #[serde(
        rename = "WorkStateOrProvince",
        skip_serializing_if = "Option::is_none"
    )]
    pub work_state_or_province: Option<String>,
}
