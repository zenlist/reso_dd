// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OtherPhoneType Lookups](https://ddwiki.reso.org/display/DDW17/OtherPhoneType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OtherPhoneType {
    /// "[Direct](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245746)": This is the contact's direct number.
    Direct,

    /// "[Fax](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245747)": This is the contact's fax.
    Fax,

    /// "[First](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245748)": This is the contact's first preferred phone.
    First,

    /// "[Home](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245749)": This is the contact's home phone.
    Home,

    /// "[Mobile](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245750)": This is the contact's mobile phone.
    Mobile,

    /// "[Modem](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245751)": This is the contact's modem.
    Modem,

    /// "[Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245752)": This is the contact's office phone.
    Office,

    /// "[Pager](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245753)": This is the contact's pager.
    Pager,

    /// "[Preferred](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245754)": This is the contact's preferred phone.
    Preferred,

    /// "[Second](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245755)": This is the contact's second preferred phone.
    Second,

    /// "[SMS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245756)": This is the contact's SMS/text number.
    SMS,

    /// "[Third](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245757)": This is the contact's third preferred phone.
    Third,

    /// "[Toll Free](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245758)": This is the contact's toll free number.
    TollFree,

    /// "[Voicemail](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245759)": This is the contact's voicemail.
    Voicemail,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for OtherPhoneType {
    fn from(s: String) -> OtherPhoneType {
        match s.as_ref() {
            "Direct" => OtherPhoneType::Direct,

            "Fax" => OtherPhoneType::Fax,

            "First" => OtherPhoneType::First,

            "Home" => OtherPhoneType::Home,

            "Mobile" => OtherPhoneType::Mobile,

            "Modem" => OtherPhoneType::Modem,

            "Office" => OtherPhoneType::Office,

            "Pager" => OtherPhoneType::Pager,

            "Preferred" => OtherPhoneType::Preferred,

            "Second" => OtherPhoneType::Second,

            "SMS" => OtherPhoneType::SMS,

            "Third" => OtherPhoneType::Third,

            "Toll Free" => OtherPhoneType::TollFree,

            "Voicemail" => OtherPhoneType::Voicemail,

            _ => OtherPhoneType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OtherPhoneType {
    fn from(s: &str) -> OtherPhoneType {
        match s {
            "Direct" => OtherPhoneType::Direct,

            "Fax" => OtherPhoneType::Fax,

            "First" => OtherPhoneType::First,

            "Home" => OtherPhoneType::Home,

            "Mobile" => OtherPhoneType::Mobile,

            "Modem" => OtherPhoneType::Modem,

            "Office" => OtherPhoneType::Office,

            "Pager" => OtherPhoneType::Pager,

            "Preferred" => OtherPhoneType::Preferred,

            "Second" => OtherPhoneType::Second,

            "SMS" => OtherPhoneType::SMS,

            "Third" => OtherPhoneType::Third,

            "Toll Free" => OtherPhoneType::TollFree,

            "Voicemail" => OtherPhoneType::Voicemail,

            _ => OtherPhoneType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OtherPhoneType> for &'a str {
    fn from(s: &'a OtherPhoneType) -> &'a str {
        match s {
            OtherPhoneType::Direct => "Direct",

            OtherPhoneType::Fax => "Fax",

            OtherPhoneType::First => "First",

            OtherPhoneType::Home => "Home",

            OtherPhoneType::Mobile => "Mobile",

            OtherPhoneType::Modem => "Modem",

            OtherPhoneType::Office => "Office",

            OtherPhoneType::Pager => "Pager",

            OtherPhoneType::Preferred => "Preferred",

            OtherPhoneType::Second => "Second",

            OtherPhoneType::SMS => "SMS",

            OtherPhoneType::Third => "Third",

            OtherPhoneType::TollFree => "Toll Free",

            OtherPhoneType::Voicemail => "Voicemail",

            OtherPhoneType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OtherPhoneType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OtherPhoneType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_other_phone_type_format {
    use super::OtherPhoneType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<OtherPhoneType>>,
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
    ) -> Result<Option<Vec<OtherPhoneType>>, D::Error>
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
