// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Permission Lookups](https://ddwiki.reso.org/display/DDW17/Permission+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Permission {
    /// "[Agent Only](https://ddwiki.reso.org/display/DDW17/Agent+Only)": The image or document is for agent use only.
    AgentOnly,

    /// "[Firm Only](https://ddwiki.reso.org/display/DDW17/Firm+Only)": The image or document is for firm use only.
    FirmOnly,

    /// "[IDX](https://ddwiki.reso.org/display/DDW17/IDX)": The image or document is okay for IDX use.
    IDX,

    /// "[Office Only](https://ddwiki.reso.org/display/DDW17/Office+Only)": The image or document is for office use only.
    OfficeOnly,

    /// "[Private](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245915)": The image or document is private and should have limited distribution.
    Private,

    /// "[Public](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245916)": The image or document may be viewed by the public.
    Public,

    /// "[VOW](https://ddwiki.reso.org/display/DDW17/VOW)": The image or document is okay for VOW use.
    VOW,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for Permission {
    fn from_str(s: &str) -> Permission {
        match s {
            "Agent Only" => Permission::AgentOnly,

            "Firm Only" => Permission::FirmOnly,

            "IDX" => Permission::IDX,

            "Office Only" => Permission::OfficeOnly,

            "Private" => Permission::Private,

            "Public" => Permission::Public,

            "VOW" => Permission::VOW,

            _ => Permission::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> Permission {
        match s.as_ref() {
            "Agent Only" => Permission::AgentOnly,

            "Firm Only" => Permission::FirmOnly,

            "IDX" => Permission::IDX,

            "Office Only" => Permission::OfficeOnly,

            "Private" => Permission::Private,

            "Public" => Permission::Public,

            "VOW" => Permission::VOW,

            _ => Permission::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Permission::AgentOnly => "Agent Only",

            Permission::FirmOnly => "Firm Only",

            Permission::IDX => "IDX",

            Permission::OfficeOnly => "Office Only",

            Permission::Private => "Private",

            Permission::Public => "Public",

            Permission::VOW => "VOW",

            Permission::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Permission::AgentOnly => "Agent Only".into(),

            Permission::FirmOnly => "Firm Only".into(),

            Permission::IDX => "IDX".into(),

            Permission::OfficeOnly => "Office Only".into(),

            Permission::Private => "Private".into(),

            Permission::Public => "Public".into(),

            Permission::VOW => "VOW".into(),

            Permission::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Permission::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for Permission {
    fn from(s: String) -> Permission {
        match s.as_ref() {
            "Agent Only" => Permission::AgentOnly,

            "Firm Only" => Permission::FirmOnly,

            "IDX" => Permission::IDX,

            "Office Only" => Permission::OfficeOnly,

            "Private" => Permission::Private,

            "Public" => Permission::Public,

            "VOW" => Permission::VOW,

            _ => Permission::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Permission {
    fn from(s: &str) -> Permission {
        match s {
            "Agent Only" => Permission::AgentOnly,

            "Firm Only" => Permission::FirmOnly,

            "IDX" => Permission::IDX,

            "Office Only" => Permission::OfficeOnly,

            "Private" => Permission::Private,

            "Public" => Permission::Public,

            "VOW" => Permission::VOW,

            _ => Permission::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Permission> for &'a str {
    fn from(s: &'a Permission) -> &'a str {
        match s {
            Permission::AgentOnly => "Agent Only",

            Permission::FirmOnly => "Firm Only",

            Permission::IDX => "IDX",

            Permission::OfficeOnly => "Office Only",

            Permission::Private => "Private",

            Permission::Public => "Public",

            Permission::VOW => "VOW",

            Permission::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Permission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
