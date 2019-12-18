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

pub(crate) mod option_vec_preferred_phone_format {
    use super::PreferredPhone;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<PreferredPhone>>,
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
    ) -> Result<Option<Vec<PreferredPhone>>, D::Error>
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
