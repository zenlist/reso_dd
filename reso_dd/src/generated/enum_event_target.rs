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

pub(crate) mod option_vec_event_target_format {
    use super::EventTarget;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<EventTarget>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match items {
            None => return serializer.serialize_none(),
            Some(ref vec) if vec.len() == 0 => serializer.serialize_str(""),
            Some(ref vec) => {
                let items: Vec<&str> = vec.iter().map(|item| item.into()).collect();
                let joined = items.join(",");
                serializer.serialize_str(&joined)
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<EventTarget>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "" {
            return Ok(Some(vec![]));
        }

        let items = s.split(",").map(|i| From::<&str>::from(i)).collect();
        Ok(Some(items))
    }
}
