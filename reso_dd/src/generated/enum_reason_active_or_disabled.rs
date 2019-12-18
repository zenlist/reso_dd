// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ReasonActiveOrDisabled Lookups](https://ddwiki.reso.org/display/DDW17/ReasonActiveOrDisabled+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReasonActiveOrDisabled {
    /// "[Agent Disabled](https://ddwiki.reso.org/display/DDW17/Agent+Disabled)": The agent has disabled this auto email.
    AgentDisabled,

    /// "[Client Disabled](https://ddwiki.reso.org/display/DDW17/Client+Disabled)": The auto email has been disabled by the client/recipient.
    ClientDisabled,

    /// "[Concierge Notification](https://ddwiki.reso.org/display/DDW17/Concierge+Notification)": The auto email is on hold pending concierge approval by the member. The auto email is temporarily disabled.
    ConciergeNotification,

    /// "[Final Ignored Warning](https://ddwiki.reso.org/display/DDW17/Final+Ignored+Warning)": The final warning that the auto email has not been viewed by the client/recipient and may be inactivated due to being ignored. The auto email is still active.
    FinalIgnoredWarning,

    /// "[Ignored](https://ddwiki.reso.org/display/DDW17/Ignored)": The auto email was not viewed by the client/recipient in the time frame designated by the host system. The auto email is disabled.
    Ignored,

    /// "[Initial Ignored Warning](https://ddwiki.reso.org/display/DDW17/Initial+Ignored+Warning)": The first warning that the auto email has not been viewed by the client/recipient. The auto email is still active.
    InitialIgnoredWarning,

    /// "[Invalid](https://ddwiki.reso.org/display/DDW17/Invalid)": The auto email is no longer valid per some conditions set by the host system. The auto email is disabled.
    Invalid,

    /// "[No Listings Found](https://ddwiki.reso.org/display/DDW17/No+Listings+Found)": The auto email has not found any listings matching the criteria and been disabled per the host system rules.
    NoListingsFound,

    /// "[No Listings Found Warning](https://ddwiki.reso.org/display/DDW17/No+Listings+Found+Warning)": The auto email has not found any listings matching the criteria and may be disabled. The auto email is still active.
    NoListingsFoundWarning,

    /// "[No One To Send To](https://ddwiki.reso.org/display/DDW17/No+One+To+Send+To)": There is no valid email address and the auto email has been inactivated.
    NoOneToSendTo,

    /// "[Over Limit](https://ddwiki.reso.org/display/DDW17/Over+Limit)": The auto email has reached the limit of listing results as set by the host system. The auto email is disabled.
    OverLimit,

    /// "[Re-Activated](https://ddwiki.reso.org/display/DDW17/Re-Activated)": The auto email has was previously disabled and has been set back to active.
    ReActivated,

    /// "[Revised](https://ddwiki.reso.org/display/DDW17/Revised)": The auto email has been revised and is active.
    Revised,

    /// "[Search Failing](https://ddwiki.reso.org/display/DDW17/Search+Failing)": The auto email's search criteria is failing and should be reviewed by the host system. The auto email is disabled.
    SearchFailing,

    /// "[Welcome Email Ignored](https://ddwiki.reso.org/display/DDW17/Welcome+Email+Ignored)": The initial auto email has not been viewed by the client/recipient and the auto email has been deactivated.
    WelcomeEmailIgnored,

    /// "[Welcome Email Ignored Warning](https://ddwiki.reso.org/display/DDW17/Welcome+Email+Ignored+Warning)": The initial auto email has not been viewed by the client/recipient. The auto email is still active.
    WelcomeEmailIgnoredWarning,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for ReasonActiveOrDisabled {
    fn from(s: String) -> ReasonActiveOrDisabled {
        match s.as_ref() {
            "Agent Disabled" => ReasonActiveOrDisabled::AgentDisabled,

            "Client Disabled" => ReasonActiveOrDisabled::ClientDisabled,

            "Concierge Notification" => ReasonActiveOrDisabled::ConciergeNotification,

            "Final Ignored Warning" => ReasonActiveOrDisabled::FinalIgnoredWarning,

            "Ignored" => ReasonActiveOrDisabled::Ignored,

            "Initial Ignored Warning" => ReasonActiveOrDisabled::InitialIgnoredWarning,

            "Invalid" => ReasonActiveOrDisabled::Invalid,

            "No Listings Found" => ReasonActiveOrDisabled::NoListingsFound,

            "No Listings Found Warning" => ReasonActiveOrDisabled::NoListingsFoundWarning,

            "No One To Send To" => ReasonActiveOrDisabled::NoOneToSendTo,

            "Over Limit" => ReasonActiveOrDisabled::OverLimit,

            "Re-Activated" => ReasonActiveOrDisabled::ReActivated,

            "Revised" => ReasonActiveOrDisabled::Revised,

            "Search Failing" => ReasonActiveOrDisabled::SearchFailing,

            "Welcome Email Ignored" => ReasonActiveOrDisabled::WelcomeEmailIgnored,

            "Welcome Email Ignored Warning" => ReasonActiveOrDisabled::WelcomeEmailIgnoredWarning,

            _ => ReasonActiveOrDisabled::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ReasonActiveOrDisabled {
    fn from(s: &str) -> ReasonActiveOrDisabled {
        match s {
            "Agent Disabled" => ReasonActiveOrDisabled::AgentDisabled,

            "Client Disabled" => ReasonActiveOrDisabled::ClientDisabled,

            "Concierge Notification" => ReasonActiveOrDisabled::ConciergeNotification,

            "Final Ignored Warning" => ReasonActiveOrDisabled::FinalIgnoredWarning,

            "Ignored" => ReasonActiveOrDisabled::Ignored,

            "Initial Ignored Warning" => ReasonActiveOrDisabled::InitialIgnoredWarning,

            "Invalid" => ReasonActiveOrDisabled::Invalid,

            "No Listings Found" => ReasonActiveOrDisabled::NoListingsFound,

            "No Listings Found Warning" => ReasonActiveOrDisabled::NoListingsFoundWarning,

            "No One To Send To" => ReasonActiveOrDisabled::NoOneToSendTo,

            "Over Limit" => ReasonActiveOrDisabled::OverLimit,

            "Re-Activated" => ReasonActiveOrDisabled::ReActivated,

            "Revised" => ReasonActiveOrDisabled::Revised,

            "Search Failing" => ReasonActiveOrDisabled::SearchFailing,

            "Welcome Email Ignored" => ReasonActiveOrDisabled::WelcomeEmailIgnored,

            "Welcome Email Ignored Warning" => ReasonActiveOrDisabled::WelcomeEmailIgnoredWarning,

            _ => ReasonActiveOrDisabled::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ReasonActiveOrDisabled> for &'a str {
    fn from(s: &'a ReasonActiveOrDisabled) -> &'a str {
        match s {
            ReasonActiveOrDisabled::AgentDisabled => "Agent Disabled",

            ReasonActiveOrDisabled::ClientDisabled => "Client Disabled",

            ReasonActiveOrDisabled::ConciergeNotification => "Concierge Notification",

            ReasonActiveOrDisabled::FinalIgnoredWarning => "Final Ignored Warning",

            ReasonActiveOrDisabled::Ignored => "Ignored",

            ReasonActiveOrDisabled::InitialIgnoredWarning => "Initial Ignored Warning",

            ReasonActiveOrDisabled::Invalid => "Invalid",

            ReasonActiveOrDisabled::NoListingsFound => "No Listings Found",

            ReasonActiveOrDisabled::NoListingsFoundWarning => "No Listings Found Warning",

            ReasonActiveOrDisabled::NoOneToSendTo => "No One To Send To",

            ReasonActiveOrDisabled::OverLimit => "Over Limit",

            ReasonActiveOrDisabled::ReActivated => "Re-Activated",

            ReasonActiveOrDisabled::Revised => "Revised",

            ReasonActiveOrDisabled::SearchFailing => "Search Failing",

            ReasonActiveOrDisabled::WelcomeEmailIgnored => "Welcome Email Ignored",

            ReasonActiveOrDisabled::WelcomeEmailIgnoredWarning => "Welcome Email Ignored Warning",

            ReasonActiveOrDisabled::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ReasonActiveOrDisabled {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ReasonActiveOrDisabled {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_reason_active_or_disabled_format {
    use super::ReasonActiveOrDisabled;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<ReasonActiveOrDisabled>>,
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
    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<Vec<ReasonActiveOrDisabled>>, D::Error>
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
