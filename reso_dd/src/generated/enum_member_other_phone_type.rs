// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [MemberOtherPhoneType Lookups](https://ddwiki.reso.org/display/DDW17/MemberOtherPhoneType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemberOtherPhoneType {
    /// "[Direct](https://ddwiki.reso.org/display/DDW17/Direct)": This is the member's direct number.
    Direct,

    /// "[Fax](https://ddwiki.reso.org/display/DDW17/Fax)": This is the member's fax.
    Fax,

    /// "[First](https://ddwiki.reso.org/display/DDW17/First)": This is the member's first preferred phone.
    First,

    /// "[Home](https://ddwiki.reso.org/display/DDW17/Home)": This is the member's home phone.
    Home,

    /// "[Mobile](https://ddwiki.reso.org/display/DDW17/Mobile)": This is the member's mobile phone.
    Mobile,

    /// "[Modem](https://ddwiki.reso.org/display/DDW17/Modem)": This is the member's modem.
    Modem,

    /// "[Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245636)": This is the member's office phone.
    Office,

    /// "[Pager](https://ddwiki.reso.org/display/DDW17/Pager)": This is the member's pager.
    Pager,

    /// "[Preferred](https://ddwiki.reso.org/display/DDW17/Preferred)": This is the member's preferred phone.
    Preferred,

    /// "[Second](https://ddwiki.reso.org/display/DDW17/Second)": This is the member's second preferred phone.
    Second,

    /// "[SMS](https://ddwiki.reso.org/display/DDW17/SMS)": This is the member's SMS/text number.
    SMS,

    /// "[Third](https://ddwiki.reso.org/display/DDW17/Third)": This is the member's third preferred phone.
    Third,

    /// "[Toll Free](https://ddwiki.reso.org/display/DDW17/Toll+Free)": This is the member's toll free number.
    TollFree,

    /// "[Voicemail](https://ddwiki.reso.org/display/DDW17/Voicemail)": This is the member's voicemail.
    Voicemail,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for MemberOtherPhoneType {
    fn from(s: String) -> MemberOtherPhoneType {
        match s.as_ref() {
            "Direct" => MemberOtherPhoneType::Direct,

            "Fax" => MemberOtherPhoneType::Fax,

            "First" => MemberOtherPhoneType::First,

            "Home" => MemberOtherPhoneType::Home,

            "Mobile" => MemberOtherPhoneType::Mobile,

            "Modem" => MemberOtherPhoneType::Modem,

            "Office" => MemberOtherPhoneType::Office,

            "Pager" => MemberOtherPhoneType::Pager,

            "Preferred" => MemberOtherPhoneType::Preferred,

            "Second" => MemberOtherPhoneType::Second,

            "SMS" => MemberOtherPhoneType::SMS,

            "Third" => MemberOtherPhoneType::Third,

            "Toll Free" => MemberOtherPhoneType::TollFree,

            "Voicemail" => MemberOtherPhoneType::Voicemail,

            _ => MemberOtherPhoneType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for MemberOtherPhoneType {
    fn from(s: &str) -> MemberOtherPhoneType {
        match s {
            "Direct" => MemberOtherPhoneType::Direct,

            "Fax" => MemberOtherPhoneType::Fax,

            "First" => MemberOtherPhoneType::First,

            "Home" => MemberOtherPhoneType::Home,

            "Mobile" => MemberOtherPhoneType::Mobile,

            "Modem" => MemberOtherPhoneType::Modem,

            "Office" => MemberOtherPhoneType::Office,

            "Pager" => MemberOtherPhoneType::Pager,

            "Preferred" => MemberOtherPhoneType::Preferred,

            "Second" => MemberOtherPhoneType::Second,

            "SMS" => MemberOtherPhoneType::SMS,

            "Third" => MemberOtherPhoneType::Third,

            "Toll Free" => MemberOtherPhoneType::TollFree,

            "Voicemail" => MemberOtherPhoneType::Voicemail,

            _ => MemberOtherPhoneType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a MemberOtherPhoneType> for &'a str {
    fn from(s: &'a MemberOtherPhoneType) -> &'a str {
        match s {
            MemberOtherPhoneType::Direct => "Direct",

            MemberOtherPhoneType::Fax => "Fax",

            MemberOtherPhoneType::First => "First",

            MemberOtherPhoneType::Home => "Home",

            MemberOtherPhoneType::Mobile => "Mobile",

            MemberOtherPhoneType::Modem => "Modem",

            MemberOtherPhoneType::Office => "Office",

            MemberOtherPhoneType::Pager => "Pager",

            MemberOtherPhoneType::Preferred => "Preferred",

            MemberOtherPhoneType::Second => "Second",

            MemberOtherPhoneType::SMS => "SMS",

            MemberOtherPhoneType::Third => "Third",

            MemberOtherPhoneType::TollFree => "Toll Free",

            MemberOtherPhoneType::Voicemail => "Voicemail",

            MemberOtherPhoneType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for MemberOtherPhoneType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for MemberOtherPhoneType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_member_other_phone_type_format {
    use super::MemberOtherPhoneType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<MemberOtherPhoneType>>,
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
    ) -> Result<Option<Vec<MemberOtherPhoneType>>, D::Error>
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
