// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [SocialMediaType Lookups](https://ddwiki.reso.org/display/DDW17/SocialMediaType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SocialMediaType {
    /// "[Blog](https://ddwiki.reso.org/display/DDW17/Blog)": SocialMedia[Type]UrlOrId has a URL or ID that relates to the member/office/contact's blog.
    Blog,

    /// "[Digg](https://ddwiki.reso.org/display/DDW17/Digg)": SocialMedia[Type]UrlOrId has a URL or ID that relates to Digg.
    Digg,

    /// "[Facebook](https://ddwiki.reso.org/display/DDW17/Facebook)": SocialMedia[Type]UrlOrId has a URL or ID that relates to Facebook.
    Facebook,

    /// "[Facebook Messenger](https://ddwiki.reso.org/display/DDW17/Facebook+Messenger)": SocialMedia[Type]UrlOrId has a URL or ID that relates to Facebook Messenger.
    FacebookMessenger,

    /// "[GooglePlus](https://ddwiki.reso.org/display/DDW17/GooglePlus)": SocialMedia[Type]UrlOrId has a URL or ID that relates to GooglePlus.
    GooglePlus,

    /// "[iMessage](https://ddwiki.reso.org/display/DDW17/iMessage)": SocialMedia[Type]UrlOrId has a URL or ID that relates to iMessage.
    IMessage,

    /// "[Instagram](https://ddwiki.reso.org/display/DDW17/Instagram)": SocialMedia[Type]UrlOrId has a URL or ID that relates to Instagram.
    Instagram,

    /// "[LinkedIn](https://ddwiki.reso.org/display/DDW17/LinkedIn)": SocialMedia[Type]UrlOrId has a URL or ID that relates to LinkedIn.
    LinkedIn,

    /// "[Pinterest](https://ddwiki.reso.org/display/DDW17/Pinterest)": SocialMedia[Type]UrlOrId has a URL or ID that relates to Pinterest.
    Pinterest,

    /// "[Reddit](https://ddwiki.reso.org/display/DDW17/Reddit)": SocialMedia[Type]UrlOrId has a URL or ID that relates to Reddit.
    Reddit,

    /// "[Slack](https://ddwiki.reso.org/display/DDW17/Slack)": SocialMedia[Type]UrlOrId has a URL or ID that relates to Slack.
    Slack,

    /// "[Snapchat](https://ddwiki.reso.org/display/DDW17/Snapchat)": SocialMedia[Type]UrlOrId has a URL or ID that relates to Snapchat.
    Snapchat,

    /// "[StumbleUpon](https://ddwiki.reso.org/display/DDW17/StumbleUpon)": SocialMedia[Type]UrlOrId has a URL or ID that relates to StumbleUpon.
    StumbleUpon,

    /// "[Tumblr](https://ddwiki.reso.org/display/DDW17/Tumblr)": SocialMedia[Type]UrlOrId has a URL or ID that relates to Tumblr.
    Tumblr,

    /// "[Twitter](https://ddwiki.reso.org/display/DDW17/Twitter)": SocialMedia[Type]UrlOrId has a URL or ID that relates to Twitter.
    Twitter,

    /// "[Website](https://ddwiki.reso.org/display/DDW17/Website)": SocialMedia[Type]UrlOrId has a URL or ID that relates to the member/office/contact's website.
    Website,

    /// "[YouTube](https://ddwiki.reso.org/display/DDW17/YouTube)": SocialMedia[Type]UrlOrId has a URL or ID that relates to YouTube.
    YouTube,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for SocialMediaType {
    fn from(s: String) -> SocialMediaType {
        match s.as_ref() {
            "Blog" => SocialMediaType::Blog,

            "Digg" => SocialMediaType::Digg,

            "Facebook" => SocialMediaType::Facebook,

            "Facebook Messenger" => SocialMediaType::FacebookMessenger,

            "GooglePlus" => SocialMediaType::GooglePlus,

            "iMessage" => SocialMediaType::IMessage,

            "Instagram" => SocialMediaType::Instagram,

            "LinkedIn" => SocialMediaType::LinkedIn,

            "Pinterest" => SocialMediaType::Pinterest,

            "Reddit" => SocialMediaType::Reddit,

            "Slack" => SocialMediaType::Slack,

            "Snapchat" => SocialMediaType::Snapchat,

            "StumbleUpon" => SocialMediaType::StumbleUpon,

            "Tumblr" => SocialMediaType::Tumblr,

            "Twitter" => SocialMediaType::Twitter,

            "Website" => SocialMediaType::Website,

            "YouTube" => SocialMediaType::YouTube,

            _ => SocialMediaType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for SocialMediaType {
    fn from(s: &str) -> SocialMediaType {
        match s {
            "Blog" => SocialMediaType::Blog,

            "Digg" => SocialMediaType::Digg,

            "Facebook" => SocialMediaType::Facebook,

            "Facebook Messenger" => SocialMediaType::FacebookMessenger,

            "GooglePlus" => SocialMediaType::GooglePlus,

            "iMessage" => SocialMediaType::IMessage,

            "Instagram" => SocialMediaType::Instagram,

            "LinkedIn" => SocialMediaType::LinkedIn,

            "Pinterest" => SocialMediaType::Pinterest,

            "Reddit" => SocialMediaType::Reddit,

            "Slack" => SocialMediaType::Slack,

            "Snapchat" => SocialMediaType::Snapchat,

            "StumbleUpon" => SocialMediaType::StumbleUpon,

            "Tumblr" => SocialMediaType::Tumblr,

            "Twitter" => SocialMediaType::Twitter,

            "Website" => SocialMediaType::Website,

            "YouTube" => SocialMediaType::YouTube,

            _ => SocialMediaType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a SocialMediaType> for &'a str {
    fn from(s: &'a SocialMediaType) -> &'a str {
        match s {
            SocialMediaType::Blog => "Blog",

            SocialMediaType::Digg => "Digg",

            SocialMediaType::Facebook => "Facebook",

            SocialMediaType::FacebookMessenger => "Facebook Messenger",

            SocialMediaType::GooglePlus => "GooglePlus",

            SocialMediaType::IMessage => "iMessage",

            SocialMediaType::Instagram => "Instagram",

            SocialMediaType::LinkedIn => "LinkedIn",

            SocialMediaType::Pinterest => "Pinterest",

            SocialMediaType::Reddit => "Reddit",

            SocialMediaType::Slack => "Slack",

            SocialMediaType::Snapchat => "Snapchat",

            SocialMediaType::StumbleUpon => "StumbleUpon",

            SocialMediaType::Tumblr => "Tumblr",

            SocialMediaType::Twitter => "Twitter",

            SocialMediaType::Website => "Website",

            SocialMediaType::YouTube => "YouTube",

            SocialMediaType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for SocialMediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SocialMediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_social_media_type_format {
    use super::SocialMediaType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<SocialMediaType>>,
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
    ) -> Result<Option<Vec<SocialMediaType>>, D::Error>
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
