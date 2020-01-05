// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [SearchQueryType Lookups](https://ddwiki.reso.org/display/DDW17/SearchQueryType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SearchQueryType {
    /// "[$filter](https://ddwiki.reso.org/display/DDW17/%24filter)": The query is in the form of Odata's $filter.
    Filter,

    /// "[DMQL2](https://ddwiki.reso.org/display/DDW17/DMQL2)": The query is in the form of DMQL2.
    DMQL2,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for SearchQueryType {
    fn from_str(s: &str) -> SearchQueryType {
        match s {
            "$filter" => SearchQueryType::Filter,

            "DMQL2" => SearchQueryType::DMQL2,

            _ => SearchQueryType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> SearchQueryType {
        match s.as_ref() {
            "$filter" => SearchQueryType::Filter,

            "DMQL2" => SearchQueryType::DMQL2,

            _ => SearchQueryType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            SearchQueryType::Filter => "$filter",

            SearchQueryType::DMQL2 => "DMQL2",

            SearchQueryType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            SearchQueryType::Filter => "$filter".into(),

            SearchQueryType::DMQL2 => "DMQL2".into(),

            SearchQueryType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            SearchQueryType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for SearchQueryType {
    fn from(s: String) -> SearchQueryType {
        match s.as_ref() {
            "$filter" => SearchQueryType::Filter,

            "DMQL2" => SearchQueryType::DMQL2,

            _ => SearchQueryType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for SearchQueryType {
    fn from(s: &str) -> SearchQueryType {
        match s {
            "$filter" => SearchQueryType::Filter,

            "DMQL2" => SearchQueryType::DMQL2,

            _ => SearchQueryType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a SearchQueryType> for &'a str {
    fn from(s: &'a SearchQueryType) -> &'a str {
        match s {
            SearchQueryType::Filter => "$filter",

            SearchQueryType::DMQL2 => "DMQL2",

            SearchQueryType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for SearchQueryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SearchQueryType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
