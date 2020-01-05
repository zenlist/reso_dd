// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [BodyType Lookups](https://ddwiki.reso.org/display/DDW17/BodyType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BodyType {
    /// "[Double Wide](https://ddwiki.reso.org/display/DDW17/Double+Wide)": The body/structure type of the mobile/manufacture home is double wide.
    DoubleWide,

    /// "[Expando](https://ddwiki.reso.org/display/DDW17/Expando)": The body/structure type of the mobile/manufacture home is Expando.
    Expando,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243847)": A body type not included in this list.
    Other,

    /// "[Quad Wide](https://ddwiki.reso.org/display/DDW17/Quad+Wide)": The body/structure type of the mobile/manufacture home is quad wide.
    QuadWide,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243844)": The body/structure type of the mobile/manufacture home is see remarks.
    SeeRemarks,

    /// "[Single Wide](https://ddwiki.reso.org/display/DDW17/Single+Wide)": The body/structure type of the mobile/manufacture home is single wide.
    SingleWide,

    /// "[Triple Wide](https://ddwiki.reso.org/display/DDW17/Triple+Wide)": The body/structure type of the mobile/manufacture home is triple wide.
    TripleWide,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for BodyType {
    fn from_str(s: &str) -> BodyType {
        match s {
            "Double Wide" => BodyType::DoubleWide,

            "Expando" => BodyType::Expando,

            "Other" => BodyType::Other,

            "Quad Wide" => BodyType::QuadWide,

            "See Remarks" => BodyType::SeeRemarks,

            "Single Wide" => BodyType::SingleWide,

            "Triple Wide" => BodyType::TripleWide,

            _ => BodyType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> BodyType {
        match s.as_ref() {
            "Double Wide" => BodyType::DoubleWide,

            "Expando" => BodyType::Expando,

            "Other" => BodyType::Other,

            "Quad Wide" => BodyType::QuadWide,

            "See Remarks" => BodyType::SeeRemarks,

            "Single Wide" => BodyType::SingleWide,

            "Triple Wide" => BodyType::TripleWide,

            _ => BodyType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            BodyType::DoubleWide => "Double Wide",

            BodyType::Expando => "Expando",

            BodyType::Other => "Other",

            BodyType::QuadWide => "Quad Wide",

            BodyType::SeeRemarks => "See Remarks",

            BodyType::SingleWide => "Single Wide",

            BodyType::TripleWide => "Triple Wide",

            BodyType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            BodyType::DoubleWide => "Double Wide".into(),

            BodyType::Expando => "Expando".into(),

            BodyType::Other => "Other".into(),

            BodyType::QuadWide => "Quad Wide".into(),

            BodyType::SeeRemarks => "See Remarks".into(),

            BodyType::SingleWide => "Single Wide".into(),

            BodyType::TripleWide => "Triple Wide".into(),

            BodyType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            BodyType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for BodyType {
    fn from(s: String) -> BodyType {
        match s.as_ref() {
            "Double Wide" => BodyType::DoubleWide,

            "Expando" => BodyType::Expando,

            "Other" => BodyType::Other,

            "Quad Wide" => BodyType::QuadWide,

            "See Remarks" => BodyType::SeeRemarks,

            "Single Wide" => BodyType::SingleWide,

            "Triple Wide" => BodyType::TripleWide,

            _ => BodyType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for BodyType {
    fn from(s: &str) -> BodyType {
        match s {
            "Double Wide" => BodyType::DoubleWide,

            "Expando" => BodyType::Expando,

            "Other" => BodyType::Other,

            "Quad Wide" => BodyType::QuadWide,

            "See Remarks" => BodyType::SeeRemarks,

            "Single Wide" => BodyType::SingleWide,

            "Triple Wide" => BodyType::TripleWide,

            _ => BodyType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a BodyType> for &'a str {
    fn from(s: &'a BodyType) -> &'a str {
        match s {
            BodyType::DoubleWide => "Double Wide",

            BodyType::Expando => "Expando",

            BodyType::Other => "Other",

            BodyType::QuadWide => "Quad Wide",

            BodyType::SeeRemarks => "See Remarks",

            BodyType::SingleWide => "Single Wide",

            BodyType::TripleWide => "Triple Wide",

            BodyType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for BodyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for BodyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
