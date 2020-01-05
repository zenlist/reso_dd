// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [PreferredPhone Lookups](https://ddwiki.reso.org/display/DDW17/PreferredPhone+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PreferredPhone {
    /// "[Direct](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245957)": The contact prefers the use of their direct phone.
    Direct,

    /// "[Home](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245958)": The contact prefers the use of their home phone.
    Home,

    /// "[Mobile](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245959)": The contact prefers the use of their mobile phone.
    Mobile,

    /// "[Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245960)": The contact prefers the use of their office phone.
    Office,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245961)": The contact prefers the use of their other phone.
    Other,

    /// "[Toll Free](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245962)": The contact prefers the use of their toll free phone.
    TollFree,

    /// "[Voicemail](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245963)": The contact prefers the use of their voicemail phone.
    Voicemail,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for PreferredPhone {
    fn from_str(s: &str) -> PreferredPhone {
        match s {
            "Direct" => PreferredPhone::Direct,

            "Home" => PreferredPhone::Home,

            "Mobile" => PreferredPhone::Mobile,

            "Office" => PreferredPhone::Office,

            "Other" => PreferredPhone::Other,

            "Toll Free" => PreferredPhone::TollFree,

            "Voicemail" => PreferredPhone::Voicemail,

            _ => PreferredPhone::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> PreferredPhone {
        match s.as_ref() {
            "Direct" => PreferredPhone::Direct,

            "Home" => PreferredPhone::Home,

            "Mobile" => PreferredPhone::Mobile,

            "Office" => PreferredPhone::Office,

            "Other" => PreferredPhone::Other,

            "Toll Free" => PreferredPhone::TollFree,

            "Voicemail" => PreferredPhone::Voicemail,

            _ => PreferredPhone::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            PreferredPhone::Direct => "Direct",

            PreferredPhone::Home => "Home",

            PreferredPhone::Mobile => "Mobile",

            PreferredPhone::Office => "Office",

            PreferredPhone::Other => "Other",

            PreferredPhone::TollFree => "Toll Free",

            PreferredPhone::Voicemail => "Voicemail",

            PreferredPhone::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            PreferredPhone::Direct => "Direct".into(),

            PreferredPhone::Home => "Home".into(),

            PreferredPhone::Mobile => "Mobile".into(),

            PreferredPhone::Office => "Office".into(),

            PreferredPhone::Other => "Other".into(),

            PreferredPhone::TollFree => "Toll Free".into(),

            PreferredPhone::Voicemail => "Voicemail".into(),

            PreferredPhone::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            PreferredPhone::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for PreferredPhone {
    fn from(s: String) -> PreferredPhone {
        match s.as_ref() {
            "Direct" => PreferredPhone::Direct,

            "Home" => PreferredPhone::Home,

            "Mobile" => PreferredPhone::Mobile,

            "Office" => PreferredPhone::Office,

            "Other" => PreferredPhone::Other,

            "Toll Free" => PreferredPhone::TollFree,

            "Voicemail" => PreferredPhone::Voicemail,

            _ => PreferredPhone::OpenEnumeration(s),
        }
    }
}

impl From<&str> for PreferredPhone {
    fn from(s: &str) -> PreferredPhone {
        match s {
            "Direct" => PreferredPhone::Direct,

            "Home" => PreferredPhone::Home,

            "Mobile" => PreferredPhone::Mobile,

            "Office" => PreferredPhone::Office,

            "Other" => PreferredPhone::Other,

            "Toll Free" => PreferredPhone::TollFree,

            "Voicemail" => PreferredPhone::Voicemail,

            _ => PreferredPhone::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a PreferredPhone> for &'a str {
    fn from(s: &'a PreferredPhone) -> &'a str {
        match s {
            PreferredPhone::Direct => "Direct",

            PreferredPhone::Home => "Home",

            PreferredPhone::Mobile => "Mobile",

            PreferredPhone::Office => "Office",

            PreferredPhone::Other => "Other",

            PreferredPhone::TollFree => "Toll Free",

            PreferredPhone::Voicemail => "Voicemail",

            PreferredPhone::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for PreferredPhone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PreferredPhone {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
