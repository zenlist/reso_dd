// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [SpaFeatures Lookups](https://ddwiki.reso.org/display/DDW17/SpaFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SpaFeatures {
    /// "[Above Ground](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246509)": The spa is not built into the ground.
    AboveGround,

    /// "[Bath](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246510)": The bath has a built in spa/jets.
    Bath,

    /// "[Community](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246511)": The property has access to a community spa.
    Community,

    /// "[Fiberglass](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246512)": The spa is lined or made of fiberglass.
    Fiberglass,

    /// "[Gunite](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246513)": The spa is lined with gunite.
    Gunite,

    /// "[Heated](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246514)": The spa is heated.
    Heated,

    /// "[In Ground](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246515)": The spa is built into the ground.
    InGround,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246516)": The property has no spa.
    None,

    /// "[Private](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246517)": The spa is privately owned or is secluded.
    Private,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246518)": See the remarks fields for more information about the spa.
    SeeRemarks,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for SpaFeatures {
    fn from_str(s: &str) -> SpaFeatures {
        match s {
            "Above Ground" => SpaFeatures::AboveGround,

            "Bath" => SpaFeatures::Bath,

            "Community" => SpaFeatures::Community,

            "Fiberglass" => SpaFeatures::Fiberglass,

            "Gunite" => SpaFeatures::Gunite,

            "Heated" => SpaFeatures::Heated,

            "In Ground" => SpaFeatures::InGround,

            "None" => SpaFeatures::None,

            "Private" => SpaFeatures::Private,

            "See Remarks" => SpaFeatures::SeeRemarks,

            _ => SpaFeatures::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> SpaFeatures {
        match s.as_ref() {
            "Above Ground" => SpaFeatures::AboveGround,

            "Bath" => SpaFeatures::Bath,

            "Community" => SpaFeatures::Community,

            "Fiberglass" => SpaFeatures::Fiberglass,

            "Gunite" => SpaFeatures::Gunite,

            "Heated" => SpaFeatures::Heated,

            "In Ground" => SpaFeatures::InGround,

            "None" => SpaFeatures::None,

            "Private" => SpaFeatures::Private,

            "See Remarks" => SpaFeatures::SeeRemarks,

            _ => SpaFeatures::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            SpaFeatures::AboveGround => "Above Ground",

            SpaFeatures::Bath => "Bath",

            SpaFeatures::Community => "Community",

            SpaFeatures::Fiberglass => "Fiberglass",

            SpaFeatures::Gunite => "Gunite",

            SpaFeatures::Heated => "Heated",

            SpaFeatures::InGround => "In Ground",

            SpaFeatures::None => "None",

            SpaFeatures::Private => "Private",

            SpaFeatures::SeeRemarks => "See Remarks",

            SpaFeatures::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            SpaFeatures::AboveGround => "Above Ground".into(),

            SpaFeatures::Bath => "Bath".into(),

            SpaFeatures::Community => "Community".into(),

            SpaFeatures::Fiberglass => "Fiberglass".into(),

            SpaFeatures::Gunite => "Gunite".into(),

            SpaFeatures::Heated => "Heated".into(),

            SpaFeatures::InGround => "In Ground".into(),

            SpaFeatures::None => "None".into(),

            SpaFeatures::Private => "Private".into(),

            SpaFeatures::SeeRemarks => "See Remarks".into(),

            SpaFeatures::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            SpaFeatures::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for SpaFeatures {
    fn from(s: String) -> SpaFeatures {
        match s.as_ref() {
            "Above Ground" => SpaFeatures::AboveGround,

            "Bath" => SpaFeatures::Bath,

            "Community" => SpaFeatures::Community,

            "Fiberglass" => SpaFeatures::Fiberglass,

            "Gunite" => SpaFeatures::Gunite,

            "Heated" => SpaFeatures::Heated,

            "In Ground" => SpaFeatures::InGround,

            "None" => SpaFeatures::None,

            "Private" => SpaFeatures::Private,

            "See Remarks" => SpaFeatures::SeeRemarks,

            _ => SpaFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for SpaFeatures {
    fn from(s: &str) -> SpaFeatures {
        match s {
            "Above Ground" => SpaFeatures::AboveGround,

            "Bath" => SpaFeatures::Bath,

            "Community" => SpaFeatures::Community,

            "Fiberglass" => SpaFeatures::Fiberglass,

            "Gunite" => SpaFeatures::Gunite,

            "Heated" => SpaFeatures::Heated,

            "In Ground" => SpaFeatures::InGround,

            "None" => SpaFeatures::None,

            "Private" => SpaFeatures::Private,

            "See Remarks" => SpaFeatures::SeeRemarks,

            _ => SpaFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a SpaFeatures> for &'a str {
    fn from(s: &'a SpaFeatures) -> &'a str {
        match s {
            SpaFeatures::AboveGround => "Above Ground",

            SpaFeatures::Bath => "Bath",

            SpaFeatures::Community => "Community",

            SpaFeatures::Fiberglass => "Fiberglass",

            SpaFeatures::Gunite => "Gunite",

            SpaFeatures::Heated => "Heated",

            SpaFeatures::InGround => "In Ground",

            SpaFeatures::None => "None",

            SpaFeatures::Private => "Private",

            SpaFeatures::SeeRemarks => "See Remarks",

            SpaFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for SpaFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SpaFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
