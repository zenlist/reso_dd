// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [EventTarget Lookups](https://ddwiki.reso.org/display/DDW17/EventTarget+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EventTarget {
    /// "[Agent](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244640)": The EventType used a destination pertaining to the listing agent (i.e. the actor's submission of lead form went to the Listing Agent's contact information)
    Agent,

    /// "[Broker](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244641)": The EventType used a destination pertaining to the listing broker (i.e. the "Clicked on Phone Number" EventType is using the Broker's contact information)
    Broker,

    /// "[Digg](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244642)": The Object was shared on Digg
    Digg,

    /// "[Email](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244643)": The Object was sent in an email
    Email,

    /// "[Facebook](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244644)": The Object was shared on Facebook
    Facebook,

    /// "[Facebook Messenger](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244645)": The Object was shared via Facebook Messenger
    FacebookMessenger,

    /// "[GooglePlus](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244646)": The Object was shared to Google Plus
    GooglePlus,

    /// "[iMessage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244647)": The Object was shared via iMessage
    IMessage,

    /// "[Instagram](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244648)": The Object was shared on Instagram
    Instagram,

    /// "[LinkedIn](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244649)": The Object was shared on LinkedIn
    LinkedIn,

    /// "[Pinterest](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244650)": The Object was pinned on Pinterest
    Pinterest,

    /// "[Reddit](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244651)": The Object was shared on Reddit
    Reddit,

    /// "[Slack](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244652)": The Object was shared via Slack
    Slack,

    /// "[SMS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244653)": The Object was sent in an SMS message
    SMS,

    /// "[Snapchat](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244654)": The Object was shared on Snapchat
    Snapchat,

    /// "[StumbleUpon](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244655)": The Object was shared on StumbleUpon
    StumbleUpon,

    /// "[Tumblr](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244656)": The Object was shared on Tumblr
    Tumblr,

    /// "[Twitter](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244657)": The Object was tweeted on Twitter
    Twitter,

    /// "[YouTube](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244658)": The Object was shared on YouTube
    YouTube,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for EventTarget {
    fn from_str(s: &str) -> EventTarget {
        match s {
            "Agent" => EventTarget::Agent,

            "Broker" => EventTarget::Broker,

            "Digg" => EventTarget::Digg,

            "Email" => EventTarget::Email,

            "Facebook" => EventTarget::Facebook,

            "Facebook Messenger" => EventTarget::FacebookMessenger,

            "GooglePlus" => EventTarget::GooglePlus,

            "iMessage" => EventTarget::IMessage,

            "Instagram" => EventTarget::Instagram,

            "LinkedIn" => EventTarget::LinkedIn,

            "Pinterest" => EventTarget::Pinterest,

            "Reddit" => EventTarget::Reddit,

            "Slack" => EventTarget::Slack,

            "SMS" => EventTarget::SMS,

            "Snapchat" => EventTarget::Snapchat,

            "StumbleUpon" => EventTarget::StumbleUpon,

            "Tumblr" => EventTarget::Tumblr,

            "Twitter" => EventTarget::Twitter,

            "YouTube" => EventTarget::YouTube,

            _ => EventTarget::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> EventTarget {
        match s.as_ref() {
            "Agent" => EventTarget::Agent,

            "Broker" => EventTarget::Broker,

            "Digg" => EventTarget::Digg,

            "Email" => EventTarget::Email,

            "Facebook" => EventTarget::Facebook,

            "Facebook Messenger" => EventTarget::FacebookMessenger,

            "GooglePlus" => EventTarget::GooglePlus,

            "iMessage" => EventTarget::IMessage,

            "Instagram" => EventTarget::Instagram,

            "LinkedIn" => EventTarget::LinkedIn,

            "Pinterest" => EventTarget::Pinterest,

            "Reddit" => EventTarget::Reddit,

            "Slack" => EventTarget::Slack,

            "SMS" => EventTarget::SMS,

            "Snapchat" => EventTarget::Snapchat,

            "StumbleUpon" => EventTarget::StumbleUpon,

            "Tumblr" => EventTarget::Tumblr,

            "Twitter" => EventTarget::Twitter,

            "YouTube" => EventTarget::YouTube,

            _ => EventTarget::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            EventTarget::Agent => "Agent",

            EventTarget::Broker => "Broker",

            EventTarget::Digg => "Digg",

            EventTarget::Email => "Email",

            EventTarget::Facebook => "Facebook",

            EventTarget::FacebookMessenger => "Facebook Messenger",

            EventTarget::GooglePlus => "GooglePlus",

            EventTarget::IMessage => "iMessage",

            EventTarget::Instagram => "Instagram",

            EventTarget::LinkedIn => "LinkedIn",

            EventTarget::Pinterest => "Pinterest",

            EventTarget::Reddit => "Reddit",

            EventTarget::Slack => "Slack",

            EventTarget::SMS => "SMS",

            EventTarget::Snapchat => "Snapchat",

            EventTarget::StumbleUpon => "StumbleUpon",

            EventTarget::Tumblr => "Tumblr",

            EventTarget::Twitter => "Twitter",

            EventTarget::YouTube => "YouTube",

            EventTarget::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            EventTarget::Agent => "Agent".into(),

            EventTarget::Broker => "Broker".into(),

            EventTarget::Digg => "Digg".into(),

            EventTarget::Email => "Email".into(),

            EventTarget::Facebook => "Facebook".into(),

            EventTarget::FacebookMessenger => "Facebook Messenger".into(),

            EventTarget::GooglePlus => "GooglePlus".into(),

            EventTarget::IMessage => "iMessage".into(),

            EventTarget::Instagram => "Instagram".into(),

            EventTarget::LinkedIn => "LinkedIn".into(),

            EventTarget::Pinterest => "Pinterest".into(),

            EventTarget::Reddit => "Reddit".into(),

            EventTarget::Slack => "Slack".into(),

            EventTarget::SMS => "SMS".into(),

            EventTarget::Snapchat => "Snapchat".into(),

            EventTarget::StumbleUpon => "StumbleUpon".into(),

            EventTarget::Tumblr => "Tumblr".into(),

            EventTarget::Twitter => "Twitter".into(),

            EventTarget::YouTube => "YouTube".into(),

            EventTarget::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            EventTarget::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for EventTarget {
    fn from(s: String) -> EventTarget {
        match s.as_ref() {
            "Agent" => EventTarget::Agent,

            "Broker" => EventTarget::Broker,

            "Digg" => EventTarget::Digg,

            "Email" => EventTarget::Email,

            "Facebook" => EventTarget::Facebook,

            "Facebook Messenger" => EventTarget::FacebookMessenger,

            "GooglePlus" => EventTarget::GooglePlus,

            "iMessage" => EventTarget::IMessage,

            "Instagram" => EventTarget::Instagram,

            "LinkedIn" => EventTarget::LinkedIn,

            "Pinterest" => EventTarget::Pinterest,

            "Reddit" => EventTarget::Reddit,

            "Slack" => EventTarget::Slack,

            "SMS" => EventTarget::SMS,

            "Snapchat" => EventTarget::Snapchat,

            "StumbleUpon" => EventTarget::StumbleUpon,

            "Tumblr" => EventTarget::Tumblr,

            "Twitter" => EventTarget::Twitter,

            "YouTube" => EventTarget::YouTube,

            _ => EventTarget::OpenEnumeration(s),
        }
    }
}

impl From<&str> for EventTarget {
    fn from(s: &str) -> EventTarget {
        match s {
            "Agent" => EventTarget::Agent,

            "Broker" => EventTarget::Broker,

            "Digg" => EventTarget::Digg,

            "Email" => EventTarget::Email,

            "Facebook" => EventTarget::Facebook,

            "Facebook Messenger" => EventTarget::FacebookMessenger,

            "GooglePlus" => EventTarget::GooglePlus,

            "iMessage" => EventTarget::IMessage,

            "Instagram" => EventTarget::Instagram,

            "LinkedIn" => EventTarget::LinkedIn,

            "Pinterest" => EventTarget::Pinterest,

            "Reddit" => EventTarget::Reddit,

            "Slack" => EventTarget::Slack,

            "SMS" => EventTarget::SMS,

            "Snapchat" => EventTarget::Snapchat,

            "StumbleUpon" => EventTarget::StumbleUpon,

            "Tumblr" => EventTarget::Tumblr,

            "Twitter" => EventTarget::Twitter,

            "YouTube" => EventTarget::YouTube,

            _ => EventTarget::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a EventTarget> for &'a str {
    fn from(s: &'a EventTarget) -> &'a str {
        match s {
            EventTarget::Agent => "Agent",

            EventTarget::Broker => "Broker",

            EventTarget::Digg => "Digg",

            EventTarget::Email => "Email",

            EventTarget::Facebook => "Facebook",

            EventTarget::FacebookMessenger => "Facebook Messenger",

            EventTarget::GooglePlus => "GooglePlus",

            EventTarget::IMessage => "iMessage",

            EventTarget::Instagram => "Instagram",

            EventTarget::LinkedIn => "LinkedIn",

            EventTarget::Pinterest => "Pinterest",

            EventTarget::Reddit => "Reddit",

            EventTarget::Slack => "Slack",

            EventTarget::SMS => "SMS",

            EventTarget::Snapchat => "Snapchat",

            EventTarget::StumbleUpon => "StumbleUpon",

            EventTarget::Tumblr => "Tumblr",

            EventTarget::Twitter => "Twitter",

            EventTarget::YouTube => "YouTube",

            EventTarget::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for EventTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for EventTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
