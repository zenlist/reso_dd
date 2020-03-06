// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// [Prospecting Resource](https://ddwiki.reso.org/display/DDW17/Prospecting+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Prospecting {
    /// If set to True, the given auto email is active.  False records may be disabled or waiting activation.
    ///
    /// [ActiveYN](https://ddwiki.reso.org/display/DDW17/ActiveYN+Field)
    #[serde(rename = "ActiveYN", skip_serializing_if = "Option::is_none")]
    pub active_yn: Option<bool>,

    /// A comma separate list of email addresses that are the "BCC", or Blind Carbon Copy, address the auto emails are being sent to.
    ///
    /// [BccEmailList](https://ddwiki.reso.org/display/DDW17/BccEmailList+Field)
    #[serde(rename = "BccEmailList", skip_serializing_if = "Option::is_none")]
    pub bcc_email_list: Option<String>,

    /// When set to True, the auto mail is also sent as a Blind Carbon Copy to the Member who created the Auto Email.
    ///
    /// [BccMeYN](https://ddwiki.reso.org/display/DDW17/BccMeYN+Field)
    #[serde(rename = "BccMeYN", skip_serializing_if = "Option::is_none")]
    pub bcc_me_yn: Option<bool>,

    /// A comma separate list of email addresses that are the "CC", or Carbon Copy, address the auto emails are being sent to.
    ///
    /// [CcEmailList](https://ddwiki.reso.org/display/DDW17/CcEmailList+Field)
    #[serde(rename = "CcEmailList", skip_serializing_if = "Option::is_none")]
    pub cc_email_list: Option<String>,

    /// If set to True, the client has clicked through to accept automatic of emails.  Recipant acceptance is an important part of CAN-SPAM and other bulk automatic emailing regulations.
    ///
    /// [ClientActivatedYN](https://ddwiki.reso.org/display/DDW17/ClientActivatedYN+Field)
    #[serde(rename = "ClientActivatedYN", skip_serializing_if = "Option::is_none")]
    pub client_activated_yn: Option<bool>,

    /// If set to True, notifications are to be sent to the member when the auto email is in Concierge mode.
    ///
    /// [ConciergeNotificationsYN](https://ddwiki.reso.org/display/DDW17/ConciergeNotificationsYN+Field)
    #[serde(
        rename = "ConciergeNotificationsYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub concierge_notifications_yn: Option<bool>,

    /// When set to True, the auto mail is in Concierge mode and to be approved by the member before sent to the client.
    ///
    /// [ConciergeYN](https://ddwiki.reso.org/display/DDW17/ConciergeYN+Field)
    #[serde(rename = "ConciergeYN", skip_serializing_if = "Option::is_none")]
    pub concierge_yn: Option<bool>,

    /// This is the foreign key relating to the Contact resource. A unique identifier for this record from the immediate source. This is a string that can include URI or other forms. Alternatively use the ContactKeyNumeric for a numeric only key field. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey variants.
    ///
    /// [ContactKey](https://ddwiki.reso.org/display/DDW17/ContactKey+%28Prospecting%29+Field)
    #[serde(rename = "ContactKey", skip_serializing_if = "Option::is_none")]
    pub contact_key: Option<String>,

    /// This is the foreign key relating to the Contact resource. A unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the ContactKey fields. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, see SourceSystemKeyNumeric and OriginatingSystemKeyNumeric variants.
    ///
    /// [ContactKeyNumeric](https://ddwiki.reso.org/display/DDW17/ContactKeyNumeric+%28Prospecting%29+Field)
    #[serde(rename = "ContactKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub contact_key_numeric: Option<f64>,

    /// When Daily is selected as the ScheduleType, a list of days of the week and AM or PM options.
    ///
    /// [DailySchedule](https://ddwiki.reso.org/display/DDW17/DailySchedule+Field)
    #[serde(rename = "DailySchedule", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub daily_schedule: Option<Vec<crate::DailySchedule>>,

    /// The system ID of the display that has been related, or set as the default, to this saved search.
    ///
    /// [DisplayTemplateID](https://ddwiki.reso.org/display/DDW17/DisplayTemplateID+Field)
    #[serde(rename = "DisplayTemplateID", skip_serializing_if = "Option::is_none")]
    pub display_template_id: Option<String>,

    /// The language that will be used in the given auto email.
    ///
    /// [Language](https://ddwiki.reso.org/display/DDW17/Language+%28Prospecting%29+Field)
    #[serde(rename = "Language", skip_serializing_if = "Option::is_none")]
    pub language: Option<crate::Languages>,

    /// Timestamp of when the prospector last found new or modified listings for this auto-email.
    ///
    /// [LastNewChangedTimestamp](https://ddwiki.reso.org/display/DDW17/LastNewChangedTimestamp+Field)
    #[serde(
        rename = "LastNewChangedTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_new_changed_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// A timestamp of when the auto email was last viewed by the client in the portal.
    ///
    /// [LastViewedTimestamp](https://ddwiki.reso.org/display/DDW17/LastViewedTimestamp+Field)
    #[serde(
        rename = "LastViewedTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_viewed_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The body of the auto email message when the first email is sent for the prospecting campaign.
    ///
    /// [MessageNew](https://ddwiki.reso.org/display/DDW17/MessageNew+Field)
    #[serde(rename = "MessageNew", skip_serializing_if = "Option::is_none")]
    pub message_new: Option<String>,

    /// The body of the auto email message to be used when the criteria or settings of this auto email have been modified.
    ///
    /// [MessageRevise](https://ddwiki.reso.org/display/DDW17/MessageRevise+Field)
    #[serde(rename = "MessageRevise", skip_serializing_if = "Option::is_none")]
    pub message_revise: Option<String>,

    /// The body of the auto email message for subsequent email messages after the first email is sent.  If a first email option isn't used, this field is used for all emails including the first.
    ///
    /// [MessageUpdate](https://ddwiki.reso.org/display/DDW17/MessageUpdate+Field)
    #[serde(rename = "MessageUpdate", skip_serializing_if = "Option::is_none")]
    pub message_update: Option<String>,

    /// The date/time this prospecting record was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28Prospecting%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// A timestamp of when the auto email is schedule to next send.
    ///
    /// [NextSendTimestamp](https://ddwiki.reso.org/display/DDW17/NextSendTimestamp+Field)
    #[serde(rename = "NextSendTimestamp", skip_serializing_if = "Option::is_none")]
    pub next_send_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The local, well-known identifier for the member owning the contact.
    ///
    /// [OwnerMemberID](https://ddwiki.reso.org/display/DDW17/OwnerMemberID+%28Prospecting%29+Field)
    #[serde(rename = "OwnerMemberID", skip_serializing_if = "Option::is_none")]
    pub owner_member_id: Option<String>,

    /// The unique identifier (key) of the member owning the contact. This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [OwnerMemberKey](https://ddwiki.reso.org/display/DDW17/OwnerMemberKey+%28Prospecting%29+Field)
    #[serde(rename = "OwnerMemberKey", skip_serializing_if = "Option::is_none")]
    pub owner_member_key: Option<String>,

    /// The unique identifier (key) of the member owning the contact. This is a foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the OwnerMemberKey field.
    ///
    /// [OwnerMemberKeyNumeric](https://ddwiki.reso.org/display/DDW17/OwnerMemberKeyNumeric+%28Prospecting%29+Field)
    #[serde(
        rename = "OwnerMemberKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_member_key_numeric: Option<f64>,

    /// A unique identifier for this record from the immediate source. This is a string that can include URI or other forms.  Alternatively use the ProspectKeyNumeric for a numeric only key field.  This is the local key of the system.  When records are received from other systems, a local key is commonly applied.  If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey variants.
    ///
    /// [ProspectingKey](https://ddwiki.reso.org/display/DDW17/ProspectingKey+Field)
    #[serde(rename = "ProspectingKey", skip_serializing_if = "Option::is_none")]
    pub prospecting_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the ProspectKey fields.  This is the local key of the system.  When records are received from other systems, a local key is commonly applied.  If conveying the original keys from the source or originating systems, see SourceSystemKeyNumeric and OriginatingSystemKeyNumeric variants.
    ///
    /// [ProspectingKeyNumeric](https://ddwiki.reso.org/display/DDW17/ProspectingKeyNumeric+Field)
    #[serde(
        rename = "ProspectingKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub prospecting_key_numeric: Option<f64>,

    /// A list of reasons the Auto Email was set to inactive or set back to active.  e.g. AgentDisabled, OverLimit, No Listings Found, Re-Activated, Client Disabled, etc.
    ///
    /// [ReasonActiveOrDisabled](https://ddwiki.reso.org/display/DDW17/ReasonActiveOrDisabled+Field)
    #[serde(
        rename = "ReasonActiveOrDisabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub reason_active_or_disabled: Option<crate::ReasonActiveOrDisabled>,

    /// This is the foreign key relating to the SavedSearch resource. A unique identifier for this record from the immediate source. This is a string that can include URI or other forms. Alternatively use the SavedSearchKeyNumeric for a numeric only key field. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey variants.
    ///
    /// [SavedSearchKey](https://ddwiki.reso.org/display/DDW17/SavedSearchKey+%28Prospecting%29+Field)
    #[serde(rename = "SavedSearchKey", skip_serializing_if = "Option::is_none")]
    pub saved_search_key: Option<String>,

    /// This is the foreign key relating to the SavedSearch resource. A unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the SavedSearchKey fields. This is the local key of the system. When records are received from other systems, a local key is commonly applied. If conveying the original keys from the source or originating systems, see SourceSystemKeyNumeric and OriginatingSystemKeyNumeric variants.
    ///
    /// [SavedSearchKeyNumeric](https://ddwiki.reso.org/display/DDW17/SavedSearchKeyNumeric+%28Prospecting%29+Field)
    #[serde(
        rename = "SavedSearchKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub saved_search_key_numeric: Option<f64>,

    /// A selection of the type of schedule that the auto email will be sent.  Daily, Monthly or ASAP.
    ///
    /// [ScheduleType](https://ddwiki.reso.org/display/DDW17/ScheduleType+Field)
    #[serde(rename = "ScheduleType", skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<crate::ScheduleType>,

    /// The subject line of the auto email being sent.
    ///
    /// [Subject](https://ddwiki.reso.org/display/DDW17/Subject+Field)
    #[serde(rename = "Subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// A comma separate list of email addresses that are the "To" address the auto emails are being sent to.
    ///
    /// [ToEmailList](https://ddwiki.reso.org/display/DDW17/ToEmailList+Field)
    #[serde(rename = "ToEmailList", skip_serializing_if = "Option::is_none")]
    pub to_email_list: Option<String>,
}
