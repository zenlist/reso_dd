// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [PatioAndPorchFeatures Lookups](https://ddwiki.reso.org/display/DDW17/PatioAndPorchFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PatioAndPorchFeatures {
    /// "[Awning(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246038)": The property has awning(s).
    Awnings,

    /// "[Covered](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246039)": The property has a covered patio or porch.
    Covered,

    /// "[Deck](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246040)": The property has a deck.
    Deck,

    /// "[Enclosed](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246041)": The property has an enclosed patio or porch.
    Enclosed,

    /// "[Front Porch](https://ddwiki.reso.org/display/DDW17/Front+Porch)": The property has a front porch.
    FrontPorch,

    /// "[Glass Enclosed](https://ddwiki.reso.org/display/DDW17/Glass+Enclosed)": The property has a glass enclosed patio or porch.
    GlassEnclosed,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246044)": The property has no patio or porch.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246045)": The property has a patio or porch feature other than what's included in this list.
    Other,

    /// "[Patio](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246046)": The property has a patio.
    Patio,

    /// "[Porch](https://ddwiki.reso.org/display/DDW17/Porch)": The property has a porch.
    Porch,

    /// "[Rear Porch](https://ddwiki.reso.org/display/DDW17/Rear+Porch)": The property has a rear porch.
    RearPorch,

    /// "[Screened](https://ddwiki.reso.org/display/DDW17/Screened)": The property has screened patio or porch.
    Screened,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246050)": See the remarks fields for more information on the patio or porch features of the property.
    SeeRemarks,

    /// "[Side Porch](https://ddwiki.reso.org/display/DDW17/Side+Porch)": The property has a side porch.
    SidePorch,

    /// "[Terrace](https://ddwiki.reso.org/display/DDW17/Terrace)": The property has a terrace.
    Terrace,

    /// "[Wrap Around](https://ddwiki.reso.org/display/DDW17/Wrap+Around)": The property has wrap around patio or porch.
    WrapAround,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for PatioAndPorchFeatures {
    fn from_str(s: &str) -> PatioAndPorchFeatures {
        match s {
            "Awning(s)" => PatioAndPorchFeatures::Awnings,

            "Covered" => PatioAndPorchFeatures::Covered,

            "Deck" => PatioAndPorchFeatures::Deck,

            "Enclosed" => PatioAndPorchFeatures::Enclosed,

            "Front Porch" => PatioAndPorchFeatures::FrontPorch,

            "Glass Enclosed" => PatioAndPorchFeatures::GlassEnclosed,

            "None" => PatioAndPorchFeatures::None,

            "Other" => PatioAndPorchFeatures::Other,

            "Patio" => PatioAndPorchFeatures::Patio,

            "Porch" => PatioAndPorchFeatures::Porch,

            "Rear Porch" => PatioAndPorchFeatures::RearPorch,

            "Screened" => PatioAndPorchFeatures::Screened,

            "See Remarks" => PatioAndPorchFeatures::SeeRemarks,

            "Side Porch" => PatioAndPorchFeatures::SidePorch,

            "Terrace" => PatioAndPorchFeatures::Terrace,

            "Wrap Around" => PatioAndPorchFeatures::WrapAround,

            _ => PatioAndPorchFeatures::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> PatioAndPorchFeatures {
        match s.as_ref() {
            "Awning(s)" => PatioAndPorchFeatures::Awnings,

            "Covered" => PatioAndPorchFeatures::Covered,

            "Deck" => PatioAndPorchFeatures::Deck,

            "Enclosed" => PatioAndPorchFeatures::Enclosed,

            "Front Porch" => PatioAndPorchFeatures::FrontPorch,

            "Glass Enclosed" => PatioAndPorchFeatures::GlassEnclosed,

            "None" => PatioAndPorchFeatures::None,

            "Other" => PatioAndPorchFeatures::Other,

            "Patio" => PatioAndPorchFeatures::Patio,

            "Porch" => PatioAndPorchFeatures::Porch,

            "Rear Porch" => PatioAndPorchFeatures::RearPorch,

            "Screened" => PatioAndPorchFeatures::Screened,

            "See Remarks" => PatioAndPorchFeatures::SeeRemarks,

            "Side Porch" => PatioAndPorchFeatures::SidePorch,

            "Terrace" => PatioAndPorchFeatures::Terrace,

            "Wrap Around" => PatioAndPorchFeatures::WrapAround,

            _ => PatioAndPorchFeatures::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            PatioAndPorchFeatures::Awnings => "Awning(s)",

            PatioAndPorchFeatures::Covered => "Covered",

            PatioAndPorchFeatures::Deck => "Deck",

            PatioAndPorchFeatures::Enclosed => "Enclosed",

            PatioAndPorchFeatures::FrontPorch => "Front Porch",

            PatioAndPorchFeatures::GlassEnclosed => "Glass Enclosed",

            PatioAndPorchFeatures::None => "None",

            PatioAndPorchFeatures::Other => "Other",

            PatioAndPorchFeatures::Patio => "Patio",

            PatioAndPorchFeatures::Porch => "Porch",

            PatioAndPorchFeatures::RearPorch => "Rear Porch",

            PatioAndPorchFeatures::Screened => "Screened",

            PatioAndPorchFeatures::SeeRemarks => "See Remarks",

            PatioAndPorchFeatures::SidePorch => "Side Porch",

            PatioAndPorchFeatures::Terrace => "Terrace",

            PatioAndPorchFeatures::WrapAround => "Wrap Around",

            PatioAndPorchFeatures::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            PatioAndPorchFeatures::Awnings => "Awning(s)".into(),

            PatioAndPorchFeatures::Covered => "Covered".into(),

            PatioAndPorchFeatures::Deck => "Deck".into(),

            PatioAndPorchFeatures::Enclosed => "Enclosed".into(),

            PatioAndPorchFeatures::FrontPorch => "Front Porch".into(),

            PatioAndPorchFeatures::GlassEnclosed => "Glass Enclosed".into(),

            PatioAndPorchFeatures::None => "None".into(),

            PatioAndPorchFeatures::Other => "Other".into(),

            PatioAndPorchFeatures::Patio => "Patio".into(),

            PatioAndPorchFeatures::Porch => "Porch".into(),

            PatioAndPorchFeatures::RearPorch => "Rear Porch".into(),

            PatioAndPorchFeatures::Screened => "Screened".into(),

            PatioAndPorchFeatures::SeeRemarks => "See Remarks".into(),

            PatioAndPorchFeatures::SidePorch => "Side Porch".into(),

            PatioAndPorchFeatures::Terrace => "Terrace".into(),

            PatioAndPorchFeatures::WrapAround => "Wrap Around".into(),

            PatioAndPorchFeatures::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            PatioAndPorchFeatures::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for PatioAndPorchFeatures {
    fn from(s: String) -> PatioAndPorchFeatures {
        match s.as_ref() {
            "Awning(s)" => PatioAndPorchFeatures::Awnings,

            "Covered" => PatioAndPorchFeatures::Covered,

            "Deck" => PatioAndPorchFeatures::Deck,

            "Enclosed" => PatioAndPorchFeatures::Enclosed,

            "Front Porch" => PatioAndPorchFeatures::FrontPorch,

            "Glass Enclosed" => PatioAndPorchFeatures::GlassEnclosed,

            "None" => PatioAndPorchFeatures::None,

            "Other" => PatioAndPorchFeatures::Other,

            "Patio" => PatioAndPorchFeatures::Patio,

            "Porch" => PatioAndPorchFeatures::Porch,

            "Rear Porch" => PatioAndPorchFeatures::RearPorch,

            "Screened" => PatioAndPorchFeatures::Screened,

            "See Remarks" => PatioAndPorchFeatures::SeeRemarks,

            "Side Porch" => PatioAndPorchFeatures::SidePorch,

            "Terrace" => PatioAndPorchFeatures::Terrace,

            "Wrap Around" => PatioAndPorchFeatures::WrapAround,

            _ => PatioAndPorchFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for PatioAndPorchFeatures {
    fn from(s: &str) -> PatioAndPorchFeatures {
        match s {
            "Awning(s)" => PatioAndPorchFeatures::Awnings,

            "Covered" => PatioAndPorchFeatures::Covered,

            "Deck" => PatioAndPorchFeatures::Deck,

            "Enclosed" => PatioAndPorchFeatures::Enclosed,

            "Front Porch" => PatioAndPorchFeatures::FrontPorch,

            "Glass Enclosed" => PatioAndPorchFeatures::GlassEnclosed,

            "None" => PatioAndPorchFeatures::None,

            "Other" => PatioAndPorchFeatures::Other,

            "Patio" => PatioAndPorchFeatures::Patio,

            "Porch" => PatioAndPorchFeatures::Porch,

            "Rear Porch" => PatioAndPorchFeatures::RearPorch,

            "Screened" => PatioAndPorchFeatures::Screened,

            "See Remarks" => PatioAndPorchFeatures::SeeRemarks,

            "Side Porch" => PatioAndPorchFeatures::SidePorch,

            "Terrace" => PatioAndPorchFeatures::Terrace,

            "Wrap Around" => PatioAndPorchFeatures::WrapAround,

            _ => PatioAndPorchFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a PatioAndPorchFeatures> for &'a str {
    fn from(s: &'a PatioAndPorchFeatures) -> &'a str {
        match s {
            PatioAndPorchFeatures::Awnings => "Awning(s)",

            PatioAndPorchFeatures::Covered => "Covered",

            PatioAndPorchFeatures::Deck => "Deck",

            PatioAndPorchFeatures::Enclosed => "Enclosed",

            PatioAndPorchFeatures::FrontPorch => "Front Porch",

            PatioAndPorchFeatures::GlassEnclosed => "Glass Enclosed",

            PatioAndPorchFeatures::None => "None",

            PatioAndPorchFeatures::Other => "Other",

            PatioAndPorchFeatures::Patio => "Patio",

            PatioAndPorchFeatures::Porch => "Porch",

            PatioAndPorchFeatures::RearPorch => "Rear Porch",

            PatioAndPorchFeatures::Screened => "Screened",

            PatioAndPorchFeatures::SeeRemarks => "See Remarks",

            PatioAndPorchFeatures::SidePorch => "Side Porch",

            PatioAndPorchFeatures::Terrace => "Terrace",

            PatioAndPorchFeatures::WrapAround => "Wrap Around",

            PatioAndPorchFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for PatioAndPorchFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PatioAndPorchFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
