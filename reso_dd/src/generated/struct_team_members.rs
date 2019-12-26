// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};

/// [TeamMembers Resource](https://ddwiki.reso.org/display/DDW17/TeamMembers+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TeamMembers {
    /// A system unique identifier. Specifically, the foreign key relating to the Member resource's MemberKey.
    ///
    /// [MemberKey](https://ddwiki.reso.org/display/DDW17/MemberKey+%28TeamMembers%29+Field)
    #[serde(rename = "MemberKey", skip_serializing_if = "Option::is_none")]
    pub member_key: Option<String>,

    /// A system unique identifier. Specifically, the foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the MemberKey field.
    ///
    /// [MemberKeyNumeric](https://ddwiki.reso.org/display/DDW17/MemberKeyNumeric+%28TeamMembers%29+Field)
    #[serde(rename = "MemberKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub member_key_numeric: Option<f64>,

    /// The ID used to logon to the MLS system.
    ///
    /// [MemberLoginId](https://ddwiki.reso.org/display/DDW17/MemberLoginId+%28TeamMembers%29+Field)
    #[serde(rename = "MemberLoginId", skip_serializing_if = "Option::is_none")]
    pub member_login_id: Option<String>,

    /// The local, well-known identifier for the member. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [MemberMlsId](https://ddwiki.reso.org/display/DDW17/MemberMlsId+%28TeamMembers%29+Field)
    #[serde(rename = "MemberMlsId", skip_serializing_if = "Option::is_none")]
    pub member_mls_id: Option<String>,

    /// Date/time the roster (member or office) record was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28TeamMembers%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Date/time the roster (member or office) record was originally input into the source system.
    ///
    /// [OriginalEntryTimestamp](https://ddwiki.reso.org/display/DDW17/OriginalEntryTimestamp+%28TeamMembers%29+Field)
    #[serde(
        rename = "OriginalEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_entry_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the Team Member was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+%28TeamMembers%29+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Originating system.  The Originating system is the system with authoritative control over the record.  For example, the Multiple Listing Service where the Team Member was input.  There may be cases where the Source System (how you received the record) is not the Originating System.  See Source System Key for more information.
    ///
    /// [OriginatingSystemKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemKey+%28TeamMembers%29+Field)
    #[serde(
        rename = "OriginatingSystemKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_key: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the Team Member is originally input.  The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+%28TeamMembers%29+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider.  The source system is the system from which the record was directly received.  In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+%28TeamMembers%29+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Source System.  The Source System is the system from which the record was directly received.  In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemKey](https://ddwiki.reso.org/display/DDW17/SourceSystemKey+%28TeamMembers%29+Field)
    #[serde(rename = "SourceSystemKey", skip_serializing_if = "Option::is_none")]
    pub source_system_key: Option<String>,

    /// The name of the Team Member record provider.  The system from which the record was directly received.  The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+%28TeamMembers%29+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,

    /// The level of impersonation the member is allowed within the team.  i.e. Impersonate (to work as the team), On Behalf (to show the team name, but also show the member's info, None (don't allow this member to appear as part of team).
    ///
    /// [TeamImpersonationLevel](https://ddwiki.reso.org/display/DDW17/TeamImpersonationLevel+Field)
    #[serde(
        rename = "TeamImpersonationLevel",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_impersonation_level: Option<String>,

    /// A system unique identifier. Specifically, a foreign key referencing the Teams resource's primary key.
    ///
    /// [TeamKey](https://ddwiki.reso.org/display/DDW17/TeamKey+%28TeamMembers%29+Field)
    #[serde(rename = "TeamKey", skip_serializing_if = "Option::is_none")]
    pub team_key: Option<String>,

    /// A system unique identifier. Specifically, a foreign key referencing the Teams resource's primary key.  This is the numeric only key and used as an alternative to the TeamKey field.
    ///
    /// [TeamKeyNumeric](https://ddwiki.reso.org/display/DDW17/TeamKeyNumeric+%28TeamMembers%29+Field)
    #[serde(rename = "TeamKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub team_key_numeric: Option<f64>,

    /// A system unique identifier. Specifically, the local key to the TeamMembers resource.
    ///
    /// [TeamMemberKey](https://ddwiki.reso.org/display/DDW17/TeamMemberKey+Field)
    #[serde(rename = "TeamMemberKey", skip_serializing_if = "Option::is_none")]
    pub team_member_key: Option<String>,

    /// A system unique identifier. Specifically, the local key to the TeamMembers resource. This is the numeric only key and used as an alternative to the TeamKey field.
    ///
    /// [TeamMemberKeyNumeric](https://ddwiki.reso.org/display/DDW17/TeamMemberKeyNumeric+Field)
    #[serde(
        rename = "TeamMemberKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_member_key_numeric: Option<f64>,

    /// The national association ID of the member. i.e. in the U.S. is the NRDS number.
    ///
    /// [TeamMemberNationalAssociationId](https://ddwiki.reso.org/display/DDW17/TeamMemberNationalAssociationId+Field)
    #[serde(
        rename = "TeamMemberNationalAssociationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_member_national_association_id: Option<String>,

    /// The license of the member. Separate multiple licenses with a comma and space.
    ///
    /// [TeamMemberStateLicense](https://ddwiki.reso.org/display/DDW17/TeamMemberStateLicense+Field)
    #[serde(
        rename = "TeamMemberStateLicense",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_member_state_license: Option<String>,

    /// The role of the member within the team. i.e. team lead, principle, associate, assistant, etc.
    ///
    /// [TeamMemberType](https://ddwiki.reso.org/display/DDW17/TeamMemberType+Field)
    #[serde(rename = "TeamMemberType", skip_serializing_if = "Option::is_none")]
    pub team_member_type: Option<String>,
}
