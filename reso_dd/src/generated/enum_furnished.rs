// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Furnished Lookups](https://ddwiki.reso.org/display/DDW17/Furnished+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Furnished {
    /// "[Furnished](https://ddwiki.reso.org/display/DDW17/Furnished)": The dwelling being leased is furnished.
    Furnished,

    /// "[Negotiable](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244748)": The property may be furnished or left unfurnished at the lessor's request.  Contact the listing agent/office to discuss options and cost differences.
    Negotiable,

    /// "[Partially](https://ddwiki.reso.org/display/DDW17/Partially)": The dwelling being leased is partially furnished.
    Partially,

    /// "[Unfurnished](https://ddwiki.reso.org/display/DDW17/Unfurnished)": The dwelling being leased is not furnished.
    Unfurnished,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for Furnished {
    fn from_str(s: &str) -> Furnished {
        match s {
            "Furnished" => Furnished::Furnished,

            "Negotiable" => Furnished::Negotiable,

            "Partially" => Furnished::Partially,

            "Unfurnished" => Furnished::Unfurnished,

            _ => Furnished::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> Furnished {
        match s.as_ref() {
            "Furnished" => Furnished::Furnished,

            "Negotiable" => Furnished::Negotiable,

            "Partially" => Furnished::Partially,

            "Unfurnished" => Furnished::Unfurnished,

            _ => Furnished::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Furnished::Furnished => "Furnished",

            Furnished::Negotiable => "Negotiable",

            Furnished::Partially => "Partially",

            Furnished::Unfurnished => "Unfurnished",

            Furnished::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Furnished::Furnished => "Furnished".into(),

            Furnished::Negotiable => "Negotiable".into(),

            Furnished::Partially => "Partially".into(),

            Furnished::Unfurnished => "Unfurnished".into(),

            Furnished::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Furnished::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for Furnished {
    fn from(s: String) -> Furnished {
        match s.as_ref() {
            "Furnished" => Furnished::Furnished,

            "Negotiable" => Furnished::Negotiable,

            "Partially" => Furnished::Partially,

            "Unfurnished" => Furnished::Unfurnished,

            _ => Furnished::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Furnished {
    fn from(s: &str) -> Furnished {
        match s {
            "Furnished" => Furnished::Furnished,

            "Negotiable" => Furnished::Negotiable,

            "Partially" => Furnished::Partially,

            "Unfurnished" => Furnished::Unfurnished,

            _ => Furnished::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Furnished> for &'a str {
    fn from(s: &'a Furnished) -> &'a str {
        match s {
            Furnished::Furnished => "Furnished",

            Furnished::Negotiable => "Negotiable",

            Furnished::Partially => "Partially",

            Furnished::Unfurnished => "Unfurnished",

            Furnished::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Furnished {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Furnished {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
